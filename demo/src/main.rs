//! Interactive RTX OptiX 3D Fractal Path Tracer + AI Denoiser + Zero-Copy `nv-wgpu` + `egui` Demo.
//!
//! Run with:
//! `cargo run --manifest-path demo/Cargo.toml --release`

#[allow(unexpected_cfgs, dead_code, unused_imports, clippy::all)]
mod camera_core;
#[allow(unexpected_cfgs, dead_code, unused_imports, deprecated, clippy::all)]
mod camera_egui;

use std::sync::Arc;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use cuda_bindings as cu;
use cuda_core::{Context as CudaContext, DeviceBuffer, Stream};
use demo_app::DemoApp;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

mod demo_app {
    use super::*;
    use crate::camera_core::input::NavScheme;
    use crate::camera_core::{
        CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, NavPrefs, ViewportSize,
    };
    use glam::Vec3;
    use nv_wgpu::{InteropContext, SharedBuffer, SharedBufferDescriptor};
    use optix::{
        Aabb, Accel, AovType, BuildOptions, CustomPrimitives, Denoiser, DenoiserModel,
        DenoiserOptions, DeviceContext, GeometryFlags, Guides, HitGroup, Image, Instance, Layer,
        LinkOptions, Module, ModuleOptions, Params as DenoiserParams, Pipeline, PipelineOptions,
        ProgramGroup, SbtRecord, Scratch, ShaderBindingTable, TriangleMesh,
    };
    use std::ffi::{CString, c_void};

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
    }

    pub struct GpuTelemetry {
        pub gpu_name: String,
        pub temp_c: u32,
        pub util_gpu: u32,
        pub vram_used_mib: u64,
        pub vram_total_mib: u64,
        pub power_w: f32,
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
        optix_ctx: Arc<DeviceContext>,
        scratch: Scratch,
        floor_gas: Arc<Accel>,
        fractal_gas: Arc<Accel>,
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
        // Rendering
        pub roughness: f32,
        pub exposure: f32,
        pub spp: u32,
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
                sun_intensity: 2.8,
                roughness: 0.18,
                exposure: 1.0,
                spp: 1,
                denoiser_on: false,
                shadows_on: true,
                auto_rotate: false,
                spin_speed: 0.045,
            }
        }
    }

    pub struct DemoApp {
        window: Arc<Window>,
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
        interop: InteropContext,
        egui_ctx: egui::Context,
        egui_winit: egui_winit::State,
        egui_renderer: egui_wgpu::Renderer,
        shared_rgba: SharedBuffer,
        display_texture: wgpu::Texture,
        display_tex_id: egui::TextureId,
        optix: OptixRig,
        nvml_dev: Option<nvml::Device>,
        _nvml: Option<&'static nvml::Nvml>,
        telemetry: GpuTelemetry,
        last_telemetry_at: Instant,
        start_time: Instant,
        last_frame_at: Instant,
        frame_index: u32,
        rt_ms: f32,
        denoise_ms: f32,
        fps: f32,
        settings: DemoSettings,
        cam_driver: CameraDriver,
        cam_home: CameraPose,
        cam_pose: CameraPose,
        cam_mode: NavMode,
        cam_fly_mode: NavMode,
        cam_scheme: NavScheme,
        cam_feel: NavFeel,
        spin_angle: f32,
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

    impl DemoApp {
        pub fn new(window: Arc<Window>) -> Self {
            let size = window.inner_size();
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::VULKAN,
                ..wgpu::InstanceDescriptor::new_without_display_handle()
            });
            let surface = instance
                .create_surface(window.clone())
                .expect("wgpu surface");

            let adapters = pollster_block(instance.enumerate_adapters(wgpu::Backends::VULKAN));
            let mut chosen = None;
            for adapter in adapters {
                if !adapter.is_surface_supported(&surface) {
                    continue;
                }
                if let Ok(ctx) = InteropContext::open_vulkan(
                    &adapter,
                    wgpu::Features::empty(),
                    &wgpu::Limits::default(),
                    Some("nv-rtx-demo"),
                ) {
                    chosen = Some((adapter, ctx));
                    break;
                }
            }
            let (adapter, interop) = chosen.expect("Vulkan+CUDA interop adapter");

            let mut surface_config = surface
                .get_default_config(&adapter, size.width.max(1), size.height.max(1))
                .expect("surface default config");
            let caps = surface.get_capabilities(&adapter);
            if let Some(fmt) = caps.formats.iter().copied().find(|f| {
                matches!(
                    f,
                    wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Rgba8Unorm
                )
            }) {
                surface_config.format = fmt;
            }
            surface.configure(interop.device(), &surface_config);

            let egui_ctx = egui::Context::default();
            egui_ctx.all_styles_mut(|style| {
                style.spacing.item_spacing = egui::vec2(4.0, 2.5);
                style.spacing.window_margin = egui::Margin::same(6);
                style.spacing.button_padding = egui::vec2(5.0, 2.0);
                style.spacing.slider_width = 108.0;
                style.spacing.interact_size = egui::vec2(32.0, 16.0);
                for (ts, fid) in style.text_styles.iter_mut() {
                    fid.size = match ts {
                        egui::TextStyle::Heading => 14.0,
                        egui::TextStyle::Small => 11.5,
                        _ => 12.0,
                    };
                    fid.family = egui::FontFamily::Proportional;
                }
            });

            let prefs = NavPrefs {
                hold_fly_mode: NavMode::SpaceSim,
                ..NavPrefs::default()
            };
            let scheme = NavScheme::default();
            let mut cam_driver = CameraDriver::new(prefs, scheme);
            enable_rmb_fly(&mut cam_driver);
            cam_driver.controller_mut().apply_intent(
                CameraIntent::FrameBounds {
                    min: Vec3::splat(-1.25),
                    max: Vec3::splat(1.25),
                    margin: 1.2,
                },
                1.0 / 60.0,
                ViewportSize::new(RENDER_W, RENDER_H),
            );
            let cam_home = cam_driver.controller().pose();
            let cam_pose = cam_home;
            let egui_winit = egui_winit::State::new(
                egui_ctx.clone(),
                egui::ViewportId::ROOT,
                &window,
                Some(window.scale_factor() as f32),
                None,
                None,
            );
            let mut egui_renderer = egui_wgpu::Renderer::new(
                interop.device(),
                surface_config.format,
                egui_wgpu::RendererOptions::default(),
            );

            // Shared RGBA8 buffer (RENDER_W * 4 = 3584, multiple of 256)
            let byte_size = u64::from(RENDER_W * RENDER_H * 4);
            let shared_rgba = interop
                .create_buffer(&SharedBufferDescriptor {
                    label: Some("optix-rgba8"),
                    size: byte_size,
                    usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
                })
                .expect("SharedBuffer");

            let display_texture = interop.device().create_texture(&wgpu::TextureDescriptor {
                label: Some("optix-viewport"),
                size: wgpu::Extent3d {
                    width: RENDER_W,
                    height: RENDER_H,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            let view = display_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let display_tex_id = egui_renderer.register_native_texture(
                interop.device(),
                &view,
                wgpu::FilterMode::Linear,
            );

            let optix = init_optix(interop.cuda_context());

            let nvml_lib = nvml::Nvml::init()
                .ok()
                .map(|n| Box::leak(Box::new(n)) as &'static nvml::Nvml);
            let nvml_dev = nvml_lib.and_then(|n| n.device_by_index(0).ok());
            let telemetry = read_telemetry(&nvml_dev);

            Self {
                window,
                surface,
                surface_config,
                interop,
                egui_ctx,
                egui_winit,
                egui_renderer,
                shared_rgba,
                display_texture,
                display_tex_id,
                optix,
                nvml_dev,
                _nvml: nvml_lib,
                telemetry,
                last_telemetry_at: Instant::now(),
                start_time: Instant::now(),
                last_frame_at: Instant::now(),
                frame_index: 0,
                rt_ms: 0.0,
                denoise_ms: 0.0,
                fps: 0.0,
                settings: DemoSettings::default(),
                cam_driver,
                cam_home,
                cam_pose,
                cam_mode: prefs.mode,
                cam_fly_mode: prefs.hold_fly_mode,
                cam_scheme: scheme,
                cam_feel: prefs.feel,
                spin_angle: 0.0,
            }
        }

        pub fn resize(&mut self, width: u32, height: u32) {
            if width > 0 && height > 0 {
                self.surface_config.width = width;
                self.surface_config.height = height;
                self.surface
                    .configure(self.interop.device(), &self.surface_config);
            }
        }

        pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
            let resp = self.egui_winit.on_window_event(&self.window, event);
            resp.repaint
        }

        pub fn render(&mut self) {
            let now = Instant::now();
            let dt = (now - self.last_frame_at).as_secs_f32().max(1e-4);
            self.last_frame_at = now;
            self.fps = self.fps * 0.9 + (1.0 / dt) * 0.1;

            if self.last_telemetry_at.elapsed().as_secs_f32() > 0.35 {
                self.telemetry = read_telemetry(&self.nvml_dev);
                self.last_telemetry_at = now;
            }

            let elapsed = self.start_time.elapsed().as_secs_f32();
            if self.settings.auto_rotate {
                self.spin_angle += dt * self.settings.spin_speed;
            }

            // 1. UI + camera-rs input pass (zero-latency for this frame's ray trace)
            let raw_input = self.egui_winit.take_egui_input(&self.window);
            let tex_id = self.display_tex_id;
            let rt_ms = self.rt_ms;
            let den_ms = self.denoise_ms;
            let fps = self.fps;
            let tel = &self.telemetry;
            let settings = &mut self.settings;
            let cam_driver = &mut self.cam_driver;
            let cam_home = self.cam_home;
            let cam_mode = &mut self.cam_mode;
            let cam_fly_mode = &mut self.cam_fly_mode;
            let cam_scheme = &mut self.cam_scheme;
            let cam_feel = &mut self.cam_feel;
            let mut next_pose = self.cam_pose;

            let full_output = self.egui_ctx.run_ui(raw_input, |ui| {
                egui::Panel::left("controls")
                    .default_size(248.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.heading("nv-rs RTX");
                        ui.label(format!(
                            "FPS {fps:.1} | RT {rt_ms:.2}ms | Den {den_ms:.2}ms"
                        ));
                        ui.separator();

                        ui.label("Formula");
                        egui::ComboBox::from_id_salt("formula")
                            .width(228.0)
                            .selected_text(match settings.fractal_kind {
                                1 => "Mandelbox",
                                2 => "KIFS Octahedron",
                                _ => "Mandelbulb",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.fractal_kind, 0, "Mandelbulb");
                                ui.selectable_value(&mut settings.fractal_kind, 1, "Mandelbox");
                                ui.selectable_value(
                                    &mut settings.fractal_kind,
                                    2,
                                    "KIFS Octahedron",
                                );
                            });

                        match settings.fractal_kind {
                            0 => {
                                ui.add(
                                    egui::Slider::new(&mut settings.bulb_power, 2.0..=14.0)
                                        .text("Power"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.bulb_iter, 4..=24)
                                        .text("Iter"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.bulb_bailout, 2.0..=32.0)
                                        .text("Bailout"),
                                );
                                ui.checkbox(&mut settings.bulb_julia, "Julia Mode");
                                if settings.bulb_julia {
                                    ui.horizontal(|ui| {
                                        ui.label("C:");
                                        ui.add(
                                            egui::DragValue::new(&mut settings.bulb_c[0])
                                                .speed(0.01)
                                                .range(-1.5..=1.5),
                                        );
                                        ui.add(
                                            egui::DragValue::new(&mut settings.bulb_c[1])
                                                .speed(0.01)
                                                .range(-1.5..=1.5),
                                        );
                                        ui.add(
                                            egui::DragValue::new(&mut settings.bulb_c[2])
                                                .speed(0.01)
                                                .range(-1.5..=1.5),
                                        );
                                    });
                                }
                            }
                            1 => {
                                ui.add(
                                    egui::Slider::new(&mut settings.box_scale, -3.5..=3.5)
                                        .text("Scale"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.box_fold, 0.2..=2.2)
                                        .text("Fold"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.box_min_r, 0.05..=1.2)
                                        .text("Min R"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.box_fixed_r, 0.4..=2.5)
                                        .text("Fixed R"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.box_iter, 4..=24).text("Iter"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.box_bailout, 4.0..=64.0)
                                        .text("Bailout"),
                                );
                            }
                            _ => {
                                ui.add(
                                    egui::Slider::new(&mut settings.kifs_scale, 1.3..=2.8)
                                        .text("Scale"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.kifs_rot1, -180.0..=180.0)
                                        .text("Rot Y"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.kifs_rot2, -180.0..=180.0)
                                        .text("Rot X"),
                                );
                                ui.horizontal(|ui| {
                                    ui.label("Off:");
                                    ui.add(
                                        egui::DragValue::new(&mut settings.kifs_offset[0])
                                            .speed(0.01)
                                            .range(0.1..=2.0),
                                    );
                                    ui.add(
                                        egui::DragValue::new(&mut settings.kifs_offset[1])
                                            .speed(0.01)
                                            .range(0.1..=2.0),
                                    );
                                    ui.add(
                                        egui::DragValue::new(&mut settings.kifs_offset[2])
                                            .speed(0.01)
                                            .range(0.1..=2.0),
                                    );
                                });
                                ui.add(
                                    egui::Slider::new(&mut settings.kifs_iter, 4..=24)
                                        .text("Iter"),
                                );
                                ui.add(
                                    egui::Slider::new(&mut settings.kifs_bailout, 4.0..=32.0)
                                        .text("Bailout"),
                                );
                            }
                        }

                        ui.separator();
                        ui.label("Detail & Shading");
                        ui.add(
                            egui::Slider::new(&mut settings.detail_eps, 0.00005..=0.005)
                                .logarithmic(true)
                                .text("Detail"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.norm_eps, 0.00005..=0.005)
                                .logarithmic(true)
                                .text("Norm Eps"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.march_steps, 32..=320).text("Steps"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.fudge, 0.3..=1.1).text("Fudge"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.bounces, 1..=6).text("Bounces"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.metallic, 0.0..=1.0).text("Metallic"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.trap_scale, 0.1..=3.0).text("Trap Sc"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.palette_shift, 0.0..=1.0).text("Hue"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.sun_intensity, 0.0..=8.0).text("Sun"),
                        );

                        ui.separator();
                        ui.label("Render");
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut settings.denoiser_on, "AI Den");
                            ui.checkbox(&mut settings.shadows_on, "Shadows");
                            ui.checkbox(&mut settings.auto_rotate, "Spin");
                        });
                        ui.add(
                            egui::Slider::new(&mut settings.spp, 1..=1024)
                                .logarithmic(true)
                                .text("SPP"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.roughness, 0.02..=0.8)
                                .text("Rough"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.exposure, 0.3..=2.5).text("Exp"),
                        );
                        ui.add(
                            egui::Slider::new(&mut settings.spin_speed, 0.0..=0.5).text("Spin"),
                        );

                        ui.separator();
                        ui.label("Camera (RMB = Fly)");
                        let (prev_m, prev_fly, prev_s, prev_f) =
                            (*cam_mode, *cam_fly_mode, *cam_scheme, *cam_feel);
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_id_salt("cam_mode")
                                .width(110.0)
                                .selected_text(cam_mode.label())
                                .show_ui(ui, |ui| {
                                    for m in NavMode::ALL.into_iter().filter(|m| !m.is_fly()) {
                                        ui.selectable_value(cam_mode, m, m.label());
                                    }
                                });
                            egui::ComboBox::from_id_salt("cam_fly")
                                .width(110.0)
                                .selected_text(cam_fly_mode.label())
                                .show_ui(ui, |ui| {
                                    for m in NavMode::ALL.into_iter().filter(|m| m.is_fly()) {
                                        ui.selectable_value(cam_fly_mode, m, m.label());
                                    }
                                });
                        });
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_id_salt("cam_scheme")
                                .width(92.0)
                                .selected_text(format!("{:?}", *cam_scheme))
                                .show_ui(ui, |ui| {
                                    for s in NavScheme::ALL {
                                        ui.selectable_value(cam_scheme, s, format!("{s:?}"));
                                    }
                                });
                            for f in NavFeel::ALL {
                                ui.selectable_value(cam_feel, f, f.label());
                            }
                            if ui.button("Home").clicked() {
                                cam_driver.controller_mut().set_pose(cam_home);
                            }
                        });
                        if *cam_mode != prev_m || *cam_fly_mode != prev_fly || *cam_feel != prev_f {
                            let prefs = NavPrefs {
                                mode: *cam_mode,
                                feel: *cam_feel,
                                hold_fly_mode: *cam_fly_mode,
                                ..*cam_driver.prefs()
                            };
                            cam_driver.set_prefs(prefs);
                        }
                        if *cam_scheme != prev_s {
                            cam_driver.set_scheme(*cam_scheme);
                            enable_rmb_fly(cam_driver);
                        }

                        ui.separator();
                        ui.label(format!("{} | {}°C {}%", tel.gpu_name, tel.temp_c, tel.util_gpu));
                        ui.label(format!(
                            "VRAM {}/{} MiB | {:.0} W",
                            tel.vram_used_mib, tel.vram_total_mib, tel.power_w
                        ));
                        });
                    });

                egui::CentralPanel::default().show(ui, |ui| {
                    let avail = ui.available_size();
                    let aspect = RENDER_W as f32 / RENDER_H as f32;
                    let draw_w = avail.x.min(avail.y * aspect).max(1.0);
                    let draw_h = (draw_w / aspect).max(1.0);
                    let resp = ui.centered_and_justified(|ui| {
                        ui.add(
                            egui::Image::new(egui::load::SizedTexture::new(
                                tex_id,
                                egui::vec2(draw_w, draw_h),
                            ))
                            .sense(egui::Sense::click_and_drag()),
                        )
                    });
                    let vp_resp = resp.inner;
                    let mut cam_frame = crate::camera_egui::drive(ui, &vp_resp, cam_driver, 1.25);
                    if cam_frame.requests.frame_selected {
                        let size = vp_resp.rect.size().round();
                        let vp = ViewportSize::new(size.x.max(1.0) as u32, size.y.max(1.0) as u32);
                        cam_driver.controller_mut().apply_intent(
                            CameraIntent::FrameBounds {
                                min: Vec3::splat(-1.25),
                                max: Vec3::splat(1.25),
                                margin: 1.2,
                            },
                            dt,
                            vp,
                        );
                        cam_frame.pose = cam_driver.controller().pose();
                    }
                    if cam_frame.requests.home {
                        cam_driver.controller_mut().set_pose(cam_home);
                        cam_frame.pose = cam_driver.controller().pose();
                    }
                    next_pose = cam_frame.pose;
                });
            });
            self.cam_pose = next_pose;

            // 2. Update IAS transform for rotating fractal instance
            let (s, c) = self.spin_angle.sin_cos();
            let mut fractal_inst = Instance::new(&self.optix.fractal_gas);
            fractal_inst.id = 1;
            fractal_inst.sbt_offset = 1;
            fractal_inst.transform = [c, 0.0, s, 0.0, 0.0, 1.0, 0.0, 0.0, -s, 0.0, c, 0.0];
            let mut floor_inst = Instance::new(&self.optix.floor_gas);
            floor_inst.id = 0;
            floor_inst.sbt_offset = 0;

            self.optix.ias = Accel::instances(
                &self.optix.optix_ctx,
                &self.optix.stream,
                &mut self.optix.scratch,
                &[floor_inst, fractal_inst],
                BuildOptions::default(),
            )
            .expect("rebuild IAS");

            // 3. Camera basis & fractal params from CameraPose
            let eye = self.cam_pose.eye;
            let fwd = self.cam_pose.orientation * -Vec3::Z;
            let right = self.cam_pose.orientation * Vec3::X;
            let up = self.cam_pose.orientation * Vec3::Y;
            let tan_half_fov = (self.cam_pose.fov_y * 0.5).tan();

            let (fractal_p0, fractal_p1, fractal_p2) = match self.settings.fractal_kind {
                1 => (
                    [
                        self.settings.box_scale,
                        self.settings.box_bailout,
                        self.settings.box_fold,
                        self.settings.box_min_r,
                    ],
                    [
                        self.settings.box_fixed_r,
                        0.0,
                        self.settings.box_iter as f32,
                        0.0,
                    ],
                    [0.0; 4],
                ),
                2 => (
                    [
                        self.settings.kifs_scale,
                        self.settings.kifs_bailout,
                        0.0,
                        0.0,
                    ],
                    [
                        self.settings.kifs_rot1.to_radians(),
                        self.settings.kifs_rot2.to_radians(),
                        self.settings.kifs_iter as f32,
                        0.0,
                    ],
                    [
                        self.settings.kifs_offset[0],
                        self.settings.kifs_offset[1],
                        self.settings.kifs_offset[2],
                        0.0,
                    ],
                ),
                _ => (
                    [
                        self.settings.bulb_power,
                        self.settings.bulb_bailout,
                        if self.settings.bulb_julia { 1.0 } else { 0.0 },
                        0.0,
                    ],
                    [0.0, 0.0, self.settings.bulb_iter as f32, 0.0],
                    [
                        self.settings.bulb_c[0],
                        self.settings.bulb_c[1],
                        self.settings.bulb_c[2],
                        0.0,
                    ],
                ),
            };

            let lp = LaunchParams {
                out_color_f32: self.optix.color_f32.cu_deviceptr(),
                out_albedo_f32: self.optix.albedo_f32.cu_deviceptr(),
                out_normal_f32: self.optix.normal_f32.cu_deviceptr(),
                handle: self.optix.ias.handle(),
                width: RENDER_W,
                height: RENDER_H,
                frame_index: self.frame_index,
                spp: self.settings.spp,
                cam_pos: [eye.x, eye.y, eye.z, 0.0],
                cam_forward: [fwd.x, fwd.y, fwd.z, 0.0],
                cam_right: [right.x, right.y, right.z, 0.0],
                cam_up: [up.x, up.y, up.z, tan_half_fov],
                sun_dir: [0.55, 0.85, 0.35, self.settings.roughness],
                fractal_p0,
                fractal_p1,
                fractal_p2,
                fractal_r0: [
                    self.settings.detail_eps,
                    self.settings.fudge,
                    self.settings.norm_eps,
                    self.settings.march_steps as f32,
                ],
                fractal_r1: [
                    self.settings.metallic,
                    self.settings.trap_scale,
                    self.settings.palette_shift,
                    self.settings.sun_intensity,
                ],
                time: elapsed,
                enable_shadows: u32::from(self.settings.shadows_on),
                fractal_kind: self.settings.fractal_kind,
                bounces: self.settings.bounces,
            };
            self.optix
                .params_buf
                .copy_from_host(&self.optix.stream, bytemuck::bytes_of(&lp))
                .expect("params upload");

            // 3. OptiX Ray Trace
            let t_rt = Instant::now();
            // SAFETY: `params_buf` matches `LaunchParams` layout; all output buffers outlive `synchronize`.
            unsafe {
                self.optix.pipeline.launch(
                    &self.optix.stream,
                    &self.optix.params_buf,
                    &self.optix.sbt,
                    [RENDER_W, RENDER_H, 1],
                )
            }
            .expect("optix launch");
            self.optix.stream.synchronize().expect("rt sync");
            self.rt_ms = self.rt_ms * 0.85 + t_rt.elapsed().as_secs_f32() * 1000.0 * 0.15;

            // 4. Optional OptiX AI Denoiser (Tensor Cores)
            let final_f32_ptr = if self.settings.denoiser_on {
                let t_den = Instant::now();
                let guides = Guides {
                    albedo: Some(
                        Image::new(&self.optix.albedo_f32, RENDER_W, RENDER_H).expect("albedo"),
                    ),
                    normal: Some(
                        Image::new(&self.optix.normal_f32, RENDER_W, RENDER_H).expect("normal"),
                    ),
                    ..Default::default()
                };
                let layer = Layer {
                    input: Image::new(&self.optix.color_f32, RENDER_W, RENDER_H).expect("in"),
                    output: Image::new(&self.optix.denoised_f32, RENDER_W, RENDER_H)
                        .expect("out"),
                    previous: None,
                    aov: AovType::Beauty,
                };
                // SAFETY: all denoiser input/output/guide images outlive `synchronize`.
                unsafe {
                    self.optix.denoiser.invoke(
                        &self.optix.stream,
                        &guides,
                        &[layer],
                        DenoiserParams::default(),
                    )
                }
                .expect("denoiser invoke");
                self.optix.stream.synchronize().expect("den sync");
                self.denoise_ms =
                    self.denoise_ms * 0.85 + t_den.elapsed().as_secs_f32() * 1000.0 * 0.15;
                self.optix.denoised_f32.cu_deviceptr()
            } else {
                self.denoise_ms = 0.0;
                self.optix.color_f32.cu_deviceptr()
            };

            // 5. Tonemap f32 -> nv_wgpu::SharedBuffer (RGBA8) on GPU
            let mut src_ptr = final_f32_ptr;
            let mut dst_ptr = self.shared_rgba.cu_deviceptr();
            let mut w = RENDER_W;
            let mut h = RENDER_H;
            let mut exposure = self.settings.exposure;
            let mut kernel_args: [*mut c_void; 5] = [
                (&raw mut src_ptr).cast(),
                (&raw mut dst_ptr).cast(),
                (&raw mut w).cast(),
                (&raw mut h).cast(),
                (&raw mut exposure).cast(),
            ];
            let gx = RENDER_W.div_ceil(16);
            let gy = RENDER_H.div_ceil(16);
            // SAFETY: `self.optix.tonemap.func` takes `(float4*, uchar4*, u32, u32, f32)` and all
            // pointers/buffers remain valid through `stream.synchronize()`.
            let res = unsafe {
                cu::cuLaunchKernel(
                    self.optix.tonemap.func,
                    gx,
                    gy,
                    1,
                    16,
                    16,
                    1,
                    0,
                    self.optix.stream.cu_stream(),
                    kernel_args.as_mut_ptr(),
                    std::ptr::null_mut(),
                )
            };
            assert_eq!(
                res,
                cu::cudaError_enum_CUDA_SUCCESS,
                "tonemap cuLaunchKernel failed"
            );
            self.optix.stream.synchronize().expect("tonemap sync");
            self.frame_index = self.frame_index.wrapping_add(1);

            // 6. wgpu copy SharedBuffer -> Texture + egui render
            let frame = match self.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(f)
                | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
                wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                    let s = self.window.inner_size();
                    self.resize(s.width, s.height);
                    return;
                }
                wgpu::CurrentSurfaceTexture::Timeout
                | wgpu::CurrentSurfaceTexture::Occluded
                | wgpu::CurrentSurfaceTexture::Validation => {
                    return;
                }
            };
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.egui_winit
                .handle_platform_output(&self.window, full_output.platform_output);
            let tris = self
                .egui_ctx
                .tessellate(full_output.shapes, full_output.pixels_per_point);
            let screen_desc = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [self.surface_config.width, self.surface_config.height],
                pixels_per_point: full_output.pixels_per_point,
            };

            let mut encoder =
                self.interop
                    .device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("nv-rtx-frame"),
                    });

            // Zero-copy GPU buffer -> GPU texture copy
            encoder.copy_buffer_to_texture(
                wgpu::TexelCopyBufferInfo {
                    buffer: self.shared_rgba.wgpu_buffer(),
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(RENDER_W * 4),
                        rows_per_image: Some(RENDER_H),
                    },
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &self.display_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: RENDER_W,
                    height: RENDER_H,
                    depth_or_array_layers: 1,
                },
            );

            for (id, deltas) in &full_output.textures_delta.set {
                for delta in deltas {
                    self.egui_renderer.update_texture(
                        self.interop.device(),
                        self.interop.queue(),
                        *id,
                        delta,
                    );
                }
            }
            self.egui_renderer.update_buffers(
                self.interop.device(),
                self.interop.queue(),
                &mut encoder,
                &tris,
                &screen_desc,
            );
            {
                let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("egui-pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                let mut pass = pass.forget_lifetime();
                self.egui_renderer.render(&mut pass, &tris, &screen_desc);
            }
            for id in &full_output.textures_delta.free {
                self.egui_renderer.free_texture(id);
            }

            self.interop.queue().submit(Some(encoder.finish()));
            self.interop.queue().present(frame);
            self.window.request_redraw();
        }
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

        // Floor triangle mesh (y = -1.35)
        let floor_verts = DeviceBuffer::from_host(
            &stream,
            &[
                [-12.0f32, -1.35, -12.0],
                [12.0, -1.35, -12.0],
                [12.0, -1.35, 12.0],
                [-12.0, -1.35, -12.0],
                [12.0, -1.35, 12.0],
                [-12.0, -1.35, 12.0],
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
            DeviceBuffer::zeroed(&stream, std::mem::size_of::<LaunchParams>()).expect("params_buf");

        OptixRig {
            stream,
            optix_ctx,
            scratch,
            floor_gas,
            fractal_gas,
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

    fn read_telemetry(dev: &Option<nvml::Device>) -> GpuTelemetry {
        if let Some(d) = dev {
            let gpu_name = d.name().unwrap_or_else(|_| "NVIDIA GPU".to_string());
            let temp_c = d.temperature_gpu().unwrap_or(0);
            let util_gpu = d.utilization_rates().map(|u| u.gpu).unwrap_or(0);
            let (vram_used_mib, vram_total_mib) = d
                .memory_info()
                .map(|m| (m.used / (1024 * 1024), m.total / (1024 * 1024)))
                .unwrap_or((0, 0));
            let power_w = d.power_usage_mw().unwrap_or(0) as f32 / 1000.0;
            GpuTelemetry {
                gpu_name,
                temp_c,
                util_gpu,
                vram_used_mib,
                vram_total_mib,
                power_w,
            }
        } else {
            GpuTelemetry {
                gpu_name: "NVML unavailable".to_string(),
                temp_c: 0,
                util_gpu: 0,
                vram_used_mib: 0,
                vram_total_mib: 0,
                power_w: 0.0,
            }
        }
    }

    fn enable_rmb_fly(driver: &mut CameraDriver) {
        use crate::camera_core::input::{Button, DragBinding, DragRole, Mods};
        let drags = &mut driver.bindings_mut().drags;
        drags.retain(|b| !(b.button == Button::Secondary && b.mods == Mods::NONE));
        drags.push(DragBinding::new(Button::Secondary, Mods::NONE, DragRole::Fly));
    }

    fn pollster_block<F: std::future::Future>(fut: F) -> F::Output {
        use std::pin::pin;
        use std::task::{Context as TaskContext, Poll, Wake, Waker};
        struct ThreadWake(std::thread::Thread);
        impl Wake for ThreadWake {
            fn wake(self: Arc<Self>) {
                self.0.unpark();
            }
            fn wake_by_ref(self: &Arc<Self>) {
                self.0.unpark();
            }
        }
        let waker = Waker::from(Arc::new(ThreadWake(std::thread::current())));
        let mut cx = TaskContext::from_waker(&waker);
        let mut fut = pin!(fut);
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(out) => return out,
                Poll::Pending => std::thread::park(),
            }
        }
    }
}

#[derive(Default)]
struct Runner {
    app: Option<DemoApp>,
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app.is_none() {
            let attrs = Window::default_attributes()
                .with_title("nv-rs RTX Fractal Path Tracer (OptiX 9 + AI Denoiser + nv-wgpu)")
                .with_inner_size(winit::dpi::LogicalSize::new(1240.0, 680.0));
            let win = Arc::new(event_loop.create_window(attrs).expect("window"));
            self.app = Some(DemoApp::new(win));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(app) = self.app.as_mut() else {
            return;
        };
        if matches!(event, WindowEvent::CloseRequested) {
            event_loop.exit();
            return;
        }
        if let WindowEvent::Resized(s) = event {
            app.resize(s.width, s.height);
        } else if matches!(event, WindowEvent::RedrawRequested) {
            app.render();
        }
        app.handle_window_event(&event);
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop");
    let mut runner = Runner::default();
    event_loop.run_app(&mut runner).expect("run app");
}
