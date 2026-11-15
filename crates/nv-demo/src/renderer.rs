//! Asynchronous OptiX path-tracing worker thread communicating with the 60 FPS GUI via channels.

use std::collections::VecDeque;
use std::ffi::{CString, c_void};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};
use optix::{
    Aabb, Accel, AovType, BuildOptions, CustomPrimitives, Denoiser, DenoiserModel,
    DenoiserOptions, DeviceContext, GeometryFlags, Guides, HitGroup, Image, Instance, Layer,
    LinkOptions, Module, ModuleOptions, Params as DenoiserParams, Pipeline, PipelineOptions,
    ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh,
};

const RAYTRACER_CU: &str = include_str!("../shaders/raytracer.cu");
const TONEMAP_CU: &str = include_str!("../shaders/tonemap.cu");

pub const RENDER_W: u32 = 896;
pub const RENDER_H: u32 = 512;

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
}

#[derive(Clone, Copy, PartialEq)]
pub struct DemoSettings {
    pub fractal_kind: u32,
    // Mandelbulb
    pub bulb_power: f32,
    pub bulb_bailout: f32,
    pub bulb_iter: u32,
    pub bulb_julia: bool,
    pub bulb_c: [f32; 3],
    // Mandelbox
    pub box_scale: f32,
    pub box_fold: f32,
    pub box_min_r: f32,
    pub box_fixed_r: f32,
    pub box_iter: u32,
    pub box_bailout: f32,
    // KIFS Octahedron
    pub kifs_scale: f32,
    pub kifs_rot1: f32,
    pub kifs_rot2: f32,
    pub kifs_offset: [f32; 3],
    pub kifs_iter: u32,
    pub kifs_bailout: f32,
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
    // Sky & Floor
    pub sky_intensity: f32,
    pub horizon_haze: f32,
    pub floor_roughness: f32,
    pub floor_scratch: f32,
    pub floor_scale: f32,
    pub floor_metallic: f32,
    // Rendering
    pub roughness: f32,
    pub exposure: f32,
    pub spp: u32,
    pub max_accum: u32,
    pub denoiser_on: bool,
    pub shadows_on: bool,
    pub auto_rotate: bool,
    pub spin_speed: f32,
}

