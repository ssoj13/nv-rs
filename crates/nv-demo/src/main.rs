//! Interactive RTX OptiX 3D Fractal Path Tracer + AI Denoiser + Zero-Copy `nv-wgpu` + `egui_dock` Demo.
//!
//! Run with:
//! `cargo run -p nv-demo --release`

#[allow(unexpected_cfgs, dead_code, unused_imports, clippy::all)]
mod camera_core;
#[allow(unexpected_cfgs, dead_code, unused_imports, deprecated, clippy::all)]
mod camera_egui;
mod renderer;
mod ui;

use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Instant;

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
    use crate::renderer::{
        CameraFrame, DEFAULT_H, DEFAULT_W, DemoSettings, FrameReady, RenderCmd,
        align_render_size, spawn_render_thread,
    };
    use crate::ui::{DockTab, GpuTelemetry, UiContext, default_dock_state, dock_style, install_fonts};
    use egui_dock::{DockArea, DockState, Style as DockStyle};
    use glam::Vec3;
    use nv_wgpu::{InteropContext, SharedBuffer, SharedBufferDescriptor};

    pub struct DemoApp {
        window: Arc<Window>,
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
        interop: InteropContext,
        egui_ctx: egui::Context,
        egui_winit: egui_winit::State,
        egui_renderer: egui_wgpu::Renderer,
        shared_rgba: [SharedBuffer; 2],
        retired_buffers: Vec<(u64, [SharedBuffer; 2])>,
        buf_gen: u64,
        render_w: u32,
        render_h: u32,
        desired_vp_size: (f32, f32),
        last_resize_at: Instant,
        front_buf_idx: usize,
        has_frame: bool,
        display_texture: wgpu::Texture,
        display_tex_id: egui::TextureId,
        last_saved_msg: Option<String>,
        cmd_tx: Sender<RenderCmd>,
        frame_rx: Receiver<FrameReady>,
        render_thread: Option<JoinHandle<()>>,
        nvml_dev: Option<nvml::Device>,
        _nvml: Option<&'static nvml::Nvml>,
        telemetry: GpuTelemetry,
        last_telemetry_at: Instant,
        last_frame_at: Instant,
        accum_count: u32,
        total_samples: u32,
        rt_ms: f32,
        denoise_ms: f32,
        fps: f32,
        settings: DemoSettings,
        last_sent_settings: DemoSettings,
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
        viewport_focused: bool,
    }

    impl Drop for DemoApp {
        fn drop(&mut self) {
            let _ = self.cmd_tx.send(RenderCmd::Shutdown);
            if let Some(handle) = self.render_thread.take() {
                let _ = handle.join();
            }
        }
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
                style.spacing.item_spacing = egui::vec2(6.0, 4.0);
                style.spacing.window_margin = egui::Margin::same(8);
                style.spacing.button_padding = egui::vec2(8.0, 3.0);
                style.spacing.interact_size = egui::vec2(36.0, 18.0);
                for (ts, fid) in style.text_styles.iter_mut() {
                    fid.size = match ts {
                        egui::TextStyle::Heading => 14.0,
                        egui::TextStyle::Small => 11.5,
                        _ => 12.0,
                    };
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
                ViewportSize::new(DEFAULT_W, DEFAULT_H),
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

            // Double-buffered CUDA<->wgpu SharedBuffers so GUI 60 FPS never stalls on OptiX.
            let (shared_rgba, display_texture) =
                allocate_render_targets(&interop, DEFAULT_W, DEFAULT_H);
            let buf_ptrs = [shared_rgba[0].cu_deviceptr(), shared_rgba[1].cu_deviceptr()];
            let view = display_texture.create_view(&wgpu::TextureViewDescriptor::default());
            let display_tex_id = egui_renderer.register_native_texture(
                interop.device(),
                &view,
                wgpu::FilterMode::Linear,
            );

            let settings = DemoSettings::default();
            let initial_camera = pose_to_frame(cam_pose);
            let (cmd_tx, frame_rx, render_thread) = spawn_render_thread(
                Arc::clone(interop.cuda_context()),
                buf_ptrs,
                settings,
                initial_camera,
            );

            let nvml_lib = nvml::Nvml::init()
                .ok()
                .map(|n| Box::leak(Box::new(n)) as &'static nvml::Nvml);
            let nvml_dev = nvml_lib.and_then(|n| n.device_by_index(0).ok());
            let telemetry = read_telemetry(&nvml_dev);

            let dock_state = default_dock_state();
            let dock_style = dock_style(&egui_ctx);

            Self {
                window,
                surface,
                surface_config,
                interop,
                egui_ctx,
                egui_winit,
                egui_renderer,
                shared_rgba,
                retired_buffers: Vec::new(),
                buf_gen: 0,
                render_w: DEFAULT_W,
                render_h: DEFAULT_H,
                desired_vp_size: (DEFAULT_W as f32, DEFAULT_H as f32),
                last_resize_at: Instant::now(),
                front_buf_idx: 0,
                has_frame: false,
                display_texture,
                display_tex_id,
                last_saved_msg: None,
                cmd_tx,
                frame_rx,
                render_thread: Some(render_thread),
                nvml_dev,
                _nvml: nvml_lib,
                telemetry,
                last_telemetry_at: Instant::now(),
                last_frame_at: Instant::now(),
                accum_count: 0,
                total_samples: 0,
                rt_ms: 0.0,
                denoise_ms: 0.0,
                fps: 60.0,
                settings,
                last_sent_settings: settings,
                cam_driver,
                cam_home,
                cam_pose,
                last_sent_camera: initial_camera,
                cam_mode: prefs.mode,
                cam_fly_mode: prefs.hold_fly_mode,
                cam_scheme: scheme,
                cam_feel: prefs.feel,
                dock_state,
                dock_style,
                viewport_focused: false,
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
            let dt = (now - self.last_frame_at).as_secs_f32().clamp(1e-4, 0.1);
            self.last_frame_at = now;
            self.fps = self.fps * 0.9 + (1.0 / dt) * 0.1;

            if self.last_telemetry_at.elapsed().as_secs_f32() > 0.35 {
                self.telemetry = read_telemetry(&self.nvml_dev);
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
                self.rt_ms = msg.rt_ms;
                self.denoise_ms = msg.denoise_ms;
                self.retired_buffers.retain(|(g, _)| *g >= msg.gen_id);
            }

            // 2. Docked egui pass (60 FPS, zero blocking; camera_egui::drive steps the camera).
            let raw_input = self.egui_winit.take_egui_input(&self.window);
            let mut reset_accum_requested = false;
            let mut save_png_requested = false;
            let mut ui_ctx = UiContext {
                settings: &mut self.settings,
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
                telemetry: &self.telemetry,
                accum_count: self.accum_count,
                total_samples: self.total_samples,
                rt_ms: self.rt_ms,
                denoise_ms: self.denoise_ms,
                fps: self.fps,
                viewport_focused: &mut self.viewport_focused,
                reset_accum_requested: &mut reset_accum_requested,
                save_png_requested: &mut save_png_requested,
                last_saved_msg: self.last_saved_msg.as_deref(),
            };
            let full_output = self.egui_ctx.run_ui(raw_input, |ui| {
                DockArea::new(&mut self.dock_state)
                    .style(self.dock_style.clone())
                    .show_close_buttons(false)
                    .show_leaf_close_all_buttons(false)
                    .show_leaf_collapse_buttons(false)
                    .show_inside(ui, &mut ui_ctx);
            });

            // 3. Dynamic viewport resolution resize (debounced 120ms to avoid thrashing on drag).
            let (target_w, target_h) = align_render_size(
                self.desired_vp_size.0,
                self.desired_vp_size.1,
                self.settings.render_scale,
            );
            if (target_w != self.render_w || target_h != self.render_h)
                && self.last_resize_at.elapsed().as_millis() >= 120
            {
                self.last_resize_at = Instant::now();
                let (new_shared, new_tex) =
                    allocate_render_targets(&self.interop, target_w, target_h);
                let new_ptrs = [new_shared[0].cu_deviceptr(), new_shared[1].cu_deviceptr()];
                let new_view = new_tex.create_view(&wgpu::TextureViewDescriptor::default());
                self.egui_renderer.update_egui_texture_from_wgpu_texture(
                    self.interop.device(),
                    &new_view,
                    wgpu::FilterMode::Linear,
                    self.display_tex_id,
                );
                let old_shared = std::mem::replace(&mut self.shared_rgba, new_shared);
                self.retired_buffers.push((self.buf_gen, old_shared));
                self.display_texture = new_tex;
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
                    gen_id: self.buf_gen,
                });
            }

            if save_png_requested && self.has_frame {
                self.last_saved_msg = Some(save_front_png(
                    &self.interop,
                    self.shared_rgba[self.front_buf_idx].cu_deviceptr(),
                    self.render_w,
                    self.render_h,
                ));
            }

            // 4. Push camera/settings updates to render thread only when changed.
            let cam_frame = pose_to_frame(self.cam_pose);
            if reset_accum_requested
                || self.settings != self.last_sent_settings
                || cam_frame != self.last_sent_camera
            {
                self.last_sent_settings = self.settings;
                self.last_sent_camera = cam_frame;
                let _ = self.cmd_tx.send(RenderCmd::Update {
                    settings: Box::new(self.settings),
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

    fn allocate_render_targets(
        interop: &InteropContext,
        width: u32,
        height: u32,
    ) -> ([SharedBuffer; 2], wgpu::Texture) {
        let byte_size = u64::from(width * height * 4);
        let make_shared = |label| {
            interop
                .create_buffer(&SharedBufferDescriptor {
                    label: Some(label),
                    size: byte_size,
                    usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
                })
                .expect("SharedBuffer")
        };
        let shared = [make_shared("optix-rgba8-0"), make_shared("optix-rgba8-1")];
        let tex = interop.device().create_texture(&wgpu::TextureDescriptor {
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
        (shared, tex)
    }

    fn save_front_png(interop: &InteropContext, dev_ptr: u64, width: u32, height: u32) -> String {
        let byte_len = (width * height * 4) as usize;
        let mut host = vec![0u8; byte_len];
        interop
            .cuda_context()
            .bind_to_thread()
            .expect("cuda bind_to_thread for save_png");
        // SAFETY: `dev_ptr` points to a `SharedBuffer` of `byte_len` bytes on the active context.
        let res = unsafe {
            cuda_bindings::cuMemcpyDtoH_v2(host.as_mut_ptr().cast(), dev_ptr, byte_len)
        };
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
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
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
