//! Interactive RTX OptiX 3D Fractal Path Tracer + AI Denoiser + Zero-Copy `nv-wgpu` + `egui_dock` Demo.
//!
//! Run with:
//! `cargo run -p nv-demo --release`

mod app_events;
mod bench;
mod camera_rail;
mod csg_scene_host;
mod cuda_crowd;
mod demo_anim;
mod fft_bloom;
mod flight_stats;
mod gpu_export;
mod is_counters;
mod mp4_muxer;
mod nvtx;
mod optix_sdk;
mod overlay;
mod perf_log;
mod persist;
mod pick;
mod profiler;
mod renderer;
mod sdg_export;
mod semantic_filter;
mod ui;
mod viewport_toolbar;
mod wireframe;

use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use demo_app::{DemoApp, enable_rmb_fly};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

mod demo_app {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::sync::{Mutex, mpsc};

    use crate::app_events::{
        AppBus, INTERACTION_SETTLE_WINDOW, PossessInfo, RenderControl, RenderEvent,
        RenderStateSnapshot, SmartViewResult, StillFormat, UiCommand,
    };
    use crate::profiler::{GpuTelemetry, Profiler};
    use crate::renderer::{
        CameraFrame, CaptureSource, DEFAULT_H, DEFAULT_W, DLSS_QUALITY_MODES, DemoSettings,
        DlssRrBufferPtrs, align_render_size, dlss_quality_index,
        spawn_render_thread,
    };
    use crate::ui::{DockTab, UiContext, UiScratch, UiTheme, default_dock_state, install_fonts};
    use appbus::downcast_event;
    use bytemuck::{Pod, Zeroable};
    use camera::input::NavScheme;
    use camera::{
        CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, NavPrefs, ViewportSize,
    };
    use egui_dock::{DockArea, DockState, Style as DockStyle};
    use glam::Vec3;
    use nv_wgpu::{
        InteropContext, SharedBuffer, SharedBufferDescriptor, SharedTexture,
        SharedTextureDescriptor, VkImageTransition, vk_image_view_raw,
    };

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
                wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
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

            let ngx_q = DLSS_QUALITY_MODES[dlss_quality_index(quality)].ngx;
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
                            // NGX expects projection-space jitter (opposite of our ray sample offset).
                            -jitter[0],
                            -jitter[1],
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

    const HDR_TONEMAP_WGSL: &str = r#"
struct TonemapParams {
    exposure: f32,
    width: u32,
    height: u32,
    _pad: u32,
};

@group(0) @binding(0) var src_hdr: texture_2d<f32>;
@group(0) @binding(1) var dst_ldr: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform> u: TonemapParams;

fn aces_channel(v: f32) -> f32 {
    let x = max(v, 0.0);
    let num = x * (2.51 * x + 0.03);
    let den = x * (2.43 * x + 0.59) + 0.14;
    let r = clamp(num / den, 0.0, 1.0);
    return pow(r, 1.0 / 2.2);
}

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= u.width || gid.y >= u.height) {
        return;
    }
    let c = textureLoad(src_hdr, vec2<i32>(gid.xy), 0);
    let e = max(u.exposure, 0.0);
    let rgb = vec3<f32>(
        aces_channel(c.r * e),
        aces_channel(c.g * e),
        aces_channel(c.b * e),
    );
    textureStore(dst_ldr, vec2<i32>(gid.xy), vec4<f32>(rgb, 1.0));
}
"#;

    const HDR_SCRGB_WGSL: &str = r#"
struct TonemapParams {
    exposure: f32,
    width: u32,
    height: u32,
    _pad: u32,
};

