//! Interactive RTX OptiX 3D Fractal Path Tracer + AI Denoiser + Zero-Copy `nv-wgpu` + `egui_dock` Demo.
//!
//! Run with:
//! `cargo run -p nv-demo --release`

mod app_events;
#[allow(unexpected_cfgs, dead_code, unused_imports, clippy::all)]
mod camera_core;
#[allow(unexpected_cfgs, dead_code, unused_imports, deprecated, clippy::all)]
mod camera_egui;
mod camera_rail;
mod csg_scene_host;
mod demo_anim;
mod fft_bloom;
mod gpu_export;
mod mp4_muxer;
mod nvtx;
mod persist;
mod profiler;
mod renderer;
mod ui;

use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;

use demo_app::{DemoApp, enable_rmb_fly};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

mod demo_app {
    use super::*;
    use crate::app_events::{
        AppBus, INTERACTION_SETTLE_WINDOW, PossessInfo, RenderControl, RenderEvent,
        RenderStateSnapshot, SmartViewResult, StillFormat, UiCommand,
    };
    use crate::camera_core::input::NavScheme;
    use crate::camera_core::{
        CameraDriver, CameraIntent, CameraPose, NavFeel, NavMode, NavPrefs, ViewportSize,
    };
    use crate::profiler::{GpuTelemetry, Profiler};
    use crate::renderer::{
        CameraFrame, CaptureSource, DEFAULT_H, DEFAULT_W, DemoSettings, DlssRrBufferPtrs,
        align_render_size, camera_projection_ray, spawn_render_thread,
    };
    use crate::ui::{DockTab, UiContext, UiTheme, default_dock_state, install_fonts};
    use appbus::downcast_event;
    use bytemuck::{Pod, Zeroable};
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
            5 => ngx::DlssPerfQuality::Dlaa,
            // `UltraQuality` (4) is deprecated/unsupported in DLSS 3.x+ runtime.
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
                quality: dlss_quality_from_ui(quality),
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
        recorder: crate::gpu_export::VideoRecorder,
        profile: crate::gpu_export::VideoProfile,
        source: CaptureSource,
        pixel_format: crate::gpu_export::SourcePixelFormat,
        width: u32,
        height: u32,
        fps: u32,
        copy_buf: Option<SharedBuffer>,
        copy_pitch_px: u32,
        last_frame_at: Option<Instant>,
    }

    pub struct DemoApp {
        window: Arc<Window>,
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
        surface_supports_copy_dst: bool,
        sdr_surface_format: wgpu::TextureFormat,
        hdr_surface_supported: bool,
        window_capture: Option<FullWindowCapture>,
        png_readback_scratch: Vec<u8>,
        png_rgba_scratch: Vec<u8>,
        dlss_pass: Option<DlssPass>,
        dlss_rr_pass: Option<DlssRrPass>,
        dlss_rr_available: bool,
        cached_dlss_opt: Option<((u32, u32, u32, bool), ngx::DlssOptimalSettings)>,
        ngx_vulkan: Option<ngx::NgxVulkan>,
        ngx_init_error: Option<String>,
        interop: InteropContext,
        hdr_tonemap: HdrTonemapPipeline,
        rr_last_exposure: f32,
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
        bus: AppBus,
        ui_cmd_scratch: Vec<appbus::BoxedEvent>,
        render_ev_scratch: Vec<appbus::BoxedEvent>,
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
        ui_fps: f32,
        rt_fps: f32,
        fast_preview: bool,
        reset_seq: u64,
        interacting_until: Option<Instant>,
        is_recording: bool,
        record_frames: u64,
        video_recorder: Option<ActiveRecorder>,
        shutter_in_flight: bool,
        ser_supported: bool,
        csg_stats: crate::csg_scene_host::CsgStats,
        scene_descriptors: Vec<csg_scene::SceneDescriptor>,
        scene_viewpoints: Vec<csg_scene::SceneViewpoint>,
        pending_scene_auto_frame: Option<(u32, u32)>,
        rail_random_seed: u64,
        smart_view_seed: u64,
        possess_info: Option<PossessInfo>,
        possess_free_yaw: f32,
        possess_free_pitch: f32,
        possess_auto_level: bool,
        last_sent_possess_look: (u32, u32, bool),
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
                let _ = active.recorder.finish();
            }
            self.save_persisted();
            self.bus.send_render(RenderControl::Shutdown);
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

            let dlss_rr_available = ngx_vulkan
                .as_ref()
                .and_then(|v| v.is_dlss_rr_available().ok())
                .unwrap_or(false);
            let hdr_tonemap = HdrTonemapPipeline::new(interop.device());

            let mut surface_config = surface
                .get_default_config(&adapter, size.width.max(1), size.height.max(1))
                .expect("surface default config");
            surface_config.present_mode = wgpu::PresentMode::AutoVsync;
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

            let mut settings = persisted.settings;
            if settings.hdr_display && !hdr_surface_supported {
                settings.hdr_display = false;
            }
            if settings.dlss_rr_on && !dlss_rr_available {
                settings.dlss_rr_on = false;
            }
            if settings.dlss_rr_on {
                settings.dlss_on = false;
                settings.denoiser_on = false;
            }
            let initial_camera = pose_to_frame(cam_pose);
            let bus = AppBus::new(RenderStateSnapshot {
                settings,
                camera: initial_camera,
                interacting_until: None,
                reset_seq: 0,
                possess_free_yaw: 0.0,
                possess_free_pitch: 0.0,
                possess_auto_level: true,
            });
            let render_thread = spawn_render_thread(
                Arc::clone(interop.cuda_context()),
                buf_ptrs,
                depth_ptrs,
                motion_ptrs,
                bus.clone(),
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
                surface_supports_copy_dst,
                sdr_surface_format,
                hdr_surface_supported,
                window_capture: None,
                png_readback_scratch: Vec::new(),
                png_rgba_scratch: Vec::new(),
                dlss_pass: None,
                dlss_rr_pass: None,
                dlss_rr_available,
                cached_dlss_opt: None,
                ngx_vulkan,
                ngx_init_error,
                interop,
                hdr_tonemap,
                rr_last_exposure: f32::NAN,
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
                last_saved_msg: None,
                bus,
                ui_cmd_scratch: Vec::with_capacity(16),
                render_ev_scratch: Vec::with_capacity(32),
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
                ui_fps: 60.0,
                rt_fps: 0.0,
                fast_preview: false,
                reset_seq: 0,
                interacting_until: None,
                is_recording: false,
                record_frames: 0,
                video_recorder: None,
                shutter_in_flight: false,
                ser_supported: false,
                csg_stats: crate::csg_scene_host::CsgStats::default(),
                scene_descriptors: crate::csg_scene_host::default_scene_descriptors(),
                scene_viewpoints: Vec::new(),
                pending_scene_auto_frame: None,
                rail_random_seed: 2026,
                smart_view_seed: 42,
                possess_info: None,
                possess_free_yaw: 0.0,
                possess_free_pitch: 0.0,
                possess_auto_level: true,
                last_sent_possess_look: (0.0f32.to_bits(), 0.0f32.to_bits(), true),
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

        fn notify_recording(&mut self, active: bool) {
            self.shutter_in_flight = false;
            let fixed_fps = if active && self.settings.record_fixed_fps {
                60
            } else {
                0
            };
            let shutter_subframes = if fixed_fps > 0 {
                self.settings.record_shutter_subframes.clamp(1, 4)
            } else {
                1
            };
            self.bus.send_render(RenderControl::SetRecording {
                active,
                fixed_fps,
                shutter_subframes,
            });
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
                camera_rail: self.camera_rail.clone(),
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
            self.ui_fps = self.ui_fps * 0.9 + (1.0 / dt) * 0.1;

            if self.last_telemetry_at.elapsed().as_secs_f32() > 0.35 {
                self.profiler
                    .update_telemetry(self.nvml_dev.as_ref(), &mut self.telemetry);
                self.last_telemetry_at = now;
            }

            // 1. Drain completed async events from the OptiX render thread (non-blocking).
            let mut frames_received = 0u32;
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
                        self.ser_supported = msg.ser_supported;
                        self.csg_stats = msg.csg_stats;
                        self.fast_preview = msg.fast_preview;
                        self.last_possess_camera = msg.possess_camera;
                        let total_gpu_ms =
                            (msg.rt_ms + msg.denoise_ms + msg.bloom_ms + msg.tonemap_ms).max(0.1);
                        let inst_rt_fps = 1000.0 / total_gpu_ms;
                        self.rt_fps = if self.rt_fps <= 0.01 {
                            inst_rt_fps
                        } else {
                            self.rt_fps * 0.85 + inst_rt_fps * 0.15
                        };
                        self.profiler.record_frame(
                            msg.rt_ms,
                            msg.denoise_ms,
                            msg.bloom_ms,
                            msg.tonemap_ms,
                            self.rt_fps,
                        );
                        self.retired_buffers.retain(|(g, _)| *g >= msg.gen_id);
                    }
                    RenderEvent::SceneMetaUpdated(meta) => {
                        self.scene_viewpoints = meta.viewpoints.clone();
                        self.camera_rail.set_scene_rails(meta.rails.clone());
                        if self.camera_rail.is_active() {
                            self.rail_blend_remaining = self.rail_blend_remaining.max(1.2);
                        }
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
                        self.rail_blend_remaining = 1.6;
                    }
                    RenderEvent::PossessChanged(info) => {
                        if let Some(pinfo) = info {
                            self.camera_rail.set_playing(false);
                            self.possess_free_yaw = 0.0;
                            self.possess_free_pitch = 0.0;
                            if let Some(m) = pinfo.mounts.get(pinfo.mount_idx) {
                                self.possess_auto_level = m.auto_level;
                            }
                            self.possess_info = Some(pinfo.clone());
                        } else {
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
                    RenderEvent::StatusMessage(status) => {
                        self.last_saved_msg = Some(status.clone());
                    }
                }
            }

            // 2. Docked egui pass (60+ FPS, zero blocking; camera_egui::drive steps the camera).
            let raw_input = self.egui_winit.take_egui_input(&self.window);
            let prev_theme = self.theme;
            let cam_fixed_dt = if self.is_recording && self.settings.record_fixed_fps {
                let shutter_n = self.settings.record_shutter_subframes.clamp(1, 4);
                let ts = self.settings.time_scale.clamp(0.05, 2.0);
                if shutter_n > 1 {
                    Some(0.0)
                } else {
                    Some((frames_received as f32 / 60.0) * ts)
                }
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
                ui_fps: self.ui_fps,
                rt_fps: self.rt_fps,
                fast_preview: self.fast_preview,
                is_recording: self.is_recording,
                record_frames: self.record_frames,
                ser_supported: self.ser_supported,
                csg_stats: self.csg_stats,
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
                pick_possess_mode: &mut self.pick_possess_mode,
                smart_view_seed: &mut self.smart_view_seed,
                last_smart_view: self.last_smart_view,
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

            let mut reset_accum_requested = false;
            let mut cam_snap_requested = false;
            let mut rail_blend_requested = false;
            let mut select_scene_target: Option<(u32, u32)> = None;
            let mut generate_random_rail_seed: Option<u64> = None;
            let mut save_png_requested = false;
            let mut save_jpeg_requested = false;
            let mut toggle_record_requested = false;
            let mut reload_shader_requested = false;
            let mut pick_possess_uv: Option<(f32, f32)> = None;
            let mut possess_next_vehicle_requested = false;
            let mut select_possess_mount: Option<usize> = None;
            let mut cycle_possess_mount_delta: i32 = 0;
            let mut eject_possess_requested = false;
            let mut run_smart_view: Option<(u64, csg_scene::ProbeBackendKind)> = None;
            let mut run_smart_rail: Option<(u64, csg_scene::ProbeBackendKind)> = None;
            self.bus.poll_ui_commands(&mut self.ui_cmd_scratch);
            for boxed in &self.ui_cmd_scratch {
                let Some(cmd) = downcast_event::<UiCommand>(boxed) else {
                    continue;
                };
                match *cmd {
                    UiCommand::ResetAccum => reset_accum_requested = true,
                    UiCommand::SaveStill(StillFormat::Png) => save_png_requested = true,
                    UiCommand::SaveStill(StillFormat::Jpeg) => save_jpeg_requested = true,
                    UiCommand::ToggleRecord => toggle_record_requested = true,
                    UiCommand::ReloadShader => reload_shader_requested = true,
                    UiCommand::SelectScene {
                        scene_idx,
                        variant_idx,
                    } => {
                        select_scene_target = Some((scene_idx, variant_idx));
                    }
                    UiCommand::GenerateRandomRail { seed } => {
                        generate_random_rail_seed = Some(seed);
                    }
                    UiCommand::SnapCamera => cam_snap_requested = true,
                    UiCommand::BlendRail => rail_blend_requested = true,
                    UiCommand::PickPossess { norm_x, norm_y } => {
                        pick_possess_uv = Some((norm_x, norm_y));
                    }
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
                    self.last_saved_msg = Some(
                        "DLSS-RR unavailable: SuperSamplingDenoising not supported on this host/runtime"
                            .to_string(),
                    );
                }
            } else if self.settings.dlss_on && self.ngx_vulkan.is_none() {
                self.settings.dlss_on = false;
                self.last_saved_msg = Some(
                    self.ngx_init_error
                        .clone()
                        .unwrap_or_else(|| "DLSS unavailable: NGX not initialized".to_string()),
                );
            }

            let dlss_any = self.settings.dlss_on || self.settings.dlss_rr_on;
            let (out_w, out_h) =
                align_render_size(self.desired_vp_size.0, self.desired_vp_size.1, 1.0);
            let (target_w, target_h) = if dlss_any {
                if let Some(ref ngx) = self.ngx_vulkan {
                    let is_rr = self.settings.dlss_rr_on;
                    let key = (out_w, out_h, self.settings.dlss_quality, is_rr);
                    let opt_res = match self.cached_dlss_opt {
                        Some((k, opt)) if k == key => Ok(opt),
                        _ => {
                            let q = dlss_quality_from_ui(self.settings.dlss_quality);
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
                        Ok(opt) => {
                            // `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT = 256` requires `render_w % 64 == 0`
                            // for 4-byte formats (`Rgba8Unorm`, `R32Float`).
                            let max_w = (opt.max_width.min(out_w) & !63).max(64);
                            let min_w = ((opt.min_width.max(64) + 63) & !63).min(max_w);
                            let min_h = opt.min_height.max(144.min(out_h)).min(out_h);
                            let max_h = opt.max_height.min(out_h).max(min_h);
                            (
                                ((opt.render_width + 31) & !63).clamp(min_w, max_w),
                                (opt.render_height & !1).clamp(min_h, max_h),
                            )
                        }
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
                    self.settings.render_scale,
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
                self.rr_last_exposure = f32::NAN;
            }

            let recording_vp_pq10 = self.video_recorder.as_ref().is_some_and(|r| {
                r.source == CaptureSource::Viewport
                    && r.profile == crate::gpu_export::VideoProfile::Hdr10Pq
            });
            let need_hdr_buf = hdr_active || (recording_vp_pq10 && !self.settings.dlss_rr_on);
            let dlss_toggled = self.settings.dlss_on != self.last_sent_settings.dlss_on
                || self.settings.dlss_rr_on != self.last_sent_settings.dlss_rr_on
                || self.settings.dlss_quality != self.last_sent_settings.dlss_quality;
            let needs_rr_bufs = self.settings.dlss_rr_on && self.rr_shared.is_none();
            let drop_rr_bufs = !self.settings.dlss_rr_on && self.rr_shared.is_some();
            let needs_hdr_bufs = need_hdr_buf && self.shared_hdr.is_none();
            let drop_hdr_bufs = !need_hdr_buf && self.shared_hdr.is_some();
            if (target_w != self.render_w
                || target_h != self.render_h
                || needs_rr_bufs
                || drop_rr_bufs
                || needs_hdr_bufs
                || drop_hdr_bufs)
                && (self.last_resize_at.elapsed().as_millis() >= 120
                    || dlss_toggled
                    || needs_rr_bufs
                    || drop_rr_bufs
                    || needs_hdr_bufs
                    || drop_hdr_bufs)
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
                        self.bus.send_render(RenderControl::Resize {
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

            if self.settings.dlss_rr_on {
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
                                if self.settings.aov_mode == 0 {
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
                                self.rr_last_exposure = f32::NAN;
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
                    if self.settings.aov_mode == 0 {
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
                            self.rr_last_exposure = f32::NAN;
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
            } else if self.settings.dlss_on {
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
            if save_png_requested {
                if self.settings.capture_source == CaptureSource::FullWindow {
                    capture_window_png = true;
                } else if self.has_frame {
                    self.last_saved_msg = Some(save_front_png(
                        &self.interop,
                        self.shared_rgba[self.front_buf_idx].cu_deviceptr(),
                        self.render_w,
                        self.render_h,
                    ));
                }
            }
            if save_jpeg_requested {
                if self.settings.capture_source == CaptureSource::FullWindow {
                    capture_window_jpeg = true;
                } else if self.has_frame {
                    let dir = std::path::Path::new("renders");
                    let _ = std::fs::create_dir_all(dir);
                    let ts = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let path = dir.join(format!("nv-demo-{ts}.jpg"));
                    self.bus.send_render(RenderControl::SaveJpeg { path, quality: 92 });
                }
            }
            if toggle_record_requested {
                if let Some(active) = self.video_recorder.take() {
                    let frames = active.recorder.frame_count();
                    self.last_saved_msg = Some(match active.recorder.finish() {
                        Ok(n) => format!("Saved MP4 ({n} frames)"),
                        Err(e) => format!("Video finish failed ({frames}f): {e}"),
                    });
                    self.is_recording = false;
                    self.record_frames = 0;
                    self.notify_recording(false);
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
                    let dir = std::path::Path::new("renders");
                    let _ = std::fs::create_dir_all(dir);
                    let ts = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);

                    if self.settings.capture_source == CaptureSource::FullWindow {
                        let win_w = self.surface_config.width;
                        let win_h = self.surface_config.height;
                        let win_tex_fmt = self.surface_config.format;
                        let win_fmt =
                            crate::gpu_export::SourcePixelFormat::from_wgpu(win_tex_fmt);
                        let even_dims = crate::gpu_export::align_even_dims(win_w, win_h);
                        let path = dir.join(format!("nv-demo-{ts}-win-{tag}.mp4"));

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
                                    _ => {
                                        match FullWindowCapture::new(
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
                                        }
                                    }
                                };
                                match cap_ok.and_then(|copy_pitch_px| {
                                    crate::gpu_export::VideoRecorder::start_with_format(
                                        self.interop.cuda_context(),
                                        rec_w,
                                        rec_h,
                                        60,
                                        profile,
                                        src_fmt,
                                        &path,
                                    )
                                    .map(|recorder| (recorder, copy_pitch_px))
                                }) {
                                    Ok((recorder, copy_pitch_px)) => {
                                        self.video_recorder = Some(ActiveRecorder {
                                            recorder,
                                            profile,
                                            source: CaptureSource::FullWindow,
                                            pixel_format: src_fmt,
                                            width: rec_w,
                                            height: rec_h,
                                            fps: 60,
                                            copy_buf: None,
                                            copy_pitch_px,
                                            last_frame_at: None,
                                        });
                                        self.is_recording = true;
                                        self.record_frames = 0;
                                        self.notify_recording(true);
                                        self.last_saved_msg = Some(format!(
                                            "Recording window to {}",
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
                            "PQ10 requires DLSS RR or OptiX/Raw (DLSS SR is 8-bit SDR)".to_string(),
                        );
                    } else {
                        let (src_w, src_h, uses_dlss_tex) =
                            if self.settings.dlss_rr_on
                                && self.settings.aov_mode == 0
                                && let Some(ref rr) = self.dlss_rr_pass
                            {
                                (rr.out_w & !1, rr.out_h & !1, true)
                            } else if self.settings.dlss_on
                                && let Some(ref dlss) = self.dlss_pass
                            {
                                (dlss.out_w & !1, dlss.out_h & !1, true)
                            } else {
                                (self.render_w & !1, self.render_h & !1, false)
                            };
                        let path = dir.join(format!("nv-demo-{ts}-{tag}.mp4"));

                        let bpp: u32 = if use_pq10 { 8 } else { 4 };
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
                            crate::gpu_export::VideoRecorder::start(
                                self.interop.cuda_context(),
                                src_w,
                                src_h,
                                60,
                                profile,
                                &path,
                            )
                            .map(|recorder| (recorder, copy_buf))
                        }) {
                            Ok((recorder, copy_buf)) => {
                                self.video_recorder = Some(ActiveRecorder {
                                    recorder,
                                    profile,
                                    source: CaptureSource::Viewport,
                                    pixel_format: if use_pq10 {
                                        crate::gpu_export::SourcePixelFormat::ScrgbF16
                                    } else {
                                        crate::gpu_export::SourcePixelFormat::Rgba8
                                    },
                                    width: src_w,
                                    height: src_h,
                                    fps: 60,
                                    copy_buf,
                                    copy_pitch_px,
                                    last_frame_at: None,
                                });
                                self.is_recording = true;
                                self.record_frames = 0;
                                self.rr_last_exposure = f32::NAN;
                                self.notify_recording(true);
                                self.last_saved_msg =
                                    Some(format!("Recording to {}", path.display()));
                            }
                            Err(e) => {
                                self.last_saved_msg = Some(format!("Record start failed: {e}"));
                            }
                        }
                    }
                }
            }
            if let Some((scene_idx, variant_idx)) = select_scene_target {
                self.pending_scene_auto_frame = Some((scene_idx, variant_idx));
                self.camera_rail.set_playing(false);
                if self.possess_info.is_some() {
                    self.bus.send_render(RenderControl::EjectPossess);
                }
            }
            if let Some(seed) = generate_random_rail_seed {
                if self.possess_info.is_some() {
                    self.bus.send_render(RenderControl::EjectPossess);
                }
                self.bus.send_render(RenderControl::GenerateRandomRail { seed });
            }
            if let Some((norm_x, norm_y)) = pick_possess_uv {
                let active_cam = self.last_possess_camera.unwrap_or(self.last_sent_camera);
                let aspect = self.render_w as f32 / self.render_h.max(1) as f32;
                if let Some((origin, dir)) = camera_projection_ray(
                    &active_cam,
                    self.settings.camera_projection,
                    norm_x,
                    norm_y,
                    aspect,
                ) {
                    self.bus.send_render(RenderControl::PickPossess { origin, dir });
                } else {
                    self.last_saved_msg = Some("Click inside the fisheye sensor circle to pick".to_string());
                }
            }
            if possess_next_vehicle_requested {
                self.bus.send_render(RenderControl::PossessNextVehicle);
            }
            if let Some(mount_idx) = select_possess_mount {
                self.bus.send_render(RenderControl::SetPossessMount { mount_idx });
            } else if cycle_possess_mount_delta != 0
                && let Some(ref info) = self.possess_info
                && !info.mounts.is_empty()
            {
                let next_idx = (info.mount_idx as i32 + cycle_possess_mount_delta)
                    .rem_euclid(info.mounts.len() as i32) as usize;
                self.bus
                    .send_render(RenderControl::SetPossessMount { mount_idx: next_idx });
            }
            if eject_possess_requested {
                self.bus.send_render(RenderControl::EjectPossess);
            }
            if let Some((seed, backend)) = run_smart_view {
                self.bus
                    .send_render(RenderControl::RunSmartView { seed, backend });
            }
            if let Some((seed, backend)) = run_smart_rail {
                self.bus
                    .send_render(RenderControl::RunSmartRail { seed, backend });
            }
            if reload_shader_requested {
                self.bus.send_render(RenderControl::ReloadShader);
            }

            // 4. Advance demo animator, camera rail, gimbal damper & push updates/subframes.
            // In Smooth 60 FPS recording mode, step everything by exact `(1/60s) * time_scale` per
            // encoded frame (and across N 180-degree shutter subframes when enabled).
            let time_scale = self.settings.time_scale.clamp(0.05, 2.0);
            let shutter_n = if self.is_recording && self.settings.record_fixed_fps {
                self.settings.record_shutter_subframes.clamp(1, 4)
            } else {
                1
            };
            if rail_blend_requested {
                if self.possess_info.is_some() {
                    self.bus.send_render(RenderControl::EjectPossess);
                }
                self.rail_blend_remaining = 1.6;
            }
            if cam_snap_requested {
                self.rail_blend_remaining = 0.0;
                self.cam_damper.snap(self.cam_pose);
            }

            if shutter_n > 1 {
                if !self.shutter_in_flight {
                    let frame_dt = (1.0 / 60.0) * time_scale;
                    for sub_idx in 0..shutter_n {
                        let sub_dt =
                            crate::camera_rail::shutter_subframe_dt(frame_dt, sub_idx, shutter_n);
                        self.demo_anim.tick(sub_dt);
                        if self.camera_rail.is_active() {
                            self.camera_rail.tick(sub_dt);
                            if let Some(rp) = self
                                .camera_rail
                                .evaluate_current(self.cam_pose.znear, self.cam_pose.zfar.max(2000.0))
                            {
                                self.cam_pose = rp;
                                self.cam_driver.controller_mut().set_pose(rp);
                            }
                        } else {
                            let raw = crate::camera_core::input::RawInput {
                                dt: sub_dt,
                                viewport: glam::Vec2::new(self.render_w as f32, self.render_h as f32),
                                ..Default::default()
                            };
                            let df = self.cam_driver.update(&raw, 2.0);
                            self.cam_pose = df.pose;
                        }
                        let effective_tau = if self.rail_blend_remaining > 0.0 {
                            let frac = (self.rail_blend_remaining / 1.6).clamp(0.0, 1.0);
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
                        self.last_sent_settings = effective_settings;
                        self.last_sent_camera = cam_frame;
                        self.bus.send_render(RenderControl::RenderShutterSubframe {
                            settings: Box::new(effective_settings),
                            camera: cam_frame,
                            sim_dt: sub_dt,
                            sub_idx,
                            sub_count: shutter_n,
                        });
                    }
                    self.shutter_in_flight = true;
                }
            } else {
                let fixed_recording = self.is_recording && self.settings.record_fixed_fps;
                let step_dt = if fixed_recording {
                    (frames_received as f32 / 60.0) * time_scale
                } else {
                    dt * time_scale
                };
                self.demo_anim.tick(step_dt);
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
                let damper_dt = if fixed_recording { step_dt } else { dt };
                let effective_tau = if self.rail_blend_remaining > 0.0 {
                    let frac = (self.rail_blend_remaining / 1.6).clamp(0.0, 1.0);
                    self.rail_blend_remaining = (self.rail_blend_remaining - damper_dt).max(0.0);
                    self.settings.cam_smooth_tau.max(0.32 * frac)
                } else {
                    self.settings.cam_smooth_tau
                };
                let render_pose =
                    self.cam_damper
                        .step(self.cam_pose, damper_dt, effective_tau);
                let mut effective_settings = self.demo_anim.apply(&self.settings);
                if !self.demo_anim.playing {
                    effective_settings.auto_rotate = false;
                    effective_settings.sat_orbit = false;
                }
                let cam_frame = pose_to_frame(render_pose);
                let possess_look = (
                    self.possess_free_yaw.to_bits(),
                    self.possess_free_pitch.to_bits(),
                    self.possess_auto_level,
                );
                if reset_accum_requested
                    || effective_settings != self.last_sent_settings
                    || cam_frame != self.last_sent_camera
                    || possess_look != self.last_sent_possess_look
                {
                    if reset_accum_requested {
                        self.reset_seq = self.reset_seq.wrapping_add(1);
                    }
                    let automated_motion = self.is_recording
                        || self.demo_anim.is_running()
                        || self.camera_rail.is_active();
                    self.interacting_until = if automated_motion {
                        None
                    } else {
                        Some(Instant::now() + INTERACTION_SETTLE_WINDOW)
                    };
                    self.last_sent_settings = effective_settings;
                    self.last_sent_camera = cam_frame;
                    self.last_sent_possess_look = possess_look;
                    self.bus.publish_render_state(RenderStateSnapshot {
                        settings: effective_settings,
                        camera: cam_frame,
                        interacting_until: self.interacting_until,
                        reset_seq: self.reset_seq,
                        possess_free_yaw: self.possess_free_yaw,
                        possess_free_pitch: self.possess_free_pitch,
                        possess_auto_level: self.possess_auto_level,
                    });
                }
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
            if let Some(vo) = full_output.viewport_output.get(&egui::ViewportId::ROOT) {
                for cmd in &vo.commands {
                    match cmd {
                        egui::ViewportCommand::CursorGrab(grab) => {
                            let mode = match grab {
                                egui::CursorGrab::None => winit::window::CursorGrabMode::None,
                                egui::CursorGrab::Confined => {
                                    winit::window::CursorGrabMode::Confined
                                }
                                egui::CursorGrab::Locked => winit::window::CursorGrabMode::Locked,
                            };
                            if self.window.set_cursor_grab(mode).is_err()
                                && matches!(mode, winit::window::CursorGrabMode::Locked)
                            {
                                let _ = self
                                    .window
                                    .set_cursor_grab(winit::window::CursorGrabMode::Confined);
                            }
                        }
                        egui::ViewportCommand::CursorVisible(visible) => {
                            self.window.set_cursor_visible(*visible);
                        }
                        _ => {}
                    }
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
                    && self.settings.aov_mode == 0
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
                        self.interop.queue().submit(Some(copy_enc.finish()));
                        let aspect = self.render_w as f32 / self.render_h.max(1) as f32;
                        let world_to_view = glam::camera::rh::view::look_to_mat4(
                            self.cam_pose.eye,
                            self.cam_pose.forward(),
                            self.cam_pose.up(),
                        )
                        .to_cols_array();
                        let view_to_clip = glam::camera::rh::proj::directx::perspective(
                            self.cam_pose.fov_y,
                            aspect,
                            self.cam_pose.znear.max(1e-3),
                            self.cam_pose.zfar.max(10.0),
                        )
                        .to_cols_array();
                        if let Err(e) = rr.evaluate(
                            &self.interop,
                            self.last_jitter,
                            world_to_view,
                            view_to_clip,
                        ) {
                            rr_failed = true;
                            self.last_saved_msg = Some(format!("DLSS-RR evaluate failed: {e}"));
                        } else {
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
                    } else if ran_rr
                        || (self.settings.exposure - self.rr_last_exposure).abs() > 1e-5
                    {
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
                        self.rr_last_exposure = self.settings.exposure;
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
                        self.interop.queue().submit(Some(copy_enc.finish()));
                        if let Err(e) = dlss.evaluate(
                            &self.interop,
                            self.last_jitter,
                            self.settings.dlss_sharpness,
                        ) {
                            self.settings.dlss_on = false;
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
            let need_win_capture =
                recording_full_win || capture_window_png || capture_window_jpeg;
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
                                let _ = active.recorder.finish();
                                self.is_recording = false;
                                self.record_frames = 0;
                                self.notify_recording(false);
                            }
                        }
                    }
                }
            } else if self.settings.capture_source == CaptureSource::Viewport {
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
                if self.surface_supports_copy_dst {
                    encoder.copy_texture_to_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &cap.texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        wgpu::TexelCopyTextureInfo {
                            texture: &frame.texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        extent,
                    );
                } else {
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
            } else {
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
                        && self.settings.aov_mode == 0
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

            self.interop.queue().submit(Some(encoder.finish()));

            if win_gpu_copied && let Some(ref cap) = self.window_capture {
                let _ = self
                    .interop
                    .device()
                    .poll(wgpu::PollType::wait_indefinitely());
                let ptr = cap.copy_buf.cu_deviceptr();
                let cap_w = cap.width;
                let cap_h = cap.height;
                let cap_pitch_px = cap.copy_pitch_px;
                let cap_fmt = cap.pixel_format;

                if capture_window_png {
                    let ts = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let path = std::path::Path::new("renders")
                        .join(format!("nv-demo-{ts}-win.png"));
                    self.last_saved_msg = Some(
                        match crate::gpu_export::save_png_from_pitched(
                            self.interop.cuda_context(),
                            ptr,
                            cap_w,
                            cap_h,
                            cap_pitch_px,
                            cap_fmt,
                            &path,
                            &mut self.png_readback_scratch,
                            &mut self.png_rgba_scratch,
                        ) {
                            Ok(()) => format!("Saved {}", path.display()),
                            Err(e) => format!("Window PNG failed: {e}"),
                        },
                    );
                }

                if capture_window_jpeg {
                    let ts = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let path = std::path::Path::new("renders")
                        .join(format!("nv-demo-{ts}-win.jpg"));
                    let jpeg_res = self
                        .interop
                        .cuda_context()
                        .bind_to_thread()
                        .map_err(|e| format!("CUDA bind failed: {e}"))
                        .and_then(|()| {
                            self.interop
                                .cuda_context()
                                .new_stream()
                                .map_err(|e| format!("CUDA stream failed: {e}"))
                        })
                        .and_then(|stream| {
                            crate::gpu_export::save_jpeg_from_pitched(
                                self.interop.cuda_context(),
                                &stream,
                                ptr,
                                cap_w,
                                cap_h,
                                cap_pitch_px,
                                cap_fmt,
                                &path,
                                92,
                            )
                        });
                    self.last_saved_msg = Some(match jpeg_res {
                        Ok(()) => format!("Saved {}", path.display()),
                        Err(e) => format!("Window JPEG failed: {e}"),
                    });
                }

                if recording_full_win
                    && (!self.settings.record_fixed_fps || frames_received > 0)
                    && let Some(mut active) = self.video_recorder.take()
                {
                    let dims_ok = crate::gpu_export::align_even_dims(cap_w, cap_h)
                        == Some((active.width, active.height))
                        && cap_fmt == active.pixel_format
                        && ptr != 0;
                    if !dims_ok {
                        let _ = active.recorder.finish();
                        self.is_recording = false;
                        self.record_frames = 0;
                        self.notify_recording(false);
                        self.last_saved_msg = Some(
                            "Recording stopped (window size or format changed)".to_string(),
                        );
                    } else {
                        let rec_now = Instant::now();
                        let frame_dur = if self.settings.record_fixed_fps {
                            1.0 / f64::from(active.fps.max(1))
                        } else {
                            let d = active.last_frame_at.map_or(
                                1.0 / f64::from(active.fps.max(1)),
                                |prev| rec_now.duration_since(prev).as_secs_f64(),
                            );
                            d.clamp(1.0 / 240.0, 2.0)
                        };
                        active.last_frame_at = Some(rec_now);
                        match active.recorder.push_frame(ptr, cap_pitch_px, frame_dur) {
                            Ok(()) => {
                                self.record_frames = active.recorder.frame_count();
                                self.video_recorder = Some(active);
                            }
                            Err(e) => {
                                let _ = active.recorder.finish();
                                self.is_recording = false;
                                self.record_frames = 0;
                                self.notify_recording(false);
                                self.last_saved_msg = Some(format!("Video encode error: {e}"));
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
                let (src_ptr, src_pitch, dims_ok) = if rec_gpu_copied {
                    let _ = self
                        .interop
                        .device()
                        .poll(wgpu::PollType::wait_indefinitely());
                    let ptr = active
                        .copy_buf
                        .as_ref()
                        .map_or(0, SharedBuffer::cu_deviceptr);
                    (ptr, active.copy_pitch_px, ptr != 0)
                } else if active.copy_buf.is_none()
                    && (self.render_w & !1) == active.width
                    && (self.render_h & !1) == active.height
                {
                    if is_pq {
                        if let Some(ref hdr_bufs) = self.shared_hdr {
                            (hdr_bufs[self.front_buf_idx].cu_deviceptr(), self.render_w, true)
                        } else {
                            // Waiting one frame for `shared_hdr` allocation after recording started.
                            self.video_recorder = Some(active);
                            self.interop.queue().present(frame);
                            self.window.request_redraw();
                            return;
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

                if !dims_ok {
                    let _ = active.recorder.finish();
                    self.is_recording = false;
                    self.record_frames = 0;
                    self.notify_recording(false);
                    self.last_saved_msg =
                        Some("Recording stopped (resolution or upscale mode changed)".to_string());
                } else {
                    let rec_now = Instant::now();
                    let frame_dur = if self.settings.record_fixed_fps {
                        1.0 / f64::from(active.fps.max(1))
                    } else {
                        let d = active.last_frame_at.map_or(
                            1.0 / f64::from(active.fps.max(1)),
                            |prev| rec_now.duration_since(prev).as_secs_f64(),
                        );
                        d.clamp(1.0 / 240.0, 2.0)
                    };
                    active.last_frame_at = Some(rec_now);
                    match active.recorder.push_frame(src_ptr, src_pitch, frame_dur) {
                        Ok(()) => {
                            self.record_frames = active.recorder.frame_count();
                            self.video_recorder = Some(active);
                        }
                        Err(e) => {
                            let _ = active.recorder.finish();
                            self.is_recording = false;
                            self.record_frames = 0;
                            self.notify_recording(false);
                            self.last_saved_msg = Some(format!("Video encode error: {e}"));
                        }
                    }
                }
            }

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
    nvtx::name_current_thread(c"nv-demo-main");
    let event_loop = EventLoop::new().expect("event loop");
    let mut runner = Runner::default();
    event_loop.run_app(&mut runner).expect("run app");
}