impl Default for DemoSettings {
    fn default() -> Self {
        Self {
            fractal_kind: 0,
            bulb_power: 8.0,
            bulb_bailout: 8.0,
            bulb_iter: 12,
            bulb_julia: false,
            bulb_c: [-0.2, 0.6, 0.2],
            box_scale: -2.0,
            box_fold: 1.0,
            box_min_r: 0.5,
            box_fixed_r: 1.0,
            box_iter: 12,
            box_bailout: 16.0,
            kifs_scale: 2.0,
            kifs_rot1: 0.0,
            kifs_rot2: 0.0,
            kifs_offset: [1.0, 1.0, 1.0],
            kifs_iter: 12,
            kifs_bailout: 12.0,
            detail_eps: 0.0005,
            fudge: 0.95,
            norm_eps: 0.0006,
            march_steps: 128,
            bounces: 3,
            metallic: 0.45,
            trap_scale: 0.85,
            palette_shift: 0.0,
            sun_intensity: 1.45,
            sky_intensity: 0.78,
            horizon_haze: 1.0,
            floor_roughness: 0.22,
            floor_scratch: 0.65,
            floor_scale: 1.5,
            floor_metallic: 0.85,
            roughness: 0.18,
            exposure: 0.92,
            spp: 1,
            max_accum: 256,
            denoiser_on: false,
            shadows_on: true,
            auto_rotate: false,
            spin_speed: 0.045,
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
    ReturnBuffer(usize),
    Shutdown,
}

pub struct FrameReady {
    pub buf_idx: usize,
    pub accum_count: u32,
    pub total_samples: u32,
    pub rt_ms: f32,
    pub denoise_ms: f32,
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
    _floor_gas: Arc<Accel>,
    _fractal_gas: Arc<Accel>,
    ias: Arc<Accel>,
    pipeline: Pipeline,
    sbt: ShaderBindingTable,
    denoiser: Denoiser,
    color_f32: DeviceBuffer<[f32; 4]>,
    albedo_f32: DeviceBuffer<[f32; 4]>,
    normal_f32: DeviceBuffer<[f32; 4]>,
    denoised_f32: DeviceBuffer<[f32; 4]>,
    params_buf: DeviceBuffer<u8>,
    tonemap: TonemapKernel,
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

fn compile_ptx(src: &str, name: &str, include_optix: bool) -> Vec<u8> {
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

    let mut prog = nvrtc::Program::new(src, Some(name), &[]).expect("nvrtcProgram");
    prog.compile(&opts)
        .unwrap_or_else(|e| panic!("NVRTC compile {name} failed: {e}"));
    let mut bytes = prog.ptx().expect("NVRTC ptx");
    if bytes.last() != Some(&0) {
        bytes.push(0);
    }
    bytes
}

fn init_optix(cuda: &Arc<CudaContext>) -> OptixRig {
    let stream = cuda.default_stream();
    let optix_ctx = DeviceContext::new(cuda, None, 0, false).expect("OptiX DeviceContext");
    let mut scratch = Scratch::new(&stream);

    let rt_ptx = compile_ptx(RAYTRACER_CU, "raytracer.cu", true);
    let tm_ptx = compile_ptx(TONEMAP_CU, "tonemap.cu", false);
    let mut cu_mod: cu::CUmodule = std::ptr::null_mut();
    // SAFETY: `tm_ptx` is NUL-terminated PTX compiled by NVRTC on the current context.
    let res = unsafe { cu::cuModuleLoadData(&mut cu_mod, tm_ptx.as_ptr().cast()) };
    assert_eq!(res, cu::cudaError_enum_CUDA_SUCCESS, "cuModuleLoadData failed");
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

    let mut opts =
        PipelineOptions::new("params", std::mem::size_of::<LaunchParams>(), 8).expect("opts");
    opts.attribute_values = 4;
    opts.primitive_types = 0; // custom AABB + triangles
    let (module, _) =
        Module::new(&optix_ctx, &ModuleOptions::default(), &opts, &rt_ptx).expect("module");

    let raygen = ProgramGroup::raygen(&module, "__raygen__render").expect("raygen");
    let miss = ProgramGroup::miss(&module, "__miss__miss").expect("miss");
    let miss_shadow = ProgramGroup::miss(&module, "__miss__shadow").expect("miss_shadow");
    let hit_floor = ProgramGroup::hit_group(
        &optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__hit")),
            ..Default::default()
        },
    )
    .expect("hit_floor");
    let hit_fractal = ProgramGroup::hit_group(
        &optix_ctx,
        HitGroup {
            closest_hit: Some((&module, "__closesthit__fractal")),
            intersection: Some((&module, "__intersection__fractal")),
            ..Default::default()
        },
    )
    .expect("hit_fractal");

    let (pipeline, _) = Pipeline::new(
        &optix_ctx,
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
    .expect("pipeline");

    let sbt = ShaderBindingTable::new(
        &stream,
        &SbtRecord::new(&raygen, 0u32).expect("rg"),
        &[
            SbtRecord::new(&miss, 0u32).expect("m0"),
            SbtRecord::new(&miss_shadow, 0u32).expect("m1"),
        ],
        &[
            SbtRecord::new(&hit_floor, 0u32).expect("h0"),
            SbtRecord::new(&hit_fractal, 0u32).expect("h1"),
        ],
    )
    .expect("sbt");

    // Large raytraced horizon floor triangle mesh (y = -1.35)
    let floor_verts = DeviceBuffer::from_host(
        &stream,
        &[
            [-180.0f32, -1.35, -180.0],
            [180.0, -1.35, -180.0],
            [180.0, -1.35, 180.0],
            [-180.0, -1.35, -180.0],
            [180.0, -1.35, 180.0],
            [-180.0, -1.35, 180.0],
        ],
    )
    .expect("floor verts");
    let floor_gas = Accel::triangles(
        &optix_ctx,
        &stream,
        &mut scratch,
        &[TriangleMesh {
            flags: &[GeometryFlags::DISABLE_ANYHIT],
            ..TriangleMesh::new(&floor_verts)
        }],
        BuildOptions::default(),
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

    let mut i0 = Instance::new(&floor_gas);
    i0.id = 0;
    i0.sbt_offset = 0;
    let mut i1 = Instance::new(&fractal_gas);
    i1.id = 1;
    i1.sbt_offset = 1;
    let ias = Accel::instances(
        &optix_ctx,
        &stream,
        &mut scratch,
        &[i0, i1],
        BuildOptions::default(),
    )
    .expect("IAS");

    let denoiser = Denoiser::new(
        &optix_ctx,
        &stream,
        DenoiserOptions {
            model: DenoiserModel::Hdr,
            albedo: true,
            normal: true,
            ..Default::default()
        },
        RENDER_W,
        RENDER_H,
    )
    .expect("Denoiser");

    let n = (RENDER_W * RENDER_H) as usize;
    let color_f32 = DeviceBuffer::zeroed(&stream, n).expect("color_f32");
    let albedo_f32 = DeviceBuffer::zeroed(&stream, n).expect("albedo_f32");
    let normal_f32 = DeviceBuffer::zeroed(&stream, n).expect("normal_f32");
    let denoised_f32 = DeviceBuffer::zeroed(&stream, n).expect("denoised_f32");
    let params_buf =
        DeviceBuffer::zeroed(&stream, std::mem::size_of::<LaunchParams>()).expect("params");
    stream.synchronize().expect("init sync");

    OptixRig {
        stream,
        _optix_ctx: optix_ctx,
        _scratch: scratch,
        _floor_gas: floor_gas,
        _fractal_gas: fractal_gas,
        ias,
        pipeline,
        sbt,
        denoiser,
        color_f32,
        albedo_f32,
        normal_f32,
        denoised_f32,
        params_buf,
        tonemap,
    }
}

pub fn spawn_render_thread(
    cuda: Arc<CudaContext>,
    buf_ptrs: [u64; 2],
    initial_settings: DemoSettings,
    initial_camera: CameraFrame,
) -> (Sender<RenderCmd>, Receiver<FrameReady>, JoinHandle<()>) {
    let (cmd_tx, cmd_rx) = mpsc::channel::<RenderCmd>();
    let (frame_tx, frame_rx) = mpsc::channel::<FrameReady>();

    let handle = thread::Builder::new()
        .name("optix-render".to_string())
        .spawn(move || {
            cuda.bind_to_thread().expect("cuda bind_to_thread on render thread");
            let mut rig = init_optix(&cuda);
            let mut free_bufs = VecDeque::from([0usize, 1usize]);
            let mut settings = initial_settings;
            let mut camera = initial_camera;
            let mut accum_count: u32 = 0;
            let mut frame_index: u32 = 0;
            let mut spin_angle: f32 = 0.0;
            let mut last_tick = Instant::now();

            loop {
                // Drain any pending commands from the 60 FPS GUI thread.
                let mut should_exit = false;
                while let Ok(cmd) = cmd_rx.try_recv() {
                    match cmd {
                        RenderCmd::Update {
                            settings: s,
                            camera: c,
                            reset_accum,
                        } => {
                            if reset_accum || *s != settings || c != camera {
                                accum_count = 0;
                            }
                            settings = *s;
                            camera = c;
                        }
                        RenderCmd::ReturnBuffer(idx) => {
                            if !free_bufs.contains(&idx) {
                                free_bufs.push_back(idx);
                            }
                        }
                        RenderCmd::Shutdown => {
                            should_exit = true;
                            break;
                        }
                    }
                }
                if should_exit {
                    break;
                }

                let converged = !settings.auto_rotate
                    && settings.max_accum > 0
                    && accum_count >= settings.max_accum;

                // If converged or waiting for a free double-buffer slot, block without spinning.
                if converged || free_bufs.is_empty() {
                    match cmd_rx.recv() {
                        Ok(RenderCmd::Update {
                            settings: s,
                            camera: c,
                            reset_accum,
                        }) => {
                            if reset_accum || *s != settings || c != camera {
                                accum_count = 0;
                            }
                            settings = *s;
                            camera = c;
                        }
                        Ok(RenderCmd::ReturnBuffer(idx)) => {
                            if !free_bufs.contains(&idx) {
                                free_bufs.push_back(idx);
                            }
                        }
                        Ok(RenderCmd::Shutdown) | Err(_) => break,
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

                let (fractal_p0, fractal_p1, fractal_p2) = match settings.fractal_kind {
                    1 => (
                        [
                            settings.box_scale,
                            settings.box_bailout,
                            settings.box_fold,
                            settings.box_min_r,
                        ],
                        [settings.box_fixed_r, 0.0, settings.box_iter as f32, 0.0],
                        [0.0; 4],
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
                    ),
                };

                let lp = LaunchParams {
                    out_color_f32: rig.color_f32.cu_deviceptr(),
                    out_albedo_f32: rig.albedo_f32.cu_deviceptr(),
                    out_normal_f32: rig.normal_f32.cu_deviceptr(),
                    handle: rig.ias.handle(),
                    width: RENDER_W,
                    height: RENDER_H,
                    frame_index,
                    spp: settings.spp,
                    cam_pos: [camera.eye[0], camera.eye[1], camera.eye[2], 0.0],
                    cam_forward: [camera.fwd[0], camera.fwd[1], camera.fwd[2], 0.0],
                    cam_right: [camera.right[0], camera.right[1], camera.right[2], 0.0],
                    cam_up: [camera.up[0], camera.up[1], camera.up[2], camera.tan_half_fov],
                    sun_dir: [0.55, 0.85, 0.35, settings.roughness],
                    fractal_p0,
                    fractal_p1,
                    fractal_p2,
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
                };
                rig.params_buf
                    .copy_from_host(&rig.stream, bytemuck::bytes_of(&lp))
                    .expect("params upload");

                let t_rt = Instant::now();
                // SAFETY: `params_buf` matches `LaunchParams` layout; all output buffers outlive `synchronize`.
                unsafe {
                    rig.pipeline.launch(
                        &rig.stream,
                        &rig.params_buf,
                        &rig.sbt,
                        [RENDER_W, RENDER_H, 1],
                    )
                }
                .expect("optix launch");
                rig.stream.synchronize().expect("rt sync");
                let rt_ms = t_rt.elapsed().as_secs_f32() * 1000.0;

                let (final_f32_ptr, denoise_ms) = if settings.denoiser_on {
                    let t_den = Instant::now();
                    let guides = Guides {
                        albedo: Some(
                            Image::new(&rig.albedo_f32, RENDER_W, RENDER_H).expect("albedo"),
                        ),
                        normal: Some(
                            Image::new(&rig.normal_f32, RENDER_W, RENDER_H).expect("normal"),
                        ),
                        ..Default::default()
                    };
                    let layer = Layer {
                        input: Image::new(&rig.color_f32, RENDER_W, RENDER_H).expect("in"),
                        output: Image::new(&rig.denoised_f32, RENDER_W, RENDER_H).expect("out"),
                        previous: None,
                        aov: AovType::Beauty,
                    };
                    // SAFETY: all denoiser input/output/guide images outlive `synchronize`.
                    unsafe {
                        rig.denoiser
                            .invoke(&rig.stream, &guides, &[layer], DenoiserParams::default())
                    }
                    .expect("denoiser invoke");
                    rig.stream.synchronize().expect("den sync");
                    (
                        rig.denoised_f32.cu_deviceptr(),
                        t_den.elapsed().as_secs_f32() * 1000.0,
                    )
                } else {
                    (rig.color_f32.cu_deviceptr(), 0.0)
                };

                let mut src_ptr = final_f32_ptr;
                let mut dst_ptr = buf_ptrs[buf_idx];
                let mut w = RENDER_W;
                let mut h = RENDER_H;
                let mut exposure = settings.exposure;
                let mut kernel_args: [*mut c_void; 5] = [
                    (&raw mut src_ptr).cast(),
                    (&raw mut dst_ptr).cast(),
                    (&raw mut w).cast(),
                    (&raw mut h).cast(),
                    (&raw mut exposure).cast(),
                ];
                let gx = RENDER_W.div_ceil(16);
                let gy = RENDER_H.div_ceil(16);
                // SAFETY: `rig.tonemap.func` takes `(float4*, uchar4*, u32, u32, f32)` and all
                // pointers/buffers remain valid through `stream.synchronize()`.
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

                accum_count = accum_count.saturating_add(1);
                frame_index = frame_index.wrapping_add(1);

                if frame_tx
                    .send(FrameReady {
                        buf_idx,
                        accum_count,
                        total_samples: accum_count.saturating_mul(settings.spp),
                        rt_ms,
                        denoise_ms,
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
