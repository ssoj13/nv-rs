//! Interactive RTX OptiX 3D Fractal Path Tracer + AI Denoiser + Zero-Copy `nv-wgpu` + `egui_dock` Demo.
//!
//! Run with:
//! `cargo run -p nv-demo --release`

#[allow(unexpected_cfgs, dead_code, unused_imports, clippy::all)]
mod camera_core;
#[allow(unexpected_cfgs, dead_code, unused_imports, deprecated, clippy::all)]
mod camera_egui;
mod demo_anim;
mod fft_bloom;
mod gpu_export;
mod persist;
mod profiler;
mod renderer;
mod ui;

use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Instant;

use demo_app::{DemoApp, enable_rmb_fly};
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
    use crate::profiler::{GpuTelemetry, Profiler};
    use crate::renderer::{
        CameraFrame, DEFAULT_H, DEFAULT_W, DemoSettings, FrameReady, RenderCmd, align_render_size,
        spawn_render_thread,
    };
    use crate::ui::{DockTab, UiContext, UiTheme, default_dock_state, install_fonts};
    use egui_dock::{DockArea, DockState, Style as DockStyle};
    use glam::Vec3;
    use nv_wgpu::{
        InteropContext, SharedBuffer, SharedBufferDescriptor, SharedTexture,
        SharedTextureDescriptor, VkImageTransition, vk_image_view_raw,
    };

    fn dlss_quality_from_ui(q: u32) -> ngx::DlssPerfQuality {
        match q {
            0 => ngx::DlssPerfQuality::UltraPerformance,
            1 => ngx::DlssPerfQuality::MaxPerf,
            2 => ngx::DlssPerfQuality::Balanced,
            3 => ngx::DlssPerfQuality::MaxQuality,
            4 => ngx::DlssPerfQuality::UltraQuality,
            5 => ngx::DlssPerfQuality::Dlaa,
            _ => ngx::DlssPerfQuality::MaxQuality,
        }
    }

    struct DlssPass {
        feature: ngx::FeatureHandle,
        eval_params: ngx::Parameters,
        color_in: SharedTexture,
        color_view: wgpu::TextureView,
        depth_in: SharedTexture,
        depth_view: wgpu::TextureView,
        motion_in: SharedTexture,
        motion_view: wgpu::TextureView,
        output_out: SharedTexture,
        output_view: wgpu::TextureView,
        render_w: u32,
        render_h: u32,
        out_w: u32,
        out_h: u32,
        quality: u32,
        reset_next: bool,
    }

    impl DlssPass {
        fn new(
            interop: &InteropContext,
            ngx: &ngx::NgxVulkan,
            render_w: u32,
            render_h: u32,
            out_w: u32,
            out_h: u32,
            quality: u32,
        ) -> Result<Self, String> {
            let make_tex = |w, h, fmt, usage, label| {
                let mut desc = SharedTextureDescriptor::d2(w, h, fmt, usage, false);
                desc.label = Some(label);
                interop
                    .create_texture(&desc)
                    .map_err(|e| format!("{label}: {e}"))
            };
            let color_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::Rgba8Unorm,
                wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                "dlss-color-in",
            )?;
            let depth_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::R32Float,
                wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                "dlss-depth-in",
            )?;
            let motion_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::Rg32Float,
                wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                "dlss-motion-in",
            )?;
            let output_out = make_tex(
                out_w,
                out_h,
                wgpu::TextureFormat::Rgba8Unorm,
                wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                "dlss-output",
            )?;

            let color_view = color_in
                .wgpu_texture()
                .create_view(&wgpu::TextureViewDescriptor::default());
            let depth_view = depth_in
                .wgpu_texture()
                .create_view(&wgpu::TextureViewDescriptor::default());
            let motion_view = motion_in
                .wgpu_texture()
                .create_view(&wgpu::TextureViewDescriptor::default());
            let output_view = output_out
                .wgpu_texture()
                .create_view(&wgpu::TextureViewDescriptor::default());

            let ngx_q = dlss_quality_from_ui(quality);
            let flags = ngx::DlssFeatureFlags::MV_LOW_RES | ngx::DlssFeatureFlags::DO_SHARPENING;
            let mut create_res: Option<Result<ngx::FeatureHandle, ngx::Error>> = None;
            interop
                .submit_vulkan_commands(|cmd| {
                    // SAFETY: `cmd` is a valid recording `VkCommandBuffer` from `interop`.
                    create_res = Some(unsafe {
                        ngx.create_dlss_sr(cmd, render_w, render_h, out_w, out_h, ngx_q, flags)
                    });
                    Ok(())
                })
                .map_err(|e| format!("submit_vulkan_commands: {e}"))?;
            let feature = create_res
                .ok_or_else(|| "DLSS feature creation did not run".to_string())?
                .map_err(|e| format!("create_dlss_sr: {e}"))?;
            let eval_params = ngx
                .allocate_parameters()
                .map_err(|e| format!("allocate_parameters: {e}"))?;

            Ok(Self {
                feature,
                eval_params,
                color_in,
                color_view,
                depth_in,
                depth_view,
                motion_in,
                motion_view,
                output_out,
                output_view,
                render_w,
                render_h,
                out_w,
                out_h,
                quality,
                reset_next: true,
            })
        }

        fn vk_info(
            tex: &SharedTexture,
            view: &wgpu::TextureView,
            w: u32,
            h: u32,
            read_write: bool,
        ) -> Result<ngx::VkImageResourceInfo, String> {
            let image = tex
                .vk_image_raw()
                .ok_or_else(|| "missing VkImage".to_string())?;
            let image_view =
                vk_image_view_raw(view).ok_or_else(|| "missing VkImageView".to_string())?;
            let format = tex
                .vk_format_raw()
                .ok_or_else(|| "missing VkFormat".to_string())?;
            Ok(ngx::VkImageResourceInfo {
                image_view,
                image,
                format,
                width: w,
                height: h,
                read_write,
            })
        }

        fn evaluate(
            &mut self,
            interop: &InteropContext,
            jitter: [f32; 2],
            sharpness: f32,
        ) -> Result<(), String> {
            let color = Self::vk_info(
                &self.color_in,
                &self.color_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let depth = Self::vk_info(
                &self.depth_in,
                &self.depth_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let motion = Self::vk_info(
                &self.motion_in,
                &self.motion_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let output = Self::vk_info(
                &self.output_out,
                &self.output_view,
                self.out_w,
                self.out_h,
                true,
            )?;

            let reset = self.reset_next;
            self.reset_next = false;
            let mut eval_res: Option<Result<(), ngx::Error>> = None;
            interop
                .submit_vulkan_commands(|cmd| {
                    // SAFETY: `cmd` is a valid recording `VkCommandBuffer` and all 4
                    // `SharedTexture`s stay alive through `submit_vulkan_commands` completion.
                    unsafe {
                        interop.vk_image_barrier(
                            cmd,
                            color.image,
                            VkImageTransition::TransferDstToShaderRead,
                        )?;
                        interop.vk_image_barrier(
                            cmd,
                            depth.image,
                            VkImageTransition::TransferDstToShaderRead,
                        )?;
                        interop.vk_image_barrier(
                            cmd,
                            motion.image,
                            VkImageTransition::TransferDstToShaderRead,
                        )?;
                        interop.vk_image_barrier(
                            cmd,
                            output.image,
                            VkImageTransition::UndefinedToGeneral,
                        )?;
                        let res = self.feature.evaluate_dlss_sr_with_params(
                            cmd,
                            &mut self.eval_params,
                            color,
                            depth,
                            motion,
                            output,
                            self.render_w,
                            self.render_h,
                            jitter[0],
                            jitter[1],
                            1.0,
                            1.0,
                            sharpness,
                            reset,
                        );
                        interop.vk_image_barrier(
                            cmd,
                            color.image,
                            VkImageTransition::ShaderReadToTransferDst,
                        )?;
                        interop.vk_image_barrier(
                            cmd,
                            depth.image,
                            VkImageTransition::ShaderReadToTransferDst,
                        )?;
                        interop.vk_image_barrier(
                            cmd,
                            motion.image,
                            VkImageTransition::ShaderReadToTransferDst,
                        )?;
                        interop.vk_image_barrier(
                            cmd,
                            output.image,
                            VkImageTransition::GeneralToShaderRead,
                        )?;
                        eval_res = Some(res);
                    }
                    Ok(())
                })
                .map_err(|e| format!("submit_vulkan_commands: {e}"))?;
            eval_res
                .ok_or_else(|| "DLSS evaluate did not run".to_string())?
                .map_err(|e| format!("evaluate_dlss_sr: {e}"))
        }
    }

    pub struct DemoApp {
        window: Arc<Window>,
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
        dlss_pass: Option<DlssPass>,
        cached_dlss_opt: Option<((u32, u32, u32), ngx::DlssOptimalSettings)>,
        ngx_vulkan: Option<ngx::NgxVulkan>,
        ngx_init_error: Option<String>,
        interop: InteropContext,
        egui_ctx: egui::Context,
        egui_winit: egui_winit::State,
        egui_renderer: egui_wgpu::Renderer,
        shared_rgba: [SharedBuffer; 2],
        shared_depth: [SharedBuffer; 2],
        shared_motion: [SharedBuffer; 2],
        retired_buffers: Vec<(u64, [SharedBuffer; 2])>,
        buf_gen: u64,
        render_w: u32,
        render_h: u32,
        desired_vp_size: (f32, f32),
        last_resize_at: Instant,
        front_buf_idx: usize,
        has_frame: bool,
        display_texture: wgpu::Texture,
        display_view: wgpu::TextureView,
        display_tex_id: egui::TextureId,
        using_dlss_tex: bool,
        last_jitter: [f32; 2],
        last_saved_msg: Option<String>,
        cmd_tx: Sender<RenderCmd>,
        frame_rx: Receiver<FrameReady>,
        render_thread: Option<JoinHandle<()>>,
        nvml_dev: Option<nvml::Device>,
        _nvml: Option<&'static nvml::Nvml>,
        profiler: Profiler,
        telemetry: GpuTelemetry,
        last_telemetry_at: Instant,
        last_frame_at: Instant,
        accum_count: u32,
        total_samples: u32,
        rt_ms: f32,
        denoise_ms: f32,
        bloom_ms: f32,
        tonemap_ms: f32,
        fps: f32,
        is_recording: bool,
        record_frames: u64,
        settings: DemoSettings,
        last_sent_settings: DemoSettings,
        demo_anim: crate::demo_anim::DemoAnimator,
        cam_driver: CameraDriver,
        cam_home: CameraPose,
        cam_pose: CameraPose,
        last_sent_camera: CameraFrame,
        cam_mode: NavMode,
        cam_fly_mode: NavMode,
        cam_scheme: NavScheme,
        cam_feel: NavFeel,
        dock_state: DockState<DockTab>,
        dock_style: DockStyle,
        theme: UiTheme,
        viewport_focused: bool,
        win_state: crate::persist::WindowState,
        last_saved_state: crate::persist::PersistedState,
        last_persist_at: Instant,
    }

    impl Drop for DemoApp {
        fn drop(&mut self) {
            self.save_persisted();
            let _ = self.cmd_tx.send(RenderCmd::Shutdown);
            if let Some(handle) = self.render_thread.take() {
                let _ = handle.join();
            }
        }
    }

    impl DemoApp {
        pub fn new(window: Arc<Window>, persisted: crate::persist::PersistedState) -> Self {
            let size = window.inner_size();
            let ngx_exts = ngx::NgxVulkan::required_extensions().ok();
            let instance = if let Some(ref exts) = ngx_exts {
                let inst_strs: Vec<&str> = exts
                    .instance_extensions
                    .iter()
                    .map(String::as_str)
                    .collect();
                nv_wgpu::create_vulkan_instance_with_extensions(&inst_strs).unwrap_or_else(|_| {
                    wgpu::Instance::new(wgpu::InstanceDescriptor {
                        backends: wgpu::Backends::VULKAN,
                        ..wgpu::InstanceDescriptor::new_without_display_handle()
                    })
                })
            } else {
                wgpu::Instance::new(wgpu::InstanceDescriptor {
                    backends: wgpu::Backends::VULKAN,
                    ..wgpu::InstanceDescriptor::new_without_display_handle()
                })
            };
            let surface = instance
                .create_surface(window.clone())
                .expect("wgpu surface");

            let adapters = pollster_block(instance.enumerate_adapters(wgpu::Backends::VULKAN));
            let mut chosen = None;
            let mut used_ngx_exts = false;
            for adapter in adapters {
                if !adapter.is_surface_supported(&surface) {
                    continue;
                }
                if let Some(ref exts) = ngx_exts {
                    let dev_strs: Vec<&str> =
                        exts.device_extensions.iter().map(String::as_str).collect();
                    if let Ok(ctx) = InteropContext::open_vulkan_with_extensions(
                        &adapter,
                        wgpu::Features::empty(),
                        &wgpu::Limits::default(),
                        Some("nv-rtx-demo"),
                        &dev_strs,
                    ) {
                        chosen = Some((adapter, ctx));
                        used_ngx_exts = true;
                        break;
                    }
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

            let (ngx_vulkan, ngx_init_error) = if used_ngx_exts {
                match (
                    ngx::ProjectId::from_guid_str("a7b3c1d2-4e5f-4a6b-8c9d-0e1f2a3b4c5d"),
                    interop.raw_vulkan_handles(),
                ) {
                    (Ok(pid), Some(raw)) => {
                        let data_path = std::env::current_exe()
                            .ok()
                            .and_then(|p| p.parent().map(std::path::Path::to_path_buf))
                            .unwrap_or_else(|| std::path::PathBuf::from("."));
                        let handles = ngx::VulkanDeviceHandles {
                            instance: raw.instance,
                            physical_device: raw.physical_device,
                            device: raw.device,
                            get_instance_proc_addr: raw.get_instance_proc_addr,
                            get_device_proc_addr: raw.get_device_proc_addr,
                        };
                        // SAFETY: `interop` keeps the Vulkan instance/device alive until after `ngx_vulkan` drops.
                        match unsafe { ngx::NgxVulkan::init(&pid, &data_path, handles) } {
                            Ok(v) => (Some(v), None),
                            Err(e) => (None, Some(format!("NGX init failed: {e}"))),
                        }
                    }
                    (Err(e), _) => (None, Some(format!("NGX ProjectId error: {e}"))),
                    (_, None) => (None, Some("NGX requires Vulkan backend".to_string())),
                }
            } else {
                (
                    None,
                    Some("NGX Vulkan extensions unavailable on this host/adapter".to_string()),
                )
            };

            let mut surface_config = surface
                .get_default_config(&adapter, size.width.max(1), size.height.max(1))
                .expect("surface default config");
            surface_config.present_mode = wgpu::PresentMode::AutoVsync;
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
            install_fonts(&egui_ctx);
            egui_ctx.all_styles_mut(|style| {
                style.spacing.item_spacing = egui::vec2(4.0, 1.0);
                style.spacing.window_margin = egui::Margin::same(4);
                style.spacing.button_padding = egui::vec2(5.0, 1.0);
                style.spacing.interact_size = egui::vec2(28.0, 13.0);
                style.spacing.slider_rail_height = 3.0;
                for (ts, fid) in style.text_styles.iter_mut() {
                    fid.size = match ts {
                        egui::TextStyle::Heading => 14.0,
                        egui::TextStyle::Small => 11.5,
                        _ => 12.0,
                    };
                }
            });

            let cam_mode = persisted.camera.nav_mode();
            let cam_fly_mode = persisted.camera.nav_fly_mode();
            let scheme = persisted.camera.nav_scheme();
            let cam_feel = persisted.camera.nav_feel();
            let prefs = NavPrefs {
                mode: cam_mode,
                hold_fly_mode: cam_fly_mode,
                feel: cam_feel,
                ..NavPrefs::default()
            };
            let mut cam_driver = CameraDriver::new(prefs, scheme);
            enable_rmb_fly(&mut cam_driver);
            cam_driver.controller_mut().apply_intent(
                CameraIntent::FrameBounds {
                    min: Vec3::splat(-1.25),
                    max: Vec3::splat(1.25),
                    margin: 1.2,
                },
                1.0 / 60.0,
                ViewportSize::new(DEFAULT_W, DEFAULT_H),
            );
            let cam_home = cam_driver.controller().pose();
            let cam_pose = persisted
                .camera
                .pose
                .and_then(|p| p.to_pose())
                .unwrap_or(cam_home);
            cam_driver.controller_mut().set_pose(cam_pose);
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

            // Double-buffered CUDA<->wgpu SharedBuffers so GUI 60 FPS never stalls on OptiX.
            let RenderTargets {
                rgba: shared_rgba,
                depth: shared_depth,
                motion: shared_motion,
                display_texture,
            } = allocate_render_targets(&interop, DEFAULT_W, DEFAULT_H)
                .expect("initial render targets");
            let buf_ptrs = [shared_rgba[0].cu_deviceptr(), shared_rgba[1].cu_deviceptr()];
            let depth_ptrs = [
                shared_depth[0].cu_deviceptr(),
                shared_depth[1].cu_deviceptr(),
            ];
            let motion_ptrs = [
                shared_motion[0].cu_deviceptr(),
                shared_motion[1].cu_deviceptr(),
            ];
            let display_view = display_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let display_tex_id = egui_renderer.register_native_texture(
                interop.device(),
                &display_view,
                wgpu::FilterMode::Linear,
            );

            let settings = persisted.settings;
            let initial_camera = pose_to_frame(cam_pose);
            let (cmd_tx, frame_rx, render_thread) = spawn_render_thread(
                Arc::clone(interop.cuda_context()),
                buf_ptrs,
                depth_ptrs,
                motion_ptrs,
                settings,
                initial_camera,
            );

            let nvml_lib = nvml::Nvml::init()
                .ok()
                .map(|n| Box::leak(Box::new(n)) as &'static nvml::Nvml);
            let nvml_dev = nvml_lib.and_then(|n| n.device_by_index(0).ok());
            let mut profiler = Profiler::new(Some(interop.cuda_context()));
            let telemetry = profiler.read_telemetry(nvml_dev.as_ref());

            let dock_state = default_dock_state();
            let theme = persisted.theme;
            let dock_style = theme.apply(&egui_ctx);

            Self {
                window,
                surface,
                surface_config,
                dlss_pass: None,
                cached_dlss_opt: None,
                ngx_vulkan,
                ngx_init_error,
                interop,
                egui_ctx,
                egui_winit,
                egui_renderer,
                shared_rgba,
                shared_depth,
                shared_motion,
                retired_buffers: Vec::new(),
                buf_gen: 0,
                render_w: DEFAULT_W,
                render_h: DEFAULT_H,
                desired_vp_size: (DEFAULT_W as f32, DEFAULT_H as f32),
                last_resize_at: Instant::now(),
                front_buf_idx: 0,
                has_frame: false,
                display_texture,
                display_view,
                display_tex_id,
                using_dlss_tex: false,
                last_jitter: [0.0, 0.0],
                last_saved_msg: None,
                cmd_tx,
                frame_rx,
                render_thread: Some(render_thread),
                nvml_dev,
                _nvml: nvml_lib,
                profiler,
                telemetry,
                last_telemetry_at: Instant::now(),
                last_frame_at: Instant::now(),
                accum_count: 0,
                total_samples: 0,
                rt_ms: 0.0,
                denoise_ms: 0.0,
                bloom_ms: 0.0,
                tonemap_ms: 0.0,
                fps: 60.0,
                is_recording: false,
                record_frames: 0,
                settings,
                last_sent_settings: settings,
                demo_anim: persisted.demo_anim.clone(),
                cam_driver,
                cam_home,
                cam_pose,
                last_sent_camera: initial_camera,
                cam_mode,
                cam_fly_mode,
                cam_scheme: scheme,
                cam_feel,
                dock_state,
                dock_style,
                theme,
                viewport_focused: false,
                win_state: persisted.window.sanitized(),
                last_saved_state: persisted,
                last_persist_at: Instant::now(),
            }
        }

        fn current_persisted_state(&self) -> crate::persist::PersistedState {
            crate::persist::PersistedState {
                window: self.win_state,
                theme: self.theme,
                settings: self.settings,
                demo_anim: self.demo_anim.clone(),
                camera: crate::persist::CameraState::from_runtime(
                    self.cam_mode,
                    self.cam_fly_mode,
                    self.cam_scheme,
                    self.cam_feel,
                    self.cam_pose,
                ),
            }
        }

        pub fn save_persisted(&mut self) {
            let cur = self.current_persisted_state();
            if cur != self.last_saved_state {
                let _ = crate::persist::save(&cur);
                self.last_saved_state = cur;
            }
            self.last_persist_at = Instant::now();
        }

        pub fn update_window_geometry(&mut self) {
            let max = self.window.is_maximized();
            self.win_state.maximized = max;
            if !max {
                let sz = self.window.inner_size();
                if sz.width >= 320 && sz.height >= 240 {
                    self.win_state.width = sz.width;
                    self.win_state.height = sz.height;
                }
                if let Ok(pos) = self.window.outer_position() {
                    self.win_state.x = Some(pos.x);
                    self.win_state.y = Some(pos.y);
                }
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

        pub fn handle_mouse_motion(&mut self, delta: (f64, f64)) {
            self.egui_winit.on_mouse_motion(delta);
        }

        pub fn render(&mut self) {
            let now = Instant::now();
            let dt = (now - self.last_frame_at).as_secs_f32().clamp(1e-4, 0.1);
            self.last_frame_at = now;
            self.fps = self.fps * 0.9 + (1.0 / dt) * 0.1;

            if self.last_telemetry_at.elapsed().as_secs_f32() > 0.35 {
                self.profiler
                    .update_telemetry(self.nvml_dev.as_ref(), &mut self.telemetry);
                self.last_telemetry_at = now;
            }

            // 1. Drain completed frames from the async OptiX render thread.
            while let Ok(msg) = self.frame_rx.try_recv() {
                if msg.gen_id != self.buf_gen {
                    continue;
                }
                if self.has_frame && self.front_buf_idx != msg.buf_idx {
                    let _ = self.cmd_tx.send(RenderCmd::ReturnBuffer {
                        buf_idx: self.front_buf_idx,
                        gen_id: self.buf_gen,
                    });
                }
                self.front_buf_idx = msg.buf_idx;
                self.has_frame = true;
                self.accum_count = msg.accum_count;
                self.total_samples = msg.total_samples;
                self.last_jitter = msg.jitter;
                self.rt_ms = msg.rt_ms;
                self.denoise_ms = msg.denoise_ms;
                self.bloom_ms = msg.bloom_ms;
                self.tonemap_ms = msg.tonemap_ms;
                self.is_recording = msg.is_recording;
                self.record_frames = msg.record_frames;
                if let Some(status) = msg.status_msg {
                    self.last_saved_msg = Some(status);
                }
                self.profiler.record_frame(
                    msg.rt_ms,
                    msg.denoise_ms,
                    msg.bloom_ms,
                    msg.tonemap_ms,
                    self.fps,
                );
                self.retired_buffers.retain(|(g, _)| *g >= msg.gen_id);
            }

            // 2. Docked egui pass (60 FPS, zero blocking; camera_egui::drive steps the camera).
            let raw_input = self.egui_winit.take_egui_input(&self.window);
            let mut reset_accum_requested = false;
            let mut save_png_requested = false;
            let mut save_jpeg_requested = false;
            let mut toggle_record_requested = false;
            let mut reload_shader_requested = false;
            let prev_theme = self.theme;
            let mut ui_ctx = UiContext {
                settings: &mut self.settings,
                demo_anim: &mut self.demo_anim,
                display_tex_id: self.display_tex_id,
                render_w: self.render_w,
                render_h: self.render_h,
                desired_vp_size: &mut self.desired_vp_size,
                cam_driver: &mut self.cam_driver,
                cam_home: self.cam_home,
                cam_pose: &mut self.cam_pose,
                cam_mode: &mut self.cam_mode,
                cam_fly_mode: &mut self.cam_fly_mode,
                cam_scheme: &mut self.cam_scheme,
                cam_feel: &mut self.cam_feel,
                theme: &mut self.theme,
                telemetry: &self.telemetry,
                timings: &self.profiler.timings,
                accum_count: self.accum_count,
                total_samples: self.total_samples,
                rt_ms: self.rt_ms,
                denoise_ms: self.denoise_ms,
                bloom_ms: self.bloom_ms,
                tonemap_ms: self.tonemap_ms,
                fps: self.fps,
                is_recording: self.is_recording,
                record_frames: self.record_frames,
                viewport_focused: &mut self.viewport_focused,
                reset_accum_requested: &mut reset_accum_requested,
                save_png_requested: &mut save_png_requested,
                save_jpeg_requested: &mut save_jpeg_requested,
                toggle_record_requested: &mut toggle_record_requested,
                reload_shader_requested: &mut reload_shader_requested,
                last_saved_msg: self.last_saved_msg.as_deref(),
            };
            let full_output = self.egui_ctx.run_ui(raw_input, |ui| {
                egui::Panel::top("top_toolbar").show(ui, |ui| {
                    ui_ctx.ui_toolbar(ui);
                });
                DockArea::new(&mut self.dock_state)
                    .style(self.dock_style.clone())
                    .show_close_buttons(false)
                    .show_leaf_close_all_buttons(false)
                    .show_leaf_collapse_buttons(false)
                    .show_inside(ui, &mut ui_ctx);
            });
            if self.theme != prev_theme {
                self.dock_style = self.theme.apply(&self.egui_ctx);
            }

            // 3. Dynamic viewport resolution resize & DLSS lifecycle.
            if self.settings.dlss_on && self.ngx_vulkan.is_none() {
                self.settings.dlss_on = false;
                self.last_saved_msg = Some(
                    self.ngx_init_error
                        .clone()
                        .unwrap_or_else(|| "DLSS unavailable: NGX not initialized".to_string()),
                );
            }

            let (out_w, out_h) =
                align_render_size(self.desired_vp_size.0, self.desired_vp_size.1, 1.0);
            let (target_w, target_h) = if self.settings.dlss_on {
                if let Some(ref ngx) = self.ngx_vulkan {
                    let key = (out_w, out_h, self.settings.dlss_quality);
                    let opt_res = match self.cached_dlss_opt {
                        Some((k, opt)) if k == key => Ok(opt),
                        _ => {
                            let res = ngx.query_dlss_optimal_settings(
                                out_w,
                                out_h,
                                dlss_quality_from_ui(self.settings.dlss_quality),
                            );
                            if let Ok(opt) = res {
                                self.cached_dlss_opt = Some((key, opt));
                            }
                            res
                        }
                    };
                    match opt_res {
                        Ok(opt) => {
                            let min_w = 256.min(out_w);
                            let min_h = 144.min(out_h);
                            (
                                ((opt.render_width + 31) & !63).clamp(min_w, out_w),
                                (opt.render_height & !1).clamp(min_h, out_h),
                            )
                        }
                        Err(e) => {
                            self.settings.dlss_on = false;
                            self.last_saved_msg = Some(format!("DLSS query failed: {e}"));
                            align_render_size(
                                self.desired_vp_size.0,
                                self.desired_vp_size.1,
                                self.settings.render_scale,
                            )
                        }
                    }
                } else {
                    align_render_size(
                        self.desired_vp_size.0,
                        self.desired_vp_size.1,
                        self.settings.render_scale,
                    )
                }
            } else {
                align_render_size(
                    self.desired_vp_size.0,
                    self.desired_vp_size.1,
                    self.settings.render_scale,
                )
            };

            let dlss_toggled = self.settings.dlss_on != self.last_sent_settings.dlss_on
                || self.settings.dlss_quality != self.last_sent_settings.dlss_quality;
            if (target_w != self.render_w || target_h != self.render_h)
                && (self.last_resize_at.elapsed().as_millis() >= 120 || dlss_toggled)
            {
                self.last_resize_at = Instant::now();
                match allocate_render_targets(&self.interop, target_w, target_h) {
                    Ok(RenderTargets {
                        rgba: new_shared,
                        depth: new_depth,
                        motion: new_motion,
                        display_texture: new_tex,
                    }) => {
                        let new_ptrs = [new_shared[0].cu_deviceptr(), new_shared[1].cu_deviceptr()];
                        let new_depth_ptrs =
                            [new_depth[0].cu_deviceptr(), new_depth[1].cu_deviceptr()];
                        let new_motion_ptrs =
                            [new_motion[0].cu_deviceptr(), new_motion[1].cu_deviceptr()];
                        let new_view = new_tex.create_view(&wgpu::TextureViewDescriptor::default());
                        if !self.using_dlss_tex {
                            self.egui_renderer.update_egui_texture_from_wgpu_texture(
                                self.interop.device(),
                                &new_view,
                                wgpu::FilterMode::Linear,
                                self.display_tex_id,
                            );
                        }
                        let old_shared = std::mem::replace(&mut self.shared_rgba, new_shared);
                        let old_depth = std::mem::replace(&mut self.shared_depth, new_depth);
                        let old_motion = std::mem::replace(&mut self.shared_motion, new_motion);
                        self.retired_buffers.push((self.buf_gen, old_shared));
                        self.retired_buffers.push((self.buf_gen, old_depth));
                        self.retired_buffers.push((self.buf_gen, old_motion));
                        self.display_texture = new_tex;
                        self.display_view = new_view;
                        self.render_w = target_w;
                        self.render_h = target_h;
                        self.buf_gen = self.buf_gen.wrapping_add(1);
                        self.has_frame = false;
                        self.front_buf_idx = 0;
                        self.accum_count = 0;
                        self.total_samples = 0;
                        let _ = self.cmd_tx.send(RenderCmd::Resize {
                            width: target_w,
                            height: target_h,
                            buf_ptrs: new_ptrs,
                            depth_ptrs: new_depth_ptrs,
                            motion_ptrs: new_motion_ptrs,
                            gen_id: self.buf_gen,
                        });
                    }
                    Err(e) => {
                        self.last_saved_msg = Some(format!("Resize failed: {e}"));
                    }
                }
            }

            if self.settings.dlss_on {
                let needs_rebuild = self.dlss_pass.as_ref().is_none_or(|p| {
                    p.render_w != self.render_w
                        || p.render_h != self.render_h
                        || p.out_w != out_w
                        || p.out_h != out_h
                        || p.quality != self.settings.dlss_quality
                });
                if needs_rebuild {
                    if self.using_dlss_tex {
                        self.egui_renderer.update_egui_texture_from_wgpu_texture(
                            self.interop.device(),
                            &self.display_view,
                            wgpu::FilterMode::Linear,
                            self.display_tex_id,
                        );
                        self.using_dlss_tex = false;
                    }
                    self.dlss_pass = None;
                    if let Some(ref ngx) = self.ngx_vulkan {
                        match DlssPass::new(
                            &self.interop,
                            ngx,
                            self.render_w,
                            self.render_h,
                            out_w,
                            out_h,
                            self.settings.dlss_quality,
                        ) {
                            Ok(pass) => {
                                self.egui_renderer.update_egui_texture_from_wgpu_texture(
                                    self.interop.device(),
                                    &pass.output_view,
                                    wgpu::FilterMode::Linear,
                                    self.display_tex_id,
                                );
                                self.using_dlss_tex = true;
                                self.dlss_pass = Some(pass);
                            }
                            Err(e) => {
                                self.settings.dlss_on = false;
                                self.last_saved_msg = Some(format!("DLSS init failed: {e}"));
                            }
                        }
                    }
                }
            } else {
                if self.using_dlss_tex {
                    self.egui_renderer.update_egui_texture_from_wgpu_texture(
                        self.interop.device(),
                        &self.display_view,
                        wgpu::FilterMode::Linear,
                        self.display_tex_id,
                    );
                    self.using_dlss_tex = false;
                }
                self.dlss_pass = None;
            }

            if save_png_requested && self.has_frame {
                self.last_saved_msg = Some(save_front_png(
                    &self.interop,
                    self.shared_rgba[self.front_buf_idx].cu_deviceptr(),
                    self.render_w,
                    self.render_h,
                ));
            }
            if save_jpeg_requested && self.has_frame {
                let dir = std::path::Path::new("renders");
                let _ = std::fs::create_dir_all(dir);
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let path = dir.join(format!("nv-demo-{ts}.jpg"));
                let _ = self.cmd_tx.send(RenderCmd::SaveJpeg { path, quality: 92 });
            }
            if toggle_record_requested {
                let dir = std::path::Path::new("renders");
                let _ = std::fs::create_dir_all(dir);
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let path = dir.join(format!("nv-demo-{ts}.h264"));
                let _ = self.cmd_tx.send(RenderCmd::ToggleRecord { path, fps: 60 });
            }
            if reload_shader_requested {
                let _ = self.cmd_tx.send(RenderCmd::ReloadShader);
            }

            // 4. Advance demo animator & push camera/settings updates when changed.
            self.demo_anim.tick(dt);
            let effective_settings = self.demo_anim.apply(&self.settings);
            let cam_frame = pose_to_frame(self.cam_pose);
            if reset_accum_requested
                || effective_settings != self.last_sent_settings
                || cam_frame != self.last_sent_camera
            {
                self.last_sent_settings = effective_settings;
                self.last_sent_camera = cam_frame;
                let _ = self.cmd_tx.send(RenderCmd::Update {
                    settings: Box::new(effective_settings),
                    camera: cam_frame,
                    reset_accum: reset_accum_requested,
                });
            }

            // 5. wgpu present (copy latest SharedBuffer -> display_texture + render egui).
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

            if self.has_frame {
                if self.settings.dlss_on
                    && let Some(ref mut dlss) = self.dlss_pass
                {
                    let mut copy_enc = self.interop.device().create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {
                            label: Some("dlss-copy-inputs"),
                        },
                    );
                    let extent = wgpu::Extent3d {
                        width: self.render_w,
                        height: self.render_h,
                        depth_or_array_layers: 1,
                    };
                    let copy_buf = |enc: &mut wgpu::CommandEncoder,
                                    buf: &wgpu::Buffer,
                                    tex: &wgpu::Texture,
                                    bpp: u32| {
                        enc.copy_buffer_to_texture(
                            wgpu::TexelCopyBufferInfo {
                                buffer: buf,
                                layout: wgpu::TexelCopyBufferLayout {
                                    offset: 0,
                                    bytes_per_row: Some(self.render_w * bpp),
                                    rows_per_image: Some(self.render_h),
                                },
                            },
                            wgpu::TexelCopyTextureInfo {
                                texture: tex,
                                mip_level: 0,
                                origin: wgpu::Origin3d::ZERO,
                                aspect: wgpu::TextureAspect::All,
                            },
                            extent,
                        );
                    };
                    copy_buf(
                        &mut copy_enc,
                        self.shared_rgba[self.front_buf_idx].wgpu_buffer(),
                        dlss.color_in.wgpu_texture(),
                        4,
                    );
                    copy_buf(
                        &mut copy_enc,
                        self.shared_depth[self.front_buf_idx].wgpu_buffer(),
                        dlss.depth_in.wgpu_texture(),
                        4,
                    );
                    copy_buf(
                        &mut copy_enc,
                        self.shared_motion[self.front_buf_idx].wgpu_buffer(),
                        dlss.motion_in.wgpu_texture(),
                        8,
                    );
                    self.interop.queue().submit(Some(copy_enc.finish()));
                    if let Err(e) = dlss.evaluate(
                        &self.interop,
                        self.last_jitter,
                        self.settings.dlss_sharpness,
                    ) {
                        self.settings.dlss_on = false;
                        if self.using_dlss_tex {
                            self.egui_renderer.update_egui_texture_from_wgpu_texture(
                                self.interop.device(),
                                &self.display_view,
                                wgpu::FilterMode::Linear,
                                self.display_tex_id,
                            );
                            self.using_dlss_tex = false;
                        }
                        self.dlss_pass = None;
                        self.last_saved_msg = Some(format!("DLSS evaluate failed: {e}"));
                    }
                } else {
                    encoder.copy_buffer_to_texture(
                        wgpu::TexelCopyBufferInfo {
                            buffer: self.shared_rgba[self.front_buf_idx].wgpu_buffer(),
                            layout: wgpu::TexelCopyBufferLayout {
                                offset: 0,
                                bytes_per_row: Some(self.render_w * 4),
                                rows_per_image: Some(self.render_h),
                            },
                        },
                        wgpu::TexelCopyTextureInfo {
                            texture: &self.display_texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::Extent3d {
                            width: self.render_w,
                            height: self.render_h,
                            depth_or_array_layers: 1,
                        },
                    );
                }
            }

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
            if self.last_persist_at.elapsed().as_secs_f32() >= 1.5 {
                self.update_window_geometry();
                self.save_persisted();
            }
            self.window.request_redraw();
        }
    }

    struct RenderTargets {
        rgba: [SharedBuffer; 2],
        depth: [SharedBuffer; 2],
        motion: [SharedBuffer; 2],
        display_texture: wgpu::Texture,
    }

    fn allocate_render_targets(
        interop: &InteropContext,
        width: u32,
        height: u32,
    ) -> Result<RenderTargets, nv_wgpu::Error> {
        let pixels = u64::from(width) * u64::from(height);
        let make_shared = |label, bytes| {
            interop.create_buffer(&SharedBufferDescriptor {
                label: Some(label),
                size: bytes,
                usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
            })
        };
        let rgba = [
            make_shared("optix-rgba8-0", pixels * 4)?,
            make_shared("optix-rgba8-1", pixels * 4)?,
        ];
        let depth = [
            make_shared("optix-depth-0", pixels * 4)?,
            make_shared("optix-depth-1", pixels * 4)?,
        ];
        let motion = [
            make_shared("optix-motion-0", pixels * 8)?,
            make_shared("optix-motion-1", pixels * 8)?,
        ];
        let display_texture = interop.device().create_texture(&wgpu::TextureDescriptor {
            label: Some("optix-viewport"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        Ok(RenderTargets {
            rgba,
            depth,
            motion,
            display_texture,
        })
    }

    fn save_front_png(interop: &InteropContext, dev_ptr: u64, width: u32, height: u32) -> String {
        let byte_len = (width * height * 4) as usize;
        let mut host = vec![0u8; byte_len];
        if let Err(e) = interop.cuda_context().bind_to_thread() {
            return format!("CUDA bind failed: {e}");
        }
        // SAFETY: `dev_ptr` points to a `SharedBuffer` of `byte_len` bytes on the active context.
        let res =
            unsafe { cuda_bindings::cuMemcpyDtoH_v2(host.as_mut_ptr().cast(), dev_ptr, byte_len) };
        if res != cuda_bindings::cudaError_enum_CUDA_SUCCESS {
            return format!("CUDA readback failed ({res:?})");
        }
        let dir = std::path::Path::new("renders");
        let _ = std::fs::create_dir_all(dir);
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let path = dir.join(format!("nv-demo-{ts}.png"));
        let Ok(file) = std::fs::File::create(&path) else {
            return format!("Failed to create {}", path.display());
        };
        let w = std::io::BufWriter::new(file);
        let mut enc = png::Encoder::new(w, width, height);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        match enc
            .write_header()
            .and_then(|mut writer| writer.write_image_data(&host))
        {
            Ok(()) => format!("Saved {}", path.display()),
            Err(e) => format!("PNG encode failed: {e}"),
        }
    }

    fn pose_to_frame(pose: CameraPose) -> CameraFrame {
        let eye = pose.eye;
        let fwd = pose.forward();
        let right = pose.right();
        let up = pose.up();
        let tan_half_fov = (pose.fov_y * 0.5).tan();
        CameraFrame {
            eye: [eye.x, eye.y, eye.z],
            fwd: [fwd.x, fwd.y, fwd.z],
            right: [right.x, right.y, right.z],
            up: [up.x, up.y, up.z],
            tan_half_fov,
        }
    }

    pub(crate) fn enable_rmb_fly(driver: &mut CameraDriver) {
        use crate::camera_core::input::{Button, DragBinding, DragRole, Mods};
        let drags = &mut driver.bindings_mut().drags;
        drags.retain(|b| !(b.button == Button::Secondary && b.mods == Mods::NONE));
        drags.push(DragBinding::new(
            Button::Secondary,
            Mods::NONE,
            DragRole::Fly,
        ));
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
            let persisted = crate::persist::load();
            let win_cfg = persisted.window.sanitized();
            let mut attrs = Window::default_attributes()
                .with_title("nv-rs RTX Fractal Path Tracer (OptiX 9 + AI Denoiser + nv-wgpu)")
                .with_inner_size(winit::dpi::PhysicalSize::new(win_cfg.width, win_cfg.height))
                .with_maximized(win_cfg.maximized);
            if let (Some(x), Some(y)) = (win_cfg.x, win_cfg.y) {
                attrs = attrs.with_position(winit::dpi::PhysicalPosition::new(x, y));
            }
            let win = Arc::new(event_loop.create_window(attrs).expect("window"));
            self.app = Some(DemoApp::new(win, persisted));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(app) = self.app.as_mut() else {
            return;
        };
        if matches!(event, WindowEvent::CloseRequested) {
            app.update_window_geometry();
            app.save_persisted();
            event_loop.exit();
            return;
        }
        if let WindowEvent::Resized(s) = event {
            app.resize(s.width, s.height);
            app.update_window_geometry();
        } else if matches!(event, WindowEvent::Moved(_)) {
            app.update_window_geometry();
        } else if matches!(event, WindowEvent::RedrawRequested) {
            app.render();
        }
        app.handle_window_event(&event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let (Some(app), winit::event::DeviceEvent::MouseMotion { delta }) =
            (self.app.as_mut(), event)
        {
            app.handle_mouse_motion(delta);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop");
    let mut runner = Runner::default();
    event_loop.run_app(&mut runner).expect("run app");
}