@group(0) @binding(0) var src_hdr: texture_2d<f32>;
@group(0) @binding(1) var dst_scrgb: texture_storage_2d<rgba16float, write>;
@group(0) @binding(2) var<uniform> u: TonemapParams;

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= u.width || gid.y >= u.height) {
        return;
    }
    let c = textureLoad(src_hdr, vec2<i32>(gid.xy), 0);
    let e = max(u.exposure, 0.0);
    var nits = max(c.rgb * e, vec3<f32>(0.0)) * 203.0;
    let mx = max(nits.r, max(nits.g, nits.b));
    let knee = 750.0;
    let peak = 1000.0;
    if (mx > knee) {
        let excess = mx - knee;
        let span = peak - knee;
        let compressed = knee + span * (excess / (excess + span));
        nits = nits * (compressed / mx);
    }
    textureStore(dst_scrgb, vec2<i32>(gid.xy), vec4<f32>(nits * 0.0125, 1.0));
}
"#;

    #[repr(C)]
    #[derive(Clone, Copy, Pod, Zeroable)]
    struct HdrTonemapUniform {
        exposure: f32,
        width: u32,
        height: u32,
        _pad: u32,
    }

    struct HdrTonemapPipeline {
        pipeline: wgpu::ComputePipeline,
        bgl: wgpu::BindGroupLayout,
        scrgb_pipeline: wgpu::ComputePipeline,
        scrgb_bgl: wgpu::BindGroupLayout,
        uniform_buf: wgpu::Buffer,
    }

    impl HdrTonemapPipeline {
        fn new(device: &wgpu::Device) -> Self {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("dlss-rr-hdr-tonemap-wgsl"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(HDR_TONEMAP_WGSL)),
            });
            let scrgb_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("dlss-rr-hdr-scrgb-wgsl"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(HDR_SCRGB_WGSL)),
            });
            let make_bgl = |label, dst_fmt| {
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(label),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                format: dst_fmt,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                })
            };
            let bgl = make_bgl("dlss-rr-hdr-tonemap-bgl", wgpu::TextureFormat::Rgba8Unorm);
            let scrgb_bgl = make_bgl("dlss-rr-hdr-scrgb-bgl", wgpu::TextureFormat::Rgba16Float);
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("dlss-rr-hdr-tonemap-layout"),
                bind_group_layouts: &[Some(&bgl)],
                immediate_size: 0,
            });
            let scrgb_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("dlss-rr-hdr-scrgb-layout"),
                bind_group_layouts: &[Some(&scrgb_bgl)],
                immediate_size: 0,
            });
            let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("dlss-rr-hdr-tonemap"),
                layout: Some(&layout),
                module: &shader,
                entry_point: Some("main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });
            let scrgb_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("dlss-rr-hdr-scrgb"),
                layout: Some(&scrgb_layout),
                module: &scrgb_shader,
                entry_point: Some("main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });
            let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("dlss-rr-hdr-tonemap-uniform"),
                size: std::mem::size_of::<HdrTonemapUniform>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            Self {
                pipeline,
                bgl,
                scrgb_pipeline,
                scrgb_bgl,
                uniform_buf,
            }
        }

        fn create_bind_group(
            &self,
            device: &wgpu::Device,
            src_hdr_view: &wgpu::TextureView,
            dst_ldr_view: &wgpu::TextureView,
        ) -> wgpu::BindGroup {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("dlss-rr-hdr-tonemap-bg"),
                layout: &self.bgl,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(src_hdr_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(dst_ldr_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.uniform_buf.as_entire_binding(),
                    },
                ],
            })
        }

        fn create_scrgb_bind_group(
            &self,
            device: &wgpu::Device,
            src_hdr_view: &wgpu::TextureView,
            dst_scrgb_view: &wgpu::TextureView,
        ) -> wgpu::BindGroup {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("dlss-rr-hdr-scrgb-bg"),
                layout: &self.scrgb_bgl,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(src_hdr_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(dst_scrgb_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.uniform_buf.as_entire_binding(),
                    },
                ],
            })
        }

        fn dispatch(
            &self,
            queue: &wgpu::Queue,
            encoder: &mut wgpu::CommandEncoder,
            bind_group: &wgpu::BindGroup,
            (width, height): (u32, u32),
            exposure: f32,
            hdr_scrgb: bool,
        ) {
            let uniform = HdrTonemapUniform {
                exposure,
                width,
                height,
                _pad: 0,
            };
            queue.write_buffer(&self.uniform_buf, 0, bytemuck::bytes_of(&uniform));
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("dlss-rr-hdr-tonemap-pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(if hdr_scrgb {
                &self.scrgb_pipeline
            } else {
                &self.pipeline
            });
            pass.set_bind_group(0, bind_group, &[]);
            pass.dispatch_workgroups(width.div_ceil(16), height.div_ceil(16), 1);
        }
    }

    struct DlssRrSharedBuffers {
        hdr_color: [SharedBuffer; 2],
        diffuse_albedo: [SharedBuffer; 2],
        spec_albedo: [SharedBuffer; 2],
        normal_rough: [SharedBuffer; 2],
    }

    impl DlssRrSharedBuffers {
        fn new(interop: &InteropContext, width: u32, height: u32) -> Result<Self, nv_wgpu::Error> {
            let bytes = u64::from(width) * u64::from(height) * 8; // Rgba16Float
            let make_pair = |prefix: &str| -> Result<[SharedBuffer; 2], nv_wgpu::Error> {
                let b0 = interop.create_buffer(&SharedBufferDescriptor {
                    label: Some(&format!("{prefix}-0")),
                    size: bytes,
                    usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
                })?;
                let b1 = interop.create_buffer(&SharedBufferDescriptor {
                    label: Some(&format!("{prefix}-1")),
                    size: bytes,
                    usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
                })?;
                Ok([b0, b1])
            };
            Ok(Self {
                hdr_color: make_pair("optix-rr-hdr")?,
                diffuse_albedo: make_pair("optix-rr-diff")?,
                spec_albedo: make_pair("optix-rr-spec")?,
                normal_rough: make_pair("optix-rr-norm")?,
            })
        }

        fn ptrs(&self) -> DlssRrBufferPtrs {
            DlssRrBufferPtrs {
                hdr_color: [
                    self.hdr_color[0].cu_deviceptr(),
                    self.hdr_color[1].cu_deviceptr(),
                ],
                diffuse_albedo: [
                    self.diffuse_albedo[0].cu_deviceptr(),
                    self.diffuse_albedo[1].cu_deviceptr(),
                ],
                spec_albedo: [
                    self.spec_albedo[0].cu_deviceptr(),
                    self.spec_albedo[1].cu_deviceptr(),
                ],
                normal_rough: [
                    self.normal_rough[0].cu_deviceptr(),
                    self.normal_rough[1].cu_deviceptr(),
                ],
            }
        }

        fn retire_into(self, gen_id: u64, retired: &mut Vec<(u64, [SharedBuffer; 2])>) {
            retired.push((gen_id, self.hdr_color));
            retired.push((gen_id, self.diffuse_albedo));
            retired.push((gen_id, self.spec_albedo));
            retired.push((gen_id, self.normal_rough));
        }
    }

    struct DlssRrPass {
        feature: ngx::FeatureHandle,
        eval_params: ngx::Parameters,
        color_in: SharedTexture,
        color_view: wgpu::TextureView,
        depth_in: SharedTexture,
        depth_view: wgpu::TextureView,
        motion_in: SharedTexture,
        motion_view: wgpu::TextureView,
        diffuse_in: SharedTexture,
        diffuse_view: wgpu::TextureView,
        specular_in: SharedTexture,
        specular_view: wgpu::TextureView,
        normals_in: SharedTexture,
        normals_view: wgpu::TextureView,
        output_hdr: SharedTexture,
        output_hdr_view: wgpu::TextureView,
        display_ldr: wgpu::Texture,
        display_view: wgpu::TextureView,
        tonemap_bg: wgpu::BindGroup,
        display_hdr: wgpu::Texture,
        display_hdr_view: wgpu::TextureView,
        scrgb_bg: wgpu::BindGroup,
        render_w: u32,
        render_h: u32,
        out_w: u32,
        out_h: u32,
        quality: u32,
        reset_next: bool,
    }

    impl DlssRrPass {
        #[expect(
            clippy::too_many_arguments,
            reason = "explicit render/output dimensions and shared resources for DLSS-RR pass setup"
        )]
        fn new(
            interop: &InteropContext,
            ngx: &ngx::NgxVulkan,
            tonemap: &HdrTonemapPipeline,
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
            let in_usage = wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING;
            let color_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::Rgba16Float,
                in_usage,
                "dlss-rr-color-in",
            )?;
            let depth_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::R32Float,
                in_usage,
                "dlss-rr-depth-in",
            )?;
            let motion_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::Rg32Float,
                in_usage,
                "dlss-rr-motion-in",
            )?;
            let diffuse_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::Rgba16Float,
                in_usage,
                "dlss-rr-diffuse-in",
            )?;
            let specular_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::Rgba16Float,
                in_usage,
                "dlss-rr-specular-in",
            )?;
            let normals_in = make_tex(
                render_w,
                render_h,
                wgpu::TextureFormat::Rgba16Float,
                in_usage,
                "dlss-rr-normals-in",
            )?;
            let output_hdr = make_tex(
                out_w,
                out_h,
                wgpu::TextureFormat::Rgba16Float,
                wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                "dlss-rr-output-hdr",
            )?;

            let display_ldr = interop.device().create_texture(&wgpu::TextureDescriptor {
                label: Some("dlss-rr-display-ldr"),
                size: wgpu::Extent3d {
                    width: out_w,
                    height: out_h,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });
            let display_hdr = interop.device().create_texture(&wgpu::TextureDescriptor {
                label: Some("dlss-rr-display-hdr"),
                size: wgpu::Extent3d {
                    width: out_w,
                    height: out_h,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba16Float,
                usage: wgpu::TextureUsages::STORAGE_BINDING
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });

            let view_of = |t: &wgpu::Texture| t.create_view(&wgpu::TextureViewDescriptor::default());
            let color_view = view_of(color_in.wgpu_texture());
            let depth_view = view_of(depth_in.wgpu_texture());
            let motion_view = view_of(motion_in.wgpu_texture());
            let diffuse_view = view_of(diffuse_in.wgpu_texture());
            let specular_view = view_of(specular_in.wgpu_texture());
            let normals_view = view_of(normals_in.wgpu_texture());
            let output_hdr_view = view_of(output_hdr.wgpu_texture());
            let display_view = view_of(&display_ldr);
            let tonemap_bg =
                tonemap.create_bind_group(interop.device(), &output_hdr_view, &display_view);
            let display_hdr_view = view_of(&display_hdr);
            let scrgb_bg = tonemap.create_scrgb_bind_group(
                interop.device(),
                &output_hdr_view,
                &display_hdr_view,
            );

            let config = ngx::DlssRrCreateParams {
                render_width: render_w,
                render_height: render_h,
                out_width: out_w,
                out_height: out_h,
                quality: DLSS_QUALITY_MODES[dlss_quality_index(quality)].ngx,
                flags: ngx::DlssFeatureFlags::IS_HDR | ngx::DlssFeatureFlags::MV_LOW_RES,
                roughness_mode: ngx::DlssRoughnessMode::Packed,
                depth_type: ngx::DlssDepthType::Linear,
                preset: ngx::DlssRrPreset::Default,
            };
            let mut create_res: Option<Result<ngx::FeatureHandle, ngx::Error>> = None;
            interop
                .submit_vulkan_commands(|cmd| {
                    // SAFETY: `cmd` is a valid recording `VkCommandBuffer` from `interop`.
                    create_res = Some(unsafe { ngx.create_dlss_rr(cmd, &config) });
                    Ok(())
                })
                .map_err(|e| format!("submit_vulkan_commands: {e}"))?;
            let feature = create_res
                .ok_or_else(|| "DLSS-RR feature creation did not run".to_string())?
                .map_err(|e| format!("create_dlss_rr: {e}"))?;
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
                diffuse_in,
                diffuse_view,
                specular_in,
                specular_view,
                normals_in,
                normals_view,
                output_hdr,
                output_hdr_view,
                display_ldr,
                display_view,
                tonemap_bg,
                display_hdr,
                display_hdr_view,
                scrgb_bg,
                render_w,
                render_h,
                out_w,
                out_h,
                quality,
                reset_next: true,
            })
        }

        fn evaluate(
            &mut self,
            interop: &InteropContext,
            jitter: [f32; 2],
            world_to_view: [f32; 16],
            view_to_clip: [f32; 16],
        ) -> Result<(), String> {
            let color = DlssPass::vk_info(
                &self.color_in,
                &self.color_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let depth = DlssPass::vk_info(
                &self.depth_in,
                &self.depth_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let motion = DlssPass::vk_info(
                &self.motion_in,
                &self.motion_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let diffuse = DlssPass::vk_info(
                &self.diffuse_in,
                &self.diffuse_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let specular = DlssPass::vk_info(
                &self.specular_in,
                &self.specular_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let normals = DlssPass::vk_info(
                &self.normals_in,
                &self.normals_view,
                self.render_w,
                self.render_h,
                false,
            )?;
            let output = DlssPass::vk_info(
                &self.output_hdr,
                &self.output_hdr_view,
                self.out_w,
                self.out_h,
                true,
            )?;

            let reset = self.reset_next;
            self.reset_next = false;
            let eval = ngx::DlssRrEvalParams {
                color,
                depth,
                motion,
                diffuse_albedo: diffuse,
                specular_albedo: specular,
                normals,
                roughness: None,
                output,
                render_width: self.render_w,
                render_height: self.render_h,
                jitter_x: -jitter[0],
                jitter_y: -jitter[1],
                mv_scale_x: 1.0,
                mv_scale_y: 1.0,
                pre_exposure: 1.0,
                exposure_scale: 1.0,
                reset,
                world_to_view: Some(world_to_view),
                view_to_clip: Some(view_to_clip),
            };
            let mut eval_res: Option<Result<(), ngx::Error>> = None;
            let in_images = [
                color.image,
                depth.image,
                motion.image,
                diffuse.image,
                specular.image,
                normals.image,
            ];
            interop
                .submit_vulkan_commands(|cmd| {
                    // SAFETY: `cmd` is a valid recording `VkCommandBuffer` and all 7
                    // `SharedTexture`s stay alive through `submit_vulkan_commands` completion.
                    unsafe {
                        for img in in_images {
                            interop.vk_image_barrier(
                                cmd,
                                img,
                                VkImageTransition::TransferDstToShaderRead,
                            )?;
                        }
                        interop.vk_image_barrier(
                            cmd,
                            output.image,
                            VkImageTransition::UndefinedToGeneral,
                        )?;
                        let res = self.feature.evaluate_dlss_rr_with_params(
                            cmd,
                            &mut self.eval_params,
                            &eval,
                        );
                        for img in in_images {
                            interop.vk_image_barrier(
                                cmd,
                                img,
                                VkImageTransition::ShaderReadToTransferDst,
                            )?;
                        }
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
                .ok_or_else(|| "DLSS-RR evaluate did not run".to_string())?
                .map_err(|e| format!("evaluate_dlss_rr: {e}"))
        }
    }

    struct FullWindowCapture {
        texture: wgpu::Texture,
        view: wgpu::TextureView,
        copy_buf: SharedBuffer,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        pixel_format: crate::gpu_export::SourcePixelFormat,
        copy_bpr: u32,
        copy_pitch_px: u32,
    }

    impl FullWindowCapture {
        fn new(
            interop: &InteropContext,
            width: u32,
            height: u32,
            format: wgpu::TextureFormat,
        ) -> Result<Self, String> {
            let pixel_format = crate::gpu_export::SourcePixelFormat::from_wgpu(format)
                .ok_or_else(|| format!("unsupported window capture format {format:?}"))?;
            let (copy_bpr, copy_pitch_px) =
                crate::gpu_export::copy_pitch_for_width(width, pixel_format.bytes_per_pixel())
                    .ok_or_else(|| format!("invalid window capture dimensions ({width}x{height})"))?;
            let texture = interop.device().create_texture(&wgpu::TextureDescriptor {
                label: Some("full-window-capture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let copy_bytes = u64::from(copy_bpr)
                .checked_mul(u64::from(height))
                .ok_or_else(|| "window capture buffer size overflow".to_string())?;
            let copy_buf = interop
                .create_buffer(&SharedBufferDescriptor {
                    label: Some("full-window-capture-copy"),
                    size: copy_bytes,
                    usage: wgpu::BufferUsages::COPY_DST,
                })
                .map_err(|e| format!("window capture buffer: {e}"))?;
            Ok(Self {
                texture,
                view,
                copy_buf,
                width,
                height,
                format,
                pixel_format,
                copy_bpr,
                copy_pitch_px,
            })
        }

        fn matches(&self, width: u32, height: u32, format: wgpu::TextureFormat) -> bool {
            self.width == width && self.height == height && self.format == format
        }
    }

    struct ActiveRecorder {
        session_id: u64,
        profile: crate::gpu_export::VideoProfile,
        source: CaptureSource,
        pixel_format: crate::gpu_export::SourcePixelFormat,
        width: u32,
        height: u32,
        fps: u32,
        copy_buf: Option<SharedBuffer>,
        copy_pitch_px: u32,
        last_frame_at: Option<Instant>,
        hdr_wait_frames: u32,
    }

    /// Maximum queued video frames in flight on [`ExportWorker`] to bound VRAM staging memory.
    const MAX_VIDEO_FRAMES_IN_FLIGHT: usize = 4;

    /// Maximum UI frames to wait for async `shared_hdr` resize before aborting PQ10 recording.
    const MAX_PQ10_HDR_WAIT_FRAMES: u32 = 8;

    /// Returns `true` when DLSS-RR owns the live Viewport output (`aov_mode == 0` with an active pass).
    pub(crate) fn viewport_uses_dlss_rr_output(
        dlss_rr_on: bool,
        aov_mode: u32,
        has_rr_pass: bool,
    ) -> bool {
        dlss_rr_on && aov_mode == 0 && has_rr_pass
    }

    /// Returns `true` when `DemoApp` must allocate `shared_hdr` (`Rgba16Float` scRGB buffers).
    ///
    /// Crucially, when recording Viewport `PQ10` while `dlss_rr_on && aov_mode != 0 && !hdr_active`,
    /// DLSS-RR does not produce AOV output so OptiX's `shared_hdr` buffer is required to prevent an
    /// infinite wait stall in the recorder.
    pub(crate) fn need_shared_hdr_buffer(
        hdr_active: bool,
        recording_vp_pq10: bool,
        dlss_rr_on: bool,
        aov_mode: u32,
        has_rr_pass: bool,
    ) -> bool {
        hdr_active
            || (recording_vp_pq10
                && !viewport_uses_dlss_rr_output(dlss_rr_on, aov_mode, has_rr_pass))
    }

    /// Returns `true` when DLSS-RR's post-upscale HDR/LDR tonemap pass must be dispatched.
    ///
    /// Uses `Option<f32>` + finiteness checks instead of an `f32::NAN` sentinel so initial frames
    /// and exposure changes after format/pass resets always refresh `display_hdr`/`display_ldr`.
    pub(crate) fn should_dispatch_rr_tonemap(
        ran_rr: bool,
        last_exposure: Option<f32>,
        exposure: f32,
    ) -> bool {
        ran_rr
            || last_exposure.is_none_or(|prev| {
                !prev.is_finite() || !exposure.is_finite() || (exposure - prev).abs() > 1e-5
            })
    }

    /// Returns `true` when the Full-Window offscreen capture + its bounded `device.poll`
    /// ([`GPU_COPY_WAIT_TIMEOUT`]) should execute on the current UI frame (skips idle video frames when `frames_received == 0`).
    pub(crate) fn should_capture_full_window(
        recording_full_win: bool,
        frames_received: u32,
        capture_png: bool,
        capture_jpeg: bool,
    ) -> bool {
        capture_png || capture_jpeg || (recording_full_win && frames_received > 0)
    }

    /// Constructs a lockstep shutter subframe [`camera::input::RawInput`] preserving
    /// active drags, held WASD fly keys, modifiers, and `keys_allowed` while clearing one-shot
    /// per-UI-frame deltas/edges that were already applied on the UI pass.
    pub(crate) fn lockstep_subframe_raw_input(
        base: Option<camera::input::RawInput>,
        sub_dt: f32,
        width: u32,
        height: u32,
    ) -> camera::input::RawInput {
        use camera::input::{ButtonSet, KeySet, RawInput, Wheel};
        match base {
            Some(mut raw) => {
                raw.pointer_delta = glam::Vec2::ZERO;
                raw.buttons_pressed = ButtonSet::EMPTY;
                raw.buttons_released = ButtonSet::EMPTY;
                raw.keys_pressed = KeySet::EMPTY;
                raw.wheel = Wheel::default();
                raw.pinch = 1.0;
                raw.dt = sub_dt;
                raw
            }
            None => RawInput {
                dt: sub_dt,
                viewport: glam::Vec2::new(width.max(1) as f32, height.max(1) as f32),
                ..Default::default()
            },
        }
    }

    /// Output directory for stills, videos and SDG exports (relative to the working directory).
    const RENDERS_DIR: &str = "renders";
    /// Encoded frame rate of every MP4 recording and of the Smooth 60 lockstep clock.
    const RECORD_FPS: u32 = 60;
    /// Duration of the damper glide into a newly started or switched rail (seconds).
    const RAIL_BLEND_SECS: f32 = 1.6;
    /// Longest UI-thread wait for one GPU copy submission before the capture is dropped.
    const GPU_COPY_WAIT_TIMEOUT: Duration = Duration::from_secs(2);

    /// Process-wide sequence number keeping output names unique within one millisecond.
    static OUTPUT_SEQ: AtomicU64 = AtomicU64::new(0);

    /// Unique output file or directory name `{prefix}-{unix_ms}-{seq}{suffix}` (F03: two saves or
    /// a stop/start within one second must not reuse a name and replace the earlier file).
    pub(crate) fn output_file_name(prefix: &str, suffix: &str) -> String {
        // A clock before 1970 only loses the timestamp part; `seq` keeps names unique.
        let ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis());
        let seq = OUTPUT_SEQ.fetch_add(1, Ordering::Relaxed);
        format!("{prefix}-{ms}-{seq}{suffix}")
    }

    /// Returns `buf` to `pool`. A full pool first evicts an entry of another size, so a size change
    /// (new recording resolution) does not leave the pool full of buffers no request can reuse.
    /// Returns the buffer the caller has to drop, if any.
    pub(crate) fn recycle_into_pool<T>(
        pool: &mut Vec<T>,
        buf: T,
        size_of: impl Fn(&T) -> usize,
        max: usize,
    ) -> Option<T> {
        if pool.len() < max {
            pool.push(buf);
            return None;
        }
        let size = size_of(&buf);
        match pool.iter().position(|b| size_of(b) != size) {
            Some(stale) => {
                let evicted = pool.swap_remove(stale);
                pool.push(buf);
                Some(evicted)
            }
            None => Some(buf),
        }
    }

    /// Duration of the next encoded frame. Live RT measures from the last *accepted* frame, so a
    /// frame dropped under back-pressure lengthens the next one instead of shortening the clip.
    pub(crate) fn video_frame_duration(
        fixed_fps: bool,
        fps: u32,
        last_accepted: Option<Instant>,
        now: Instant,
    ) -> f64 {
        let nominal = 1.0 / f64::from(fps.max(1));
        if fixed_fps {
            return nominal;
        }
        last_accepted
            .map_or(nominal, |prev| now.duration_since(prev).as_secs_f64())
            .clamp(1.0 / 240.0, 2.0)
    }

    /// DLSS render size: the width comes from the runtime's optimal settings aligned to 64 (the
    /// 256-byte `bytes_per_row` rule for 4-byte copies) and the height is derived from that width
    /// and the output aspect, so the render target and the upscaled output share one aspect ratio
    /// (the shader's `aspect = width / height` would otherwise stretch the image).
    pub(crate) fn dlss_render_dims(
        opt: &ngx::DlssOptimalSettings,
        out_w: u32,
        out_h: u32,
    ) -> (u32, u32) {
        let max_w = (opt.max_width.min(out_w) & !63).max(64);
        let min_w = ((opt.min_width.max(64) + 63) & !63).min(max_w);
        let w = ((opt.render_width + 31) & !63).clamp(min_w, max_w);
        let h_exact = u64::from(w) * u64::from(out_h) / u64::from(out_w.max(1));
        let h_even = u32::try_from(h_exact).unwrap_or(out_h).saturating_add(1) & !1;
        let floor_h = 144.min(out_h);
        (w, h_even.clamp(floor_h, out_h.max(floor_h)))
    }

    /// Which texture or buffer holds the image the Viewport currently shows.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum ViewportStillSource {
        /// OptiX tonemapped `shared_rgba` (no upscaler, or an AOV under DLSS-RR).
        RenderTarget,
        /// DLSS-RR LDR display texture (SDR view).
        DlssRrLdr,
        /// DLSS-RR scRGB display texture (HDR view).
        DlssRrHdr,
        /// DLSS-SR upscaled output texture.
        DlssSr,
    }

    /// Picks the still-capture source matching what the viewport draws (F01/F10): under DLSS the
    /// worker's `shared_rgba` is the stale or pre-upscale render-resolution image.
    pub(crate) fn viewport_still_source(
        dlss_rr_on: bool,
        aov_mode: u32,
        has_rr_pass: bool,
        dlss_on: bool,
        has_dlss_pass: bool,
        hdr_active: bool,
    ) -> ViewportStillSource {
        if viewport_uses_dlss_rr_output(dlss_rr_on, aov_mode, has_rr_pass) {
            if hdr_active {
                ViewportStillSource::DlssRrHdr
            } else {
                ViewportStillSource::DlssRrLdr
            }
        } else if dlss_on && !dlss_rr_on && has_dlss_pass {
            ViewportStillSource::DlssSr
        } else {
            ViewportStillSource::RenderTarget
        }
    }

    /// Lighting fields a [`TodTransition`] animates, plus the scene selection.
    #[derive(Clone, Copy, Debug)]
    pub(crate) struct LightingKey {
        sun_azimuth: f32,
        sun_elevation: f32,
        sun_intensity: f32,
        sky_intensity: f32,
        exposure: f32,
        sky_mode: u32,
        csg_mode: u32,
    }

    impl LightingKey {
        pub(crate) fn of(s: &DemoSettings) -> Self {
            Self {
                sun_azimuth: s.sun_azimuth,
                sun_elevation: s.sun_elevation,
                sun_intensity: s.sun_intensity,
                sky_intensity: s.sky_intensity,
                exposure: s.exposure,
                sky_mode: s.sky_mode,
                csg_mode: s.csg_mode,
            }
        }
    }

    /// Bitwise comparison: "did anything write a different value", NaN-safe.
    impl PartialEq for LightingKey {
        fn eq(&self, o: &Self) -> bool {
            self.sun_azimuth.to_bits() == o.sun_azimuth.to_bits()
                && self.sun_elevation.to_bits() == o.sun_elevation.to_bits()
                && self.sun_intensity.to_bits() == o.sun_intensity.to_bits()
                && self.sky_intensity.to_bits() == o.sky_intensity.to_bits()
                && self.exposure.to_bits() == o.exposure.to_bits()
                && self.sky_mode == o.sky_mode
                && self.csg_mode == o.csg_mode
        }
    }

    /// Cancels a running time-of-day transition when the UI pass changed any field it animates
    /// since `before` (slider, sky combo, toolbar EV, scene switch), so the user's edit is not
    /// overwritten every frame (F05). The sky crossfade snaps to the current `sky_mode`.
    pub(crate) fn cancel_tod_on_user_edit(
        before: LightingKey,
        s: &mut DemoSettings,
        tr: &mut Option<TodTransition>,
    ) -> bool {
        if tr.is_none() || LightingKey::of(s) == before {
            return false;
        }
        *tr = None;
        s.sky_mode_from = s.sky_mode;
        s.sky_blend = 0.0;
        true
    }

    /// `(sim_dt, damper_dt)` for one Live (non-lockstep) UI frame (F16). Simulation-time
    /// subsystems (animator, rail, TOD transition, rail glide) advance by `dt * time_scale`, as they
    /// do in Smooth 60 lockstep; the damper follows the clock of whatever drives the camera (rail:
    /// simulation time; manual navigation: wall time, which the driver also uses).
    pub(crate) fn live_step_dts(dt: f32, time_scale: f32, rail_active: bool) -> (f32, f32) {
        let sim_dt = dt * time_scale;
        (sim_dt, if rail_active { sim_dt } else { dt })
    }

    /// Auto-framing request for a `SelectScene` (F30): none for Classic (`scene_idx == 0`, the
    /// worker builds no scene) and none when the target already is the loaded scene (A -> B -> A
    /// coalesced in the `StateSlot` rebuilds nothing, so no `SceneMetaUpdated` would clear the
    /// flag and a later rebuild of the same key would teleport the camera).
    pub(crate) fn scene_auto_frame_target(
        loaded: Option<(u32, u32)>,
        target: (u32, u32),
    ) -> Option<(u32, u32)> {
        (target.0 != 0 && loaded != Some(target)).then_some(target)
    }

    /// Whether a render-target reallocation may be sent this frame (F14). Pure size changes
    /// (window or dock drags) wait until the worker acknowledged the previous `Resize` with a frame
    /// of the newest generation: a blocked worker (multi-second scene build) would otherwise retire
    /// one full buffer set per 120 ms throttle tick. Buffer-set changes (DLSS / HDR toggles) are
    /// rare user actions and go through at once.
    pub(crate) fn resize_allowed(
        size_changed: bool,
        buffers_changed: bool,
        throttle_elapsed: bool,
        dlss_toggled: bool,
        prev_acked: bool,
    ) -> bool {
        buffers_changed || (size_changed && (dlss_toggled || (throttle_elapsed && prev_acked)))
    }

    /// Joins a render thread that has already finished and describes why it ended (F12).
    pub(crate) fn render_exit_reason(handle: JoinHandle<()>) -> String {
        match handle.join() {
            Ok(()) => "render worker exited without reporting an error".to_string(),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "non-string panic payload".to_string());
                format!("render worker panicked: {msg}")
            }
        }
    }

    /// Startup values this session overrode because the host lacks the feature (F18).
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub(crate) struct StartupIntent {
        /// Persisted `hdr_display = true` forced off (no HDR surface).
        pub hdr_display: Option<bool>,
        /// Persisted `dlss_rr_on = true` forced off (DLSS-RR unavailable).
        pub dlss_rr_on: Option<bool>,
        /// `(dlss_on, denoiser_on)` right after startup; restoring `dlss_rr_on` only makes sense
        /// while the user has not picked another AI mode.
        pub ai_at_start: (bool, bool),
    }

    /// Settings as the user chose them (F18): transient session overrides are undone before saving
    /// (LiDAR mount lens switch, an unfinished TOD transition, features forced off on this host).
    pub(crate) fn user_intent_settings(
        live: &DemoSettings,
        saved_projection: Option<crate::renderer::CameraProjection>,
        tod: Option<&TodTransition>,
        intent: StartupIntent,
    ) -> DemoSettings {
        let mut s = *live;
        if let Some(p) = saved_projection {
            s.camera_projection = p;
        }
        if let Some(tr) = tod {
            tr.apply_target(&mut s);
        }
        if let Some(v) = intent.hdr_display {
            s.hdr_display = v;
        }
        if let Some(v) = intent.dlss_rr_on
            && (live.dlss_on, live.denoiser_on) == intent.ai_at_start
        {
            s.dlss_rr_on = v;
        }
        s
    }

    /// Stable reference to the selected scene for persistence (F17).
    pub(crate) fn saved_scene_of(
        descs: &[csg_scene::SceneDescriptor],
        csg_mode: u32,
        asset_idx: u32,
    ) -> Option<crate::persist::SavedScene> {
        let Some(idx) = csg_mode.checked_sub(1) else {
            return Some(crate::persist::SavedScene::default());
        };
        let d = descs.get(idx as usize)?;
        Some(crate::persist::SavedScene {
            id: d.id.to_string(),
            variant: d
                .variant_labels
                .get(asset_idx as usize)
                .map_or_else(String::new, |v| (*v).to_string()),
        })
    }

    /// Resolves a saved scene reference to `(csg_mode, csg_asset_idx)` (F17); `None` when the id
    /// is no longer registered. An unknown variant label keeps `fallback_asset`, clamped.
    pub(crate) fn resolve_saved_scene(
        descs: &[csg_scene::SceneDescriptor],
        saved: &crate::persist::SavedScene,
        fallback_asset: u32,
    ) -> Option<(u32, u32)> {
        if saved.id.is_empty() {
            return Some((0, fallback_asset));
        }
        let (pos, d) = descs.iter().enumerate().find(|(_, d)| d.id == saved.id)?;
        let max_asset = u32::try_from(d.variant_labels.len().saturating_sub(1)).unwrap_or(0);
        let asset = d
            .variant_labels
            .iter()
            .position(|v| *v == saved.variant)
            .and_then(|i| u32::try_from(i).ok())
            .unwrap_or(fallback_asset.min(max_asset));
        let mode = u32::try_from(pos + 1).ok()?;
        Some((mode, asset))
    }

    /// Persistent staging buffer for copying a displayed texture (DLSS output) into CUDA memory.
    struct TexReadback {
        copy_buf: SharedBuffer,
        width: u32,
        height: u32,
        pixel_format: crate::gpu_export::SourcePixelFormat,
        copy_bpr: u32,
        copy_pitch_px: u32,
    }

    /// Copies `tex` into `staging` (reallocated on size or format change), waits for that one
    /// submission with [`GPU_COPY_WAIT_TIMEOUT`] and returns the pitched CUDA view of the copy.
    fn copy_texture_for_cuda(
        interop: &InteropContext,
        staging: &mut Option<TexReadback>,
        tex: &wgpu::Texture,
        width: u32,
        height: u32,
    ) -> Result<PitchedDeviceImage, String> {
        let fmt = tex.format();
        let pixel_format = crate::gpu_export::SourcePixelFormat::from_wgpu(fmt)
            .ok_or_else(|| format!("unsupported still source format {fmt:?}"))?;
        let reusable = staging.as_ref().is_some_and(|s| {
            s.width == width && s.height == height && s.pixel_format == pixel_format
        });
        if !reusable {
            let (copy_bpr, copy_pitch_px) =
                crate::gpu_export::copy_pitch_for_width(width, pixel_format.bytes_per_pixel())
                    .ok_or_else(|| format!("invalid still dimensions ({width}x{height})"))?;
            let size = u64::from(copy_bpr)
                .checked_mul(u64::from(height))
                .ok_or_else(|| "still copy buffer size overflow".to_string())?;
            let copy_buf = interop
                .create_buffer(&SharedBufferDescriptor {
                    label: Some("viewport-still-copy"),
                    size,
                    usage: wgpu::BufferUsages::COPY_DST,
                })
                .map_err(|e| format!("still copy buffer: {e}"))?;
            *staging = Some(TexReadback {
                copy_buf,
                width,
                height,
                pixel_format,
                copy_bpr,
                copy_pitch_px,
            });
        }
        let Some(st) = staging.as_ref() else {
            return Err("still copy buffer unavailable".to_string());
        };
        let mut enc = interop
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("viewport-still-copy"),
            });
        enc.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: st.copy_buf.wgpu_buffer(),
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(st.copy_bpr),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        let idx = interop.queue().submit(Some(enc.finish()));
        if let Err(e) = interop.device().poll(wgpu::PollType::Wait {
            submission_index: Some(idx),
            timeout: Some(GPU_COPY_WAIT_TIMEOUT),
        }) {
            return Err(format!("still GPU copy wait failed: {e}"));
        }
        Ok(PitchedDeviceImage {
            ptr: st.copy_buf.cu_deviceptr(),
            width,
            height,
            pitch_px: st.copy_pitch_px,
            fmt: pixel_format,
        })
    }

    /// RAII CUDA device staging buffer (`Send`) for zero-copy async export handoff to [`ExportWorker`].
    struct OwnedDeviceBytes {
        cuda: Arc<cuda_core::Context>,
        ptr: u64,
        cap: usize,
    }

    impl OwnedDeviceBytes {
        fn alloc(cuda: &Arc<cuda_core::Context>, bytes: usize) -> Result<Self, String> {
            if bytes == 0 {
                return Err("CUDA export buffer size must be non-zero".to_string());
            }
            cuda.bind_to_thread()
                .map_err(|e| format!("CUDA bind failed: {e}"))?;
            let mut ptr: u64 = 0;
            // SAFETY: `cuda` is bound to this thread and `ptr` receives a fresh device allocation.
            let res = unsafe { cuda_bindings::cuMemAlloc_v2(&mut ptr, bytes) };
            if res != cuda_bindings::cudaError_enum_CUDA_SUCCESS || ptr == 0 {
                return Err(format!("cuMemAlloc_v2({bytes}) failed ({res:?})"));
            }
            Ok(Self {
                cuda: Arc::clone(cuda),
                ptr,
                cap: bytes,
            })
        }

        fn copy_from_device(&mut self, src_ptr: u64, bytes: usize) -> Result<(), String> {
            if src_ptr == 0 || bytes == 0 || bytes > self.cap {
                return Err(format!(
                    "invalid CUDA D2D export copy (src={src_ptr:#x}, bytes={bytes}, cap={})",
                    self.cap
                ));
            }
            self.cuda
                .bind_to_thread()
                .map_err(|e| format!("CUDA bind failed: {e}"))?;
            // SAFETY: `src_ptr` and `self.ptr` are valid CUDA allocations of at least `bytes` bytes.
            let res = unsafe { cuda_bindings::cuMemcpyDtoD_v2(self.ptr, src_ptr, bytes) };
            if res != cuda_bindings::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("cuMemcpyDtoD_v2({bytes}) failed ({res:?})"));
            }
            // A device-to-device `cuMemcpy` returns before the copy has run (legacy stream). The
            // source may be reused by wgpu right after this returns and the export thread reads
            // the copy on a non-blocking stream that does not order after the legacy stream, so
            // wait for the legacy stream here. Non-blocking streams (the render worker) are not
            // waited on.
            // SAFETY: the null handle names the legacy default stream of the bound context.
            let res = unsafe { cuda_bindings::cuStreamSynchronize(std::ptr::null_mut()) };
            if res != cuda_bindings::cudaError_enum_CUDA_SUCCESS {
                return Err(format!("cuStreamSynchronize after D2D copy failed ({res:?})"));
            }
            Ok(())
        }
    }

    impl Drop for OwnedDeviceBytes {
        fn drop(&mut self) {
            if self.ptr != 0 {
                if let Err(e) = self.cuda.bind_to_thread() {
                    eprintln!("nv-demo export CUDA bind in Drop failed: {e}");
                    return;
                }
                // SAFETY: `self.ptr` was allocated by `cuMemAlloc_v2` on `self.cuda`.
                let res = unsafe { cuda_bindings::cuMemFree_v2(self.ptr) };
                if res != cuda_bindings::cudaError_enum_CUDA_SUCCESS {
                    eprintln!("nv-demo export cuMemFree_v2 failed ({res:?})");
                }
                self.ptr = 0;
            }
        }
    }

    /// Writes tightly packed `RGBA8` pixels as a lossless PNG atomically via temp file + rename.
    fn write_png_rgba_atomic(
        path: &std::path::Path,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) -> Result<(), String> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create '{}': {e}", parent.display()))?;
        }
        let tmp = path.with_extension(format!("png.tmp.{}", std::process::id()));
        let write_res = (|| -> Result<(), String> {
            let file = std::fs::File::create(&tmp)
                .map_err(|e| format!("failed to create '{}': {e}", tmp.display()))?;
            let mut writer = std::io::BufWriter::new(file);
            {
                let mut enc = png::Encoder::new(&mut writer, width, height);
                enc.set_color(png::ColorType::Rgba);
                enc.set_depth(png::BitDepth::Eight);
                enc.write_header()
                    .and_then(|mut wtr| wtr.write_image_data(rgba))
                    .map_err(|e| format!("PNG encode failed: {e}"))?;
            }
            // Same durability contract as `persist::save_to`: data on disk before the rename.
            let file = writer
                .into_inner()
                .map_err(|e| format!("PNG flush failed: {}", e.error()))?;
            file.sync_all()
                .map_err(|e| format!("PNG sync failed: {e}"))?;
            Ok(())
        })();
        if let Err(e) = write_res {
            if let Err(rm_err) = std::fs::remove_file(&tmp) {
                eprintln!("PNG temp cleanup failed ('{}'): {rm_err}", tmp.display());
            }
            return Err(e);
        }
        if let Err(err) = std::fs::rename(&tmp, path) {
            if let Err(rm_err) = std::fs::remove_file(&tmp) {
                eprintln!("PNG temp cleanup failed ('{}'): {rm_err}", tmp.display());
            }
            return Err(format!("PNG rename failed ('{}'): {err}", path.display()));
        }
        Ok(())
    }

    /// Descriptor of a pitched CUDA device image passed into [`ExportWorker`].
    #[derive(Clone, Copy)]
    struct PitchedDeviceImage {
        ptr: u64,
        width: u32,
        height: u32,
        pitch_px: u32,
        fmt: crate::gpu_export::SourcePixelFormat,
    }

    enum ExportJob {
        SavePngRgba {
            rgba: Vec<u8>,
            width: u32,
            height: u32,
            path: PathBuf,
        },
        SavePngPitched {
            raw: Vec<u8>,
            width: u32,
            height: u32,
            pitch_px: u32,
            fmt: crate::gpu_export::SourcePixelFormat,
            path: PathBuf,
        },
        SaveJpegDevice {
            buf: OwnedDeviceBytes,
            width: u32,
            height: u32,
            pitch_px: u32,
            fmt: crate::gpu_export::SourcePixelFormat,
            path: PathBuf,
            quality: u8,
        },
        StartVideo {
            session_id: u64,
            width: u32,
            height: u32,
            fps: u32,
            profile: crate::gpu_export::VideoProfile,
            fmt: crate::gpu_export::SourcePixelFormat,
            path: PathBuf,
        },
        PushVideoFrame {
            session_id: u64,
            buf: OwnedDeviceBytes,
            pitch_px: u32,
            duration_secs: f64,
        },
        FinishVideo {
            session_id: u64,
            reason: Option<String>,
        },
        Shutdown,
    }

    /// Background export worker thread offloading PNG compression, Full-Window nvJPEG encoding,
    /// and NVENC MP4 recording/muxing off the 60 Hz UI thread.
    struct ExportWorker {
        cuda: Arc<cuda_core::Context>,
        tx: Option<mpsc::Sender<ExportJob>>,
        handle: Option<JoinHandle<()>>,
        device_pool: Arc<Mutex<Vec<OwnedDeviceBytes>>>,
        video_in_flight: Arc<AtomicUsize>,
        video_frames: Arc<AtomicU64>,
        video_failed_session: Arc<AtomicU64>,
        next_session_id: u64,
    }

    impl ExportWorker {
        fn spawn(cuda: Arc<cuda_core::Context>, bus: AppBus) -> Self {
            let (tx, rx) = mpsc::channel::<ExportJob>();
            let device_pool: Arc<Mutex<Vec<OwnedDeviceBytes>>> = Arc::new(Mutex::new(Vec::new()));
            let video_in_flight = Arc::new(AtomicUsize::new(0));
            let video_frames = Arc::new(AtomicU64::new(0));
            let video_failed_session = Arc::new(AtomicU64::new(0));

            let w_cuda = Arc::clone(&cuda);
            let w_pool = Arc::clone(&device_pool);
            let w_in_flight = Arc::clone(&video_in_flight);
            let w_frames = Arc::clone(&video_frames);
            let w_failed = Arc::clone(&video_failed_session);

            let recycle_buf = |pool: &Mutex<Vec<OwnedDeviceBytes>>, buf: OwnedDeviceBytes| {
                let evicted = {
                    let mut guard = pool.lock().unwrap_or_else(|e| e.into_inner());
                    recycle_into_pool(
                        &mut *guard,
                        buf,
                        |b: &OwnedDeviceBytes| b.cap,
                        MAX_VIDEO_FRAMES_IN_FLIGHT,
                    )
                };
                // Freed after the lock is released: `cuMemFree` synchronizes the device.
                drop(evicted);
            };

            let handle = std::thread::Builder::new()
                .name("nv-demo-export".to_string())
                .spawn(move || {
                    crate::nvtx::name_current_thread(c"nv-demo-export");
                    let mut rgba_scratch: Vec<u8> = Vec::new();
                    let mut jpeg_stream: Option<Arc<cuda_core::Stream>> = None;
                    let mut active_video: Option<(u64, crate::gpu_export::VideoRecorder)> = None;

                    while let Ok(job) = rx.recv() {
                        match job {
                            ExportJob::SavePngRgba {
                                rgba,
                                width,
                                height,
                                path,
                            } => {
                                let msg = match write_png_rgba_atomic(&path, width, height, &rgba) {
                                    Ok(()) => format!("Saved {}", path.display()),
                                    Err(e) => e,
                                };
                                bus.emit_render_event(RenderEvent::StatusMessage(msg));
                            }
                            ExportJob::SavePngPitched {
                                raw,
                                width,
                                height,
                                pitch_px,
                                fmt,
                                path,
                            } => {
                                let res = crate::gpu_export::unpack_pitched_to_rgba8(
                                    &raw,
                                    &mut rgba_scratch,
                                    width,
                                    height,
                                    pitch_px,
                                    fmt,
                                )
                                .and_then(|()| {
                                    write_png_rgba_atomic(&path, width, height, &rgba_scratch)
                                });
                                let msg = match res {
                                    Ok(()) => format!("Saved {}", path.display()),
                                    Err(e) => format!("Window PNG failed: {e}"),
                                };
                                bus.emit_render_event(RenderEvent::StatusMessage(msg));
                            }
                            ExportJob::SaveJpegDevice {
                                buf,
                                width,
                                height,
                                pitch_px,
                                fmt,
                                path,
                                quality,
                            } => {
                                let res = w_cuda
                                    .bind_to_thread()
                                    .map_err(|e| format!("CUDA bind failed: {e}"))
                                    .and_then(|()| {
                                        if jpeg_stream.is_none() {
                                            jpeg_stream = Some(
                                                w_cuda
                                                    .new_stream()
                                                    .map_err(|e| format!("CUDA stream failed: {e}"))?,
                                            );
                                        }
                                        let Some(ref stream) = jpeg_stream else {
                                            return Err("CUDA stream unavailable".to_string());
                                        };
                                        crate::gpu_export::save_jpeg_from_pitched(
                                            &w_cuda,
                                            stream,
                                            buf.ptr,
                                            width,
                                            height,
                                            pitch_px,
                                            fmt,
                                            &path,
                                            quality,
                                        )
                                    });
                                recycle_buf(&w_pool, buf);
                                let msg = match res {
                                    Ok(()) => format!("Saved {}", path.display()),
                                    Err(e) => format!("Window JPEG failed: {e}"),
                                };
                                bus.emit_render_event(RenderEvent::StatusMessage(msg));
                            }
                            ExportJob::StartVideo {
                                session_id,
                                width,
                                height,
                                fps,
                                profile,
                                fmt,
                                path,
                            } => {
                                if let Some((_, old)) = active_video.take()
                                    && let Err(e) = old.finish()
                                {
                                    bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                        "Previous video finish failed: {e}"
                                    )));
                                }
                                w_frames.store(0, Ordering::Release);
                                match crate::gpu_export::VideoRecorder::start_with_format(
                                    &w_cuda, width, height, fps, profile, fmt, &path,
                                ) {
                                    Ok(rec) => {
                                        active_video = Some((session_id, rec));
                                        bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                            "Recording to {}",
                                            path.display()
                                        )));
                                    }
                                    Err(e) => {
                                        w_failed.store(session_id, Ordering::Release);
                                        bus.emit_render_event(RenderEvent::StatusMessage(format!(
                                            "Record start failed: {e}"
                                        )));
                                    }
                                }
                            }
                            ExportJob::PushVideoFrame {
                                session_id,
                                buf,
                                pitch_px,
                                duration_secs,
                            } => {
                                if let Some((cur_id, ref mut rec)) = active_video
                                    && cur_id == session_id
                                {
                                    match rec.push_frame(buf.ptr, pitch_px, duration_secs) {
                                        Ok(()) => {
                                            w_frames.store(rec.frame_count(), Ordering::Release);
                                        }
                                        Err(e) => {
                                            let finish_err = active_video
                                                .take()
                                                .and_then(|(_, failed)| failed.finish().err())
                                                .map(|fe| format!("; finish also failed: {fe}"))
                                                .unwrap_or_default();
                                            w_failed.store(session_id, Ordering::Release);
                                            bus.emit_render_event(RenderEvent::StatusMessage(
                                                format!("Video encode error: {e}{finish_err}"),
                                            ));
                                        }
                                    }
                                }
                                recycle_buf(&w_pool, buf);
                                w_in_flight.fetch_sub(1, Ordering::AcqRel);
                            }
                            ExportJob::FinishVideo { session_id, reason } => {
                                if let Some((cur_id, rec)) = active_video.take() {
                                    if cur_id == session_id {
                                        let frames = rec.frame_count();
                                        let msg = match rec.finish() {
                                            Ok(n) => reason
                                                .unwrap_or_else(|| format!("Saved MP4 ({n} frames)")),
                                            Err(e) => {
                                                format!("Video finish failed ({frames}f): {e}")
                                            }
                                        };
                                        bus.emit_render_event(RenderEvent::StatusMessage(msg));
                                    } else {
                                        active_video = Some((cur_id, rec));
                                    }
                                }
                            }
                            ExportJob::Shutdown => {
                                if let Some((_, rec)) = active_video.take()
                                    && let Err(e) = rec.finish()
                                {
                                    eprintln!("nv-demo shutdown video finish error: {e}");
                                }
                                break;
                            }
                        }
                    }
                    // Drop pooled CUDA buffers on the worker thread before exiting.
                    let mut guard = w_pool.lock().unwrap_or_else(|e| e.into_inner());
                    guard.clear();
                })
                .ok();

            Self {
                cuda,
                tx: handle.as_ref().map(|_| tx),
                handle,
                device_pool,
                video_in_flight,
                video_frames,
                video_failed_session,
                next_session_id: 0,
            }
        }

        fn acquire_device_copy(&self, src_ptr: u64, bytes: usize) -> Result<OwnedDeviceBytes, String> {
            let mut reused = {
                let mut guard = self.device_pool.lock().unwrap_or_else(|e| e.into_inner());
                guard
                    .iter()
                    .position(|b| b.cap == bytes)
                    .map(|pos| guard.swap_remove(pos))
            };
            let mut buf = match reused.take() {
                Some(b) => b,
                None => OwnedDeviceBytes::alloc(&self.cuda, bytes)?,
            };
            buf.copy_from_device(src_ptr, bytes)?;
            Ok(buf)
        }

        fn enqueue_png_rgba(
            &self,
            rgba: Vec<u8>,
            width: u32,
            height: u32,
            path: PathBuf,
        ) -> Result<(), String> {
            let Some(ref tx) = self.tx else {
                return write_png_rgba_atomic(&path, width, height, &rgba);
            };
            tx.send(ExportJob::SavePngRgba {
                rgba,
                width,
                height,
                path,
            })
            .map_err(|e| format!("Export worker disconnected: {e}"))
        }

        fn enqueue_png_pitched(
            &self,
            raw: Vec<u8>,
            width: u32,
            height: u32,
            pitch_px: u32,
            fmt: crate::gpu_export::SourcePixelFormat,
            path: PathBuf,
        ) -> Result<(), String> {
            let Some(ref tx) = self.tx else {
                let mut rgba = Vec::new();
                crate::gpu_export::unpack_pitched_to_rgba8(&raw, &mut rgba, width, height, pitch_px, fmt)?;
                return write_png_rgba_atomic(&path, width, height, &rgba);
            };
            tx.send(ExportJob::SavePngPitched {
                raw,
                width,
                height,
                pitch_px,
                fmt,
                path,
            })
            .map_err(|e| format!("Export worker disconnected: {e}"))
        }

        fn enqueue_jpeg_device(
            &self,
            img: PitchedDeviceImage,
            path: PathBuf,
            quality: u8,
        ) -> Result<(), String> {
            let bytes = (img.pitch_px as usize)
                .checked_mul(img.height as usize)
                .and_then(|px| px.checked_mul(img.fmt.bytes_per_pixel() as usize))
                .ok_or_else(|| "JPEG buffer size overflow".to_string())?;
            let buf = self.acquire_device_copy(img.ptr, bytes)?;
            let Some(ref tx) = self.tx else {
                return Err("Export worker unavailable".to_string());
            };
            tx.send(ExportJob::SaveJpegDevice {
                buf,
                width: img.width,
                height: img.height,
                pitch_px: img.pitch_px,
                fmt: img.fmt,
                path,
                quality,
            })
            .map_err(|e| format!("Export worker disconnected: {e}"))
        }

        fn start_video(
            &mut self,
            width: u32,
            height: u32,
            fps: u32,
            profile: crate::gpu_export::VideoProfile,
            fmt: crate::gpu_export::SourcePixelFormat,
            path: PathBuf,
        ) -> Result<u64, String> {
            let Some(ref tx) = self.tx else {
                return Err("Export worker unavailable".to_string());
            };
            self.next_session_id = self.next_session_id.wrapping_add(1).max(1);
            let session_id = self.next_session_id;
            self.video_frames.store(0, Ordering::Release);
            tx.send(ExportJob::StartVideo {
                session_id,
                width,
                height,
                fps,
                profile,
                fmt,
                path,
            })
            .map_err(|e| format!("Export worker disconnected: {e}"))?;
            Ok(session_id)
        }

        fn push_video_frame(
            &self,
            session_id: u64,
            img: PitchedDeviceImage,
            duration_secs: f64,
            allow_drop_when_full: bool,
        ) -> Result<bool, String> {
            if allow_drop_when_full
                && self.video_in_flight.load(Ordering::Acquire) >= MAX_VIDEO_FRAMES_IN_FLIGHT
            {
                return Ok(false);
            }
            let bytes = (img.pitch_px as usize)
                .checked_mul(img.height as usize)
                .and_then(|px| px.checked_mul(img.fmt.bytes_per_pixel() as usize))
                .ok_or_else(|| "video frame buffer size overflow".to_string())?;
            let buf = self.acquire_device_copy(img.ptr, bytes)?;
            let Some(ref tx) = self.tx else {
                return Err("Export worker unavailable".to_string());
            };
            self.video_in_flight.fetch_add(1, Ordering::AcqRel);
            if let Err(e) = tx.send(ExportJob::PushVideoFrame {
                session_id,
                buf,
                pitch_px: img.pitch_px,
                duration_secs,
            }) {
                self.video_in_flight.fetch_sub(1, Ordering::AcqRel);
                return Err(format!("Export worker disconnected: {e}"));
            }
            Ok(true)
        }

        fn finish_video(&self, session_id: u64, reason: Option<String>) {
            if let Some(ref tx) = self.tx
                && let Err(e) = tx.send(ExportJob::FinishVideo { session_id, reason })
            {
                eprintln!("nv-demo finish_video send failed: {e}");
            }
        }

        fn recorded_frames(&self) -> u64 {
            self.video_frames.load(Ordering::Acquire)
        }

        fn session_failed(&self, session_id: u64) -> bool {
            self.video_failed_session.load(Ordering::Acquire) == session_id
        }

        fn shutdown(&mut self) {
            if let Some(tx) = self.tx.take()
                && let Err(e) = tx.send(ExportJob::Shutdown)
            {
                eprintln!("nv-demo export shutdown send failed: {e}");
            }
            if let Some(handle) = self.handle.take()
                && let Err(e) = handle.join()
            {
                eprintln!("nv-demo export worker join error: {e:?}");
            }
            let mut guard = self.device_pool.lock().unwrap_or_else(|e| e.into_inner());
            guard.clear();
        }
    }

    impl Drop for ExportWorker {
        fn drop(&mut self) {
            self.shutdown();
        }
    }

    /// Returns true when `mount` represents a 360-degree / LiDAR roof sensor mount.
    pub(crate) fn is_360_lidar_mount(mount: &csg_scene::CameraMount) -> bool {
        let id_lc = mount.id.to_ascii_lowercase();
        let lbl_lc = mount.label.to_ascii_lowercase();
        id_lc.contains("360")
            || id_lc.contains("lidar")
            || lbl_lc.contains("360")
            || lbl_lc.contains("lidar")
    }

    /// Automatically switch `projection` to `Equirect360` when possessing a 360°/LiDAR mount,
    /// saving the previous projection in `saved` and restoring it upon switching to a regular
    /// mount or ejecting (`mount == None`).
    pub(crate) fn sync_mount_camera_projection(
        mount: Option<&csg_scene::CameraMount>,
        projection: &mut crate::renderer::CameraProjection,
        saved: &mut Option<crate::renderer::CameraProjection>,
    ) {
        if let Some(m) = mount
            && is_360_lidar_mount(m)
        {
            if saved.is_none() {
                *saved = Some(*projection);
            }
            *projection = crate::renderer::CameraProjection::Equirect360;
        } else if let Some(prev) = saved.take() {
            *projection = prev;
        }
    }

    /// Smooth time-of-day lighting interpolator (`Day` / `Sunset` / `Night`) driven over `duration_s`.
    #[derive(Clone, Copy, Debug)]
    pub(crate) struct TodTransition {
        from_sky_mode: u32,
        from_azimuth: f32,
        from_elevation: f32,
        from_sun_intensity: f32,
        from_sky_intensity: f32,
        from_exposure: f32,
        to_tod: u32,
        to_sky_mode: u32,
        to_azimuth: f32,
        to_elevation: f32,
        to_sun_intensity: f32,
        to_sky_intensity: f32,
        to_exposure: f32,
        elapsed_s: f32,
        duration_s: f32,
    }

    impl TodTransition {
        /// Capture current lighting state as the start keyframe and target `tod` preset as the end.
        pub(crate) fn new(current: &DemoSettings, target_tod: u32, duration_s: f32) -> Self {
            let mut target = *current;
            crate::ui::apply_csg_time_of_day_preset(&mut target, target_tod);
            // Restarted mid-crossfade: start from the sky that dominates the current blend. The
            // shader blends two skies only, so up to half a blend can still pop (F15).
            let from_sky_mode = if current.sky_blend > 0.0 && current.sky_blend < 0.5 {
                current.sky_mode_from
            } else {
                current.sky_mode
            };
            Self {
                from_sky_mode,
                from_azimuth: current.sun_azimuth,
                from_elevation: current.sun_elevation,
                from_sun_intensity: current.sun_intensity,
                from_sky_intensity: current.sky_intensity,
                from_exposure: current.exposure,
                to_tod: target.csg_time_of_day,
                to_sky_mode: target.sky_mode,
                to_azimuth: target.sun_azimuth,
                to_elevation: target.sun_elevation,
                to_sun_intensity: target.sun_intensity,
                to_sky_intensity: target.sky_intensity,
                to_exposure: target.exposure,
                elapsed_s: 0.0,
                duration_s: duration_s.clamp(0.1, 15.0),
            }
        }

        /// Advance transition by `dt` seconds. Returns `true` when finished.
        pub(crate) fn step(&mut self, s: &mut DemoSettings, dt: f32) -> bool {
            self.elapsed_s = (self.elapsed_s + dt.max(0.0)).min(self.duration_s);
            let t = (self.elapsed_s / self.duration_s).clamp(0.0, 1.0);
            let u = t * t * (3.0 - 2.0 * t);

            // Shortest angular interpolation for sun azimuth in degrees.
            let mut daz = (self.to_azimuth - self.from_azimuth) % 360.0;
            if daz > 180.0 {
                daz -= 360.0;
            } else if daz < -180.0 {
                daz += 360.0;
            }
            s.sun_azimuth = self.from_azimuth + daz * u;
            s.sun_elevation = self.from_elevation + (self.to_elevation - self.from_elevation) * u;
            s.sun_intensity =
                self.from_sun_intensity + (self.to_sun_intensity - self.from_sun_intensity) * u;
            s.sky_intensity =
                self.from_sky_intensity + (self.to_sky_intensity - self.from_sky_intensity) * u;
            s.exposure = self.from_exposure + (self.to_exposure - self.from_exposure) * u;

            // Continuous shader sky crossfade across [0, 1] instead of a discrete 1-frame jump.
            s.sky_mode_from = self.from_sky_mode;
            s.sky_mode = self.to_sky_mode;
            s.sky_blend = u;

            if t >= 0.5 {
                s.csg_time_of_day = self.to_tod;
            }
            if t >= 1.0 {
                self.apply_target(s);
                true
            } else {
                false
            }
        }

        /// Writes the transition's end state (target preset, crossfade finished) into `s`.
        pub(crate) fn apply_target(&self, s: &mut DemoSettings) {
            crate::ui::apply_csg_time_of_day_preset(s, self.to_tod);
            s.sky_mode_from = s.sky_mode;
            s.sky_blend = 0.0;
        }
    }

    /// Deterministic DCC-style OS cursor grab & hide state machine (`Click -> Drag -> Locked -> Restore`).
    ///
    /// - Single clicks and double-clicks (`drag_dist < DRAG_THRESHOLD_PX`) never engage OS
    ///   `CursorGrabMode::Locked`, preventing Win32 `ClipCursor` center-warping on click.
    /// - Once a real drag exceeds `DRAG_THRESHOLD_PX`, the exact pre-grab OS cursor position is
    ///   saved, the cursor is hidden and locked (`Locked` with `Confined` fallback) for infinite
    ///   unbounded orbit/pan/fly, and synthetic center-warp `CursorMoved` events are ignored.
    /// - On release (or focus loss), the grab is cleared, the OS cursor is restored to the exact
    ///   saved pre-grab position, and visibility is restored.
    #[derive(Clone, Debug, Default)]
    pub(crate) struct CursorGrabController {
        last_cursor_pos: Option<winit::dpi::PhysicalPosition<f64>>,
        pre_grab_pos: Option<winit::dpi::PhysicalPosition<f64>>,
        press_origin: Option<winit::dpi::PhysicalPosition<f64>>,
        drag_accum_px: f64,
        buttons_held: u8,
        grab_active: bool,
        grab_from_drag: bool,
        pending_grab: Option<winit::window::CursorGrabMode>,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub(crate) struct CursorOsAction {
        pub grab: Option<winit::window::CursorGrabMode>,
        pub visible: Option<bool>,
        pub restore_pos: Option<winit::dpi::PhysicalPosition<f64>>,
    }

    impl CursorOsAction {
        const NONE: Self = Self {
            grab: None,
            visible: None,
            restore_pos: None,
        };
    }

    impl CursorGrabController {
        const DRAG_THRESHOLD_PX: f64 = 4.0;

        fn engage_grab(
            &mut self,
            mode: winit::window::CursorGrabMode,
            from_drag: bool,
        ) -> CursorOsAction {
            self.pre_grab_pos = self.press_origin.or(self.last_cursor_pos);
            self.grab_active = true;
            self.grab_from_drag = from_drag;
            self.pending_grab = None;
            CursorOsAction {
                grab: Some(mode),
                visible: Some(false),
                restore_pos: None,
            }
        }

        pub(crate) fn on_cursor_moved(
            &mut self,
            pos: winit::dpi::PhysicalPosition<f64>,
        ) -> CursorOsAction {
            if self.grab_active {
                // Ignore synthetic center-warp events while OS grab is active.
                return CursorOsAction::NONE;
            }
            if let Some(origin) = self.press_origin {
                let dx = pos.x - origin.x;
                let dy = pos.y - origin.y;
                self.drag_accum_px = self.drag_accum_px.max((dx * dx + dy * dy).sqrt());
            }
            self.last_cursor_pos = Some(pos);
            if let Some(mode) = self.pending_grab
                && self.drag_accum_px >= Self::DRAG_THRESHOLD_PX
            {
                return self.engage_grab(mode, true);
            }
            CursorOsAction::NONE
        }

        pub(crate) fn on_mouse_motion(&mut self, delta: (f64, f64)) -> CursorOsAction {
            if !self.grab_active && self.buttons_held > 0 {
                self.drag_accum_px += (delta.0 * delta.0 + delta.1 * delta.1).sqrt();
                if let Some(mode) = self.pending_grab
                    && self.drag_accum_px >= Self::DRAG_THRESHOLD_PX
                {
                    return self.engage_grab(mode, true);
                }
            }
            CursorOsAction::NONE
        }

        pub(crate) fn on_mouse_button(&mut self, pressed: bool) -> CursorOsAction {
            if pressed {
                self.buttons_held = self.buttons_held.saturating_add(1);
                if self.buttons_held == 1 {
                    self.press_origin = self.last_cursor_pos;
                    self.drag_accum_px = 0.0;
                }
                CursorOsAction::NONE
            } else {
                self.buttons_held = self.buttons_held.saturating_sub(1);
                if self.buttons_held == 0 {
                    self.press_origin = None;
                    self.drag_accum_px = 0.0;
                    self.pending_grab = None;
                    if self.grab_active && self.grab_from_drag {
                        return self.release_grab();
                    }
                }
                CursorOsAction::NONE
            }
        }

        pub(crate) fn on_focus_lost(&mut self) -> CursorOsAction {
            self.buttons_held = 0;
            self.press_origin = None;
            self.drag_accum_px = 0.0;
            self.pending_grab = None;
            if self.grab_active {
                self.release_grab()
            } else {
                CursorOsAction::NONE
            }
        }

        pub(crate) fn apply_egui_request(
            &mut self,
            want_grab: Option<egui::CursorGrab>,
            want_visible: Option<bool>,
        ) -> CursorOsAction {
            let requesting_lock = matches!(
                want_grab,
                Some(egui::CursorGrab::Locked | egui::CursorGrab::Confined)
            );
            let requesting_release = matches!(want_grab, Some(egui::CursorGrab::None));

            if requesting_release {
                self.pending_grab = None;
                // `CursorGrab::None` + `CursorVisible(false)` is camera-rs `CursorRequest::Hidden`
                // (RMB hold-to-fly): an explicit hide of a free cursor, honoured as such. Leaving
                // a lock for it (persistent fly -> RMB hold) must not show the cursor or warp it
                // back to the pre-grab position: the fly look reads window pointer deltas, so a
                // warp would turn into a look jump.
                let hide = want_visible == Some(false);
                if self.grab_active {
                    let mut act = self.release_grab();
                    if hide {
                        act.visible = Some(false);
                        act.restore_pos = None;
                    }
                    return act;
                }
                return CursorOsAction {
                    grab: None,
                    visible: want_visible,
                    restore_pos: None,
                };
            }

            if requesting_lock {
                if self.grab_active {
                    self.pending_grab = None;
                    return CursorOsAction::NONE;
                }
                let mode = match want_grab {
                    Some(egui::CursorGrab::Confined) => winit::window::CursorGrabMode::Confined,
                    _ => winit::window::CursorGrabMode::Locked,
                };
                if self.buttons_held == 0 {
                    // Persistent fly modes lock the cursor without holding a mouse button.
                    return self.engage_grab(mode, false);
                }
                if self.drag_accum_px >= Self::DRAG_THRESHOLD_PX {
                    return self.engage_grab(mode, true);
                }
                // Still within click/double-click deadzone: defer the one-shot grab request
                // until motion crosses DRAG_THRESHOLD_PX so camera_egui does not need to re-emit.
                self.pending_grab = Some(mode);
                return CursorOsAction::NONE;
            }

            // No grab change requested; honor explicit show (never hide while ungrabbed).
            CursorOsAction {
                grab: None,
                visible: if self.grab_active {
                    want_visible
                } else {
                    want_visible.filter(|&v| v)
                },
                restore_pos: None,
            }
        }

        fn release_grab(&mut self) -> CursorOsAction {
            self.grab_active = false;
            self.grab_from_drag = false;
            self.pending_grab = None;
            let restore_pos = self.pre_grab_pos.take();
            if let Some(pos) = restore_pos {
                self.last_cursor_pos = Some(pos);
            }
            CursorOsAction {
                grab: Some(winit::window::CursorGrabMode::None),
                visible: Some(true),
                restore_pos,
            }
        }
    }

    pub struct DemoApp {
        window: Arc<Window>,
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
        surface_supports_copy_dst: bool,
        sdr_surface_format: wgpu::TextureFormat,
        hdr_surface_supported: bool,
        window_capture: Option<FullWindowCapture>,
        dlss_pass: Option<DlssPass>,
        dlss_rr_pass: Option<DlssRrPass>,
        dlss_rr_available: bool,
        cached_dlss_opt: Option<((u32, u32, u32, bool), ngx::DlssOptimalSettings)>,
        ngx_vulkan: Option<ngx::NgxVulkan>,
        ngx_init_error: Option<String>,
        interop: InteropContext,
        hdr_tonemap: HdrTonemapPipeline,
        rr_last_exposure: Option<f32>,
        egui_ctx: egui::Context,
        egui_winit: egui_winit::State,
        egui_renderer: egui_wgpu::Renderer,
        egui_renderer_hdr: Option<egui_wgpu::Renderer>,
        shared_rgba: [SharedBuffer; 2],
        shared_depth: [SharedBuffer; 2],
        shared_motion: [SharedBuffer; 2],
        shared_hdr: Option<[SharedBuffer; 2]>,
        rr_shared: Option<DlssRrSharedBuffers>,
        retired_buffers: Vec<(u64, [SharedBuffer; 2])>,
        buf_gen: u64,
        render_w: u32,
        render_h: u32,
        desired_vp_size: (f32, f32),
        last_resize_at: Instant,
        front_buf_idx: usize,
        has_frame: bool,
        viewport_tex_dirty: bool,
        dlss_dirty: bool,
        display_texture: wgpu::Texture,
        display_view: wgpu::TextureView,
        display_texture_hdr: Option<wgpu::Texture>,
        display_view_hdr: Option<wgpu::TextureView>,
        display_tex_id: egui::TextureId,
        using_dlss_tex: bool,
        bound_hdr_tex: bool,
        last_jitter: [f32; 2],
        last_saved_msg: Option<String>,
        /// Rail fly-through statistics, fed from `FrameReady` while the rail drives the camera.
        flight: crate::flight_stats::FlightStats,
        /// Rail id and Auto-Drive state the Flight stats were accumulated for (a new cassette or
        /// an Auto-Drive start resets them).
        flight_rail_id: String,
        flight_rail_active: bool,
        bus: AppBus,
        ui_cmd_scratch: Vec<appbus::BoxedEvent>,
        render_ev_scratch: Vec<appbus::BoxedEvent>,
        ui_scratch: UiScratch,
        render_thread: Option<JoinHandle<()>>,
        export_worker: ExportWorker,
        persist_worker: crate::persist::PersistWorker,
        nvml_dev: Option<nvml::Device>,
        _nvml: Option<&'static nvml::Nvml>,
        profiler: Profiler,
        telemetry: GpuTelemetry,
        last_telemetry_at: Instant,
        last_frame_at: Instant,
        accum_count: u32,
        total_samples: u32,
        rt_ms: f32,
        /// Intersection counts of the latest counted launch (`is_counters.rs`).
        is_counts: Option<crate::is_counters::IsCounts>,
        denoise_ms: f32,
        bloom_ms: f32,
        tonemap_ms: f32,
        cuda_compute_ms: f32,
        dlss_ms: f32,
        engine_phase: crate::app_events::EnginePhase,
        ui_fps: f32,
        rt_fps: f32,
        fast_preview: bool,
        reset_seq: u64,
        interacting_until: Option<Instant>,
        is_recording: bool,
        record_frames: u64,
        video_recorder: Option<ActiveRecorder>,
        shutter_in_flight: bool,
        /// `--bench-rail` run (`None` = interactive app).
        bench: Option<crate::bench::BenchRun>,
        ser_supported: bool,
        csg_stats: crate::csg_scene_host::CsgStats,
        crowd_step_ms: f32,
        crowd_active_backend: crate::sdg_export::ActiveCrowdBackend,
        crowd_actor_count: u32,
        last_rendered_camera: CameraFrame,
        scene_descriptors: Vec<csg_scene::SceneDescriptor>,
        scene_viewpoints: Vec<csg_scene::SceneViewpoint>,
        pending_scene_auto_frame: Option<(u32, u32)>,
        rail_random_seed: u64,
        smart_view_seed: u64,
        possess_info: Option<PossessInfo>,
        possess_free_yaw: f32,
        possess_free_pitch: f32,
        possess_auto_level: bool,
        possess_zoom_offset: f32,
        possess_saved_projection: Option<crate::renderer::CameraProjection>,
        viewport_toolbar: crate::viewport_toolbar::ToolbarState,
        possess_toolbar: crate::viewport_toolbar::PossessToolbar,
        tod_transition: Option<TodTransition>,
        /// Set once the render worker reported `Fatal` or ended; no render work is sent after it.
        render_fatal: Option<String>,
        /// `false` while the newest `Resize` has not been answered by a frame of its generation.
        resize_acked: bool,
        startup_intent: StartupIntent,
        still_readback: Option<TexReadback>,
        /// Why `is_dlss_rr_available` failed at startup (`None` when it answered).
        dlss_rr_error: Option<String>,
        /// `(scene_idx, variant_idx)` of the last `SceneMetaUpdated` (`None` = Classic / none yet).
        loaded_scene_key: Option<(u32, u32)>,
        cursor_grab: CursorGrabController,
        last_sent_possess_look: (u32, u32, u32, bool),
        last_possess_camera: Option<CameraFrame>,
        pick_possess_mode: bool,
        last_smart_view: Option<SmartViewResult>,
        settings: DemoSettings,
        last_sent_settings: DemoSettings,
        demo_anim: crate::demo_anim::DemoAnimator,
        camera_rail: crate::camera_rail::CameraRail,
        cam_damper: crate::camera_rail::CameraDamper,
        rail_blend_remaining: f32,
        cam_driver: CameraDriver,
        last_cam_raw_input: Option<camera::input::RawInput>,
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
            if let Some(active) = self.video_recorder.take() {
                self.export_worker.finish_video(active.session_id, None);
            }
            self.export_worker.shutdown();
            self.flush_persisted_on_exit();
            self.bus.send_render(RenderControl::Shutdown);
            if let Some(handle) = self.render_thread.take()
                && let Err(e) = handle.join()
            {
                eprintln!("nv-demo render thread join error: {e:?}");
            }
        }
    }

    impl DemoApp {
        /// Sends a render control unless the worker is gone (F12: no work after `Fatal`).
        fn send_render(&self, cmd: RenderControl) {
            if self.render_fatal.is_none() {
                self.bus.send_render(cmd);
            }
        }

        /// Publishes the coalesced render state unless the worker is gone (F12).
        fn publish_state(&self, snapshot: RenderStateSnapshot) {
            if self.render_fatal.is_none() {
                self.bus.publish_render_state(snapshot);
            }
        }

        /// Enters the terminal render-failure state once: persistent status, recording finished
        /// cleanly (the MP4 is finalized by the export worker), lockstep left.
        fn enter_render_fatal(&mut self, reason: String) {
            if self.render_fatal.is_some() {
                return;
            }
            eprintln!("nv-demo render worker stopped: {reason}");
            if let Some(active) = self.video_recorder.take() {
                self.export_worker.finish_video(
                    active.session_id,
                    Some("Recording stopped: render worker failed".to_string()),
                );
            }
            self.is_recording = false;
            self.record_frames = 0;
            self.shutter_in_flight = false;
            self.interacting_until = None;
            self.render_fatal = Some(format!("Render worker stopped: {reason}"));
        }

        /// Copies the DLSS texture the viewport currently shows into CUDA-visible memory, or
        /// `None` when the viewport shows the worker's `shared_rgba` (F01/F10).
        fn copy_displayed_viewport(
            &mut self,
            src: ViewportStillSource,
        ) -> Option<Result<PitchedDeviceImage, String>> {
            match src {
                ViewportStillSource::RenderTarget => None,
                ViewportStillSource::DlssRrLdr | ViewportStillSource::DlssRrHdr => {
                    let rr = self.dlss_rr_pass.as_ref()?;
                    let tex = if src == ViewportStillSource::DlssRrHdr {
                        &rr.display_hdr
                    } else {
                        &rr.display_ldr
                    };
                    Some(copy_texture_for_cuda(
                        &self.interop,
                        &mut self.still_readback,
                        tex,
                        rr.out_w,
                        rr.out_h,
                    ))
                }
                ViewportStillSource::DlssSr => {
                    let dlss = self.dlss_pass.as_ref()?;
                    Some(copy_texture_for_cuda(
                        &self.interop,
                        &mut self.still_readback,
                        dlss.output_out.wgpu_texture(),
                        dlss.out_w,
                        dlss.out_h,
                    ))
                }
            }
        }
    }

    impl DemoApp {
        pub fn new(
            window: Arc<Window>,
            loaded: crate::persist::LoadReport,
            bench_args: Option<crate::bench::BenchArgs>,
            is_counters: bool,
        ) -> Self {
            let crate::persist::LoadReport {
                state: mut persisted,
                problem: persist_problem,
            } = loaded;
            if let Some(b) = bench_args.as_ref() {
                // Identical geometry and camera model in every benchmark run: frozen traffic and
                // crowd, free Pinhole camera, no demo animation; the rail drives the camera.
                // A views run starts in its first view's scene.
                let scene = match &b.target {
                    crate::bench::BenchTarget::Rail(_) => b.scene.clone(),
                    crate::bench::BenchTarget::Views(v) => v.first().map(|v| v.scene.clone()),
                };
                if let Some(id) = scene {
                    persisted.scene = Some(crate::persist::SavedScene {
                        id,
                        variant: String::new(),
                    });
                }
                persisted.settings.csg_freeze_pose = true;
                persisted.settings.camera_projection = crate::renderer::CameraProjection::Pinhole;
                persisted.demo_anim.playing = false;
            }
            let size = window.inner_size();
            // F26: keep the first failure of every startup stage so the UI can name the real
            // reason instead of a generic "unavailable".
            let mut ngx_ext_error: Option<String> = None;
            let ngx_exts = match ngx::NgxVulkan::required_extensions() {
                Ok(exts) => Some(exts),
                Err(e) => {
                    ngx_ext_error = Some(format!("NGX required extensions query failed: {e}"));
                    None
                }
            };
            let plain_instance = || {
                wgpu::Instance::new(wgpu::InstanceDescriptor {
                    backends: wgpu::Backends::VULKAN,
                    ..wgpu::InstanceDescriptor::new_without_display_handle()
                })
            };
            let instance = if let Some(ref exts) = ngx_exts {
                let inst_strs: Vec<&str> = exts
                    .instance_extensions
                    .iter()
                    .map(String::as_str)
                    .collect();
                match nv_wgpu::create_vulkan_instance_with_extensions(&inst_strs) {
                    Ok(inst) => inst,
                    Err(e) => {
                        ngx_ext_error =
                            Some(format!("Vulkan instance with NGX extensions failed: {e}"));
                        plain_instance()
                    }
                }
            } else {
                plain_instance()
            };
            let surface = instance
                .create_surface(window.clone())
                .expect("wgpu surface");

            let adapters = pollster_block(instance.enumerate_adapters(wgpu::Backends::VULKAN));
            let mut chosen = None;
            let mut used_ngx_exts = false;
            let mut open_errors: Vec<String> = Vec::new();
            for adapter in adapters {
                if !adapter.is_surface_supported(&surface) {
                    open_errors.push(format!("{}: surface not supported", adapter.get_info().name));
                    continue;
                }
                if let Some(ref exts) = ngx_exts {
                    let dev_strs: Vec<&str> =
                        exts.device_extensions.iter().map(String::as_str).collect();
                    match InteropContext::open_vulkan_with_extensions(
                        &adapter,
                        wgpu::Features::empty(),
                        &wgpu::Limits::default(),
                        Some("nv-rtx-demo"),
                        &dev_strs,
                    ) {
                        Ok(ctx) => {
                            chosen = Some((adapter, ctx));
                            used_ngx_exts = true;
                            break;
                        }
                        Err(e) => {
                            let msg = format!(
                                "{} with NGX device extensions: {e}",
                                adapter.get_info().name
                            );
                            if ngx_ext_error.is_none() {
                                ngx_ext_error = Some(msg.clone());
                            }
                            open_errors.push(msg);
                        }
                    }
                }
                match InteropContext::open_vulkan(
                    &adapter,
                    wgpu::Features::empty(),
                    &wgpu::Limits::default(),
                    Some("nv-rtx-demo"),
                ) {
                    Ok(ctx) => {
                        chosen = Some((adapter, ctx));
                        break;
                    }
                    Err(e) => open_errors.push(format!("{}: {e}", adapter.get_info().name)),
                }
            }
            let Some((adapter, interop)) = chosen else {
                panic!(
                    "no Vulkan+CUDA interop adapter ({})",
                    if open_errors.is_empty() {
                        "no Vulkan adapters found".to_string()
                    } else {
                        open_errors.join("; ")
                    }
                );
            };

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
                    Some(match ngx_ext_error {
                        Some(e) => format!("NGX Vulkan extensions unavailable: {e}"),
                        None => "NGX Vulkan extensions unavailable on this host/adapter".to_string(),
                    }),
                )
            };

            let (dlss_rr_available, dlss_rr_error) = match ngx_vulkan.as_ref() {
                Some(v) => match v.is_dlss_rr_available() {
                    Ok(avail) => (avail, None),
                    Err(e) => (false, Some(format!("DLSS-RR capability query failed: {e}"))),
                },
                None => (false, None),
            };
            let hdr_tonemap = HdrTonemapPipeline::new(interop.device());

            let mut surface_config = surface
                .get_default_config(&adapter, size.width.max(1), size.height.max(1))
                .expect("surface default config");
            surface_config.present_mode = wgpu::PresentMode::Fifo;
            let caps = surface.get_capabilities(&adapter);
            let surface_supports_copy_dst = caps.usages.contains(wgpu::TextureUsages::COPY_DST);
            if surface_supports_copy_dst {
                surface_config.usage |= wgpu::TextureUsages::COPY_DST;
            }
            let sdr_surface_format = caps
                .formats
                .iter()
                .copied()
                .find(|f| {
                    matches!(
                        f,
                        wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Rgba8Unorm
                    )
                })
                .unwrap_or(surface_config.format);
            let hdr_surface_supported = caps.formats.contains(&wgpu::TextureFormat::Rgba16Float);
            let initial_hdr = persisted.settings.hdr_display && hdr_surface_supported;
            surface_config.format = if initial_hdr {
                wgpu::TextureFormat::Rgba16Float
            } else {
                sdr_surface_format
            };
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
            // City-scale flight feel (set once: it survives `set_prefs` / `set_scheme`):
            // 1.25x per `RMB + Wheel` notch down to 0.1x, progressive Shift ramp, and the cursor
            // locked during orbit / pan / dolly drags.
            cam_driver.set_nav_extras(camera::input::NavExtras {
                fly_wheel_step: 1.25,
                fly_speed_min: 0.1,
                shift_fly_ramp: true,
                lock_cursor_on_drag: true,
            });
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
                sdr_surface_format,
                egui_wgpu::RendererOptions::default(),
            );
            let mut egui_renderer_hdr = hdr_surface_supported.then(|| {
                egui_wgpu::Renderer::new(
                    interop.device(),
                    wgpu::TextureFormat::Rgba16Float,
                    egui_wgpu::RendererOptions::default(),
                )
            });

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
            if let Some(ref mut hdr_r) = egui_renderer_hdr {
                let hdr_id = hdr_r.register_native_texture(
                    interop.device(),
                    &display_view,
                    wgpu::FilterMode::Linear,
                );
                assert_eq!(hdr_id, display_tex_id);
            }

            // F17: the scene is persisted by descriptor id; the 1-based index is only a fallback
            // for files written before `scene` existed.
            let scene_descriptors = crate::csg_scene_host::default_scene_descriptors();
            // A benchmark run never writes the user's config (its overrides are not user intent).
            let protect_config = persist_problem.is_some() || bench_args.is_some();
            if let Some(ref p) = persist_problem {
                eprintln!("nv-demo config: {p}");
            }
            let mut startup_msg = persist_problem;
            match persisted.scene.as_ref() {
                Some(saved) => match resolve_saved_scene(
                    &scene_descriptors,
                    saved,
                    persisted.settings.csg_asset_idx,
                ) {
                    Some((mode, asset)) => {
                        persisted.settings.csg_mode = mode;
                        persisted.settings.csg_asset_idx = asset;
                    }
                    None => {
                        persisted.settings.csg_mode = 0;
                        if startup_msg.is_none() {
                            startup_msg =
                                Some(format!("Saved scene '{}' is no longer registered", saved.id));
                        }
                    }
                },
                None => {
                    if persisted.settings.csg_mode as usize > scene_descriptors.len() {
                        persisted.settings.csg_mode = 0;
                    }
                }
            }

            let bench = bench_args.map(|args| {
                let mut run = crate::bench::BenchRun::new(args);
                if persisted.settings.csg_mode == 0 {
                    run.phase = crate::bench::BenchPhase::Done(Err(format!(
                        "no CSG scene to benchmark (saved scene {:?}; available: {})",
                        persisted.scene.as_ref().map(|s| s.id.as_str()),
                        scene_descriptors
                            .iter()
                            .map(|d| d.id)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )));
                }
                run
            });
            let mut settings = persisted.settings;
            settings.is_counters = is_counters;
            let mut startup_intent = StartupIntent::default();
            if settings.hdr_display && !hdr_surface_supported {
                settings.hdr_display = false;
                startup_intent.hdr_display = Some(true);
            }
            if settings.dlss_rr_on && !dlss_rr_available {
                settings.dlss_rr_on = false;
                startup_intent.dlss_rr_on = Some(true);
            }
            if settings.dlss_rr_on {
                settings.dlss_on = false;
                settings.denoiser_on = false;
            }
            startup_intent.ai_at_start = (settings.dlss_on, settings.denoiser_on);
            let initial_camera = pose_to_frame(cam_pose);
            let bus = AppBus::new(RenderStateSnapshot {
                settings,
                camera: initial_camera,
                interacting_until: None,
                reset_seq: 0,
                possess_free_yaw: 0.0,
                possess_free_pitch: 0.0,
                possess_auto_level: true,
                possess_zoom_offset: 0.0,
            });
            let render_thread = spawn_render_thread(
                Arc::clone(interop.cuda_context()),
                buf_ptrs,
                depth_ptrs,
                motion_ptrs,
                bus.clone(),
            );

            // NVML is optional telemetry: a failure only hides the GPU gauges, so it is logged.
            let nvml_lib = match nvml::Nvml::init() {
                Ok(n) => Some(Box::leak(Box::new(n)) as &'static nvml::Nvml),
                Err(e) => {
                    eprintln!("nv-demo NVML init failed (GPU telemetry disabled): {e}");
                    None
                }
            };
            let nvml_dev = nvml_lib.and_then(|n| match n.device_by_index(0) {
                Ok(d) => Some(d),
                Err(e) => {
                    eprintln!("nv-demo NVML device 0 unavailable (GPU telemetry disabled): {e}");
                    None
                }
            });
            let mut profiler = Profiler::new(Some(interop.cuda_context()));
            let telemetry = profiler.read_telemetry(nvml_dev.as_ref());

            let export_worker =
                ExportWorker::spawn(Arc::clone(interop.cuda_context()), bus.clone());
            // F07: a config file that could not be read in full is never overwritten; this
            // session then writes nowhere (see `persist` module docs).
            let persist_worker = if protect_config {
                crate::persist::PersistWorker::spawn_for_path(None)
            } else {
                crate::persist::PersistWorker::spawn()
            };

            // APP:F19: `persist::load_from` keeps only a layout that decodes and holds every tab
            // once, so a decode failure here means the file changed underneath; say so and
            // start from the default layout.
            let dock_state = match persisted
                .dock
                .as_ref()
                .map(crate::persist::dock_state_from_value)
            {
                Some(Ok(dock)) => dock,
                Some(Err(e)) => {
                    if startup_msg.is_none() {
                        startup_msg = Some(format!("Saved panel layout ignored: {e}"));
                    }
                    default_dock_state()
                }
                None => default_dock_state(),
            };
            let theme = persisted.theme;
            let dock_style = theme.apply(&egui_ctx);

            Self {
                window,
                surface,
                surface_config,
                surface_supports_copy_dst,
                sdr_surface_format,
                hdr_surface_supported,
                window_capture: None,
                dlss_pass: None,
                dlss_rr_pass: None,
                dlss_rr_available,
                cached_dlss_opt: None,
                ngx_vulkan,
                ngx_init_error,
                interop,
                hdr_tonemap,
                rr_last_exposure: None,
                egui_ctx,
                egui_winit,
                egui_renderer,
                egui_renderer_hdr,
                shared_rgba,
                shared_depth,
                shared_motion,
                shared_hdr: None,
                rr_shared: None,
                retired_buffers: Vec::new(),
                buf_gen: 0,
                render_w: DEFAULT_W,
                render_h: DEFAULT_H,
                desired_vp_size: (DEFAULT_W as f32, DEFAULT_H as f32),
                last_resize_at: Instant::now(),
                front_buf_idx: 0,
                has_frame: false,
                viewport_tex_dirty: false,
                dlss_dirty: false,
                display_texture,
                display_view,
                display_texture_hdr: None,
                display_view_hdr: None,
                display_tex_id,
                using_dlss_tex: false,
                bound_hdr_tex: false,
                last_jitter: [0.0, 0.0],
                last_saved_msg: startup_msg,
                flight: crate::flight_stats::FlightStats::new(),
                flight_rail_id: String::new(),
                flight_rail_active: false,
                bus,
                ui_cmd_scratch: Vec::with_capacity(16),
                render_ev_scratch: Vec::with_capacity(32),
                ui_scratch: UiScratch::default(),
                render_thread: Some(render_thread),
                export_worker,
                persist_worker,
                nvml_dev,
                _nvml: nvml_lib,
                profiler,
                telemetry,
                last_telemetry_at: Instant::now(),
                last_frame_at: Instant::now(),
                accum_count: 0,
                total_samples: 0,
                rt_ms: 0.0,
                is_counts: None,
                denoise_ms: 0.0,
                bloom_ms: 0.0,
                tonemap_ms: 0.0,
                cuda_compute_ms: 0.0,
                dlss_ms: 0.0,
                engine_phase: crate::app_events::EnginePhase::CompilingOptix,
                ui_fps: 60.0,
                rt_fps: 0.0,
                fast_preview: false,
                reset_seq: 0,
                interacting_until: None,
                is_recording: false,
                record_frames: 0,
                video_recorder: None,
                shutter_in_flight: false,
                bench,
                ser_supported: false,
                csg_stats: crate::csg_scene_host::CsgStats::default(),
                crowd_step_ms: 0.0,
                crowd_active_backend: crate::sdg_export::ActiveCrowdBackend::None,
                crowd_actor_count: 0,
                last_rendered_camera: initial_camera,
                scene_descriptors,
                scene_viewpoints: Vec::new(),
                pending_scene_auto_frame: None,
                rail_random_seed: 2026,
                smart_view_seed: 42,
                possess_info: None,
                possess_free_yaw: 0.0,
                possess_free_pitch: 0.0,
                possess_auto_level: true,
                possess_zoom_offset: 0.0,
                possess_saved_projection: None,
                viewport_toolbar: crate::viewport_toolbar::ToolbarState {
                    edge: persisted.viewport_toolbar.edge,
                    expanded: persisted.viewport_toolbar.expanded,
                    ..crate::viewport_toolbar::ToolbarState::default()
                },
                possess_toolbar: crate::viewport_toolbar::PossessToolbar::new(
                    persisted.viewport_toolbar.possess_bar,
                ),
                tod_transition: None,
                render_fatal: None,
                resize_acked: true,
                startup_intent,
                still_readback: None,
                dlss_rr_error,
                loaded_scene_key: None,
                cursor_grab: CursorGrabController::default(),
                last_sent_possess_look: (
                    0.0f32.to_bits(),
                    0.0f32.to_bits(),
                    0.0f32.to_bits(),
                    true,
                ),
                last_possess_camera: None,
                pick_possess_mode: false,
                last_smart_view: None,
                settings,
                last_sent_settings: settings,
                demo_anim: persisted.demo_anim.clone(),
                camera_rail: persisted.camera_rail.clone(),
                cam_damper: {
                    let mut d = crate::camera_rail::CameraDamper::default();
                    d.snap(cam_pose);
                    d
                },
                rail_blend_remaining: 0.0,
                cam_driver,
                last_cam_raw_input: None,
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

        /// `Some` once a `--bench-rail` run finished: the CSV path or why it failed.
        pub fn bench_outcome(&self) -> Option<Result<std::path::PathBuf, String>> {
            match self.bench.as_ref().map(|b| &b.phase) {
                Some(crate::bench::BenchPhase::Done(r)) => Some(r.clone()),
                _ => None,
            }
        }

        /// Starts a waiting benchmark once the scene's rails are installed: selects the
        /// requested cassette at rail time 0, starts the perf log and the 60 FPS lockstep.
        fn start_bench_if_ready(&mut self) {
            let Some(run) = self.bench.as_mut() else {
                return;
            };
            if run.views.as_ref().is_some_and(|v| v.phase == crate::bench::ViewPhase::WaitScene)
                && run.phase == crate::bench::BenchPhase::Running
            {
                // A views run switched scene: the new scene is in, place the camera.
                self.bench_view_begin();
                return;
            }
            if run.phase != crate::bench::BenchPhase::WaitingForRails {
                return;
            }
            let rail_id = match &run.args.target {
                crate::bench::BenchTarget::Rail(id) => id.clone(),
                crate::bench::BenchTarget::Views(_) => {
                    run.phase = crate::bench::BenchPhase::Running;
                    self.tod_transition = None;
                    self.rail_blend_remaining = 0.0;
                    self.shutter_in_flight = false;
                    self.send_render(RenderControl::SetRecording {
                        active: true,
                        fixed_fps: RECORD_FPS,
                        shutter_subframes: 1,
                    });
                    self.bench_view_begin();
                    return;
                }
            };
            let rails = self.camera_rail.scene_rails();
            let Some(idx) = rails.iter().position(|r| r.spec.id == rail_id) else {
                let ids: Vec<&str> = rails.iter().map(|r| r.spec.id.as_str()).collect();
                run.phase = crate::bench::BenchPhase::Done(Err(format!(
                    "rail {:?} not in this scene (available: {})",
                    rail_id,
                    ids.join(", ")
                )));
                return;
            };
            self.camera_rail.select_scene_rail(idx);
            if !self.camera_rail.set_playing(true) {
                run.phase = crate::bench::BenchPhase::Done(Err(format!(
                    "rail {:?} cannot play",
                    rail_id
                )));
                return;
            }
            let frames = run.args.frames.unwrap_or_else(|| {
                crate::bench::loop_frames(
                    self.camera_rail.duration_s,
                    RECORD_FPS,
                    self.settings.clamped_time_scale(),
                )
            });
            let start = RenderControl::StartPerfLog {
                path: run.args.out.clone(),
                frames,
                warmup: run.args.warmup,
                image: run.args.image.clone(),
                png_dir: run.args.png_dir.clone(),
            };
            run.phase = crate::bench::BenchPhase::Running;
            self.tod_transition = None;
            self.rail_blend_remaining = 0.0;
            self.shutter_in_flight = false;
            // Lockstep first: from here on the worker renders only the subframes this thread
            // sends, so the perf log (and its frame index reset) starts on the first rail frame.
            self.send_render(RenderControl::SetRecording {
                active: true,
                fixed_fps: RECORD_FPS,
                shutter_subframes: 1,
            });
            self.send_render(start);
        }

        /// Puts the current view of a `--bench-views` run in place: requests its scene when
        /// another one is loaded (`start_bench_if_ready` calls back on `SceneMetaUpdated`), else
        /// sets the camera and starts the view's warm-up. Past the last view, writes the CSV.
        fn bench_view_begin(&mut self) {
            let Some(run) = self.bench.as_mut() else {
                return;
            };
            let Some(vr) = run.views.as_mut() else {
                return;
            };
            self.settings.is_counters = false;
            let Some(view) = vr.views.get(vr.idx).cloned() else {
                run.phase = crate::bench::BenchPhase::Done(
                    crate::bench::write_views_csv(&run.args.out, &vr.rows).map(|()| run.args.out.clone()),
                );
                return;
            };
            let Some(pos) = self.scene_descriptors.iter().position(|d| d.id == view.scene) else {
                let ids: Vec<&str> = self.scene_descriptors.iter().map(|d| d.id).collect();
                run.phase = crate::bench::BenchPhase::Done(Err(format!(
                    "view {:?}: scene {:?} is not registered (available: {})",
                    view.name,
                    view.scene,
                    ids.join(", ")
                )));
                return;
            };
            let scene_idx = pos as u32 + 1;
            if self.loaded_scene_key != Some((scene_idx, 0)) {
                vr.phase = crate::bench::ViewPhase::WaitScene;
                self.settings.csg_mode = scene_idx;
                self.settings.csg_asset_idx = 0;
                return;
            }
            let (eye, target, vfov_deg) = match &view.cam {
                crate::bench::ViewCam::Explicit { eye, target, vfov_deg } => (*eye, *target, *vfov_deg),
                crate::bench::ViewCam::Viewpoint(id) => {
                    let Some(vp) = self.scene_viewpoints.iter().find(|v| &v.id == id) else {
                        let ids: Vec<&str> = self.scene_viewpoints.iter().map(|v| v.id.as_str()).collect();
                        run.phase = crate::bench::BenchPhase::Done(Err(format!(
                            "view {:?}: viewpoint {id:?} not in scene {:?} (available: {})",
                            view.name,
                            view.scene,
                            ids.join(", ")
                        )));
                        return;
                    };
                    ([vp.eye.x, vp.eye.y, vp.eye.z], [vp.target.x, vp.target.y, vp.target.z], vp.vfov_deg)
                }
            };
            let eye_v = glam::Vec3::from_array(eye);
            let delta = glam::Vec3::from_array(target) - eye_v;
            let dist = delta.length().max(0.25);
            let pose = CameraPose::from_eye_forward_up(
                eye_v,
                delta / dist,
                glam::Vec3::Y,
                vfov_deg.clamp(15.0, 110.0).to_radians(),
                self.cam_pose.znear,
                self.cam_pose.zfar.max(2000.0),
            );
            self.camera_rail.set_playing(false);
            self.cam_driver.controller_mut().set_pose(pose);
            self.cam_driver.controller_mut().set_orbit_distance(dist);
            self.cam_pose = self.cam_driver.controller().pose();
            self.rail_blend_remaining = 0.0;
            self.cam_damper.snap(self.cam_pose);
            vr.begin_view();
        }

        /// Feeds one rendered frame to a running `--bench-views` run.
        fn bench_view_frame(
            &mut self,
            rt_ms: f32,
            state: crate::perf_log::PerfState,
            counts: Option<crate::is_counters::IsCounts>,
        ) {
            let Some(run) = self.bench.as_mut().filter(|r| r.is_running()) else {
                return;
            };
            let Some(vr) = run.views.as_mut() else {
                return;
            };
            match vr.on_frame(rt_ms, state, counts) {
                crate::bench::ViewEvent::None => {}
                crate::bench::ViewEvent::SetCounters(on) => self.settings.is_counters = on,
                crate::bench::ViewEvent::ViewDone { .. } => self.bench_view_begin(),
            }
        }

        fn notify_recording(&mut self, active: bool) {
            self.shutter_in_flight = false;
            let fixed_fps = if active && self.settings.record_fixed_fps {
                RECORD_FPS
            } else {
                0
            };
            let shutter_subframes = if fixed_fps > 0 {
                self.settings.shutter_subframes()
            } else {
                1
            };
            self.send_render(RenderControl::SetRecording {
                active,
                fixed_fps,
                shutter_subframes,
            });
        }

        fn current_persisted_state(&self) -> crate::persist::PersistedState {
            crate::persist::PersistedState {
                window: self.win_state,
                theme: self.theme,
                settings: user_intent_settings(
                    &self.settings,
                    self.possess_saved_projection,
                    self.tod_transition.as_ref(),
                    self.startup_intent,
                ),
                demo_anim: self.demo_anim.clone(),
                camera: crate::persist::CameraState::from_runtime(
                    self.cam_mode,
                    self.cam_fly_mode,
                    self.cam_scheme,
                    self.cam_feel,
                    self.cam_pose,
                ),
                camera_rail: self.camera_rail.clone(),
                scene: saved_scene_of(
                    &self.scene_descriptors,
                    self.settings.csg_mode,
                    self.settings.csg_asset_idx,
                ),
                viewport_toolbar: crate::persist::PersistedToolbar {
                    edge: self.viewport_toolbar.edge,
                    expanded: self.viewport_toolbar.expanded,
                    possess_bar: self.possess_toolbar.prefs,
                },
                dock: match crate::persist::dock_state_to_value(&self.dock_state) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        // Keep the layout last written rather than erasing it from the file.
                        eprintln!("nv-demo config: {e}");
                        self.last_saved_state.dock.clone()
                    }
                },
            }
        }

        pub fn save_persisted(&mut self) {
            let cur = self.current_persisted_state();
            if cur != self.last_saved_state {
                self.persist_worker.enqueue(cur.clone());
                self.last_saved_state = cur;
            }
            self.last_persist_at = Instant::now();
        }

        pub fn flush_persisted_on_exit(&mut self) {
            let cur = self.current_persisted_state();
            let final_state = if cur != self.last_saved_state {
                self.last_saved_state = cur.clone();
                Some(cur)
            } else {
                None
            };
            self.persist_worker.flush_and_join(final_state);
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

        fn apply_cursor_os_action(&self, action: CursorOsAction) {
            if let Some(mode) = action.grab
                && let Err(e) = self.window.set_cursor_grab(mode)
            {
                if matches!(mode, winit::window::CursorGrabMode::Locked) {
                    if let Err(fallback_err) = self
                        .window
                        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                    {
                        eprintln!(
                            "nv-demo cursor grab failed (Locked: {e}; Confined fallback: {fallback_err})"
                        );
                    }
                } else {
                    eprintln!("nv-demo cursor grab ({mode:?}) failed: {e}");
                }
            }
            if let Some(pos) = action.restore_pos
                && let Err(e) = self.window.set_cursor_position(pos)
            {
                eprintln!("nv-demo cursor position restore failed: {e}");
            }
            if let Some(vis) = action.visible {
                self.window.set_cursor_visible(vis);
            }
        }

        pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
            match event {
                WindowEvent::CursorMoved { position, .. } => {
                    let act = self.cursor_grab.on_cursor_moved(*position);
                    self.apply_cursor_os_action(act);
                }
                WindowEvent::MouseInput { state, .. } => {
                    let act = self
                        .cursor_grab
                        .on_mouse_button(*state == winit::event::ElementState::Pressed);
                    self.apply_cursor_os_action(act);
                }
                WindowEvent::Focused(false) => {
                    self.last_cam_raw_input = None;
                    let act = self.cursor_grab.on_focus_lost();
                    self.apply_cursor_os_action(act);
                }
                _ => {}
            }
            let resp = self.egui_winit.on_window_event(&self.window, event);
            resp.repaint
        }

        pub fn handle_mouse_motion(&mut self, delta: (f64, f64)) {
            let act = self.cursor_grab.on_mouse_motion(delta);
            self.apply_cursor_os_action(act);
            self.egui_winit.on_mouse_motion(delta);
        }

        pub fn render(&mut self) {
            let now = Instant::now();
            let dt = (now - self.last_frame_at).as_secs_f32().clamp(1e-4, 0.1);
            self.last_frame_at = now;
            self.ui_fps = self.ui_fps * 0.9 + (1.0 / dt) * 0.1;

            if self.last_telemetry_at.elapsed().as_secs_f32() > 0.35 {
                self.profiler
                    .update_telemetry(self.nvml_dev.as_ref(), &mut self.telemetry);
                self.last_telemetry_at = now;
            }

            if let Some(ref active) = self.video_recorder {
                if self.export_worker.session_failed(active.session_id) {
                    self.video_recorder = None;
                    self.is_recording = false;
                    self.record_frames = 0;
                    self.notify_recording(false);
                } else {
                    self.record_frames = self.export_worker.recorded_frames();
                }
            }

            // Flight stats belong to one Auto-Drive run of one cassette.
            let rail_active = self.camera_rail.is_active();
            if rail_active
                && (!self.flight_rail_active
                    || self.flight_rail_id != self.camera_rail.selected_rail_id)
            {
                self.flight.reset();
                self.flight_rail_id
                    .clone_from(&self.camera_rail.selected_rail_id);
            }
            self.flight_rail_active = rail_active;

            // 1. Drain completed async events from the OptiX render thread (non-blocking).
            let mut frames_received = 0u32;
            let mut scene_rails_updated = false;
            let mut bench_frame: Option<(f32, crate::perf_log::PerfState, Option<crate::is_counters::IsCounts>)> = None;
            let mut perf_log_written: Option<Result<std::path::PathBuf, String>> = None;
            let mut fatal_reason: Option<String> = None;
            self.bus.poll_render_events(&mut self.render_ev_scratch);
            for boxed in &self.render_ev_scratch {
                let Some(ev) = downcast_event::<RenderEvent>(boxed) else {
                    continue;
                };
                match ev {
                    RenderEvent::FrameReady(msg) => {
                        if msg.gen_id != self.buf_gen {
                            continue;
                        }
                        frames_received += 1;
                        self.resize_acked = true;
                        self.shutter_in_flight = false;
                        if self.has_frame && self.front_buf_idx != msg.buf_idx {
                            self.bus.send_render(RenderControl::ReturnBuffer {
                                buf_idx: self.front_buf_idx,
                                gen_id: self.buf_gen,
                            });
                        }
                        self.front_buf_idx = msg.buf_idx;
                        self.has_frame = true;
                        self.viewport_tex_dirty = true;
                        self.dlss_dirty = true;
                        self.accum_count = msg.accum_count;
                        self.total_samples = msg.total_samples;
                        self.last_jitter = msg.jitter;
                        self.rt_ms = msg.rt_ms;
                        self.denoise_ms = msg.denoise_ms;
                        self.bloom_ms = msg.bloom_ms;
                        self.tonemap_ms = msg.tonemap_ms;
                        self.cuda_compute_ms = msg.cuda_compute_ms;
                        if !self.settings.dlss_on && !self.settings.dlss_rr_on {
                            self.dlss_ms = 0.0;
                        }
                        self.ser_supported = msg.ser_supported;
                        self.csg_stats = msg.csg_stats;
                        self.crowd_step_ms = msg.crowd_step_ms;
                        self.crowd_active_backend = msg.crowd_active_backend;
                        self.crowd_actor_count = msg.crowd_actor_count;
                        self.fast_preview = msg.fast_preview;
                        if msg.is_counts.is_some() {
                            self.is_counts = msg.is_counts;
                        }
                        // Lockstep (the only mode a views run uses) has one frame in flight, so
                        // at most one FrameReady arrives per UI frame.
                        bench_frame = Some((msg.perf.rt_ms, msg.perf.state, msg.is_counts));
                        self.engine_phase = crate::app_events::EnginePhase::Rendering;
                        self.last_possess_camera = msg.possess_camera;
                        self.last_rendered_camera = msg.rendered_camera;
                        let tensor_cores_ms = msg.denoise_ms + self.dlss_ms;
                        let total_gpu_ms =
                            (msg.rt_ms + tensor_cores_ms + msg.cuda_compute_ms).max(0.1);
                        let inst_rt_fps = 1000.0 / total_gpu_ms;
                        self.rt_fps = if self.rt_fps <= 0.01 {
                            inst_rt_fps
                        } else {
                            self.rt_fps * 0.85 + inst_rt_fps * 0.15
                        };
                        self.profiler.record_frame_engines(
                            msg.rt_ms,
                            msg.denoise_ms,
                            msg.bloom_ms,
                            msg.tonemap_ms,
                            msg.rt_ms,
                            tensor_cores_ms,
                            msg.cuda_compute_ms,
                            self.rt_fps,
                        );
                        self.retired_buffers.retain(|(g, _)| *g >= msg.gen_id);
                        if rail_active {
                            let st = &msg.csg_stats;
                            self.flight.push(
                                &msg.perf,
                                self.camera_rail.time_s,
                                self.camera_rail.duration_s,
                                msg.rendered_camera.fwd,
                                crate::flight_stats::SceneLoad {
                                    visible: st.visible_instances,
                                    lod: [st.lod0_count, st.lod1_count, st.lod2_count],
                                    culled: st.culled_count,
                                    paths: st.path_instances,
                                },
                            );
                        }
                    }
                    RenderEvent::SceneMetaUpdated(meta) => {
                        self.loaded_scene_key = Some((meta.scene_idx, meta.variant_idx));
                        self.scene_viewpoints = meta.viewpoints.clone();
                        self.camera_rail.set_scene_rails(meta.rails.clone());
                        if self.camera_rail.is_active() {
                            self.rail_blend_remaining = self.rail_blend_remaining.max(1.2);
                        }
                        scene_rails_updated = true;
                        if self.pending_scene_auto_frame == Some((meta.scene_idx, meta.variant_idx))
                        {
                            self.pending_scene_auto_frame = None;
                            if !self.camera_rail.is_active()
                                && let Some(vp) = self.scene_viewpoints.first()
                            {
                                let eye_v = glam::Vec3::new(vp.eye.x, vp.eye.y, vp.eye.z);
                                let tgt_v = glam::Vec3::new(vp.target.x, vp.target.y, vp.target.z);
                                let delta = tgt_v - eye_v;
                                let dist = delta.length().max(0.25);
                                let fwd = delta / dist;
                                let pose = CameraPose::from_eye_forward_up(
                                    eye_v,
                                    fwd,
                                    glam::Vec3::Y,
                                    vp.vfov_deg.clamp(15.0, 110.0).to_radians(),
                                    self.cam_pose.znear,
                                    self.cam_pose.zfar.max(2000.0),
                                );
                                self.cam_driver.controller_mut().set_pose(pose);
                                self.cam_driver.controller_mut().set_orbit_distance(dist);
                                self.cam_pose = self.cam_driver.controller().pose();
                                self.rail_blend_remaining = 0.0;
                                self.cam_damper.snap(self.cam_pose);
                            }
                        }
                    }
                    RenderEvent::RailGenerated(rail_spec) => {
                        self.camera_rail.push_generated_rail(rail_spec.clone());
                        self.rail_blend_remaining = RAIL_BLEND_SECS;
                    }
                    RenderEvent::SelectionChanged(sel) => {
                        self.last_saved_msg = Some(crate::pick::selection_status(sel));
                    }
                    RenderEvent::PossessChanged(info) => {
                        if let Some(pinfo) = info {
                            self.camera_rail.set_playing(false);
                            self.possess_free_yaw = 0.0;
                            self.possess_free_pitch = 0.0;
                            self.possess_zoom_offset = 0.0;
                            if let Some(m) = pinfo.mounts.get(pinfo.mount_idx) {
                                self.possess_auto_level = m.auto_level;
                                sync_mount_camera_projection(
                                    Some(m),
                                    &mut self.settings.camera_projection,
                                    &mut self.possess_saved_projection,
                                );
                            }
                            self.possess_info = Some(pinfo.clone());
                        } else {
                            self.possess_zoom_offset = 0.0;
                            sync_mount_camera_projection(
                                None,
                                &mut self.settings.camera_projection,
                                &mut self.possess_saved_projection,
                            );
                            if self.possess_info.is_some()
                                && let Some(pc) = self.last_possess_camera.take()
                            {
                                let fov_y = (2.0 * pc.tan_half_fov.max(0.05).atan()).clamp(0.2, 2.2);
                                let pose = CameraPose::from_eye_forward_up(
                                    Vec3::from_array(pc.eye),
                                    Vec3::from_array(pc.fwd),
                                    Vec3::from_array(pc.up),
                                    fov_y,
                                    self.cam_pose.znear,
                                    self.cam_pose.zfar.max(2000.0),
                                );
                                self.cam_driver.controller_mut().set_pose(pose);
                                self.cam_pose = self.cam_driver.controller().pose();
                                self.rail_blend_remaining = 0.0;
                                self.cam_damper.snap(self.cam_pose);
                            }
                            self.possess_info = None;
                        }
                    }
                    RenderEvent::SmartViewReady(sv) => {
                        self.last_smart_view = Some(*sv);
                        self.camera_rail.set_playing(false);
                        let pose = CameraPose::from_eye_forward_up(
                            Vec3::from_array(sv.eye),
                            Vec3::from_array(sv.forward),
                            Vec3::from_array(sv.up),
                            sv.vfov_deg.clamp(15.0, 110.0).to_radians(),
                            self.cam_pose.znear,
                            self.cam_pose.zfar.max(2000.0),
                        );
                        self.cam_driver.controller_mut().set_pose(pose);
                        self.cam_driver.controller_mut().set_orbit_distance(16.0);
                        self.cam_pose = self.cam_driver.controller().pose();
                        self.rail_blend_remaining = 1.0;
                    }
                    RenderEvent::PhaseChanged(phase) => {
                        self.engine_phase = *phase;
                    }
                    RenderEvent::StatusMessage(status) => {
                        self.last_saved_msg = Some(status.clone());
                    }
                    RenderEvent::PerfLogWritten(result) => {
                        perf_log_written = Some(result.clone());
                    }
                    RenderEvent::Fatal(reason) => {
                        if fatal_reason.is_none() {
                            fatal_reason = Some(reason.clone());
                        }
                    }
                }
            }
            if let Some((rt_ms, state, counts)) = bench_frame {
                self.bench_view_frame(rt_ms, state, counts);
            }
            if scene_rails_updated {
                self.start_bench_if_ready();
            }
            if let Some(result) = perf_log_written {
                self.last_saved_msg = Some(match &result {
                    Ok(path) => format!("Perf log written: {}", path.display()),
                    Err(e) => format!("Perf log failed: {e}"),
                });
                if let Some(run) = self.bench.as_mut()
                    && run.is_running()
                {
                    run.phase = crate::bench::BenchPhase::Done(result);
                }
            }
            if let Some(reason) = fatal_reason {
                self.enter_render_fatal(reason);
            }
            // F12: a worker that ended without `Fatal` (panic) is detected here; the check is one
            // atomic load per frame and the join cannot block because the thread has finished.
            if self.render_fatal.is_none()
                && self
                    .render_thread
                    .as_ref()
                    .is_some_and(JoinHandle::is_finished)
                && let Some(handle) = self.render_thread.take()
            {
                let reason = render_exit_reason(handle);
                self.enter_render_fatal(reason);
            }
            if let (Some(reason), Some(run)) = (self.render_fatal.as_ref(), self.bench.as_mut())
                && !matches!(run.phase, crate::bench::BenchPhase::Done(_))
            {
                run.phase = crate::bench::BenchPhase::Done(Err(format!(
                    "render worker stopped: {reason}"
                )));
            }

            // 2. Docked egui pass (60+ FPS, zero blocking; camera_egui::drive steps the camera).
            let raw_input = self.egui_winit.take_egui_input(&self.window);
            let prev_theme = self.theme;
            let lighting_before = LightingKey::of(&self.settings);
            let bench_lockstep = self.bench.as_ref().is_some_and(crate::bench::BenchRun::is_running);
            let cam_fixed_dt = if bench_lockstep || (self.is_recording && self.settings.record_fixed_fps) {
                // Lockstep recording steps camera strictly inside `RenderShutterSubframe` batches.
                Some(0.0)
            } else {
                None
            };
            let mut ui_ctx = UiContext {
                bus: &self.bus,
                settings: &mut self.settings,
                demo_anim: &mut self.demo_anim,
                display_tex_id: self.display_tex_id,
                render_w: self.render_w,
                render_h: self.render_h,
                desired_vp_size: &mut self.desired_vp_size,
                cam_driver: &mut self.cam_driver,
                cam_home: self.cam_home,
                cam_pose: &mut self.cam_pose,
                camera_rail: &mut self.camera_rail,
                cam_fixed_dt,
                last_cam_raw_input: &mut self.last_cam_raw_input,
                scratch: &mut self.ui_scratch,
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
                is_counts: self.is_counts,
                denoise_ms: self.denoise_ms,
                bloom_ms: self.bloom_ms,
                tonemap_ms: self.tonemap_ms,
                tensor_cores_ms: self.denoise_ms + self.dlss_ms,
                cuda_compute_ms: self.cuda_compute_ms,
                engine_phase: self.engine_phase,
                ui_fps: self.ui_fps,
                rt_fps: self.rt_fps,
                fast_preview: self.fast_preview,
                is_recording: self.is_recording,
                record_frames: self.record_frames,
                ser_supported: self.ser_supported,
                csg_stats: self.csg_stats,
                crowd_step_ms: self.crowd_step_ms,
                crowd_active_backend: self.crowd_active_backend,
                crowd_actor_count: self.crowd_actor_count,
                scene_descriptors: &self.scene_descriptors,
                scene_viewpoints: &self.scene_viewpoints,
                rail_random_seed: &mut self.rail_random_seed,
                dlss_rr_available: self.dlss_rr_available,
                hdr_surface_supported: self.hdr_surface_supported,
                viewport_focused: &mut self.viewport_focused,
                possess_info: self.possess_info.as_ref(),
                possess_free_yaw: &mut self.possess_free_yaw,
                possess_free_pitch: &mut self.possess_free_pitch,
                possess_auto_level: &mut self.possess_auto_level,
                possess_zoom_offset: &mut self.possess_zoom_offset,
                viewport_toolbar: &mut self.viewport_toolbar,
                possess_toolbar: &mut self.possess_toolbar,
                pick_possess_mode: &mut self.pick_possess_mode,
                smart_view_seed: &mut self.smart_view_seed,
                last_smart_view: self.last_smart_view,
                // A render-worker failure stays on screen; later status messages do not hide it.
                last_saved_msg: self
                    .render_fatal
                    .as_deref()
                    .or(self.last_saved_msg.as_deref()),
                flight: &mut self.flight,
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
            cancel_tod_on_user_edit(lighting_before, &mut self.settings, &mut self.tod_transition);

            let mut reset_accum_requested = false;
            let mut cam_snap_requested = false;
            let mut rail_blend_requested = false;
            let mut select_scene_target: Option<(u32, u32)> = None;
            let mut generate_random_rail_seed: Option<u64> = None;
            let mut save_png_requested = false;
            let mut save_jpeg_requested = false;
            let mut export_sdg_json_requested = false;
            let mut export_flight_requested = false;
            let mut toggle_record_requested = false;
            let mut reload_shader_requested = false;
            let mut possess_next_vehicle_requested = false;
            let mut select_possess_mount: Option<usize> = None;
            let mut cycle_possess_mount_delta: i32 = 0;
            let mut eject_possess_requested = false;
            let mut run_smart_view: Option<(u64, csg_scene::ProbeBackendKind)> = None;
            let mut run_smart_rail: Option<(u64, csg_scene::ProbeBackendKind)> = None;
            let mut poke_crowd_req: Option<(f32, bool)> = None;
            let mut spawn_crowd_stress: Option<usize> = None;
            let mut start_sdg_burst: Option<u32> = None;
            let mut cancel_sdg_burst_requested = false;
            self.bus.poll_ui_commands(&mut self.ui_cmd_scratch);
            if self.bench.is_some() {
                // A benchmark renders a fixed camera sequence: possess, scene, rail, record and
                // other UI commands (a stray click or key on the window) are dropped.
                self.ui_cmd_scratch.clear();
            }
            for boxed in &self.ui_cmd_scratch {
                let Some(cmd) = downcast_event::<UiCommand>(boxed) else {
                    continue;
                };
                match *cmd {
                    UiCommand::ResetAccum => reset_accum_requested = true,
                    UiCommand::SaveStill(StillFormat::Png) => save_png_requested = true,
                    UiCommand::SaveStill(StillFormat::Jpeg) => save_jpeg_requested = true,
                    UiCommand::ExportSdgJson => export_sdg_json_requested = true,
                    UiCommand::ExportFlightCsv => export_flight_requested = true,
                    UiCommand::ToggleRecord => toggle_record_requested = true,
                    UiCommand::ReloadShader => reload_shader_requested = true,
                    UiCommand::SelectScene {
                        scene_idx,
                        variant_idx,
                    } => {
                        select_scene_target = Some((scene_idx, variant_idx));
                    }
                    UiCommand::SetRoadMode(mode) => {
                        self.settings.csg_road_mode = mode.min(3);
                        reset_accum_requested = true;
                    }
                    UiCommand::ToggleFreezePose => {
                        self.settings.csg_freeze_pose = !self.settings.csg_freeze_pose;
                        reset_accum_requested = true;
                    }
                    UiCommand::GenerateRandomRail { seed } => {
                        generate_random_rail_seed = Some(seed);
                    }
                    UiCommand::SnapCamera => cam_snap_requested = true,
                    UiCommand::BlendRail => rail_blend_requested = true,
                    UiCommand::Pick(req) => self.send_render(RenderControl::Pick(req)),
                    UiCommand::PossessNextVehicle => possess_next_vehicle_requested = true,
                    UiCommand::SelectPossessMount { mount_idx } => {
                        select_possess_mount = Some(mount_idx);
                    }
                    UiCommand::CyclePossessMount { delta } => {
                        cycle_possess_mount_delta += delta;
                    }
                    UiCommand::EjectPossess => eject_possess_requested = true,
                    UiCommand::RunSmartView { seed, backend } => {
                        run_smart_view = Some((seed, backend));
                    }
                    UiCommand::RunSmartRail { seed, backend } => {
                        run_smart_rail = Some((seed, backend));
                    }
                    UiCommand::PokeCrowd { impulse, all } => {
                        poke_crowd_req = Some((impulse, all));
                    }
                    UiCommand::SpawnCrowdStress { target_count } => {
                        spawn_crowd_stress = Some(target_count);
                    }
                    UiCommand::StartSdgBurst { frames } => {
                        start_sdg_burst = Some(frames);
                    }
                    UiCommand::CancelSdgBurst => cancel_sdg_burst_requested = true,
                    UiCommand::SetTimeOfDay(tod) => {
                        let target_tod = tod.min(2);
                        if self.settings.tod_transition_enabled
                            && self.settings.tod_transition_secs > 0.05
                        {
                            self.tod_transition = Some(TodTransition::new(
                                &self.settings,
                                target_tod,
                                self.settings.tod_transition_secs,
                            ));
                        } else {
                            self.tod_transition = None;
                            crate::ui::apply_csg_time_of_day_preset(
                                &mut self.settings,
                                target_tod,
                            );
                        }
                    }
                }
            }
            if self.theme != prev_theme {
                self.dock_style = self.theme.apply(&self.egui_ctx);
            }

            // 3. Dynamic viewport resolution resize & DLSS (SR / RR) lifecycle.
            if self.settings.dlss_rr_on {
                self.settings.dlss_on = false;
                self.settings.denoiser_on = false;
                if self.ngx_vulkan.is_none() {
                    self.settings.dlss_rr_on = false;
                    self.last_saved_msg = Some(
                        self.ngx_init_error.clone().unwrap_or_else(|| {
                            "DLSS-RR unavailable: NGX not initialized".to_string()
                        }),
                    );
                } else if !self.dlss_rr_available {
                    self.settings.dlss_rr_on = false;
                    self.last_saved_msg = Some(self.dlss_rr_error.clone().unwrap_or_else(|| {
                        "DLSS-RR unavailable: SuperSamplingDenoising not supported on this host/runtime"
                            .to_string()
                    }));
                }
            } else if self.settings.dlss_on && self.ngx_vulkan.is_none() {
                self.settings.dlss_on = false;
                self.last_saved_msg = Some(
                    self.ngx_init_error
                        .clone()
                        .unwrap_or_else(|| "DLSS unavailable: NGX not initialized".to_string()),
                );
            }

            // Wireframe lines are drawn at the native viewport resolution: no DLSS input scale, no
            // render scale (a reduced internal resolution upscaled to the viewport blurs them).
            let wire = self.settings.display_mode == crate::wireframe::DisplayMode::Wireframe;
            // The renderer leaves the overlay out of recorded frames when captures exclude it.
            self.settings.capture_active = self.video_recorder.is_some();
            let dlss_any = !wire && (self.settings.dlss_on || self.settings.dlss_rr_on);
            let (out_w, out_h) =
                align_render_size(self.desired_vp_size.0, self.desired_vp_size.1, 1.0);
            let (target_w, target_h) = if dlss_any {
                if let Some(ref ngx) = self.ngx_vulkan {
                    let is_rr = self.settings.dlss_rr_on;
                    let key = (out_w, out_h, self.settings.dlss_quality, is_rr);
                    let opt_res = match self.cached_dlss_opt {
                        Some((k, opt)) if k == key => Ok(opt),
                        _ => {
                            let q = DLSS_QUALITY_MODES[dlss_quality_index(self.settings.dlss_quality)].ngx;
                            let res = if is_rr {
                                ngx.query_dlss_rr_optimal_settings(out_w, out_h, q)
                            } else {
                                ngx.query_dlss_optimal_settings(out_w, out_h, q)
                            };
                            if let Ok(opt) = res {
                                self.cached_dlss_opt = Some((key, opt));
                            }
                            res
                        }
                    };
                    match opt_res {
                        Ok(opt) => dlss_render_dims(&opt, out_w, out_h),
                        Err(e) => {
                            if is_rr {
                                self.settings.dlss_rr_on = false;
                                self.last_saved_msg = Some(format!("DLSS-RR query failed: {e}"));
                            } else {
                                self.settings.dlss_on = false;
                                self.last_saved_msg = Some(format!("DLSS query failed: {e}"));
                            }
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
                    if wire { 1.0 } else { self.settings.render_scale },
                )
            };

            let hdr_active = self.settings.hdr_display && self.hdr_surface_supported;
            let desired_surface_fmt = if hdr_active {
                wgpu::TextureFormat::Rgba16Float
            } else {
                self.sdr_surface_format
            };
            if desired_surface_fmt != self.surface_config.format {
                self.surface_config.format = desired_surface_fmt;
                self.surface
                    .configure(self.interop.device(), &self.surface_config);
                self.rr_last_exposure = None;
            }

            let recording_vp_pq10 = self.video_recorder.as_ref().is_some_and(|r| {
                r.source == CaptureSource::Viewport
                    && r.profile == crate::gpu_export::VideoProfile::Hdr10Pq
            });
            let need_hdr_buf = need_shared_hdr_buffer(
                hdr_active,
                recording_vp_pq10,
                self.settings.dlss_rr_on,
                self.settings.launch_aov_mode(),
                self.dlss_rr_pass.is_some(),
            );
            let dlss_toggled = self.settings.dlss_on != self.last_sent_settings.dlss_on
                || self.settings.dlss_rr_on != self.last_sent_settings.dlss_rr_on
                || self.settings.dlss_quality != self.last_sent_settings.dlss_quality;
            let needs_rr_bufs = self.settings.dlss_rr_on && self.rr_shared.is_none();
            let drop_rr_bufs = !self.settings.dlss_rr_on && self.rr_shared.is_some();
            let needs_hdr_bufs = need_hdr_buf && self.shared_hdr.is_none();
            let drop_hdr_bufs = !need_hdr_buf && self.shared_hdr.is_some();
            if self.render_fatal.is_none()
                && resize_allowed(
                    target_w != self.render_w || target_h != self.render_h,
                    needs_rr_bufs || drop_rr_bufs || needs_hdr_bufs || drop_hdr_bufs,
                    self.last_resize_at.elapsed().as_millis() >= 120,
                    dlss_toggled,
                    self.resize_acked,
                )
            {
                self.last_resize_at = Instant::now();
                let rr_alloc = if self.settings.dlss_rr_on {
                    DlssRrSharedBuffers::new(&self.interop, target_w, target_h).map(Some)
                } else {
                    Ok(None)
                };
                let hdr_alloc = if need_hdr_buf {
                    allocate_hdr_target(&self.interop, target_w, target_h).map(Some)
                } else {
                    Ok(None)
                };
                match (
                    allocate_render_targets(&self.interop, target_w, target_h),
                    rr_alloc,
                    hdr_alloc,
                ) {
                    (
                        Ok(RenderTargets {
                            rgba: new_shared,
                            depth: new_depth,
                            motion: new_motion,
                            display_texture: new_tex,
                        }),
                        Ok(new_rr_shared),
                        Ok(new_hdr_target),
                    ) => {
                        let new_ptrs = [new_shared[0].cu_deviceptr(), new_shared[1].cu_deviceptr()];
                        let new_depth_ptrs =
                            [new_depth[0].cu_deviceptr(), new_depth[1].cu_deviceptr()];
                        let new_motion_ptrs =
                            [new_motion[0].cu_deviceptr(), new_motion[1].cu_deviceptr()];
                        let (new_hdr_shared, new_hdr_tex, new_hdr_view, new_hdr_ptrs) =
                            if let Some((h_bufs, h_tex)) = new_hdr_target {
                                let ptrs = [h_bufs[0].cu_deviceptr(), h_bufs[1].cu_deviceptr()];
                                let v = h_tex.create_view(&wgpu::TextureViewDescriptor::default());
                                (Some(h_bufs), Some(h_tex), Some(v), Some(ptrs))
                            } else {
                                (None, None, None, None)
                            };
                        let new_rr_ptrs = new_rr_shared.as_ref().map(DlssRrSharedBuffers::ptrs);
                        let new_view = new_tex.create_view(&wgpu::TextureViewDescriptor::default());
                        if !self.using_dlss_tex {
                            let active_v = if hdr_active {
                                new_hdr_view.as_ref().unwrap_or(&new_view)
                            } else {
                                &new_view
                            };
                            bind_display_view(
                                &mut self.egui_renderer,
                                self.egui_renderer_hdr.as_mut(),
                                self.interop.device(),
                                active_v,
                                self.display_tex_id,
                            );
                            self.bound_hdr_tex = hdr_active && new_hdr_view.is_some();
                        }
                        let old_shared = std::mem::replace(&mut self.shared_rgba, new_shared);
                        let old_depth = std::mem::replace(&mut self.shared_depth, new_depth);
                        let old_motion = std::mem::replace(&mut self.shared_motion, new_motion);
                        self.retired_buffers.push((self.buf_gen, old_shared));
                        self.retired_buffers.push((self.buf_gen, old_depth));
                        self.retired_buffers.push((self.buf_gen, old_motion));
                        if let Some(old_hdr) =
                            std::mem::replace(&mut self.shared_hdr, new_hdr_shared)
                        {
                            self.retired_buffers.push((self.buf_gen, old_hdr));
                        }
                        if let Some(old_rr) = std::mem::replace(&mut self.rr_shared, new_rr_shared)
                        {
                            old_rr.retire_into(self.buf_gen, &mut self.retired_buffers);
                        }
                        self.display_texture = new_tex;
                        self.display_view = new_view;
                        self.display_texture_hdr = new_hdr_tex;
                        self.display_view_hdr = new_hdr_view;
                        self.render_w = target_w;
                        self.render_h = target_h;
                        self.buf_gen = self.buf_gen.wrapping_add(1);
                        self.shutter_in_flight = false;
                        self.has_frame = false;
                        self.viewport_tex_dirty = false;
                        self.dlss_dirty = false;
                        self.front_buf_idx = 0;
                        self.accum_count = 0;
                        self.total_samples = 0;
                        self.resize_acked = false;
                        self.send_render(RenderControl::Resize {
                            width: target_w,
                            height: target_h,
                            buf_ptrs: new_ptrs,
                            depth_ptrs: new_depth_ptrs,
                            motion_ptrs: new_motion_ptrs,
                            hdr_ptrs: new_hdr_ptrs,
                            rr_ptrs: new_rr_ptrs,
                            gen_id: self.buf_gen,
                        });
                    }
                    (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => {
                        if self.settings.dlss_rr_on && self.rr_shared.is_none() {
                            self.settings.dlss_rr_on = false;
                        }
                        if self.settings.hdr_display && self.shared_hdr.is_none() {
                            self.settings.hdr_display = false;
                        }
                        self.last_saved_msg = Some(format!("Resize failed: {e}"));
                    }
                }
            }

            if self.settings.dlss_rr_on && !wire {
                self.dlss_pass = None;
                let needs_rebuild = self.dlss_rr_pass.as_ref().is_none_or(|p| {
                    p.render_w != self.render_w
                        || p.render_h != self.render_h
                        || p.out_w != out_w
                        || p.out_h != out_h
                        || p.quality != self.settings.dlss_quality
                });
                if needs_rebuild {
                    if self.using_dlss_tex {
                        let base_v = if hdr_active {
                            self.display_view_hdr.as_ref().unwrap_or(&self.display_view)
                        } else {
                            &self.display_view
                        };
                        bind_display_view(
                            &mut self.egui_renderer,
                            self.egui_renderer_hdr.as_mut(),
                            self.interop.device(),
                            base_v,
                            self.display_tex_id,
                        );
                        self.using_dlss_tex = false;
                        self.bound_hdr_tex = hdr_active && self.display_view_hdr.is_some();
                    }
                    self.dlss_rr_pass = None;
                    if let Some(ref ngx) = self.ngx_vulkan {
                        match DlssRrPass::new(
                            &self.interop,
                            ngx,
                            &self.hdr_tonemap,
                            self.render_w,
                            self.render_h,
                            out_w,
                            out_h,
                            self.settings.dlss_quality,
                        ) {
                            Ok(pass) => {
                                if self.settings.launch_aov_mode() == 0 {
                                    let rr_v = if hdr_active {
                                        &pass.display_hdr_view
                                    } else {
                                        &pass.display_view
                                    };
                                    bind_display_view(
                                        &mut self.egui_renderer,
                                        self.egui_renderer_hdr.as_mut(),
                                        self.interop.device(),
                                        rr_v,
                                        self.display_tex_id,
                                    );
                                    self.using_dlss_tex = true;
                                    self.bound_hdr_tex = hdr_active;
                                }
                                self.dlss_dirty = self.has_frame;
                                self.rr_last_exposure = None;
                                self.dlss_rr_pass = Some(pass);
                            }
                            Err(e) => {
                                self.settings.dlss_rr_on = false;
                                self.last_saved_msg = Some(format!("DLSS-RR init failed: {e}"));
                            }
                        }
                    }
                }
                if let Some(ref mut pass) = self.dlss_rr_pass {
                    if self.settings.launch_aov_mode() == 0 {
                        if !self.using_dlss_tex || self.bound_hdr_tex != hdr_active {
                            let rr_v = if hdr_active {
                                &pass.display_hdr_view
                            } else {
                                &pass.display_view
                            };
                            bind_display_view(
                                &mut self.egui_renderer,
                                self.egui_renderer_hdr.as_mut(),
                                self.interop.device(),
                                rr_v,
                                self.display_tex_id,
                            );
                            let was_aov = !self.using_dlss_tex;
                            self.using_dlss_tex = true;
                            self.bound_hdr_tex = hdr_active;
                            if was_aov {
                                self.dlss_dirty = self.has_frame;
                                pass.reset_next = true;
                            }
                            self.rr_last_exposure = None;
                        }
                    } else if self.using_dlss_tex {
                        let base_v = if hdr_active {
                            self.display_view_hdr.as_ref().unwrap_or(&self.display_view)
                        } else {
                            &self.display_view
                        };
                        bind_display_view(
                            &mut self.egui_renderer,
                            self.egui_renderer_hdr.as_mut(),
                            self.interop.device(),
                            base_v,
                            self.display_tex_id,
                        );
                        self.using_dlss_tex = false;
                        self.bound_hdr_tex = hdr_active && self.display_view_hdr.is_some();
                        self.viewport_tex_dirty = self.has_frame;
                    }
                }
            } else if self.settings.dlss_on && !wire {
                self.dlss_rr_pass = None;
                let needs_rebuild = self.dlss_pass.as_ref().is_none_or(|p| {
                    p.render_w != self.render_w
                        || p.render_h != self.render_h
                        || p.out_w != out_w
                        || p.out_h != out_h
                        || p.quality != self.settings.dlss_quality
                });
                if needs_rebuild {
                    if self.using_dlss_tex {
                        let base_v = if hdr_active {
                            self.display_view_hdr.as_ref().unwrap_or(&self.display_view)
                        } else {
                            &self.display_view
                        };
                        bind_display_view(
                            &mut self.egui_renderer,
                            self.egui_renderer_hdr.as_mut(),
                            self.interop.device(),
                            base_v,
                            self.display_tex_id,
                        );
                        self.using_dlss_tex = false;
                        self.bound_hdr_tex = hdr_active && self.display_view_hdr.is_some();
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
                                bind_display_view(
                                    &mut self.egui_renderer,
                                    self.egui_renderer_hdr.as_mut(),
                                    self.interop.device(),
                                    &pass.output_view,
                                    self.display_tex_id,
                                );
                                self.using_dlss_tex = true;
                                self.bound_hdr_tex = false;
                                self.dlss_dirty = self.has_frame;
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
                let want_hdr_view = hdr_active && self.display_view_hdr.is_some();
                if self.using_dlss_tex || self.bound_hdr_tex != want_hdr_view {
                    let base_v = if want_hdr_view {
                        self.display_view_hdr.as_ref().unwrap_or(&self.display_view)
                    } else {
                        &self.display_view
                    };
                    bind_display_view(
                        &mut self.egui_renderer,
                        self.egui_renderer_hdr.as_mut(),
                        self.interop.device(),
                        base_v,
                        self.display_tex_id,
                    );
                    self.using_dlss_tex = false;
                    self.bound_hdr_tex = want_hdr_view;
                    self.viewport_tex_dirty = self.has_frame;
                }
                self.dlss_pass = None;
                self.dlss_rr_pass = None;
            }

            if reset_accum_requested {
                if let Some(ref mut p) = self.dlss_pass {
                    p.reset_next = true;
                }
                if let Some(ref mut p) = self.dlss_rr_pass {
                    p.reset_next = true;
                }
            }

            let mut capture_window_png = false;
            let mut capture_window_jpeg = false;
            let mut companion_sdg_requested = false;
            let still_src = viewport_still_source(
                self.settings.dlss_rr_on,
                self.settings.launch_aov_mode(),
                self.dlss_rr_pass.is_some(),
                self.settings.dlss_on,
                self.dlss_pass.is_some(),
                hdr_active,
            );
            if save_png_requested {
                if self.settings.capture_source == CaptureSource::FullWindow {
                    capture_window_png = true;
                } else if self.has_frame {
                    let path = PathBuf::from(RENDERS_DIR).join(output_file_name("nv-demo", ".png"));
                    let res = match self.copy_displayed_viewport(still_src) {
                        Some(Ok(img)) => {
                            let byte_len = (img.pitch_px as usize)
                                .saturating_mul(img.height as usize)
                                .saturating_mul(img.fmt.bytes_per_pixel() as usize);
                            readback_device_bytes(self.interop.cuda_context(), img.ptr, byte_len)
                                .and_then(|raw| {
                                    self.export_worker.enqueue_png_pitched(
                                        raw,
                                        img.width,
                                        img.height,
                                        img.pitch_px,
                                        img.fmt,
                                        path.clone(),
                                    )
                                })
                        }
                        Some(Err(e)) => Err(e),
                        None => {
                            let byte_len = (self.render_w as usize)
                                .saturating_mul(self.render_h as usize)
                                .saturating_mul(4);
                            readback_device_bytes(
                                self.interop.cuda_context(),
                                self.shared_rgba[self.front_buf_idx].cu_deviceptr(),
                                byte_len,
                            )
                            .and_then(|rgba| {
                                self.export_worker.enqueue_png_rgba(
                                    rgba,
                                    self.render_w,
                                    self.render_h,
                                    path.clone(),
                                )
                            })
                        }
                    };
                    self.last_saved_msg = Some(match res {
                        Ok(()) => format!("Saving {}...", path.display()),
                        Err(e) => e,
                    });
                } else {
                    self.last_saved_msg = Some("No rendered frame yet: PNG not saved".to_string());
                }
                if self.settings.csg_mode > 0 && self.settings.csg_freeze_pose {
                    companion_sdg_requested = true;
                }
            }
            if save_jpeg_requested {
                if self.settings.capture_source == CaptureSource::FullWindow {
                    capture_window_jpeg = true;
                } else if self.has_frame {
                    let dir = std::path::Path::new(RENDERS_DIR);
                    if let Err(e) = std::fs::create_dir_all(dir) {
                        self.last_saved_msg = Some(format!("Failed to create {RENDERS_DIR}/: {e}"));
                    } else {
                        let path = dir.join(output_file_name("nv-demo", ".jpg"));
                        match self.copy_displayed_viewport(still_src) {
                            Some(Ok(img)) => {
                                self.last_saved_msg = Some(
                                    match self.export_worker.enqueue_jpeg_device(
                                        img,
                                        path.clone(),
                                        92,
                                    ) {
                                        Ok(()) => format!("Saving {}...", path.display()),
                                        Err(e) => format!("Viewport JPEG failed: {e}"),
                                    },
                                );
                            }
                            Some(Err(e)) => {
                                self.last_saved_msg = Some(format!("Viewport JPEG failed: {e}"));
                            }
                            None => self.send_render(RenderControl::SaveJpeg { path, quality: 92 }),
                        }
                    }
                } else {
                    self.last_saved_msg = Some("No rendered frame yet: JPEG not saved".to_string());
                }
                if self.settings.csg_mode > 0 && self.settings.csg_freeze_pose {
                    companion_sdg_requested = true;
                }
            }
            if export_flight_requested {
                let dir = std::path::Path::new(RENDERS_DIR);
                self.last_saved_msg = Some(match std::fs::create_dir_all(dir) {
                    Err(e) => format!("Failed to create {RENDERS_DIR}/: {e}"),
                    Ok(()) => {
                        let path = dir.join(output_file_name("nv-demo-flight", ".csv"));
                        match self.flight.export_csv(&path) {
                            Ok(()) => format!(
                                "Flight stats: {} frames written to {}",
                                self.flight.frames(),
                                path.display()
                            ),
                            Err(e) => format!("Flight stats export failed: {e}"),
                        }
                    }
                });
            }
            if (export_sdg_json_requested || companion_sdg_requested) && self.settings.csg_mode > 0 {
                let dir = std::path::Path::new(RENDERS_DIR);
                if let Err(e) = std::fs::create_dir_all(dir) {
                    self.last_saved_msg = Some(format!("Failed to create {RENDERS_DIR}/: {e}"));
                } else {
                    let path = dir.join(output_file_name("nv-demo-sdg", ".json"));
                    self.send_render(RenderControl::ExportSdgJson {
                        path,
                        occluder: self.settings.sdg_occluder_backend,
                    });
                }
            }
            if let Some(frames) = start_sdg_burst
                && self.settings.csg_mode > 0
            {
                let dir = std::path::Path::new(RENDERS_DIR).join(output_file_name("sdg-burst", ""));
                self.send_render(RenderControl::StartSdgBurst {
                    dir,
                    frames: frames.clamp(1, 240),
                    occluder: self.settings.sdg_occluder_backend,
                });
            }
            if cancel_sdg_burst_requested {
                self.send_render(RenderControl::CancelSdgBurst);
            }
            if let Some((impulse, all)) = poke_crowd_req
                && self.settings.csg_mode > 0
            {
                let active_cam = self.last_possess_camera.unwrap_or(self.last_sent_camera);
                self.send_render(RenderControl::PokeCrowd {
                    origin: active_cam.eye,
                    forward: active_cam.fwd,
                    impulse: impulse.clamp(0.5, 4.0),
                    all,
                });
            }
            if let Some(target_count) = spawn_crowd_stress
                && self.settings.csg_mode > 0
            {
                let active_cam = self.last_possess_camera.unwrap_or(self.last_sent_camera);
                self.send_render(RenderControl::SpawnCrowdStress {
                    target_count,
                    origin: active_cam.eye,
                });
            }
            if toggle_record_requested {
                if let Some(active) = self.video_recorder.take() {
                    self.export_worker.finish_video(active.session_id, None);
                    self.last_saved_msg = Some("Finalizing MP4...".to_string());
                    self.is_recording = false;
                    self.record_frames = 0;
                    self.notify_recording(false);
                } else if self.render_fatal.is_some() {
                    self.last_saved_msg =
                        Some("Recording unavailable: the render worker has stopped".to_string());
                } else {
                    let use_pq10 = match self.settings.record_video_mode {
                        1 => false,
                        2 => true,
                        _ => hdr_active,
                    };
                    let profile = if use_pq10 {
                        crate::gpu_export::VideoProfile::Hdr10Pq
                    } else {
                        crate::gpu_export::VideoProfile::SdrH264
                    };
                    let tag = if use_pq10 { "pq10" } else { "sdr" };
                    let dir = std::path::Path::new(RENDERS_DIR);
                    if let Err(e) = std::fs::create_dir_all(dir) {
                        self.last_saved_msg = Some(format!("Failed to create {RENDERS_DIR}/: {e}"));
                    } else {
                        let full_window = self.settings.capture_source == CaptureSource::FullWindow;
                        if full_window {
                            let win_w = self.surface_config.width;
                            let win_h = self.surface_config.height;
                            let win_tex_fmt = self.surface_config.format;
                            let win_fmt =
                                crate::gpu_export::SourcePixelFormat::from_wgpu(win_tex_fmt);
                            let even_dims = crate::gpu_export::align_even_dims(win_w, win_h);
                            let path = dir.join(output_file_name("nv-demo", &format!("-win-{tag}.mp4")));

                            if let (Some(src_fmt), Some((rec_w, rec_h))) = (win_fmt, even_dims) {
                                if use_pq10
                                    && src_fmt != crate::gpu_export::SourcePixelFormat::ScrgbF16
                                {
                                    self.last_saved_msg = Some(
                                        "Full Window PQ10 requires HDR View (switch View to HDR or Video to SDR/Auto)"
                                            .to_string(),
                                    );
                                } else {
                                    let cap_ok = match self.window_capture.as_ref() {
                                        Some(cap) if cap.matches(win_w, win_h, win_tex_fmt) => {
                                            Ok(cap.copy_pitch_px)
                                        }
                                        _ => match FullWindowCapture::new(
                                            &self.interop,
                                            win_w,
                                            win_h,
                                            win_tex_fmt,
                                        ) {
                                            Ok(cap) => {
                                                let pitch = cap.copy_pitch_px;
                                                self.window_capture = Some(cap);
                                                Ok(pitch)
                                            }
                                            Err(e) => Err(e),
                                        },
                                    };
                                    match cap_ok.and_then(|copy_pitch_px| {
                                        self.export_worker
                                            .start_video(
                                                rec_w,
                                                rec_h,
                                                RECORD_FPS,
                                                profile,
                                                src_fmt,
                                                path.clone(),
                                            )
                                            .map(|session_id| (session_id, copy_pitch_px))
                                    }) {
                                        Ok((session_id, copy_pitch_px)) => {
                                            self.video_recorder = Some(ActiveRecorder {
                                                session_id,
                                                profile,
                                                source: CaptureSource::FullWindow,
                                                pixel_format: src_fmt,
                                                width: rec_w,
                                                height: rec_h,
                                                fps: RECORD_FPS,
                                                copy_buf: None,
                                                copy_pitch_px,
                                                last_frame_at: None,
                                                hdr_wait_frames: 0,
                                            });
                                            self.is_recording = true;
                                            self.record_frames = 0;
                                            self.notify_recording(true);
                                            self.last_saved_msg = Some(format!(
                                                "Starting window record: {}",
                                                path.display()
                                            ));
                                        }
                                        Err(e) => {
                                            self.last_saved_msg =
                                                Some(format!("Record start failed: {e}"));
                                        }
                                    }
                                }
                            } else {
                                self.last_saved_msg = Some(format!(
                                    "Full Window record unsupported ({win_w}x{win_h}, {win_tex_fmt:?})"
                                ));
                            }
                        } else if use_pq10 && self.settings.dlss_on && !self.settings.dlss_rr_on {
                            self.last_saved_msg = Some(
                                "PQ10 requires DLSS RR or OptiX/Raw (DLSS SR is 8-bit SDR)"
                                    .to_string(),
                            );
                        } else {
                            let (src_w, src_h, uses_dlss_tex) =
                                if viewport_uses_dlss_rr_output(
                                    self.settings.dlss_rr_on,
                                    self.settings.launch_aov_mode(),
                                    self.dlss_rr_pass.is_some(),
                                ) && let Some(ref rr) = self.dlss_rr_pass
                                {
                                    (rr.out_w & !1, rr.out_h & !1, true)
                                } else if self.settings.dlss_on
                                    && let Some(ref dlss) = self.dlss_pass
                                {
                                    (dlss.out_w & !1, dlss.out_h & !1, true)
                                } else {
                                    (self.render_w & !1, self.render_h & !1, false)
                                };
                            let path = dir.join(output_file_name("nv-demo", &format!("-{tag}.mp4")));

                            let src_fmt = if use_pq10 {
                                crate::gpu_export::SourcePixelFormat::ScrgbF16
                            } else {
                                crate::gpu_export::SourcePixelFormat::Rgba8
                            };
                            let bpp: u32 = src_fmt.bytes_per_pixel();
                            let copy_bpr = (src_w * bpp).next_multiple_of(256);
                            let copy_pitch_px = copy_bpr / bpp;
                            let copy_buf_res = if uses_dlss_tex {
                                self.interop
                                    .create_buffer(&SharedBufferDescriptor {
                                        label: Some("video-rec-copy"),
                                        size: u64::from(copy_bpr) * u64::from(src_h),
                                        usage: wgpu::BufferUsages::COPY_DST,
                                    })
                                    .map(Some)
                                    .map_err(|e| format!("record copy buffer: {e}"))
                            } else {
                                Ok(None)
                            };
                            match copy_buf_res.and_then(|copy_buf| {
                                self.export_worker
                                    .start_video(src_w, src_h, RECORD_FPS, profile, src_fmt, path.clone())
                                    .map(|session_id| (session_id, copy_buf))
                            }) {
                                Ok((session_id, copy_buf)) => {
                                    self.video_recorder = Some(ActiveRecorder {
                                        session_id,
                                        profile,
                                        source: CaptureSource::Viewport,
                                        pixel_format: src_fmt,
                                        width: src_w,
                                        height: src_h,
                                        fps: RECORD_FPS,
                                        copy_buf,
                                        copy_pitch_px,
                                        last_frame_at: None,
                                        hdr_wait_frames: 0,
                                    });
                                    self.is_recording = true;
                                    self.record_frames = 0;
                                    self.rr_last_exposure = None;
                                    self.notify_recording(true);
                                    self.last_saved_msg =
                                        Some(format!("Starting record: {}", path.display()));
                                }
                                Err(e) => {
                                    self.last_saved_msg =
                                        Some(format!("Record start failed: {e}"));
                                }
                            }
                        }
                    }
                }
            }
            if let Some((scene_idx, variant_idx)) = select_scene_target {
                self.pending_scene_auto_frame =
                    scene_auto_frame_target(self.loaded_scene_key, (scene_idx, variant_idx));
                if scene_idx == 0 {
                    // Classic unloads the CSG world: returning to the previous scene rebuilds it.
                    self.loaded_scene_key = None;
                }
                self.camera_rail.set_playing(false);
                if self.possess_info.is_some() {
                    self.send_render(RenderControl::EjectPossess);
                }
            }
            if let Some(seed) = generate_random_rail_seed {
                if self.possess_info.is_some() {
                    self.send_render(RenderControl::EjectPossess);
                }
                self.send_render(RenderControl::GenerateRandomRail { seed });
            }
            if possess_next_vehicle_requested {
                self.send_render(RenderControl::PossessNextVehicle);
            }
            if let Some(mount_idx) = select_possess_mount {
                self.send_render(RenderControl::SetPossessMount { mount_idx });
            } else if cycle_possess_mount_delta != 0
                && let Some(ref info) = self.possess_info
                && !info.mounts.is_empty()
            {
                let next_idx = (info.mount_idx as i32 + cycle_possess_mount_delta)
                    .rem_euclid(info.mounts.len() as i32) as usize;
                self.send_render(RenderControl::SetPossessMount { mount_idx: next_idx });
            }
            if eject_possess_requested {
                self.send_render(RenderControl::EjectPossess);
            }
            if let Some((seed, backend)) = run_smart_view {
                self.send_render(RenderControl::RunSmartView { seed, backend });
            }
            if let Some((seed, backend)) = run_smart_rail {
                self.send_render(RenderControl::RunSmartRail { seed, backend });
            }
            if reload_shader_requested {
                self.send_render(RenderControl::ReloadShader);
            }

            // 4. Advance demo animator, camera rail, gimbal damper & push updates/subframes.
            // In Smooth 60 FPS recording mode, step everything by exact `(1/60s) * time_scale` per
            // encoded frame (and across N 180-degree shutter subframes when enabled).
            let time_scale = self.settings.clamped_time_scale();
            let shutter_n = if self.is_recording && self.settings.record_fixed_fps {
                self.settings.shutter_subframes()
            } else {
                1
            };
            if rail_blend_requested {
                if self.possess_info.is_some() {
                    self.send_render(RenderControl::EjectPossess);
                }
                self.rail_blend_remaining = RAIL_BLEND_SECS;
            }
            if cam_snap_requested {
                self.rail_blend_remaining = 0.0;
                self.cam_damper.snap(self.cam_pose);
            }

            if reset_accum_requested {
                self.reset_seq = self.reset_seq.wrapping_add(1);
            }
            let possess_look = (
                self.possess_free_yaw.to_bits(),
                self.possess_free_pitch.to_bits(),
                self.possess_zoom_offset.to_bits(),
                self.possess_auto_level,
            );

            if bench_lockstep || (self.is_recording && self.settings.record_fixed_fps) {
                if !self.shutter_in_flight {
                    let frame_dt = time_scale / RECORD_FPS as f32;
                    let mut final_settings = self.last_sent_settings;
                    let mut final_cam = self.last_sent_camera;
                    for sub_idx in 0..shutter_n {
                        let sub_dt =
                            crate::camera_rail::shutter_subframe_dt(frame_dt, sub_idx, shutter_n);
                        self.demo_anim.tick(sub_dt);
                        if let Some(tr) = self.tod_transition.as_mut()
                            && tr.step(&mut self.settings, sub_dt)
                        {
                            self.tod_transition = None;
                        }
                        if self.camera_rail.is_active() {
                            self.camera_rail.tick(sub_dt);
                            if let Some(rp) = self
                                .camera_rail
                                .evaluate_current(self.cam_pose.znear, self.cam_pose.zfar.max(2000.0))
                            {
                                self.cam_pose = rp;
                                self.cam_driver.controller_mut().set_pose(rp);
                                if bench_lockstep {
                                    // The benchmark renders the rail pose itself, not a glide
                                    // from wherever the camera was when the scene loaded.
                                    self.cam_damper.snap(rp);
                                }
                            }
                        } else if self.possess_info.is_none() {
                            let raw = lockstep_subframe_raw_input(
                                self.last_cam_raw_input,
                                sub_dt,
                                self.render_w,
                                self.render_h,
                            );
                            let fly_scene_radius =
                                2.0 * crate::ui::altitude_speed_multiplier(self.cam_pose.eye.y);
                            let df = self.cam_driver.update(&raw, fly_scene_radius);
                            self.cam_pose = df.pose;
                        }
                        let effective_tau = if self.rail_blend_remaining > 0.0 {
                            let frac = (self.rail_blend_remaining / RAIL_BLEND_SECS).clamp(0.0, 1.0);
                            self.rail_blend_remaining = (self.rail_blend_remaining - sub_dt).max(0.0);
                            self.settings.cam_smooth_tau.max(0.32 * frac)
                        } else {
                            self.settings.cam_smooth_tau
                        };
                        let smoothed = self.cam_damper.step(
                            self.cam_pose,
                            sub_dt,
                            effective_tau,
                        );
                        let mut effective_settings = self.demo_anim.apply(&self.settings);
                        if !self.demo_anim.playing {
                            effective_settings.auto_rotate = false;
                            effective_settings.sat_orbit = false;
                        }
                        let cam_frame = pose_to_frame(smoothed);
                        final_settings = effective_settings;
                        final_cam = cam_frame;
                        self.send_render(RenderControl::RenderShutterSubframe {
                            settings: Box::new(effective_settings),
                            camera: cam_frame,
                            sim_dt: sub_dt,
                            sub_idx,
                            sub_count: shutter_n,
                            reset_seq: self.reset_seq,
                            possess_free_yaw: self.possess_free_yaw,
                            possess_free_pitch: self.possess_free_pitch,
                            possess_auto_level: self.possess_auto_level,
                            possess_zoom_offset: self.possess_zoom_offset,
                        });
                    }
                    self.interacting_until = None;
                    self.last_sent_settings = final_settings;
                    self.last_sent_camera = final_cam;
                    self.last_sent_possess_look = possess_look;
                    self.publish_state(RenderStateSnapshot {
                        settings: final_settings,
                        camera: final_cam,
                        interacting_until: None,
                        reset_seq: self.reset_seq,
                        possess_free_yaw: self.possess_free_yaw,
                        possess_free_pitch: self.possess_free_pitch,
                        possess_auto_level: self.possess_auto_level,
                        possess_zoom_offset: self.possess_zoom_offset,
                    });
                    self.shutter_in_flight = true;
                } else {
                    let mut effective_settings = self.demo_anim.apply(&self.settings);
                    if !self.demo_anim.playing {
                        effective_settings.auto_rotate = false;
                        effective_settings.sat_orbit = false;
                    }
                    if reset_accum_requested
                        || effective_settings != self.last_sent_settings
                        || possess_look != self.last_sent_possess_look
                    {
                        self.interacting_until = None;
                        self.last_sent_settings = effective_settings;
                        self.last_sent_possess_look = possess_look;
                        self.publish_state(RenderStateSnapshot {
                            settings: effective_settings,
                            camera: self.last_sent_camera,
                            interacting_until: None,
                            reset_seq: self.reset_seq,
                            possess_free_yaw: self.possess_free_yaw,
                            possess_free_pitch: self.possess_free_pitch,
                            possess_auto_level: self.possess_auto_level,
                            possess_zoom_offset: self.possess_zoom_offset,
                        });
                    }
                }
            } else {
                let (step_dt, damper_dt) =
                    live_step_dts(dt, time_scale, self.camera_rail.is_active());
                self.demo_anim.tick(step_dt);
                if let Some(tr) = self.tod_transition.as_mut()
                    && tr.step(&mut self.settings, step_dt)
                {
                    self.tod_transition = None;
                }
                if self.camera_rail.is_active() {
                    self.camera_rail.tick(step_dt);
                    if let Some(rp) = self
                        .camera_rail
                        .evaluate_current(self.cam_pose.znear, self.cam_pose.zfar.max(2000.0))
                    {
                        self.cam_pose = rp;
                        self.cam_driver.controller_mut().set_pose(rp);
                    }
                }
                let effective_tau = if self.rail_blend_remaining > 0.0 {
                    let frac = (self.rail_blend_remaining / RAIL_BLEND_SECS).clamp(0.0, 1.0);
                    self.rail_blend_remaining = (self.rail_blend_remaining - step_dt).max(0.0);
                    self.settings.cam_smooth_tau.max(0.32 * frac)
                } else {
                    self.settings.cam_smooth_tau
                };
                let render_pose = self.cam_damper.step(self.cam_pose, damper_dt, effective_tau);
                let mut effective_settings = self.demo_anim.apply(&self.settings);
                if !self.demo_anim.playing {
                    effective_settings.auto_rotate = false;
                    effective_settings.sat_orbit = false;
                }
                let cam_frame = pose_to_frame(render_pose);
                if reset_accum_requested
                    || effective_settings != self.last_sent_settings
                    || cam_frame != self.last_sent_camera
                    || possess_look != self.last_sent_possess_look
                {
                    let automated_motion = self.is_recording
                        || self.demo_anim.is_running()
                        || self.camera_rail.is_active()
                        || self.tod_transition.is_some();
                    self.interacting_until = if automated_motion {
                        None
                    } else {
                        Some(Instant::now() + INTERACTION_SETTLE_WINDOW)
                    };
                    self.last_sent_settings = effective_settings;
                    self.last_sent_camera = cam_frame;
                    self.last_sent_possess_look = possess_look;
                    self.publish_state(RenderStateSnapshot {
                        settings: effective_settings,
                        camera: cam_frame,
                        interacting_until: self.interacting_until,
                        reset_seq: self.reset_seq,
                        possess_free_yaw: self.possess_free_yaw,
                        possess_free_pitch: self.possess_free_pitch,
                        possess_auto_level: self.possess_auto_level,
                        possess_zoom_offset: self.possess_zoom_offset,
                    });
                }
            }

            // 5. wgpu present (copy latest SharedBuffer -> display_texture + render egui).
            // F02: a frame without a surface texture (resize, minimize, timeout) skips only the
            // surface draw and present. Texture deltas, platform output, cursor commands, window
            // captures and recorder pushes below still run, or glyph-atlas patches, the one-shot
            // cursor grab and Smooth 60 frames would be lost.
            let frame = match self.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(f)
                | wgpu::CurrentSurfaceTexture::Suboptimal(f) => Some(f),
                wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                    let s = self.window.inner_size();
                    self.resize(s.width, s.height);
                    None
                }
                wgpu::CurrentSurfaceTexture::Timeout
                | wgpu::CurrentSurfaceTexture::Occluded
                | wgpu::CurrentSurfaceTexture::Validation => None,
            };
            let view = frame.as_ref().map(|f| {
                f.texture
                    .create_view(&wgpu::TextureViewDescriptor::default())
            });

            self.egui_winit
                .handle_platform_output(&self.window, full_output.platform_output);
            if let Some(vo) = full_output.viewport_output.get(&egui::ViewportId::ROOT) {
                let mut want_grab: Option<egui::CursorGrab> = None;
                let mut want_visible: Option<bool> = None;
                for cmd in &vo.commands {
                    match cmd {
                        egui::ViewportCommand::CursorGrab(grab) => {
                            want_grab = Some(*grab);
                        }
                        egui::ViewportCommand::CursorVisible(visible) => {
                            want_visible = Some(*visible);
                        }
                        _ => {}
                    }
                }
                if want_grab.is_some() || want_visible.is_some() {
                    let act = self.cursor_grab.apply_egui_request(want_grab, want_visible);
                    self.apply_cursor_os_action(act);
                }
            }
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
                if self.settings.dlss_rr_on
                    && self.settings.launch_aov_mode() == 0
                    && let Some(ref mut rr) = self.dlss_rr_pass
                    && let Some(ref rr_bufs) = self.rr_shared
                {
                    let mut ran_rr = false;
                    let mut rr_failed = false;
                    if self.dlss_dirty {
                        let _nvtx_rr =
                            crate::nvtx::NvtxRange::new(c"DLSS-RR", crate::nvtx::COLOR_DLSS);
                        self.dlss_dirty = false;
                        let mut copy_enc = self.interop.device().create_command_encoder(
                            &wgpu::CommandEncoderDescriptor {
                                label: Some("dlss-rr-copy-inputs"),
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
                        let idx = self.front_buf_idx;
                        copy_buf(
                            &mut copy_enc,
                            rr_bufs.hdr_color[idx].wgpu_buffer(),
                            rr.color_in.wgpu_texture(),
                            8,
                        );
                        copy_buf(
                            &mut copy_enc,
                            self.shared_depth[idx].wgpu_buffer(),
                            rr.depth_in.wgpu_texture(),
                            4,
                        );
                        copy_buf(
                            &mut copy_enc,
                            self.shared_motion[idx].wgpu_buffer(),
                            rr.motion_in.wgpu_texture(),
                            8,
                        );
                        copy_buf(
                            &mut copy_enc,
                            rr_bufs.diffuse_albedo[idx].wgpu_buffer(),
                            rr.diffuse_in.wgpu_texture(),
                            8,
                        );
                        copy_buf(
                            &mut copy_enc,
                            rr_bufs.spec_albedo[idx].wgpu_buffer(),
                            rr.specular_in.wgpu_texture(),
                            8,
                        );
                        copy_buf(
                            &mut copy_enc,
                            rr_bufs.normal_rough[idx].wgpu_buffer(),
                            rr.normals_in.wgpu_texture(),
                            8,
                        );
                        // No host wait (F27): `submit_vulkan_commands` now records the NGX work
                        // into a wgpu encoder on the same queue, so it is submitted after these
                        // copies, and its `TransferDst -> ShaderRead` barriers order their writes.
                        self.interop.queue().submit(Some(copy_enc.finish()));
                        let aspect = self.render_w as f32 / self.render_h.max(1) as f32;
                        let rc = self.last_rendered_camera;
                        let rc_fov_y = (2.0 * rc.tan_half_fov.max(0.05).atan()).clamp(0.2, 2.2);
                        let world_to_view = glam::camera::rh::view::look_to_mat4(
                            Vec3::from_array(rc.eye),
                            Vec3::from_array(rc.fwd),
                            Vec3::from_array(rc.up),
                        )
                        .to_cols_array();
                        let view_to_clip = glam::camera::rh::proj::directx::perspective(
                            rc_fov_y,
                            aspect,
                            self.cam_pose.znear.max(1e-3),
                            self.cam_pose.zfar.max(10.0),
                        )
                        .to_cols_array();
                        let t_rr = Instant::now();
                        if let Err(e) = rr.evaluate(
                            &self.interop,
                            self.last_jitter,
                            world_to_view,
                            view_to_clip,
                        ) {
                            rr_failed = true;
                            self.dlss_ms = 0.0;
                            self.last_saved_msg = Some(format!("DLSS-RR evaluate failed: {e}"));
                        } else {
                            self.dlss_ms = t_rr.elapsed().as_secs_f32() * 1000.0;
                            ran_rr = true;
                        }
                    }
                    if rr_failed {
                        self.settings.dlss_rr_on = false;
                        if self.using_dlss_tex {
                            let base_v = if hdr_active {
                                self.display_view_hdr.as_ref().unwrap_or(&self.display_view)
                            } else {
                                &self.display_view
                            };
                            bind_display_view(
                                &mut self.egui_renderer,
                                self.egui_renderer_hdr.as_mut(),
                                self.interop.device(),
                                base_v,
                                self.display_tex_id,
                            );
                            self.using_dlss_tex = false;
                            self.bound_hdr_tex = hdr_active && self.display_view_hdr.is_some();
                            self.viewport_tex_dirty = self.has_frame;
                        }
                        self.dlss_rr_pass = None;
                    } else if should_dispatch_rr_tonemap(
                        ran_rr,
                        self.rr_last_exposure,
                        self.settings.exposure,
                    ) {
                        let rec_wants_pq10 = self.video_recorder.as_ref().is_some_and(|r| {
                            r.profile == crate::gpu_export::VideoProfile::Hdr10Pq
                        });
                        let rec_wants_sdr = self.video_recorder.as_ref().is_some_and(|r| {
                            r.profile == crate::gpu_export::VideoProfile::SdrH264
                        });
                        if !hdr_active || rec_wants_sdr {
                            self.hdr_tonemap.dispatch(
                                self.interop.queue(),
                                &mut encoder,
                                &rr.tonemap_bg,
                                (rr.out_w, rr.out_h),
                                self.settings.exposure,
                                false,
                            );
                        }
                        if hdr_active || rec_wants_pq10 {
                            self.hdr_tonemap.dispatch(
                                self.interop.queue(),
                                &mut encoder,
                                &rr.scrgb_bg,
                                (rr.out_w, rr.out_h),
                                self.settings.exposure,
                                true,
                            );
                        }
                        self.rr_last_exposure = Some(self.settings.exposure);
                    }
                } else if self.settings.dlss_on
                    && let Some(ref mut dlss) = self.dlss_pass
                {
                    if self.dlss_dirty {
                        let _nvtx_dlss =
                            crate::nvtx::NvtxRange::new(c"DLSS", crate::nvtx::COLOR_DLSS);
                        self.dlss_dirty = false;
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
                        // Same-queue ordering as for DLSS-RR above: no host wait needed (F27).
                        self.interop.queue().submit(Some(copy_enc.finish()));
                        let t_dlss = Instant::now();
                        if let Err(e) = dlss.evaluate(
                            &self.interop,
                            self.last_jitter,
                            self.settings.dlss_sharpness,
                        ) {
                            self.settings.dlss_on = false;
                            self.dlss_ms = 0.0;
                            if self.using_dlss_tex {
                                let base_v = if hdr_active {
                                    self.display_view_hdr.as_ref().unwrap_or(&self.display_view)
                                } else {
                                    &self.display_view
                                };
                                bind_display_view(
                                    &mut self.egui_renderer,
                                    self.egui_renderer_hdr.as_mut(),
                                    self.interop.device(),
                                    base_v,
                                    self.display_tex_id,
                                );
                                self.using_dlss_tex = false;
                                self.bound_hdr_tex = hdr_active && self.display_view_hdr.is_some();
                                self.viewport_tex_dirty = self.has_frame;
                            }
                            self.dlss_pass = None;
                            self.last_saved_msg = Some(format!("DLSS evaluate failed: {e}"));
                        } else {
                            self.dlss_ms = t_dlss.elapsed().as_secs_f32() * 1000.0;
                        }
                    }
                } else if self.viewport_tex_dirty {
                    self.viewport_tex_dirty = false;
                    if hdr_active
                        && let (Some(hdr_bufs), Some(hdr_tex)) =
                            (&self.shared_hdr, &self.display_texture_hdr)
                    {
                        encoder.copy_buffer_to_texture(
                            wgpu::TexelCopyBufferInfo {
                                buffer: hdr_bufs[self.front_buf_idx].wgpu_buffer(),
                                layout: wgpu::TexelCopyBufferLayout {
                                    offset: 0,
                                    bytes_per_row: Some(self.render_w * 8),
                                    rows_per_image: Some(self.render_h),
                                },
                            },
                            wgpu::TexelCopyTextureInfo {
                                texture: hdr_tex,
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
            }

            for (id, deltas) in &full_output.textures_delta.set {
                for delta in deltas {
                    self.egui_renderer.update_texture(
                        self.interop.device(),
                        self.interop.queue(),
                        *id,
                        delta,
                    );
                    if let Some(ref mut hdr_r) = self.egui_renderer_hdr {
                        hdr_r.update_texture(
                            self.interop.device(),
                            self.interop.queue(),
                            *id,
                            delta,
                        );
                    }
                }
            }
            let recording_full_win = self
                .video_recorder
                .as_ref()
                .is_some_and(|r| r.source == CaptureSource::FullWindow);
            let need_win_capture = should_capture_full_window(
                recording_full_win,
                frames_received,
                capture_window_png,
                capture_window_jpeg,
            );
            if need_win_capture {
                let win_w = self.surface_config.width;
                let win_h = self.surface_config.height;
                let win_fmt = self.surface_config.format;
                let needs_alloc = self
                    .window_capture
                    .as_ref()
                    .is_none_or(|c| !c.matches(win_w, win_h, win_fmt));
                if needs_alloc {
                    match FullWindowCapture::new(&self.interop, win_w, win_h, win_fmt) {
                        Ok(cap) => self.window_capture = Some(cap),
                        Err(e) => {
                            self.last_saved_msg = Some(format!("Window capture alloc failed: {e}"));
                            capture_window_png = false;
                            capture_window_jpeg = false;
                            if recording_full_win && let Some(active) = self.video_recorder.take() {
                                self.export_worker.finish_video(
                                    active.session_id,
                                    Some(format!("Window capture alloc failed: {e}")),
                                );
                                self.is_recording = false;
                                self.record_frames = 0;
                                self.notify_recording(false);
                            }
                        }
                    }
                }
            } else if !recording_full_win && self.settings.capture_source == CaptureSource::Viewport
            {
                self.window_capture = None;
            }

            let active_egui = if hdr_active {
                self.egui_renderer_hdr
                    .as_mut()
                    .unwrap_or(&mut self.egui_renderer)
            } else {
                &mut self.egui_renderer
            };
            active_egui.update_buffers(
                self.interop.device(),
                self.interop.queue(),
                &mut encoder,
                &tris,
                &screen_desc,
            );

            let mut win_gpu_copied = false;
            if need_win_capture
                && let Some(ref cap) = self.window_capture
                && cap.matches(
                    self.surface_config.width,
                    self.surface_config.height,
                    self.surface_config.format,
                )
            {
                {
                    let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("egui-full-window-capture-pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &cap.view,
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
                    active_egui.render(&mut pass, &tris, &screen_desc);
                }
                let extent = wgpu::Extent3d {
                    width: cap.width,
                    height: cap.height,
                    depth_or_array_layers: 1,
                };
                if self.surface_supports_copy_dst
                    && let Some(ref surface_frame) = frame
                {
                    encoder.copy_texture_to_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &cap.texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::TexelCopyTextureInfo {
                            texture: &surface_frame.texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        extent,
                    );
                } else if !self.surface_supports_copy_dst
                    && let Some(ref surface_view) = view
                {
                    let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("egui-pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: surface_view,
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
                    active_egui.render(&mut pass, &tris, &screen_desc);
                }
                encoder.copy_texture_to_buffer(
                    wgpu::TexelCopyTextureInfo {
                        texture: &cap.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyBufferInfo {
                        buffer: cap.copy_buf.wgpu_buffer(),
                        layout: wgpu::TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(cap.copy_bpr),
                            rows_per_image: Some(cap.height),
                        },
                    },
                    extent,
                );
                win_gpu_copied = true;
            } else if let Some(ref surface_view) = view {
                let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("egui-pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: surface_view,
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
                active_egui.render(&mut pass, &tris, &screen_desc);
            }
            for id in &full_output.textures_delta.free {
                self.egui_renderer.free_texture(id);
                if let Some(ref mut hdr_r) = self.egui_renderer_hdr {
                    hdr_r.free_texture(id);
                }
            }

            // Capture the final post-DLSS-RR / post-DLSS-SR / post-OptiX viewport into MP4 when recording Viewport.
            let mut rec_gpu_copied = false;
            if frames_received > 0
                && self.has_frame
                && let Some(ref mut active) = self.video_recorder
                && active.source == CaptureSource::Viewport
            {
                let is_pq = active.profile == crate::gpu_export::VideoProfile::Hdr10Pq;
                let src_tex: Option<(&wgpu::Texture, u32, u32, u32)> =
                    if self.settings.dlss_rr_on
                        && self.settings.launch_aov_mode() == 0
                        && let Some(ref rr) = self.dlss_rr_pass
                    {
                        let tex = if is_pq {
                            &rr.display_hdr
                        } else {
                            &rr.display_ldr
                        };
                        Some((tex, rr.out_w & !1, rr.out_h & !1, if is_pq { 8 } else { 4 }))
                    } else if !is_pq
                        && self.settings.dlss_on
                        && let Some(ref dlss) = self.dlss_pass
                    {
                        Some((
                            dlss.output_out.wgpu_texture(),
                            dlss.out_w & !1,
                            dlss.out_h & !1,
                            4,
                        ))
                    } else {
                        None
                    };
                if let (Some((tex, w, h, bpp)), Some(copy_buf)) = (src_tex, &active.copy_buf)
                    && w == active.width
                    && h == active.height
                {
                    encoder.copy_texture_to_buffer(
                        wgpu::TexelCopyTextureInfo {
                            texture: tex,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::TexelCopyBufferInfo {
                            buffer: copy_buf.wgpu_buffer(),
                            layout: wgpu::TexelCopyBufferLayout {
                                offset: 0,
                                bytes_per_row: Some(active.copy_pitch_px * bpp),
                                rows_per_image: Some(h),
                            },
                        },
                        wgpu::Extent3d {
                            width: w,
                            height: h,
                            depth_or_array_layers: 1,
                        },
                    );
                    rec_gpu_copied = true;
                }
            }

            let frame_submission = self.interop.queue().submit(Some(encoder.finish()));
            // F27: the copies below are read by CUDA, so wait for this submission only, bounded:
            // a GPU hang drops the capture with a message instead of freezing the UI thread.
            let frame_copy_wait = wgpu::PollType::Wait {
                submission_index: Some(frame_submission),
                timeout: Some(GPU_COPY_WAIT_TIMEOUT),
            };

            if win_gpu_copied && let Some(ref cap) = self.window_capture {
                let poll_err = self
                    .interop
                    .device()
                    .poll(frame_copy_wait.clone())
                    .err();
                if let Some(e) = poll_err {
                    eprintln!("nv-demo wgpu poll (window capture) failed: {e}");
                    self.last_saved_msg = Some(format!("Window capture GPU poll failed: {e}"));
                } else {
                    let ptr = cap.copy_buf.cu_deviceptr();
                    let cap_w = cap.width;
                    let cap_h = cap.height;
                    let cap_pitch_px = cap.copy_pitch_px;
                    let cap_fmt = cap.pixel_format;

                    if capture_window_png {
                        let path =
                            PathBuf::from(RENDERS_DIR).join(output_file_name("nv-demo", "-win.png"));
                        let byte_len = (cap_pitch_px as usize)
                            .saturating_mul(cap_h as usize)
                            .saturating_mul(cap_fmt.bytes_per_pixel() as usize);
                        self.last_saved_msg = Some(
                            match readback_device_bytes(self.interop.cuda_context(), ptr, byte_len)
                                .and_then(|raw| {
                                    self.export_worker.enqueue_png_pitched(
                                        raw,
                                        cap_w,
                                        cap_h,
                                        cap_pitch_px,
                                        cap_fmt,
                                        path.clone(),
                                    )
                                }) {
                                Ok(()) => format!("Saving {}...", path.display()),
                                Err(e) => format!("Window PNG failed: {e}"),
                            },
                        );
                    }

                    if capture_window_jpeg {
                        let path =
                            PathBuf::from(RENDERS_DIR).join(output_file_name("nv-demo", "-win.jpg"));
                        self.last_saved_msg = Some(
                            match self.export_worker.enqueue_jpeg_device(
                                PitchedDeviceImage {
                                    ptr,
                                    width: cap_w,
                                    height: cap_h,
                                    pitch_px: cap_pitch_px,
                                    fmt: cap_fmt,
                                },
                                path.clone(),
                                92,
                            ) {
                                Ok(()) => format!("Saving {}...", path.display()),
                                Err(e) => format!("Window JPEG failed: {e}"),
                            },
                        );
                    }

                    if recording_full_win
                        && frames_received > 0
                        && let Some(mut active) = self.video_recorder.take()
                    {
                        let dims_ok = crate::gpu_export::align_even_dims(cap_w, cap_h)
                            == Some((active.width, active.height))
                            && cap_fmt == active.pixel_format
                            && ptr != 0;
                        if !dims_ok {
                            let reason =
                                "Recording stopped (window size or format changed)".to_string();
                            self.export_worker
                                .finish_video(active.session_id, Some(reason.clone()));
                            self.is_recording = false;
                            self.record_frames = 0;
                            self.notify_recording(false);
                            self.last_saved_msg = Some(reason);
                        } else {
                            let rec_now = Instant::now();
                            let frame_dur = video_frame_duration(
                                self.settings.record_fixed_fps,
                                active.fps,
                                active.last_frame_at,
                                rec_now,
                            );
                            match self.export_worker.push_video_frame(
                                active.session_id,
                                PitchedDeviceImage {
                                    ptr,
                                    width: active.width,
                                    height: active.height,
                                    pitch_px: cap_pitch_px,
                                    fmt: active.pixel_format,
                                },
                                frame_dur,
                                !self.settings.record_fixed_fps,
                            ) {
                                Ok(accepted) => {
                                    // F04: a dropped frame's interval goes to the next frame.
                                    if accepted {
                                        active.last_frame_at = Some(rec_now);
                                    }
                                    self.record_frames = self.export_worker.recorded_frames();
                                    self.video_recorder = Some(active);
                                }
                                Err(e) => {
                                    let reason = format!("Video encode error: {e}");
                                    self.export_worker
                                        .finish_video(active.session_id, Some(reason.clone()));
                                    self.is_recording = false;
                                    self.record_frames = 0;
                                    self.notify_recording(false);
                                    self.last_saved_msg = Some(reason);
                                }
                            }
                        }
                    }
                }
            }

            if !recording_full_win
                && frames_received > 0
                && self.has_frame
                && let Some(mut active) = self.video_recorder.take()
            {
                let is_pq = active.profile == crate::gpu_export::VideoProfile::Hdr10Pq;
                let mut waiting_for_hdr = false;
                let mut stop_reason: Option<String> = None;
                let (src_ptr, src_pitch, dims_ok) = if rec_gpu_copied {
                    if let Err(e) = self.interop.device().poll(frame_copy_wait) {
                        stop_reason = Some(format!("Video GPU poll failed: {e}"));
                        (0, 0, false)
                    } else {
                        let ptr = active
                            .copy_buf
                            .as_ref()
                            .map_or(0, SharedBuffer::cu_deviceptr);
                        (ptr, active.copy_pitch_px, ptr != 0)
                    }
                } else if active.copy_buf.is_none()
                    && (self.render_w & !1) == active.width
                    && (self.render_h & !1) == active.height
                {
                    if is_pq {
                        if let Some(ref hdr_bufs) = self.shared_hdr {
                            active.hdr_wait_frames = 0;
                            (hdr_bufs[self.front_buf_idx].cu_deviceptr(), self.render_w, true)
                        } else if active.hdr_wait_frames < MAX_PQ10_HDR_WAIT_FRAMES {
                            // Waiting briefly for async `shared_hdr` resize allocation after start.
                            active.hdr_wait_frames += 1;
                            waiting_for_hdr = true;
                            (0, 0, false)
                        } else {
                            stop_reason = Some(
                                "Recording stopped (HDR buffer unavailable for PQ10)".to_string(),
                            );
                            (0, 0, false)
                        }
                    } else {
                        (
                            self.shared_rgba[self.front_buf_idx].cu_deviceptr(),
                            self.render_w,
                            true,
                        )
                    }
                } else {
                    (0, 0, false)
                };

                if waiting_for_hdr {
                    self.video_recorder = Some(active);
                } else if !dims_ok {
                    let reason = stop_reason.unwrap_or_else(|| {
                        "Recording stopped (resolution or upscale mode changed)".to_string()
                    });
                    self.export_worker
                        .finish_video(active.session_id, Some(reason.clone()));
                    self.is_recording = false;
                    self.record_frames = 0;
                    self.notify_recording(false);
                    self.last_saved_msg = Some(reason);
                } else {
                    let rec_now = Instant::now();
                    let frame_dur = video_frame_duration(
                        self.settings.record_fixed_fps,
                        active.fps,
                        active.last_frame_at,
                        rec_now,
                    );
                    match self.export_worker.push_video_frame(
                        active.session_id,
                        PitchedDeviceImage {
                            ptr: src_ptr,
                            width: active.width,
                            height: active.height,
                            pitch_px: src_pitch,
                            fmt: active.pixel_format,
                        },
                        frame_dur,
                        !self.settings.record_fixed_fps,
                    ) {
                        Ok(accepted) => {
                            // F04: a dropped frame's interval goes to the next frame.
                            if accepted {
                                active.last_frame_at = Some(rec_now);
                            }
                            self.record_frames = self.export_worker.recorded_frames();
                            self.video_recorder = Some(active);
                        }
                        Err(e) => {
                            let reason = format!("Video encode error: {e}");
                            self.export_worker
                                .finish_video(active.session_id, Some(reason.clone()));
                            self.is_recording = false;
                            self.record_frames = 0;
                            self.notify_recording(false);
                            self.last_saved_msg = Some(reason);
                        }
                    }
                }
            }

            if let Some(frame) = frame {
                self.interop.queue().present(frame);
            }
            if self.last_persist_at.elapsed().as_secs_f32() >= 1.5 {
                self.update_window_geometry();
                self.save_persisted();
            }
        }

        pub fn request_redraw(&self) {
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

    fn allocate_hdr_target(
        interop: &InteropContext,
        width: u32,
        height: u32,
    ) -> Result<([SharedBuffer; 2], wgpu::Texture), nv_wgpu::Error> {
        let bytes = u64::from(width) * u64::from(height) * 8; // Rgba16Float scRGB
        let make = |label| {
            interop.create_buffer(&SharedBufferDescriptor {
                label: Some(label),
                size: bytes,
                usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
            })
        };
        let bufs = [make("optix-scrgb-0")?, make("optix-scrgb-1")?];
        let tex = interop.device().create_texture(&wgpu::TextureDescriptor {
            label: Some("optix-viewport-hdr"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        Ok((bufs, tex))
    }

    fn bind_display_view(
        egui_sdr: &mut egui_wgpu::Renderer,
        egui_hdr: Option<&mut egui_wgpu::Renderer>,
        device: &wgpu::Device,
        view: &wgpu::TextureView,
        tex_id: egui::TextureId,
    ) {
        egui_sdr.update_egui_texture_from_wgpu_texture(
            device,
            view,
            wgpu::FilterMode::Linear,
            tex_id,
        );
        if let Some(hdr_r) = egui_hdr {
            hdr_r.update_egui_texture_from_wgpu_texture(
                device,
                view,
                wgpu::FilterMode::Linear,
                tex_id,
            );
        }
    }

    fn readback_device_bytes(
        cuda: &Arc<cuda_core::Context>,
        dev_ptr: u64,
        byte_len: usize,
    ) -> Result<Vec<u8>, String> {
        if dev_ptr == 0 || byte_len == 0 {
            return Err("CUDA readback source pointer or length is empty".to_string());
        }
        cuda.bind_to_thread()
            .map_err(|e| format!("CUDA bind failed: {e}"))?;
        let mut host = vec![0u8; byte_len];
        // SAFETY: `dev_ptr` points to a valid `SharedBuffer` of at least `byte_len` bytes on `cuda`.
        let res =
            unsafe { cuda_bindings::cuMemcpyDtoH_v2(host.as_mut_ptr().cast(), dev_ptr, byte_len) };
        if res != cuda_bindings::cudaError_enum_CUDA_SUCCESS {
            return Err(format!("CUDA readback failed ({res:?})"));
        }
        Ok(host)
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
        use camera::input::{Button, DragBinding, DragRole, Mods};
        let bindings = driver.bindings_mut();
        bindings.fly_wheel_speed = true;
        let drags = &mut bindings.drags;
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

/// Target UI redraw cadence (`60 Hz` = `16.667 ms`) preventing 250+ FPS GPU/CPU spin-burn.
const UI_FRAME_INTERVAL: Duration = Duration::from_micros(16_667);

/// Computes the next 60 Hz UI redraw deadline without spiral-of-death catch-up bursts.
fn compute_next_ui_tick(now: Instant, prev_deadline: Instant, interval: Duration) -> Instant {
    let stepped = prev_deadline + interval;
    if stepped <= now || prev_deadline > now {
        now + interval
    } else {
        stepped
    }
}

#[derive(Default)]
struct Runner {
    app: Option<DemoApp>,
    next_redraw_at: Option<Instant>,
    /// `--bench-rail` / `--bench-views` arguments, handed to the app when it is created.
    bench: Option<bench::BenchArgs>,
    /// `--is-counters`: start with the intersection counters on.
    is_counters: bool,
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app.is_none() {
            let loaded = crate::persist::load();
            let win_cfg = loaded.state.window.sanitized();
            let mut attrs = Window::default_attributes()
                .with_title("nv-rs RTX Fractal Path Tracer (OptiX 9 + AI Denoiser + nv-wgpu)")
                .with_inner_size(winit::dpi::PhysicalSize::new(win_cfg.width, win_cfg.height))
                .with_maximized(win_cfg.maximized);
            if let (Some(x), Some(y)) = (win_cfg.x, win_cfg.y) {
                attrs = attrs.with_position(winit::dpi::PhysicalPosition::new(x, y));
            }
            let win = Arc::new(event_loop.create_window(attrs).expect("window"));
            self.app = Some(DemoApp::new(win, loaded, self.bench.take(), self.is_counters));
            let now = Instant::now();
            self.next_redraw_at = Some(now);
            event_loop.set_control_flow(ControlFlow::WaitUntil(now));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(app) = self.app.as_mut() else {
            return;
        };
        if matches!(event, WindowEvent::CloseRequested) {
            app.update_window_geometry();
            app.flush_persisted_on_exit();
            event_loop.exit();
            return;
        }
        if let WindowEvent::Resized(s) = event {
            app.resize(s.width, s.height);
            app.update_window_geometry();
        } else if matches!(event, WindowEvent::Moved(_)) {
            app.update_window_geometry();
        } else if matches!(event, WindowEvent::RedrawRequested) {
            let now = Instant::now();
            app.render();
            let prev = self.next_redraw_at.unwrap_or(now);
            self.next_redraw_at = Some(compute_next_ui_tick(now, prev, UI_FRAME_INTERVAL));
        }
        app.handle_window_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Some(app) = self.app.as_ref() else {
            return;
        };
        if app.bench_outcome().is_some() {
            event_loop.exit();
            return;
        }
        let now = Instant::now();
        let deadline = self.next_redraw_at.unwrap_or(now);
        if now >= deadline {
            app.request_redraw();
            event_loop.set_control_flow(ControlFlow::WaitUntil(now + UI_FRAME_INTERVAL));
        } else {
            event_loop.set_control_flow(ControlFlow::WaitUntil(deadline));
        }
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

/// One stderr line per native teardown failure recorded by `Drop` impls (`cuda_core_ext::teardown`)
/// plus one line for failures that overflowed the sink.
fn teardown_report(drained: &cuda_core_ext::teardown::Drained) -> Vec<String> {
    let mut lines: Vec<String> = drained.errors.iter().map(|e| format!("nv-demo: {e}")).collect();
    if drained.lost > 0 {
        lines.push(format!(
            "nv-demo: {} further teardown failure(s) were dropped (sink full)",
            drained.lost
        ));
    }
    lines
}

fn main() -> std::process::ExitCode {
    nvtx::name_current_thread(c"nv-demo-main");
    let cli = match bench::parse_args(std::env::args_os().skip(1)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("nv-demo: {e}\n{}", bench::USAGE);
            return std::process::ExitCode::FAILURE;
        }
    };
    let result = EventLoop::new()
        .map_err(|e| format!("event loop creation failed: {e}"))
        .and_then(|event_loop| {
            let mut runner = Runner {
                bench: cli.bench,
                is_counters: cli.is_counters,
                ..Runner::default()
            };
            event_loop
                .run_app(&mut runner)
                .map_err(|e| format!("event loop failed: {e}"))?;
            // A benchmark ends the event loop itself; its outcome is the process result.
            match runner.app.as_ref().and_then(DemoApp::bench_outcome) {
                Some(Ok(path)) => {
                    println!("nv-demo bench: wrote {}", path.display());
                    Ok(())
                }
                Some(Err(e)) => Err(format!("bench failed: {e}")),
                None => Ok(()),
            }
            // `runner` (and with it the app and every native handle) drops here, before the report.
        });
    for line in teardown_report(&cuda_core_ext::teardown::take(None)) {
        eprintln!("{line}");
    }
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("nv-demo: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod ui_pacing_tests {
    use super::*;

    #[test]
    fn teardown_report_lists_each_failure_and_the_overflow_count() {
        use cuda_core_ext::teardown::{Drained, Library, TeardownError};
        let drained = Drained {
            errors: vec![TeardownError {
                library: Library::Cublas,
                op: "cublasDestroy_v2",
                status: 14,
            }],
            lost: 3,
        };
        let lines = teardown_report(&drained);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("cublasDestroy_v2") && lines[0].contains("14"));
        assert!(lines[1].contains('3'));
        assert!(teardown_report(&Drained::default()).is_empty());
    }

    #[test]
    fn compute_next_ui_tick_locks_60hz_and_avoids_catchup_bursts() {
        let t0 = Instant::now();
        let interval = UI_FRAME_INTERVAL;
        // Normal on-time frame advances by one exact interval from previous deadline.
        let on_time = t0 + Duration::from_millis(2);
        assert_eq!(compute_next_ui_tick(on_time, t0, interval), t0 + interval);
        // Slow frame (> 1 interval behind) resets smoothly to now + interval (no burst).
        let late = t0 + Duration::from_millis(40);
        assert_eq!(compute_next_ui_tick(late, t0, interval), late + interval);
        // Early OS-triggered redraw schedules one full interval after now.
        let future_deadline = t0 + Duration::from_millis(10);
        assert_eq!(
            compute_next_ui_tick(t0, future_deadline, interval),
            t0 + interval
        );
    }

    #[test]
    fn lidar_360_mount_auto_switches_and_restores_lens_projection() {
        use crate::renderer::CameraProjection;

        let mut proj = CameraProjection::Pinhole;
        let mut saved = None;
        let lidar = csg_scene::CameraMount::new(
            "roof_lidar",
            "360° Roof LiDAR",
            csg_core::math::Vec3::new(0.0, 2.0, 0.0),
            csg_core::math::Vec3::Z,
            60.0,
        );
        let chase = csg_scene::CameraMount::new(
            "chase_cam",
            "Chase Cam",
            csg_core::math::Vec3::new(0.0, 2.5, -6.0),
            csg_core::math::Vec3::Z,
            52.0,
        );

        demo_app::sync_mount_camera_projection(Some(&lidar), &mut proj, &mut saved);
        assert_eq!(proj, CameraProjection::Equirect360);
        assert_eq!(saved, Some(CameraProjection::Pinhole));

        // Switching to a regular mount restores the saved projection.
        demo_app::sync_mount_camera_projection(Some(&chase), &mut proj, &mut saved);
        assert_eq!(proj, CameraProjection::Pinhole);
        assert_eq!(saved, None);

        // Ejecting from LiDAR also restores the saved projection.
        demo_app::sync_mount_camera_projection(Some(&lidar), &mut proj, &mut saved);
        assert_eq!(proj, CameraProjection::Equirect360);
        demo_app::sync_mount_camera_projection(None, &mut proj, &mut saved);
        assert_eq!(proj, CameraProjection::Pinhole);
    }

    #[test]
    fn tod_transition_smoothly_interpolates_from_day_to_night() {
        let mut s = crate::renderer::DemoSettings::default();
        crate::ui::apply_csg_time_of_day_preset(&mut s, 0); // Day
        let day_elev = s.sun_elevation;
        let day_sun = s.sun_intensity;

        let mut tr = demo_app::TodTransition::new(&s, 2, 2.0); // -> Night in 2.0 s
        assert!(!tr.step(&mut s, 0.5));
        assert!(s.sun_elevation < day_elev && s.sun_elevation > 24.0);
        assert!(s.sun_intensity < day_sun && s.sun_intensity > 0.16);
        assert_eq!(s.csg_time_of_day, 0);

        assert!(!tr.step(&mut s, 0.5)); // t = 0.5
        assert_eq!(s.csg_time_of_day, 2);

        assert!(tr.step(&mut s, 1.0)); // t = 1.0
        assert!((s.sun_elevation - 24.0).abs() < 1e-5);
        assert!((s.sun_intensity - 0.16).abs() < 1e-5);
        assert_eq!(s.sky_mode, 8);
    }

    #[test]
    fn cursor_grab_controller_preserves_clicks_and_restores_pre_grab_pos_on_drag_end() {
        let mut ctl = demo_app::CursorGrabController::default();
        let start = winit::dpi::PhysicalPosition::new(320.0, 210.0);
        ctl.on_cursor_moved(start);

        // 1. Single click (no drag >= 4 px) must NOT lock or hide the cursor.
        ctl.on_mouse_button(true);
        let a_click = ctl.apply_egui_request(Some(egui::CursorGrab::Locked), Some(false));
        assert_eq!(a_click.grab, None);
        assert_eq!(a_click.visible, None);
        let a_up = ctl.on_mouse_button(false);
        assert_eq!(a_up.grab, None);
        assert_eq!(a_up.restore_pos, None);

        // 2. Real drag (>= 4 px) hides & locks cursor, ignores center-warp CursorMoved, and
        //    restores the exact pre-grab position when released.
        ctl.on_mouse_button(true);
        ctl.on_mouse_motion((6.0, 0.0));
        let a_lock = ctl.apply_egui_request(Some(egui::CursorGrab::Locked), Some(false));
        assert_eq!(a_lock.grab, Some(winit::window::CursorGrabMode::Locked));
        assert_eq!(a_lock.visible, Some(false));

        // Synthetic center-warp during lock must NOT overwrite pre_grab_pos.
        ctl.on_cursor_moved(winit::dpi::PhysicalPosition::new(960.0, 540.0));

        let a_rel = ctl.on_mouse_button(false);
        assert_eq!(a_rel.grab, Some(winit::window::CursorGrabMode::None));
        assert_eq!(a_rel.visible, Some(true));
        assert_eq!(a_rel.restore_pos, Some(start));
    }

    #[test]
    fn cursor_grab_controller_re_emits_deferred_lock_and_supports_persistent_fly_mode() {
        let mut ctl = demo_app::CursorGrabController::default();
        let start = winit::dpi::PhysicalPosition::new(400.0, 300.0);
        assert_eq!(ctl.on_cursor_moved(start).grab, None);

        // 1. Deferred one-shot lock: egui requests Locked on frame 0 before drag >= 4 px.
        //    When motion crosses 4 px on frame 1 (without any second egui request), lock engages.
        ctl.on_mouse_button(true);
        let a_frame0 = ctl.apply_egui_request(Some(egui::CursorGrab::Locked), Some(false));
        assert_eq!(a_frame0.grab, None);
        assert_eq!(ctl.on_mouse_motion((2.0, 0.0)).grab, None);
        let a_frame1 = ctl.on_mouse_motion((3.0, 0.0));
        assert_eq!(a_frame1.grab, Some(winit::window::CursorGrabMode::Locked));
        assert_eq!(a_frame1.visible, Some(false));

        let a_rel = ctl.on_mouse_button(false);
        assert_eq!(a_rel.grab, Some(winit::window::CursorGrabMode::None));
        assert_eq!(a_rel.visible, Some(true));
        assert_eq!(a_rel.restore_pos, Some(start));

        // 2. Persistent fly mode (`buttons_held == 0`): lock engages immediately without a button.
        let a_fly = ctl.apply_egui_request(Some(egui::CursorGrab::Locked), Some(false));
        assert_eq!(a_fly.grab, Some(winit::window::CursorGrabMode::Locked));
        assert_eq!(a_fly.visible, Some(false));

        // Clicking and releasing while in persistent fly must NOT drop the persistent fly lock.
        assert_eq!(ctl.on_mouse_button(true).grab, None);
        assert_eq!(ctl.on_mouse_button(false).grab, None);

        // Explicit release (Escape) or focus loss restores cursor cleanly.
        let a_esc = ctl.apply_egui_request(Some(egui::CursorGrab::None), Some(true));
        assert_eq!(a_esc.grab, Some(winit::window::CursorGrabMode::None));
        assert_eq!(a_esc.visible, Some(true));
        assert_eq!(a_esc.restore_pos, Some(start));
    }

    #[test]
    fn rr_tonemap_exposure_sentinel_and_pq10_aov_hdr_buffer_rules() {
        // Initial frame (`None`) and exposure change must dispatch even when `ran_rr == false`.
        assert!(demo_app::should_dispatch_rr_tonemap(false, None, 1.0));
        assert!(demo_app::should_dispatch_rr_tonemap(false, Some(f32::NAN), 1.0));
        assert!(demo_app::should_dispatch_rr_tonemap(false, Some(1.0), 1.25));
        assert!(!demo_app::should_dispatch_rr_tonemap(false, Some(1.0), 1.0));
        assert!(demo_app::should_dispatch_rr_tonemap(true, Some(1.0), 1.0));

        // PQ10 viewport recording under `dlss_rr_on && aov_mode != 0 && !hdr_active` MUST request
        // `shared_hdr` because DLSS-RR does not produce AOV output.
        assert!(demo_app::need_shared_hdr_buffer(false, true, true, 2, true));
        // Beauty DLSS-RR with an active pass renders PQ10 from `rr.display_hdr` directly.
        assert!(!demo_app::need_shared_hdr_buffer(false, true, true, 0, true));
        // Missing RR pass falls back to `shared_hdr`.
        assert!(demo_app::need_shared_hdr_buffer(false, true, true, 0, false));
        // HDR display always needs `shared_hdr`.
        assert!(demo_app::need_shared_hdr_buffer(true, false, true, 0, true));
    }

    #[test]
    fn lockstep_subframe_input_preserves_held_fly_keys_and_clears_one_shot_deltas() {
        use camera::input::{Button, ButtonSet, Key, KeySet, RawInput, Wheel};

        let base = RawInput {
            pointer: Some(glam::Vec2::new(640.0, 360.0)),
            pointer_delta: glam::Vec2::new(15.0, -4.0),
            buttons_down: ButtonSet::EMPTY.with(Button::Secondary),
            buttons_pressed: ButtonSet::EMPTY.with(Button::Secondary),
            buttons_released: ButtonSet::EMPTY,
            keys_down: KeySet::EMPTY.with(Key::W),
            keys_pressed: KeySet::EMPTY.with(Key::W),
            wheel: Wheel {
                lines: glam::Vec2::new(0.0, 2.0),
                points: glam::Vec2::ZERO,
            },
            pinch: 1.25,
            hovered: true,
            keys_allowed: true,
            dt: 0.0,
            viewport: glam::Vec2::new(1280.0, 720.0),
            ..RawInput::default()
        };

        let sub = demo_app::lockstep_subframe_raw_input(Some(base), 1.0 / 240.0, 1280, 720);
        assert!(sub.buttons_down.contains(Button::Secondary));
        assert!(sub.keys_down.contains(Key::W));
        assert!(sub.keys_allowed);
        assert!(sub.hovered);
        assert_eq!(sub.pointer_delta, glam::Vec2::ZERO);
        assert!(sub.buttons_pressed.is_empty());
        assert!(sub.keys_pressed.is_empty());
        assert_eq!(sub.wheel.lines, glam::Vec2::ZERO);
        assert_eq!(sub.pinch, 1.0);
        assert!((sub.dt - 1.0 / 240.0).abs() < 1e-6);
    }

    /// F03: names created within the same millisecond differ.
    #[test]
    fn output_names_are_unique_within_one_millisecond() {
        let a = demo_app::output_file_name("nv-demo", ".png");
        let b = demo_app::output_file_name("nv-demo", ".png");
        assert_ne!(a, b);
        assert!(a.starts_with("nv-demo-") && a.ends_with(".png"));
    }

    /// F11: a full pool of stale-size buffers converges to the new size instead of staying full.
    #[test]
    fn export_pool_evicts_buffers_of_another_size() {
        let size = |b: &usize| *b;
        let mut pool: Vec<usize> = vec![10, 10, 10, 10];
        for _ in 0..4 {
            let dropped = demo_app::recycle_into_pool(&mut pool, 20, size, 4);
            assert_eq!(dropped, Some(10));
        }
        assert_eq!(pool, vec![20, 20, 20, 20]);
        // Same size when full: the incoming buffer is the one dropped.
        assert_eq!(demo_app::recycle_into_pool(&mut pool, 20, size, 4), Some(20));
    }

    /// F04: in Live RT a dropped frame's interval is carried by the next accepted frame, so the
    /// accepted durations add up to wall time.
    #[test]
    fn live_video_duration_carries_dropped_frame_interval() {
        let t0 = Instant::now();
        let t1 = t0 + Duration::from_millis(16);
        let t2 = t0 + Duration::from_millis(33);
        let last_accepted = Some(t0);
        // The frame at t1 is dropped (back-pressure), so `last_accepted` stays at t0.
        let d1 = demo_app::video_frame_duration(false, 60, last_accepted, t1);
        assert!((d1 - 0.016).abs() < 1e-9);
        let d2 = demo_app::video_frame_duration(false, 60, last_accepted, t2);
        assert!((d2 - 0.033).abs() < 1e-9);
        // Smooth 60 always uses the nominal duration.
        assert!((demo_app::video_frame_duration(true, 60, None, t2) - 1.0 / 60.0).abs() < 1e-12);
    }

    /// F09: the DLSS render target keeps the output aspect (1920x1080 Balanced used to give
    /// 1088x626, a 2.3 % horizontal stretch).
    #[test]
    fn dlss_render_dims_keep_the_output_aspect() {
        let opt = ngx::DlssOptimalSettings {
            render_width: 1114,
            render_height: 626,
            min_width: 1114,
            min_height: 626,
            max_width: 1114,
            max_height: 626,
            sharpness: 0.0,
        };
        let (w, h) = demo_app::dlss_render_dims(&opt, 1920, 1080);
        assert_eq!(w % 64, 0);
        assert_eq!(h % 2, 0);
        let err = ((w as f32 / h as f32) / (1920.0 / 1080.0) - 1.0).abs();
        assert!(err < 0.005, "{w}x{h} aspect error {err}");

        let opt4k = ngx::DlssOptimalSettings {
            render_width: 2560,
            render_height: 1440,
            min_width: 1280,
            min_height: 720,
            max_width: 3840,
            max_height: 2160,
            sharpness: 0.0,
        };
        assert_eq!(demo_app::dlss_render_dims(&opt4k, 3840, 2160), (2560, 1440));
    }

    /// F01/F10: stills come from the texture the viewport draws.
    #[test]
    fn viewport_still_source_follows_the_displayed_texture() {
        use demo_app::{ViewportStillSource as S, viewport_still_source as src};
        // (dlss_rr_on, aov, has_rr, dlss_on, has_dlss, hdr)
        assert_eq!(src(true, 0, true, false, false, false), S::DlssRrLdr);
        assert_eq!(src(true, 0, true, false, false, true), S::DlssRrHdr);
        assert_eq!(src(true, 3, true, false, false, false), S::RenderTarget);
        assert_eq!(src(true, 0, false, false, false, false), S::RenderTarget);
        assert_eq!(src(false, 0, false, true, true, false), S::DlssSr);
        assert_eq!(src(false, 0, false, true, false, false), S::RenderTarget);
        assert_eq!(src(false, 0, false, false, false, true), S::RenderTarget);
    }

    /// F05: editing a lighting field during a transition cancels it and finishes the crossfade;
    /// an untouched frame keeps it running.
    #[test]
    fn user_lighting_edit_cancels_tod_transition() {
        let mut s = crate::renderer::DemoSettings::default();
        crate::ui::apply_csg_time_of_day_preset(&mut s, 0);
        let mut tr = Some(demo_app::TodTransition::new(&s, 2, 4.0));
        if let Some(t) = tr.as_mut() {
            assert!(!t.step(&mut s, 1.0));
        }
        let before = demo_app::LightingKey::of(&s);
        assert!(!demo_app::cancel_tod_on_user_edit(before, &mut s, &mut tr));
        assert!(tr.is_some());

        s.exposure = 1.7; // user drags Exposure / toolbar EV
        assert!(demo_app::cancel_tod_on_user_edit(before, &mut s, &mut tr));
        assert!(tr.is_none());
        assert_eq!(s.sky_blend, 0.0);
        assert_eq!(s.sky_mode_from, s.sky_mode);
        assert_eq!(s.exposure, 1.7);
    }

    /// F15: a transition restarted mid-crossfade starts from the sky that dominates the blend.
    #[test]
    fn tod_restart_starts_from_the_dominant_sky() {
        let mut s = crate::renderer::DemoSettings::default();
        crate::ui::apply_csg_time_of_day_preset(&mut s, 0); // Day, sky 0
        let mut tr = demo_app::TodTransition::new(&s, 2, 10.0); // -> Night, sky 8
        assert!(!tr.step(&mut s, 2.0)); // u < 0.5: Day still dominates
        assert!(s.sky_blend > 0.0 && s.sky_blend < 0.5);
        let mut restart = demo_app::TodTransition::new(&s, 1, 10.0); // -> Sunset
        let mut probe = s;
        assert!(!restart.step(&mut probe, 0.0));
        assert_eq!(probe.sky_mode_from, 0, "restart must start from the dominant Day sky");
    }

    /// F16: Live mode advances simulation time like lockstep; the damper follows its driver.
    #[test]
    fn live_step_dts_scale_simulation_time() {
        assert_eq!(demo_app::live_step_dts(0.02, 0.25, true), (0.005, 0.005));
        assert_eq!(demo_app::live_step_dts(0.02, 0.25, false), (0.005, 0.02));
    }

    /// F30: no auto-frame request for Classic or for the scene that is already loaded.
    #[test]
    fn scene_auto_frame_is_not_armed_for_the_loaded_scene() {
        use demo_app::scene_auto_frame_target as arm;
        assert_eq!(arm(Some((1, 0)), (2, 0)), Some((2, 0)));
        assert_eq!(arm(Some((1, 0)), (1, 0)), None); // A -> B -> A coalesced
        assert_eq!(arm(None, (1, 0)), Some((1, 0)));
        assert_eq!(arm(Some((1, 0)), (0, 0)), None);
    }

    /// F14: size-only resizes wait for the previous `Resize` to be acknowledged.
    #[test]
    fn size_only_resize_waits_for_the_previous_acknowledgement() {
        use demo_app::resize_allowed as ok;
        // (size_changed, buffers_changed, throttle_elapsed, dlss_toggled, prev_acked)
        assert!(!ok(true, false, true, false, false));
        assert!(ok(true, false, true, false, true));
        assert!(!ok(true, false, false, false, true));
        assert!(ok(true, false, false, true, false));
        assert!(ok(false, true, false, false, false));
        assert!(!ok(false, false, true, true, true));
    }

    /// F12: a render thread that ended without `Fatal` is described with its panic payload.
    #[test]
    fn render_exit_reason_reports_the_panic_payload() {
        let panicked = std::thread::spawn(|| panic!("cuLaunchKernel assert"));
        let reason = demo_app::render_exit_reason(panicked);
        assert!(reason.contains("panicked") && reason.contains("cuLaunchKernel assert"), "{reason}");
        let clean = std::thread::spawn(|| {});
        assert!(demo_app::render_exit_reason(clean).contains("without reporting"));
    }

    /// F18: transient overrides are not persisted as user preferences.
    #[test]
    fn persisted_settings_undo_transient_overrides() {
        use crate::renderer::CameraProjection;
        let mut live = crate::renderer::DemoSettings::default();
        crate::ui::apply_csg_time_of_day_preset(&mut live, 0);
        let mut tr = demo_app::TodTransition::new(&live, 2, 4.0);
        assert!(!tr.step(&mut live, 1.0));
        live.camera_projection = CameraProjection::Equirect360;
        live.hdr_display = false;
        live.dlss_rr_on = false;
        let intent = demo_app::StartupIntent {
            hdr_display: Some(true),
            dlss_rr_on: Some(true),
            ai_at_start: (live.dlss_on, live.denoiser_on),
        };
        let saved = demo_app::user_intent_settings(
            &live,
            Some(CameraProjection::Pinhole),
            Some(&tr),
            intent,
        );
        let mut night = live;
        crate::ui::apply_csg_time_of_day_preset(&mut night, 2);
        assert_eq!(saved.camera_projection, CameraProjection::Pinhole);
        assert_eq!(saved.sun_elevation, night.sun_elevation);
        assert_eq!(saved.sky_blend, 0.0);
        assert!(saved.hdr_display);
        assert!(saved.dlss_rr_on);

        // The user picked another AI mode this session: the forced-off RR is not restored.
        let mut changed = live;
        changed.dlss_on = !live.dlss_on;
        assert!(!demo_app::user_intent_settings(&changed, None, None, intent).dlss_rr_on);
    }

    /// F17: the scene is persisted by id and resolves to its current registry index.
    #[test]
    fn saved_scene_resolves_by_id_after_a_registry_reorder() {
        let d = |id: &'static str, variants: &'static [&'static str]| csg_scene::SceneDescriptor {
            id,
            name: id,
            description: "",
            default_tod: csg_scene::TimeOfDay::Day,
            default_wetness: 0.0,
            supports_traffic: false,
            variant_labels: variants,
            geo_origin: None,
        };
        let old = [d("gundo", &[]), d("inspector", &["Robotaxi", "Bridge"])];
        let saved = demo_app::saved_scene_of(&old, 2, 1).expect("scene ref");
        assert_eq!(saved.id, "inspector");
        assert_eq!(saved.variant, "Bridge");

        let reordered = [
            d("marina", &[]),
            d("inspector", &["Bridge", "Robotaxi"]),
            d("gundo", &[]),
        ];
        assert_eq!(demo_app::resolve_saved_scene(&reordered, &saved, 1), Some((2, 0)));
        let removed = [d("gundo", &[])];
        assert_eq!(demo_app::resolve_saved_scene(&removed, &saved, 1), None);
        let classic = demo_app::saved_scene_of(&old, 0, 3).expect("classic ref");
        assert_eq!(demo_app::resolve_saved_scene(&reordered, &classic, 3), Some((0, 3)));
    }

    /// CAM-CURSOR: camera-rs `Hidden` (grab None + invisible) during RMB hold-to-fly hides the
    /// cursor, also when it replaces a persistent-fly lock, and does not warp it.
    #[test]
    fn cursor_hidden_request_hides_without_warp() {
        let mut ctl = demo_app::CursorGrabController::default();
        ctl.on_cursor_moved(winit::dpi::PhysicalPosition::new(200.0, 150.0));
        let hidden = ctl.apply_egui_request(Some(egui::CursorGrab::None), Some(false));
        assert_eq!(hidden.grab, None);
        assert_eq!(hidden.visible, Some(false));

        // Persistent fly lock, then RMB hold switches the request to Hidden.
        let lock = ctl.apply_egui_request(Some(egui::CursorGrab::Locked), Some(false));
        assert_eq!(lock.grab, Some(winit::window::CursorGrabMode::Locked));
        let to_hidden = ctl.apply_egui_request(Some(egui::CursorGrab::None), Some(false));
        assert_eq!(to_hidden.grab, Some(winit::window::CursorGrabMode::None));
        assert_eq!(to_hidden.visible, Some(false));
        assert_eq!(to_hidden.restore_pos, None);

        // Back to Default shows the cursor.
        let shown = ctl.apply_egui_request(Some(egui::CursorGrab::None), Some(true));
        assert_eq!(shown.visible, Some(true));
    }

    #[test]
    fn full_window_capture_skips_idle_video_frames_but_allows_stills() {
        assert!(!demo_app::should_capture_full_window(true, 0, false, false));
        assert!(demo_app::should_capture_full_window(true, 1, false, false));
        assert!(demo_app::should_capture_full_window(false, 0, true, false));
        assert!(demo_app::should_capture_full_window(false, 0, false, true));
    }
}
