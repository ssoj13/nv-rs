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
    DeviceContext, GeometryFlags, Guides, HitGroup, Image, Instance, Layer, LinkOptions, Module,
    ModuleOptions, Params as DenoiserParams, Pipeline, PipelineOptions, ProgramGroup, SbtRecord,
    Scratch, ShaderBindingTable, TriangleMesh,
};

use crate::fft_bloom::FftBloomPass;
use crate::gpu_export::{VideoRecorder, save_jpeg_from_rgba8};

const RAYTRACER_CU: &str = include_str!("../shaders/raytracer.cu");
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
    _pad0: [u32; 3],
    out_motion_f32: u64,
    out_depth_f32: u64,
    prev_cam_pos: [f32; 4],
    prev_cam_forward: [f32; 4],
    prev_cam_right: [f32; 4],
    prev_cam_up: [f32; 4],
    sat_pos: [f32; 4],
    glass_params: [f32; 4],
    aov_mode: u32,
    _pad1: [u32; 3],
}

const _: () = assert!(std::mem::size_of::<LaunchParams>() == 432);

#[derive(Clone, Copy, PartialEq)]
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
    pub shadows_on: bool,
    pub auto_rotate: bool,
    pub spin_speed: f32,
    // Multi-instance Scene (Glass Satellite + Neon Accent)
    pub sat_on: bool,
    pub sat_orbit: bool,
    pub sat_radius: f32,
    pub sat_orbit_radius: f32,
    pub sat_height: f32,
    pub glass_ior: f32,
    pub glass_absorb: f32,
    pub glass_roughness: f32,
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
            bulb_iter: 12,
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
            box_iter: 12,
            box_bailout: 16.0,
            box_fold_mult: 2.0,
            box_iter_rot: 0.0,
            kifs_scale: 2.0,
            kifs_rot1: 0.0,
            kifs_rot2: 0.0,
            kifs_offset: [1.0, 1.0, 1.0],
            kifs_iter: 12,
            kifs_bailout: 12.0,
            kifs_thickness: 0.05,
            kifs_z_bias: 0.5,
            detail_eps: 0.0005,
            fudge: 0.95,
            norm_eps: 0.0006,
            march_steps: 128,
            bounces: 3,
            metallic: 0.45,
            trap_scale: 0.85,
            palette_shift: 0.0,
            sun_intensity: 1.45,
            sun_azimuth: 32.0,
            sun_elevation: 48.0,
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
            shadows_on: true,
            auto_rotate: false,
            spin_speed: 0.045,
            sat_on: true,
            sat_orbit: true,
            sat_radius: 0.36,
            sat_orbit_radius: 1.65,
            sat_height: 0.25,
            glass_ior: 1.52,
            glass_absorb: 0.45,
            glass_roughness: 0.02,
            neon_on: true,
            bloom_on: false,
            bloom_threshold: 1.25,
            bloom_intensity: 0.35,
            bloom_blades: 6,
            split_on: false,
            split_x: 0.5,
            right_is_aov: 0,
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
    },
    ReloadShader,
    Shutdown,
}

pub struct FrameReady {
    pub buf_idx: usize,
    pub gen_id: u64,
    pub accum_count: u32,
    pub total_samples: u32,
    pub rt_ms: f32,
    pub denoise_ms: f32,
    pub bloom_ms: f32,
    pub tonemap_ms: f32,
    pub is_recording: bool,
    pub record_frames: u64,
    pub status_msg: Option<String>,
}

struct TonemapKernel {
    module: cu::CUmodule,
    func: cu::CUfunction,
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
    glass_gas: Arc<Accel>,
    neon_gas: Arc<Accel>,
    ias: Arc<Accel>,
    pipeline: Pipeline,
    sbt: ShaderBindingTable,
    denoiser: Denoiser,
    denoiser_is_temporal: bool,
    temporal_has_prev: bool,
    color_f32: DeviceBuffer<[f32; 4]>,
    albedo_f32: DeviceBuffer<[f32; 4]>,
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
            model: DenoiserModel::Hdr,
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

struct SceneGas<'a> {
    floor: &'a Arc<Accel>,
    fractal: &'a Arc<Accel>,
    glass: &'a Arc<Accel>,
    neon: &'a Arc<Accel>,
}

struct SceneState {
    sat_pos: [f32; 3],
    sat_radius: f32,
    sat_on: bool,
    neon_on: bool,
}

