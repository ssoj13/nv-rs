//! [`Denoiser`]: the OptiX AI denoiser for RGBA float images.
//!
//! **What.** Creates the denoiser for one model ([`DenoiserModel::Hdr`] for beauty passes or
//! [`DenoiserModel::Aov`], the model NVIDIA recommends for new code), sizes and allocates its state
//! and scratch once for a fixed image size (checked against an optional byte budget), runs the
//! setup, and then denoises frames with [`Denoiser::invoke`]: it computes the HDR intensity of
//! the input and runs the network, optionally guided by albedo and normal images.
//! **Images** are `width x height` tightly packed RGBA `f32` buffers (`DeviceBuffer<[f32; 4]>`);
//! normals are camera-space XYZ in RGB.
//! **Ordering.** State and scratch are stream-ordered allocations on the denoiser's stream; every
//! call goes on that stream, so reuse across frames is ordered without synchronization.

use crate::{DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, Stream};
use std::sync::Arc;

/// Which trained network to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenoiserModel {
    /// HDR beauty denoising.
    Hdr,
    /// The AOV model: HDR beauty with guides (and AOV layers).
    Aov,
}

/// A `width x height` RGBA `f32` image on the device.
#[derive(Clone, Copy)]
pub struct Image<'a> {
    buffer: &'a DeviceBuffer<[f32; 4]>,
    width: u32,
    height: u32,
}

impl<'a> Image<'a> {
    /// Wraps `buffer`, which must hold exactly `width * height` pixels.
    pub fn new(buffer: &'a DeviceBuffer<[f32; 4]>, width: u32, height: u32) -> Result<Self> {
        if width == 0 || height == 0 || u64::from(width) * u64::from(height) != buffer.len() as u64 {
            return Err(Error::Invalid("image buffer length is not width * height"));
        }
        Ok(Self { buffer, width, height })
    }

    fn to_sys(self) -> sys::OptixImage2D {
        sys::OptixImage2D {
            data: self.buffer.cu_deviceptr(),
            width: self.width,
            height: self.height,
            rowStrideInBytes: self.width * 16,
            pixelStrideInBytes: 16,
            format: sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT4,
        }
    }
}

/// How a [`Denoiser`] is created.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DenoiserOptions {
    /// The network.
    pub model: DenoiserModel,
    /// An albedo guide is given to every invoke.
    pub albedo: bool,
    /// A normal guide is given to every invoke (requires `albedo`).
    pub normal: bool,
    /// Device bytes state + scratch may take; `None` admits any size.
    pub budget: Option<usize>,
}

impl Default for DenoiserOptions {
    /// The AOV model without guides, no budget.
    fn default() -> Self {
        Self { model: DenoiserModel::Aov, albedo: false, normal: false, budget: None }
    }
}

/// The OptiX denoiser, sized for one image size.
pub struct Denoiser {
    raw: sys::OptixDenoiser,
    width: u32,
    height: u32,
    albedo: bool,
    normal: bool,
    state: DeviceBuffer<u8>,
    scratch: DeviceBuffer<u8>,
    intensity: DeviceBuffer<f32>,
    stream: Arc<Stream>,
    ctx: Arc<DeviceContext>,
}

// SAFETY: the handle is used only through `&mut self` (invoke) or `Drop`.
unsafe impl Send for Denoiser {}

impl Denoiser {
    /// A denoiser for `width x height` images on `stream`.
    pub fn new(ctx: &Arc<DeviceContext>, stream: &Arc<Stream>, options: DenoiserOptions, width: u32, height: u32) -> Result<Self> {
        let DenoiserOptions { model, albedo, normal, budget } = options;
        if width == 0 || height == 0 {
            return Err(Error::Invalid("denoiser image size must be non-zero"));
        }
        if normal && !albedo {
            return Err(Error::Invalid("the normal guide requires the albedo guide"));
        }
        let t = ctx.table();
        let kind = match model {
            DenoiserModel::Hdr => sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_HDR,
            DenoiserModel::Aov => sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_AOV,
        };
        let opts = sys::OptixDenoiserOptions {
            guideAlbedo: u32::from(albedo),
            guideNormal: u32::from(normal),
            denoiseAlpha: sys::OptixDenoiserAlphaMode_OPTIX_DENOISER_ALPHA_MODE_COPY,
        };
        ctx.cuda().bind_to_thread()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: options outlive the call; the driver loads the model weights.
        check("optixDenoiserCreate", unsafe { entry!(t, optixDenoiserCreate)(ctx.raw(), kind, &opts, &mut raw) })?;
        // From here `Drop` owns `raw`, so an error below cannot leak it.
        let destroy_on_error = Guard { raw, ctx };
        let mut sizes = sys::OptixDenoiserSizes::default();
        // SAFETY: owned handle; writable sizes struct.
        check("optixDenoiserComputeMemoryResources", unsafe {
            entry!(t, optixDenoiserComputeMemoryResources)(raw, width, height, &mut sizes)
        })?;
        let scratch_len = sizes.withoutOverlapScratchSizeInBytes.max(sizes.computeIntensitySizeInBytes).max(1);
        let peak = sizes.stateSizeInBytes.saturating_add(scratch_len).saturating_add(4);
        if let Some(available) = budget
            && peak > available
        {
            return Err(Error::Budget { requested: peak, available });
        }
        // SAFETY: the setup below initializes the state; scratch and intensity are written by
        // OptiX before they are read.
        let (state, scratch, intensity) = unsafe {
            (
                DeviceBuffer::<u8>::uninitialized_async(stream, sizes.stateSizeInBytes.max(1))?,
                DeviceBuffer::<u8>::uninitialized_async(stream, scratch_len)?,
                DeviceBuffer::<f32>::uninitialized_async(stream, 1)?,
            )
        };
        // SAFETY: buffers sized by OptiX for this image size, ordered on `stream`.
        check("optixDenoiserSetup", unsafe {
            entry!(t, optixDenoiserSetup)(
                raw,
                stream.cu_stream(),
                width,
                height,
                state.cu_deviceptr(),
                sizes.stateSizeInBytes,
                scratch.cu_deviceptr(),
                scratch.len(),
            )
        })?;
        let ctx = destroy_on_error.disarm();
        Ok(Self { raw, width, height, albedo, normal, state, scratch, intensity, stream: stream.clone(), ctx })
    }

    /// Denoises `input` into `output` on the denoiser's stream; `blend` in `[0, 1]` mixes the
    /// noisy input back in (0 = fully denoised). Guides must be given exactly when the denoiser
    /// was created with them; every image must have the denoiser's size.
    ///
    /// # Safety
    ///
    /// The images must stay alive and unmodified (`output`: unread) until the work completes on
    /// the stream: synchronize it, or order their reuse and release after this call on it.
    pub unsafe fn invoke(&mut self, stream: &Arc<Stream>, input: Image<'_>, albedo: Option<Image<'_>>, normal: Option<Image<'_>>, output: &mut DeviceBuffer<[f32; 4]>, blend: f32) -> Result<()> {
        if !Arc::ptr_eq(stream, &self.stream) {
            return Err(Error::Invalid("denoiser used on a different stream than it was created for"));
        }
        if albedo.is_some() != self.albedo || normal.is_some() != self.normal {
            return Err(Error::Invalid("guide images must match the guides the denoiser was created with"));
        }
        if !(0.0..=1.0).contains(&blend) {
            return Err(Error::Invalid("blend must be within [0, 1]"));
        }
        let output = Image::new(output, self.width, self.height)?;
        for img in [Some(input), albedo, normal].into_iter().flatten() {
            if (img.width, img.height) != (self.width, self.height) {
                return Err(Error::Invalid("image size differs from the denoiser's"));
            }
            if img.buffer.cu_deviceptr() == output.buffer.cu_deviceptr() {
                return Err(Error::Invalid("output must not alias an input"));
            }
        }
        let t = self.ctx.table();
        let input_sys = input.to_sys();
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: input image valid per the caller's contract; intensity and scratch owned and
        // ordered on the stream.
        check("optixDenoiserComputeIntensity", unsafe {
            entry!(t, optixDenoiserComputeIntensity)(self.raw, stream.cu_stream(), &input_sys, self.intensity.cu_deviceptr(), self.scratch.cu_deviceptr(), self.scratch.len())
        })?;
        let params = sys::OptixDenoiserParams { hdrIntensity: self.intensity.cu_deviceptr(), blendFactor: blend, ..Default::default() };
        let guides = sys::OptixDenoiserGuideLayer {
            albedo: albedo.map(Image::to_sys).unwrap_or_default(),
            normal: normal.map(Image::to_sys).unwrap_or_default(),
            ..Default::default()
        };
        let layer = sys::OptixDenoiserLayer { input: input_sys, output: output.to_sys(), ..Default::default() };
        // SAFETY: state/scratch were set up for this size; images valid per the caller's
        // contract; descriptors outlive the call.
        check("optixDenoiserInvoke", unsafe {
            entry!(t, optixDenoiserInvoke)(
                self.raw,
                stream.cu_stream(),
                &params,
                self.state.cu_deviceptr(),
                self.state.len(),
                &guides,
                &layer,
                1,
                0,
                0,
                self.scratch.cu_deviceptr(),
                self.scratch.len(),
            )
        })
    }

    /// Image size the denoiser was set up for.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Device bytes held for state and scratch.
    pub fn memory_bytes(&self) -> usize {
        self.state.len() + self.scratch.len()
    }
}

impl Drop for Denoiser {
    fn drop(&mut self) {
        if let Some(destroy) = self.ctx.table().optixDenoiserDestroy {
            // SAFETY: owned handle; invocations must have completed (the `invoke` contract).
            let code = unsafe { destroy(self.raw) };
            self.ctx.record(code);
        }
    }
}

/// Destroys a freshly created denoiser if construction fails part-way.
struct Guard<'a> {
    raw: sys::OptixDenoiser,
    ctx: &'a Arc<DeviceContext>,
}

impl Guard<'_> {
    fn disarm(self) -> Arc<DeviceContext> {
        let ctx = self.ctx.clone();
        std::mem::forget(self);
        ctx
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        if let Some(destroy) = self.ctx.table().optixDenoiserDestroy {
            // SAFETY: the handle was created above and is not owned by anything else yet.
            let code = unsafe { destroy(self.raw) };
            self.ctx.record(code);
        }
    }
}