fn build_ias(
    optix_ctx: &Arc<DeviceContext>,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    gas: SceneGas<'_>,
    state: SceneState,
) -> Arc<Accel> {
    let mut instances = Vec::with_capacity(4);
    let mut i0 = Instance::new(gas.floor);
    i0.id = 0;
    i0.sbt_offset = 0;
    instances.push(i0);

    let mut i1 = Instance::new(gas.fractal);
    i1.id = 1;
    i1.sbt_offset = 1;
    instances.push(i1);

    if state.sat_on {
        let r = state.sat_radius.max(0.05);
        let mut i2 = Instance::new(gas.glass);
        i2.id = 2;
        i2.sbt_offset = 0;
        i2.transform = [
            r,
            0.0,
            0.0,
            state.sat_pos[0],
            0.0,
            r,
            0.0,
            state.sat_pos[1],
            0.0,
            0.0,
            r,
            state.sat_pos[2],
        ];
        instances.push(i2);
    }

    if state.neon_on {
        let mut i3 = Instance::new(gas.neon);
        i3.id = 3;
        i3.sbt_offset = 0;
        i3.transform = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.24, 0.0, 0.0, 1.0, 0.0];
        instances.push(i3);
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

fn init_optix(cuda: &Arc<CudaContext>, initial_settings: &DemoSettings) -> OptixRig {
    let stream = cuda.default_stream();
    let optix_ctx = DeviceContext::new(cuda, None, 0, false).expect("OptiX DeviceContext");
    let mut scratch = Scratch::new(&stream);

    let rt_ptx = try_compile_ptx(RAYTRACER_CU, "raytracer.cu", true).expect("raytracer.cu PTX");
    let tm_ptx = try_compile_ptx(TONEMAP_CU, "tonemap.cu", false).expect("tonemap.cu PTX");
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
    let tonemap = TonemapKernel {
        module: cu_mod,
        func: cu_func,
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
        tri_build_opts.clone(),
    )
    .expect("floor GAS");

    // Fractal AABB custom primitive [-1.5, 1.5]^3
    let aabb_buf = DeviceBuffer::from_host(
        &stream,
        &[Aabb {
            min: [-1.5, -1.5, -1.5],
            max: [1.5, 1.5, 1.5],
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

    // Glass satellite sphere (unit sphere, transformed via IAS)
    let sphere_tris = build_uv_sphere(24, 36);
    let sphere_verts = DeviceBuffer::from_host(&stream, &sphere_tris).expect("sphere verts");
    let glass_gas = Accel::triangles(
        &optix_ctx,
        &stream,
        &mut scratch,
        &[TriangleMesh {
            flags: &[GeometryFlags::DISABLE_ANYHIT],
            ..TriangleMesh::new(&sphere_verts)
        }],
        tri_build_opts.clone(),
    )
    .expect("glass GAS");

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

    let sat_pos = [
        initial_settings.sat_orbit_radius,
        initial_settings.sat_height,
        0.0,
    ];
    let ias = build_ias(
        &optix_ctx,
        &stream,
        &mut scratch,
        SceneGas {
            floor: &floor_gas,
            fractal: &fractal_gas,
            glass: &glass_gas,
            neon: &neon_gas,
        },
        SceneState {
            sat_pos,
            sat_radius: initial_settings.sat_radius,
            sat_on: initial_settings.sat_on,
            neon_on: initial_settings.neon_on,
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
        glass_gas,
        neon_gas,
        ias,
        pipeline,
        sbt,
        denoiser,
        denoiser_is_temporal,
        temporal_has_prev: false,
        color_f32,
        albedo_f32,
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

pub fn spawn_render_thread(
    cuda: Arc<CudaContext>,
    initial_buf_ptrs: [u64; 2],
    initial_settings: DemoSettings,
    initial_camera: CameraFrame,
) -> (Sender<RenderCmd>, Receiver<FrameReady>, JoinHandle<()>) {
    let (cmd_tx, cmd_rx) = mpsc::channel::<RenderCmd>();
    let (frame_tx, frame_rx) = mpsc::channel::<FrameReady>();

    let handle = thread::Builder::new()
        .name("optix-render".to_string())
        .spawn(move || {
            cuda.bind_to_thread()
                .expect("cuda bind_to_thread on render thread");
            let mut rig = init_optix(&cuda, &initial_settings);
            let mut cur_w = DEFAULT_W;
            let mut cur_h = DEFAULT_H;
            let mut cur_gen: u64 = 0;
            let mut buf_ptrs = initial_buf_ptrs;
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
                settings.neon_on,
                [
                    settings.sat_orbit_radius,
                    settings.sat_height,
                    0.0,
                    settings.sat_radius,
                ],
            );
            let mut last_tick = Instant::now();
            let mut pending_jpeg: Option<(PathBuf, u8)> = None;
            let mut recorder: Option<VideoRecorder> = None;
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
                                 free_bufs: &mut VecDeque<usize>,
                                 settings: &mut DemoSettings,
                                 camera: &mut CameraFrame,
                                 accum_count: &mut u32,
                                 pending_jpeg: &mut Option<(PathBuf, u8)>,
                                 recorder: &mut Option<VideoRecorder>,
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
                            gen_id,
                        } => {
                            if let Some(rec) = recorder.take() {
                                let _ = rec.finish();
                                *pending_status = Some("Recording stopped on resize".to_string());
                            }
                            if width != *cur_w || height != *cur_h {
                                resize_rig(rig, width, height, settings.temporal_denoise);
                                *cur_w = width;
                                *cur_h = height;
                            }
                            *buf_ptrs = new_ptrs;
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
                        RenderCmd::ToggleRecord { path, fps } => {
                            if let Some(rec) = recorder.take() {
                                let frames = rec.frame_count();
                                *pending_status = Some(match rec.finish() {
                                    Ok(n) => format!("Saved video ({n} frames)"),
                                    Err(e) => format!("Video finish failed ({frames}f): {e}"),
                                });
                            } else {
                                match VideoRecorder::start(cuda, *cur_w, *cur_h, fps, &path) {
                                    Ok(rec) => {
                                        *recorder = Some(rec);
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
                            if let Some(rec) = recorder.take() {
                                let _ = rec.finish();
                            }
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
                        &mut free_bufs,
                        &mut settings,
                        &mut camera,
                        &mut accum_count,
                        &mut pending_jpeg,
                        &mut recorder,
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
                    && !(settings.sat_on && settings.sat_orbit)
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
                                &mut free_bufs,
                                &mut settings,
                                &mut camera,
                                &mut accum_count,
                                &mut pending_jpeg,
                                &mut recorder,
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
                let dt = now.duration_since(last_tick).as_secs_f32().min(0.1);
                last_tick = now;
                if settings.auto_rotate {
                    spin_angle += dt * settings.spin_speed;
                    accum_count = 0;
                }
                if settings.sat_on && settings.sat_orbit {
                    sat_angle += dt * 0.65;
                    accum_count = 0;
                }

                let sat_xyz = [
                    settings.sat_orbit_radius * sat_angle.cos(),
                    settings.sat_height + 0.12 * (sat_angle * 2.0).sin(),
                    settings.sat_orbit_radius * sat_angle.sin(),
                ];
                let ias_state = (
                    settings.sat_on,
                    settings.neon_on,
                    [sat_xyz[0], sat_xyz[1], sat_xyz[2], settings.sat_radius],
                );
                if ias_state != last_ias_state {
                    rig.ias = build_ias(
                        &rig._optix_ctx,
                        &rig.stream,
                        &mut rig._scratch,
                        SceneGas {
                            floor: &rig.floor_gas,
                            fractal: &rig.fractal_gas,
                            glass: &rig.glass_gas,
                            neon: &rig.neon_gas,
                        },
                        SceneState {
                            sat_pos: sat_xyz,
                            sat_radius: settings.sat_radius,
                            sat_on: settings.sat_on,
                            neon_on: settings.neon_on,
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
                        settings.rot_y.to_radians(),
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
                    sky_params: [settings.sky_intensity, 1.0, settings.horizon_haze, 0.0],
                    accum_count,
                    _pad0: [0; 3],
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
                    sat_pos: [
                        sat_xyz[0],
                        sat_xyz[1],
                        sat_xyz[2],
                        if settings.sat_on {
                            settings.sat_radius
                        } else {
                            0.0
                        },
                    ],
                    glass_params: [
                        settings.glass_ior,
                        settings.glass_absorb,
                        settings.glass_roughness,
                        if settings.sat_on { 1.0 } else { 0.0 },
                    ],
                    aov_mode: settings.aov_mode,
                    _pad1: [0; 3],
                };
                prev_camera = camera;
                rig.params_buf
                    .copy_from_host(&rig.stream, bytemuck::bytes_of(&lp))
                    .expect("params upload");

                let t_rt = Instant::now();
                // SAFETY: `params_buf` matches `LaunchParams` layout; all output buffers outlive `synchronize`.
                unsafe {
                    rig.pipeline
                        .launch(&rig.stream, &rig.params_buf, &rig.sbt, [cur_w, cur_h, 1])
                }
                .expect("optix launch");
                rig.stream.synchronize().expect("rt sync");
                let rt_ms = t_rt.elapsed().as_secs_f32() * 1000.0;

                let (final_f32_ptr, denoise_ms) = if settings.denoiser_on && settings.aov_mode == 0
                {
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

                let bloom_ms = if settings.bloom_on && settings.aov_mode == 0 {
                    if let Some(ref mut bloom) = rig.bloom {
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
                };

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
                rig.stream.synchronize().expect("tonemap sync");
                let tonemap_ms = t_tm.elapsed().as_secs_f32() * 1000.0;

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

                if let Some(ref mut rec) = recorder
                    && let Err(e) = rec.push_frame(&rig.stream, buf_ptrs[buf_idx])
                {
                    pending_status = Some(format!("Video encode error: {e}"));
                    if let Some(failed) = recorder.take() {
                        let _ = failed.finish();
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
                        rt_ms,
                        denoise_ms,
                        bloom_ms,
                        tonemap_ms,
                        is_recording: recorder.is_some(),
                        record_frames: recorder.as_ref().map_or(0, VideoRecorder::frame_count),
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
