//! [`Denoiser`]: the OptiX AI denoiser: every built-in model, user models, AOV layers, tiling.
//!
//! **What.** Creates the denoiser for one [`DenoiserModel`] (or for user weights,
//! [`Denoiser::with_user_model`]), sizes and allocates its state, scratch and, for temporal
//! models, the two internal guide layers once for a fixed input size (checked against an optional
//! byte budget), runs the setup, and then denoises frames with [`Denoiser::invoke`]: per frame it
//! computes the beauty's HDR intensity (and, for AOV-capable models, its average log colour) over
//! the full image, then runs the network on every [`Layer`] (beauty first, then AOVs) guided by
//! the [`Guides`] (albedo, normal, flow, flow trustworthiness).
//! **Images** ([`Image`]) are checked views into device buffers: half or float with 1-4 channels,
//! tightly packed or strided (byte offset, row and pixel stride). Upscaling models write outputs
//! (and read previous outputs) at twice the input size.
//! **Temporal models** keep the internal guide layers double-buffered: an invoke reads the
//! previous frame's layer and writes the current one, then the two swap. The first invoke after
//! creation or [`Denoiser::reset`] tells OptiX that the previous layers hold nothing.
//! **Tiling** ([`DenoiserOptions::tile`]) sizes scratch with overlap and denoises tile by tile, a
//! port of `optix_denoiser_tiling.h`; [`split_image`] is public for callers that run tiles
//! concurrently with scratch of their own. A layer may denoise in place (output = input, or
//! output = previous output) only untiled: a later tile's overlap would read pixels an earlier
//! tile already overwrote.
//! **Ordering.** State, scratch and guide layers are stream-ordered allocations on the denoiser's
//! stream and every call goes on that stream, so reuse across frames is ordered without
//! synchronization. **Used by** render-rs and the nv-rs OptiX GPU tests.

use crate::accel::same_ctx;
use crate::{DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, DeviceCopy, Stream};
use cuda_core_ext::same_stream;
use std::marker::PhantomData;
use std::sync::Arc;

/// Which trained network to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenoiserModel {
    /// LDR beauty (deprecated by OptiX, which maps it to the AOV model).
    Ldr,
    /// HDR beauty (deprecated by OptiX, which maps it to the AOV model).
    Hdr,
    /// Single image with optional AOV layers; the model NVIDIA recommends for new code.
    Aov,
    /// Temporally stable beauty for image sequences (deprecated; use [`Self::TemporalAov`]).
    Temporal,
    /// Temporally stable image sequences with optional AOV layers.
    TemporalAov,
    /// Single image, denoised and upscaled 2x, with optional AOV layers.
    Upscale2x,
    /// Image sequence, denoised and upscaled 2x, temporally stable, with optional AOV layers.
    TemporalUpscale2x,
}

impl DenoiserModel {
    fn kind(self) -> sys::OptixDenoiserModelKind {
        match self {
            Self::Ldr => sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_LDR,
            Self::Hdr => sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_HDR,
            Self::Aov => sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_AOV,
            Self::Temporal => sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_TEMPORAL,
            Self::TemporalAov => sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_TEMPORAL_AOV,
            Self::Upscale2x => sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_UPSCALE2X,
            Self::TemporalUpscale2x => {
                sys::OptixDenoiserModelKind_OPTIX_DENOISER_MODEL_KIND_TEMPORAL_UPSCALE2X
            }
        }
    }

    /// Frames need a flow guide, previous outputs and internal guide layers.
    pub fn is_temporal(self) -> bool {
        matches!(
            self,
            Self::Temporal | Self::TemporalAov | Self::TemporalUpscale2x
        )
    }

    /// Output size divided by input size (2 for the upscaling models).
    pub fn scale(self) -> u32 {
        if matches!(self, Self::Upscale2x | Self::TemporalUpscale2x) {
            2
        } else {
            1
        }
    }

    /// Layers after the beauty (AOVs) are accepted.
    pub fn has_aovs(self) -> bool {
        matches!(
            self,
            Self::Aov | Self::TemporalAov | Self::Upscale2x | Self::TemporalUpscale2x
        )
    }
}

/// What happens to the alpha channel of 4-channel images.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlphaMode {
    /// Copied from the input unchanged.
    #[default]
    Copy,
    /// Denoised with the colour.
    Denoise,
}

impl AlphaMode {
    fn to_sys(self) -> sys::OptixDenoiserAlphaMode {
        match self {
            Self::Copy => sys::OptixDenoiserAlphaMode_OPTIX_DENOISER_ALPHA_MODE_COPY,
            Self::Denoise => sys::OptixDenoiserAlphaMode_OPTIX_DENOISER_ALPHA_MODE_DENOISE,
        }
    }
}

/// What a layer holds; temporal AOV models use it as a hint.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AovType {
    /// Unspecified.
    #[default]
    None,
    /// The beauty pass.
    Beauty,
    /// Specular contribution.
    Specular,
    /// Reflection contribution.
    Reflection,
    /// Refraction contribution.
    Refraction,
    /// Diffuse contribution.
    Diffuse,
}

impl AovType {
    fn to_sys(self) -> sys::OptixDenoiserAOVType {
        match self {
            Self::None => sys::OptixDenoiserAOVType_OPTIX_DENOISER_AOV_TYPE_NONE,
            Self::Beauty => sys::OptixDenoiserAOVType_OPTIX_DENOISER_AOV_TYPE_BEAUTY,
            Self::Specular => sys::OptixDenoiserAOVType_OPTIX_DENOISER_AOV_TYPE_SPECULAR,
            Self::Reflection => sys::OptixDenoiserAOVType_OPTIX_DENOISER_AOV_TYPE_REFLECTION,
            Self::Refraction => sys::OptixDenoiserAOVType_OPTIX_DENOISER_AOV_TYPE_REFRACTION,
            Self::Diffuse => sys::OptixDenoiserAOVType_OPTIX_DENOISER_AOV_TYPE_DIFFUSE,
        }
    }
}

/// Pixel formats the denoiser reads and writes (8-bit formats are not supported: the intensity
/// and average-colour passes reject them).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelFormat {
    /// One half (flow trustworthiness).
    Half1,
    /// Two halves, XY (flow; normals of the LDR/HDR models).
    Half2,
    /// Three halves, RGB.
    Half3,
    /// Four halves, RGBA.
    Half4,
    /// One float (flow trustworthiness).
    Float1,
    /// Two floats, XY (flow; normals of the LDR/HDR models).
    Float2,
    /// Three floats, RGB.
    Float3,
    /// Four floats, RGBA.
    Float4,
}

impl PixelFormat {
    fn to_sys(self) -> sys::OptixPixelFormat {
        match self {
            Self::Half1 => sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_HALF1,
            Self::Half2 => sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_HALF2,
            Self::Half3 => sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_HALF3,
            Self::Half4 => sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_HALF4,
            Self::Float1 => sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT1,
            Self::Float2 => sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT2,
            Self::Float3 => sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT3,
            Self::Float4 => sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT4,
        }
    }

    /// Channels per pixel.
    pub fn channels(self) -> u32 {
        match self {
            Self::Half1 | Self::Float1 => 1,
            Self::Half2 | Self::Float2 => 2,
            Self::Half3 | Self::Float3 => 3,
            Self::Half4 | Self::Float4 => 4,
        }
    }

    /// Bytes per channel; also the alignment data and strides must have.
    pub fn channel_bytes(self) -> u32 {
        match self {
            Self::Half1 | Self::Half2 | Self::Half3 | Self::Half4 => size_of::<u16>() as u32,
            Self::Float1 | Self::Float2 | Self::Float3 | Self::Float4 => size_of::<f32>() as u32,
        }
    }

    /// Bytes of one tightly packed pixel.
    pub fn pixel_bytes(self) -> u32 {
        self.channels() * self.channel_bytes()
    }
}

/// An element type whose buffers [`Image::new`] wraps without naming the format.
pub trait Pixel: DeviceCopy {
    /// The format of one element.
    const FORMAT: PixelFormat;
}

impl Pixel for f32 {
    const FORMAT: PixelFormat = PixelFormat::Float1;
}
impl Pixel for [f32; 2] {
    const FORMAT: PixelFormat = PixelFormat::Float2;
}
impl Pixel for [f32; 3] {
    const FORMAT: PixelFormat = PixelFormat::Float3;
}
impl Pixel for [f32; 4] {
    const FORMAT: PixelFormat = PixelFormat::Float4;
}

/// A `width x height` image inside a device buffer, bounds- and alignment-checked against it.
#[derive(Clone, Copy, Debug)]
pub struct Image<'a> {
    data: sys::CUdeviceptr,
    width: u32,
    height: u32,
    row_stride: u32,
    pixel_stride: u32,
    format: PixelFormat,
    // Bytes from `data` to the end of the last pixel.
    span: u64,
    _buffer: PhantomData<&'a ()>,
}

impl<'a> Image<'a> {
    /// Wraps `buffer`, which must hold exactly `width * height` tightly packed pixels.
    pub fn new<P: Pixel>(buffer: &'a DeviceBuffer<P>, width: u32, height: u32) -> Result<Self> {
        Self::with_format(buffer, width, height, P::FORMAT)
    }

    /// Wraps `buffer`, which must hold exactly `width * height` tightly packed `format` pixels
    /// (e.g. half images in a `DeviceBuffer<u16>` or `DeviceBuffer<[u16; 4]>`).
    pub fn with_format<T: DeviceCopy>(
        buffer: &'a DeviceBuffer<T>,
        width: u32,
        height: u32,
        format: PixelFormat,
    ) -> Result<Self> {
        let pixel = format.pixel_bytes();
        let bytes = (u64::from(width) * u64::from(height)).checked_mul(u64::from(pixel));
        if width == 0 || height == 0 || bytes != Some(buffer.num_bytes() as u64) {
            return Err(Error::Invalid(
                "image buffer size is not width * height * pixel bytes",
            ));
        }
        let row = width
            .checked_mul(pixel)
            .ok_or(Error::Invalid("image row exceeds 4 GiB"))?;
        Self::strided(buffer, 0, width, height, format, row, pixel)
    }

    /// A strided view: pixel `(x, y)` starts at byte `offset + y * row_stride + x * pixel_stride`
    /// of `buffer` (e.g. RGB of an RGBA buffer, or one AOV of an interleaved buffer). Rows must
    /// not overlap, pixels must hold `format`, and data and strides must be aligned to the
    /// channel size.
    pub fn strided<T: DeviceCopy>(
        buffer: &'a DeviceBuffer<T>,
        offset: usize,
        width: u32,
        height: u32,
        format: PixelFormat,
        row_stride: u32,
        pixel_stride: u32,
    ) -> Result<Self> {
        let pixel = format.pixel_bytes();
        let channel = u64::from(format.channel_bytes());
        if width == 0 || height == 0 {
            return Err(Error::Invalid("image size must be non-zero"));
        }
        if pixel_stride < pixel {
            return Err(Error::Invalid(
                "pixel stride is smaller than the pixel format",
            ));
        }
        if u64::from(row_stride) < u64::from(width) * u64::from(pixel_stride) {
            return Err(Error::Invalid(
                "row stride is smaller than width * pixel stride",
            ));
        }
        let invalid = || Error::Invalid("image extends past the end of its buffer");
        let data = buffer
            .cu_deviceptr()
            .checked_add(offset as u64)
            .ok_or_else(invalid)?;
        if !data.is_multiple_of(channel)
            || !u64::from(row_stride).is_multiple_of(channel)
            || !u64::from(pixel_stride).is_multiple_of(channel)
        {
            return Err(Error::Invalid(
                "image data or strides are not aligned to the channel size",
            ));
        }
        // Each product is below 2^64; the sum is checked.
        let span = (u64::from(height - 1) * u64::from(row_stride))
            .checked_add(u64::from(width - 1) * u64::from(pixel_stride))
            .and_then(|s| s.checked_add(u64::from(pixel)))
            .ok_or_else(invalid)?;
        let end = (offset as u64).checked_add(span).ok_or_else(invalid)?;
        if end > buffer.num_bytes() as u64 {
            return Err(invalid());
        }
        Ok(Self {
            data,
            width,
            height,
            row_stride,
            pixel_stride,
            format,
            span,
            _buffer: PhantomData,
        })
    }

    /// `(width, height)` in pixels.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// The pixel format.
    pub fn format(&self) -> PixelFormat {
        self.format
    }

    fn to_sys(self) -> sys::OptixImage2D {
        sys::OptixImage2D {
            data: self.data,
            width: self.width,
            height: self.height,
            rowStrideInBytes: self.row_stride,
            pixelStrideInBytes: self.pixel_stride,
            format: self.format.to_sys(),
        }
    }

    /// Byte ranges intersect (conservative for interleaved images).
    fn overlaps(&self, other: &Image<'_>) -> bool {
        self.data < other.data.saturating_add(other.span)
            && other.data < self.data.saturating_add(self.span)
    }

    /// The very same pixels with the same layout.
    fn same(&self, other: &Image<'_>) -> bool {
        (
            self.data,
            self.width,
            self.height,
            self.row_stride,
            self.pixel_stride,
            self.format,
        ) == (
            other.data,
            other.width,
            other.height,
            other.row_stride,
            other.pixel_stride,
            other.format,
        )
    }
}

/// Guide images of one invoke; every image has the denoiser's input size.
#[derive(Clone, Copy, Debug, Default)]
pub struct Guides<'a> {
    /// RGB albedo (3 or 4 channels); exactly when [`DenoiserOptions::albedo`].
    pub albedo: Option<Image<'a>>,
    /// Normals (3 or 4 channels; 2 suffice for LDR/HDR); exactly when [`DenoiserOptions::normal`].
    pub normal: Option<Image<'a>>,
    /// Per-pixel motion from the previous to this frame in pixels (2 channels); exactly for
    /// temporal models.
    pub flow: Option<Image<'a>>,
    /// Trustworthiness of `flow` in `[0, 1]` (1 channel); temporal models only, optional.
    pub flow_trust: Option<Image<'a>>,
}

fn raw_or_null(image: Option<Image<'_>>) -> sys::OptixImage2D {
    image.map(Image::to_sys).unwrap_or_default()
}

/// One input/output pair: the beauty (first layer) or an AOV.
#[derive(Clone, Copy, Debug)]
pub struct Layer<'a> {
    /// Noisy input, at the denoiser's input size, 3 or 4 channels.
    pub input: Image<'a>,
    /// Denoised output, at [`Denoiser::output_size`]. It may be `input` itself (same layout) or,
    /// untiled, `previous` itself; it may not otherwise overlap any image of the invoke.
    pub output: Image<'a>,
    /// Temporal models: the previous frame's denoised output, at [`Denoiser::output_size`] (on
    /// the first frame, any substitute of that size, e.g. the noisy input upscaled or as is).
    /// Other models: `None`.
    pub previous: Option<Image<'a>>,
    /// What the layer holds.
    pub aov: AovType,
}

impl<'a> Layer<'a> {
    /// A non-temporal layer of unspecified type.
    pub fn new(input: Image<'a>, output: Image<'a>) -> Self {
        Self {
            input,
            output,
            previous: None,
            aov: AovType::None,
        }
    }

    fn to_sys(self) -> sys::OptixDenoiserLayer {
        sys::OptixDenoiserLayer {
            input: self.input.to_sys(),
            previousOutput: raw_or_null(self.previous),
            output: self.output.to_sys(),
            type_: self.aov.to_sys(),
        }
    }
}

/// Per-invoke parameters.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Params {
    /// `[0, 1]`: 0 = fully denoised, 1 = the noisy input.
    pub blend: f32,
    /// Factors applied to the flow vectors; 0 leaves them unscaled.
    pub flow_mul: [f32; 2],
}

/// How a [`Denoiser`] is created.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DenoiserOptions {
    /// The network (for a user model: the kind its weights implement, used to check invokes).
    pub model: DenoiserModel,
    /// An albedo guide is given to every invoke.
    pub albedo: bool,
    /// A normal guide is given to every invoke (requires `albedo`).
    pub normal: bool,
    /// Alpha handling (built-in models; a user model's weights fix it, so it must stay `Copy`).
    pub alpha: AlphaMode,
    /// Denoise in tiles of at most `[width, height]` input pixels (plus OptiX's overlap): less
    /// scratch for large images. `None` denoises the whole image at once.
    pub tile: Option<[u32; 2]>,
    /// Device bytes state, scratch and internal guide layers may take; `None` admits any size.
    pub budget: Option<usize>,
}

impl Default for DenoiserOptions {
    /// The AOV model without guides, alpha copied, untiled, no budget.
    fn default() -> Self {
        Self {
            model: DenoiserModel::Aov,
            albedo: false,
            normal: false,
            alpha: AlphaMode::Copy,
            tile: None,
            budget: None,
        }
    }
}

/// The OptiX denoiser, sized for one input size.
pub struct Denoiser {
    raw: sys::OptixDenoiser,
    options: DenoiserOptions,
    width: u32,
    height: u32,
    out_width: u32,
    out_height: u32,
    // Tile size clamped to the image; `None` when untiled.
    tile: Option<[u32; 2]>,
    overlap: u32,
    state: DeviceBuffer<u8>,
    scratch: DeviceBuffer<u8>,
    intensity: DeviceBuffer<f32>,
    average: DeviceBuffer<[f32; 3]>,
    // Temporal models: (previous frame's, this frame's) internal guide layer.
    guide: Option<(DeviceBuffer<u8>, DeviceBuffer<u8>)>,
    guide_pixel: u32,
    guide_row: u32,
    // The previous internal guide layer holds data of this sequence.
    warm: bool,
    stream: Arc<Stream>,
    ctx: Arc<DeviceContext>,
}

// SAFETY: the handle is used only through `&mut self` (invoke, compute_*) or `Drop`.
unsafe impl Send for Denoiser {}

impl Denoiser {
    /// A denoiser running a built-in model on `width x height` input images on `stream`.
    pub fn new(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        options: DenoiserOptions,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        check_options(&options, width, height)?;
        same_ctx(stream, ctx)?;
        let opts = sys::OptixDenoiserOptions {
            guideAlbedo: u32::from(options.albedo),
            guideNormal: u32::from(options.normal),
            denoiseAlpha: options.alpha.to_sys(),
        };
        ctx.cuda().bind_to_thread()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: options outlive the call; the driver loads the model weights.
        check("optixDenoiserCreate", unsafe {
            entry!(ctx.table(), optixDenoiserCreate)(
                ctx.raw(),
                options.model.kind(),
                &opts,
                &mut raw,
            )
        })?;
        Self::setup(ctx, stream, raw, options, width, height)
    }

    /// A denoiser running user `weights` (one exported weight set, which fixes the model kind,
    /// guides and alpha handling). `options` must describe what the weights expect: its model,
    /// guides and tiling drive the checks and allocations of this wrapper; `alpha` must be
    /// [`AlphaMode::Copy`].
    pub fn with_user_model(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        weights: &[u8],
        options: DenoiserOptions,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        check_options(&options, width, height)?;
        same_ctx(stream, ctx)?;
        if weights.is_empty() {
            return Err(Error::Invalid("user model weights are empty"));
        }
        if options.alpha != AlphaMode::Copy {
            return Err(Error::Invalid(
                "a user model fixes its own alpha handling; leave options.alpha at Copy",
            ));
        }
        ctx.cuda().bind_to_thread()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: OptiX reads `weights` only during the call.
        check("optixDenoiserCreateWithUserModel", unsafe {
            entry!(ctx.table(), optixDenoiserCreateWithUserModel)(
                ctx.raw(),
                weights.as_ptr().cast(),
                weights.len(),
                &mut raw,
            )
        })?;
        Self::setup(ctx, stream, raw, options, width, height)
    }

    /// Sizes, allocates and sets up a freshly created handle; destroys it on error.
    fn setup(
        ctx: &Arc<DeviceContext>,
        stream: &Arc<Stream>,
        raw: sys::OptixDenoiser,
        options: DenoiserOptions,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        // From here `Drop` owns `raw`, so an error below cannot leak it.
        let destroy_on_error = Guard { raw, ctx };
        let t = ctx.table();
        let overflow = || Error::Invalid("denoiser image size overflows");
        let tile = options.tile.map(|[w, h]| [w.min(width), h.min(height)]);
        let [tile_w, tile_h] = tile.unwrap_or([width, height]);
        let mut sizes = sys::OptixDenoiserSizes::default();
        // SAFETY: owned handle; writable sizes struct.
        check("optixDenoiserComputeMemoryResources", unsafe {
            entry!(t, optixDenoiserComputeMemoryResources)(raw, tile_w, tile_h, &mut sizes)
        })?;
        let (overlap, invoke_scratch) = if tile.is_some() {
            (
                sizes.overlapWindowSizeInPixels,
                sizes.withOverlapScratchSizeInBytes,
            )
        } else {
            (0, sizes.withoutOverlapScratchSizeInBytes)
        };
        let scratch_len = invoke_scratch
            .max(sizes.computeIntensitySizeInBytes)
            .max(sizes.computeAverageColorSizeInBytes)
            .max(1);
        let scale = options.model.scale();
        let out_width = width.checked_mul(scale).ok_or_else(overflow)?;
        let out_height = height.checked_mul(scale).ok_or_else(overflow)?;
        let temporal = options.model.is_temporal();
        let guide_pixel = if temporal {
            u32::try_from(sizes.internalGuideLayerPixelSizeInBytes).map_err(|_| overflow())?
        } else {
            0
        };
        if temporal && guide_pixel == 0 {
            return Err(Error::Invalid(
                "OptiX reported no internal guide layer pixel size for a temporal model",
            ));
        }
        // Internal guide layers have the output size, as the tiling utility splits them.
        let guide_row = out_width.checked_mul(guide_pixel).ok_or_else(overflow)?;
        let guide_bytes = (guide_row as usize)
            .checked_mul(out_height as usize)
            .ok_or_else(overflow)?;
        let peak = sizes
            .stateSizeInBytes
            .saturating_add(scratch_len)
            .saturating_add(guide_bytes.saturating_mul(2))
            .saturating_add(size_of::<f32>() + size_of::<[f32; 3]>());
        if let Some(available) = options.budget
            && peak > available
        {
            return Err(Error::Budget {
                requested: peak,
                available,
            });
        }
        // Setup takes the largest input it will see: a tile plus overlap on both sides.
        let (setup_w, setup_h) = match tile {
            Some(_) => {
                let pad = overlap.checked_mul(2).ok_or_else(overflow)?;
                (
                    tile_w.checked_add(pad).ok_or_else(overflow)?,
                    tile_h.checked_add(pad).ok_or_else(overflow)?,
                )
            }
            None => (width, height),
        };
        // SAFETY: the setup below initializes the state; scratch, intensity and average colour
        // are written by OptiX before they are read.
        let (state, scratch, intensity, average) = unsafe {
            (
                DeviceBuffer::<u8>::uninitialized_async(stream, sizes.stateSizeInBytes.max(1))?,
                DeviceBuffer::<u8>::uninitialized_async(stream, scratch_len)?,
                DeviceBuffer::<f32>::uninitialized_async(stream, 1)?,
                DeviceBuffer::<[f32; 3]>::uninitialized_async(stream, 1)?,
            )
        };
        let guide = if temporal {
            // SAFETY: zeroed on `stream` right below, before OptiX reads them; the header requires
            // the first frame's previous layer to be zero or valid data.
            let (mut prev, mut out) = unsafe {
                (
                    DeviceBuffer::<u8>::uninitialized_async(stream, guide_bytes)?,
                    DeviceBuffer::<u8>::uninitialized_async(stream, guide_bytes)?,
                )
            };
            prev.zero_async(stream)?;
            out.zero_async(stream)?;
            Some((prev, out))
        } else {
            None
        };
        // SAFETY: buffers sized by OptiX for this input size, ordered on `stream`.
        check("optixDenoiserSetup", unsafe {
            entry!(t, optixDenoiserSetup)(
                raw,
                stream.cu_stream(),
                setup_w,
                setup_h,
                state.cu_deviceptr(),
                state.len(),
                scratch.cu_deviceptr(),
                scratch.len(),
            )
        })?;
        let ctx = destroy_on_error.disarm();
        Ok(Self {
            raw,
            options,
            width,
            height,
            out_width,
            out_height,
            tile,
            overlap,
            state,
            scratch,
            intensity,
            average,
            guide,
            guide_pixel,
            guide_row,
            warm: false,
            stream: stream.clone(),
            ctx,
        })
    }

    /// Denoises one frame on the denoiser's stream: the beauty is `layers[0]`, AOVs follow
    /// (AOV-capable models only). The HDR intensity, and for AOV-capable models the average log
    /// colour, are computed from the full beauty input first, so every tile shares one exposure.
    /// Guides must match the options and the model (see [`Guides`] and [`Layer`]).
    ///
    /// # Safety
    ///
    /// The images must stay alive and unmodified (outputs: unread) until the work completes on
    /// the stream: synchronize it, or order their reuse and release after this call on it. The
    /// denoiser itself must not be dropped before the work completes either: its buffers are
    /// freed in stream order, but `optixDenoiserDestroy` runs at once on drop.
    pub unsafe fn invoke(
        &mut self,
        stream: &Arc<Stream>,
        guides: &Guides<'_>,
        layers: &[Layer<'_>],
        params: Params,
    ) -> Result<()> {
        self.on(stream)?;
        if !(0.0..=1.0).contains(&params.blend) {
            return Err(Error::Invalid("blend must be within [0, 1]"));
        }
        if !params.flow_mul.iter().all(|m| m.is_finite()) {
            return Err(Error::Invalid("flow multipliers must be finite"));
        }
        self.check_frame(guides, layers)?;
        let beauty = layers
            .first()
            .ok_or(Error::Invalid("the denoiser needs at least one layer"))?
            .input
            .to_sys();
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: the beauty image is valid per the caller's contract; the result buffer is owned.
        unsafe { self.intensity_into(stream, &beauty, self.intensity.cu_deviceptr()) }?;
        let average = if self.options.model.has_aovs() {
            // SAFETY: as above.
            unsafe { self.average_into(stream, &beauty, self.average.cu_deviceptr()) }?;
            self.average.cu_deviceptr()
        } else {
            0
        };
        let [flow_x, flow_y] = params.flow_mul;
        let p = sys::OptixDenoiserParams {
            hdrIntensity: self.intensity.cu_deviceptr(),
            blendFactor: params.blend,
            hdrAverageColor: average,
            temporalModeUsePreviousLayers: u32::from(self.warm),
            flowMulX: flow_x,
            flowMulY: flow_y,
        };
        let mut gl = sys::OptixDenoiserGuideLayer {
            albedo: raw_or_null(guides.albedo),
            normal: raw_or_null(guides.normal),
            flow: raw_or_null(guides.flow),
            flowTrustworthiness: raw_or_null(guides.flow_trust),
            ..Default::default()
        };
        if let Some((prev, out)) = &self.guide {
            gl.previousOutputInternalGuideLayer = self.guide_image(prev);
            gl.outputInternalGuideLayer = self.guide_image(out);
        }
        let raw_layers: Vec<sys::OptixDenoiserLayer> = layers.iter().map(|l| l.to_sys()).collect();
        match self.tile {
            // SAFETY: images valid per the caller's contract and checked above; state, scratch and
            // internal guide layers are owned and sized for this denoiser.
            None => unsafe { self.run(stream, &p, &gl, &raw_layers, (0, 0)) }?,
            // SAFETY: as above; scratch was sized with overlap for this tile size.
            Some(tile) => unsafe { self.run_tiled(stream, &p, &gl, &raw_layers, tile) }?,
        }
        if let Some((prev, out)) = &mut self.guide {
            std::mem::swap(prev, out);
        }
        self.warm = self.guide.is_some();
        Ok(())
    }

    /// Starts a new sequence (e.g. after a camera cut): the next invoke of a temporal model tells
    /// OptiX that the previous layers hold no data of it.
    pub fn reset(&mut self) {
        self.warm = false;
    }

    /// Writes the log average intensity of `image` (3 or 4 channels) to `out[0]`, the value
    /// [`Denoiser::invoke`] computes for the beauty.
    ///
    /// # Safety
    ///
    /// `image` must stay alive and unmodified, and `out` unread, until the work completes on the
    /// stream; so must the denoiser (see [`Denoiser::invoke`]).
    pub unsafe fn compute_intensity(
        &mut self,
        stream: &Arc<Stream>,
        image: Image<'_>,
        out: &mut DeviceBuffer<f32>,
    ) -> Result<()> {
        self.on(stream)?;
        color_input(&image)?;
        if out.is_empty() {
            return Err(Error::Invalid("intensity output buffer is empty"));
        }
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: forwarded contract; `out` holds at least one float.
        unsafe { self.intensity_into(stream, &image.to_sys(), out.cu_deviceptr()) }
    }

    /// Writes the per-channel log average colour of `image` (3 or 4 channels) to `out[0]`, the
    /// value [`Denoiser::invoke`] computes for the beauty of AOV-capable models.
    ///
    /// # Safety
    ///
    /// As [`Denoiser::compute_intensity`].
    pub unsafe fn compute_average_color(
        &mut self,
        stream: &Arc<Stream>,
        image: Image<'_>,
        out: &mut DeviceBuffer<[f32; 3]>,
    ) -> Result<()> {
        self.on(stream)?;
        color_input(&image)?;
        if out.is_empty() {
            return Err(Error::Invalid("average colour output buffer is empty"));
        }
        self.ctx.cuda().bind_to_thread()?;
        // SAFETY: forwarded contract; `out` holds at least three floats.
        unsafe { self.average_into(stream, &image.to_sys(), out.cu_deviceptr()) }
    }

    /// Input size the denoiser was set up for.
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Size of outputs and previous outputs (twice the input for upscaling models).
    pub fn output_size(&self) -> (u32, u32) {
        (self.out_width, self.out_height)
    }

    /// Overlap OptiX adds on every tile side (0 when untiled).
    pub fn overlap(&self) -> u32 {
        self.overlap
    }

    /// Device bytes held for state, scratch and internal guide layers.
    pub fn memory_bytes(&self) -> usize {
        let guides = self.guide.as_ref().map_or(0, |(a, b)| a.len() + b.len());
        self.state.len() + self.scratch.len() + guides
    }

    fn on(&self, stream: &Arc<Stream>) -> Result<()> {
        if same_stream(stream, &self.stream) {
            Ok(())
        } else {
            Err(Error::Invalid(
                "denoiser used on a different stream than it was created for",
            ))
        }
    }

    fn guide_image(&self, buffer: &DeviceBuffer<u8>) -> sys::OptixImage2D {
        sys::OptixImage2D {
            data: buffer.cu_deviceptr(),
            width: self.out_width,
            height: self.out_height,
            rowStrideInBytes: self.guide_row,
            pixelStrideInBytes: self.guide_pixel,
            format: sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_INTERNAL_GUIDE_LAYER,
        }
    }

    /// Rejects a frame OptiX would misread: guides vs options and model, sizes, formats, layer
    /// count, aliasing between outputs and everything else.
    fn check_frame(&self, g: &Guides<'_>, layers: &[Layer<'_>]) -> Result<()> {
        let model = self.options.model;
        let input = (self.width, self.height);
        let output = (self.out_width, self.out_height);
        if g.albedo.is_some() != self.options.albedo || g.normal.is_some() != self.options.normal {
            return Err(Error::Invalid(
                "guide images must match the guides the denoiser was created with",
            ));
        }
        if g.flow.is_some() != model.is_temporal() {
            return Err(Error::Invalid(
                "temporal models need a flow guide; other models take none",
            ));
        }
        if g.flow_trust.is_some() && !model.is_temporal() {
            return Err(Error::Invalid(
                "flow trustworthiness is for temporal models only",
            ));
        }
        for img in [g.albedo, g.normal, g.flow, g.flow_trust]
            .into_iter()
            .flatten()
        {
            if img.size() != input {
                return Err(Error::Invalid(
                    "guide size differs from the denoiser's input size",
                ));
            }
        }
        if let Some(a) = g.albedo
            && a.format.channels() < 3
        {
            return Err(Error::Invalid("the albedo guide needs 3 or 4 channels"));
        }
        if let Some(n) = g.normal
            && n.format.channels() < 3
            && !(n.format.channels() == 2
                && matches!(model, DenoiserModel::Ldr | DenoiserModel::Hdr))
        {
            return Err(Error::Invalid(
                "the normal guide needs 3 or 4 channels (2 only for the LDR/HDR models)",
            ));
        }
        if let Some(f) = g.flow
            && f.format.channels() != 2
        {
            return Err(Error::Invalid("the flow guide needs 2 channels"));
        }
        if let Some(f) = g.flow_trust
            && f.format.channels() != 1
        {
            return Err(Error::Invalid(
                "the flow trustworthiness guide needs 1 channel",
            ));
        }
        if layers.is_empty() {
            return Err(Error::Invalid("the denoiser needs at least one layer"));
        }
        if layers.len() > 1 && !model.has_aovs() {
            return Err(Error::Invalid(
                "this model denoises the beauty only; AOV layers need an AOV-capable model",
            ));
        }
        for l in layers {
            if l.input.size() != input {
                return Err(Error::Invalid(
                    "layer input size differs from the denoiser's input size",
                ));
            }
            if l.output.size() != output {
                return Err(Error::Invalid(
                    "layer output size differs from Denoiser::output_size",
                ));
            }
            if l.input.format.channels() < 3 || l.output.format.channels() < 3 {
                return Err(Error::Invalid("layer images need 3 or 4 channels"));
            }
            match l.previous {
                None if model.is_temporal() => {
                    return Err(Error::Invalid(
                        "temporal models need every layer's previous output",
                    ));
                }
                Some(_) if !model.is_temporal() => {
                    return Err(Error::Invalid(
                        "previous outputs are for temporal models only",
                    ));
                }
                Some(p) if p.size() != output || p.format.channels() < 3 => {
                    return Err(Error::Invalid(
                        "previous output needs Denoiser::output_size and 3 or 4 channels",
                    ));
                }
                _ => {}
            }
        }
        for (i, l) in layers.iter().enumerate() {
            let out = l.output;
            if [g.albedo, g.normal, g.flow, g.flow_trust]
                .into_iter()
                .flatten()
                .any(|r| out.overlaps(&r))
            {
                return Err(Error::Invalid("an output overlaps a guide image"));
            }
            for (j, other) in layers.iter().enumerate() {
                let own = i == j;
                // In place is per invoke: tiles run one after another, and a later tile's input
                // overlap would read pixels an earlier tile already overwrote with output.
                if out.overlaps(&other.input)
                    && !(own && self.tile.is_none() && out.same(&other.input))
                {
                    return Err(Error::Invalid(
                        "an output overlaps an input other than its own, identically laid out, untiled one",
                    ));
                }
                if let Some(p) = other.previous
                    && out.overlaps(&p)
                    && !(own && self.tile.is_none() && out.same(&p))
                {
                    return Err(Error::Invalid(
                        "an output overlaps a previous output other than its own, identically laid out, untiled one",
                    ));
                }
                if !own && out.overlaps(&other.output) {
                    return Err(Error::Invalid("two layer outputs overlap"));
                }
            }
        }
        Ok(())
    }

    /// One `optixDenoiserInvoke` with the denoiser's state and scratch.
    ///
    /// # Safety
    ///
    /// Every image in `guide` and `layers` must be valid until the work completes on `stream`.
    unsafe fn run(
        &self,
        stream: &Stream,
        params: &sys::OptixDenoiserParams,
        guide: &sys::OptixDenoiserGuideLayer,
        layers: &[sys::OptixDenoiserLayer],
        offset: (u32, u32),
    ) -> Result<()> {
        let n =
            u32::try_from(layers.len()).map_err(|_| Error::Invalid("too many denoiser layers"))?;
        // SAFETY: state and scratch were set up for this denoiser; descriptors outlive the call;
        // images valid per the caller's contract.
        check("optixDenoiserInvoke", unsafe {
            entry!(self.ctx.table(), optixDenoiserInvoke)(
                self.raw,
                stream.cu_stream(),
                params,
                self.state.cu_deviceptr(),
                self.state.len(),
                guide,
                layers.as_ptr(),
                n,
                offset.0,
                offset.1,
                self.scratch.cu_deviceptr(),
                self.scratch.len(),
            )
        })
    }

    /// `optixUtilDenoiserInvokeTiled`: splits every image into tiles of at most `tile` input
    /// pixels plus overlap and invokes once per tile, reusing the scratch.
    ///
    /// # Safety
    ///
    /// As [`Denoiser::run`].
    unsafe fn run_tiled(
        &self,
        stream: &Stream,
        params: &sys::OptixDenoiserParams,
        guide: &sys::OptixDenoiserGuideLayer,
        layers: &[sys::OptixDenoiserLayer],
        [tile_w, tile_h]: [u32; 2],
    ) -> Result<()> {
        let overlap = self.overlap;
        let scale = self.options.model.scale();
        let overflow = || Error::Invalid("tile size overflows");
        let (up_overlap, up_w, up_h) = (
            overlap.checked_mul(scale).ok_or_else(overflow)?,
            tile_w.checked_mul(scale).ok_or_else(overflow)?,
            tile_h.checked_mul(scale).ok_or_else(overflow)?,
        );
        let mut tiles = Vec::with_capacity(layers.len());
        let mut prev_tiles = Vec::with_capacity(layers.len());
        for l in layers {
            tiles.push(split_image(&l.input, &l.output, overlap, tile_w, tile_h)?);
            prev_tiles.push(if l.previousOutput.data != 0 {
                Some(split_image(
                    &l.previousOutput,
                    &l.previousOutput,
                    up_overlap,
                    up_w,
                    up_h,
                )?)
            } else {
                None
            });
        }
        let split = |img: &sys::OptixImage2D| -> Result<Option<Vec<Tile>>> {
            if img.data != 0 {
                split_image(img, img, overlap, tile_w, tile_h).map(Some)
            } else {
                Ok(None)
            }
        };
        let albedo = split(&guide.albedo)?;
        let normal = split(&guide.normal)?;
        let flow = split(&guide.flow)?;
        let trust = split(&guide.flowTrustworthiness)?;
        let (prev_ig, out_ig) = (
            &guide.previousOutputInternalGuideLayer,
            &guide.outputInternalGuideLayer,
        );
        let internal = if prev_ig.data != 0 && out_ig.data != 0 {
            Some(split_image(prev_ig, out_ig, up_overlap, up_w, up_h)?)
        } else {
            None
        };
        let beauty = tiles
            .first()
            .ok_or(Error::Invalid("the denoiser needs at least one layer"))?;
        let mut tile_layers = Vec::with_capacity(layers.len());
        for (t, first) in beauty.iter().enumerate() {
            tile_layers.clear();
            for ((l, lt), pt) in layers.iter().zip(&tiles).zip(&prev_tiles) {
                let cur = nth(lt, t)?;
                let prev = pt.as_deref().map(|v| nth(v, t)).transpose()?;
                tile_layers.push(sys::OptixDenoiserLayer {
                    input: cur.input,
                    previousOutput: prev.map(|p| p.input).unwrap_or_default(),
                    output: cur.output,
                    type_: l.type_,
                });
            }
            let pick = |v: &Option<Vec<Tile>>| -> Result<sys::OptixImage2D> {
                Ok(v.as_deref()
                    .map(|v| nth(v, t))
                    .transpose()?
                    .map(|x| x.input)
                    .unwrap_or_default())
            };
            let ig = internal.as_deref().map(|v| nth(v, t)).transpose()?;
            let gl = sys::OptixDenoiserGuideLayer {
                albedo: pick(&albedo)?,
                normal: pick(&normal)?,
                flow: pick(&flow)?,
                previousOutputInternalGuideLayer: ig.map(|g| g.input).unwrap_or_default(),
                outputInternalGuideLayer: ig.map(|g| g.output).unwrap_or_default(),
                flowTrustworthiness: pick(&trust)?,
            };
            // SAFETY: tiles are sub-images of images valid per the caller's contract.
            unsafe {
                self.run(
                    stream,
                    params,
                    &gl,
                    &tile_layers,
                    (first.offset_x, first.offset_y),
                )
            }?;
        }
        Ok(())
    }

    /// # Safety
    ///
    /// `image` valid until the work completes on `stream`; `out` holds one float.
    unsafe fn intensity_into(
        &self,
        stream: &Stream,
        image: &sys::OptixImage2D,
        out: sys::CUdeviceptr,
    ) -> Result<()> {
        // SAFETY: owned handle; scratch holds at least computeIntensitySizeInBytes; forwarded
        // image and output contract.
        check("optixDenoiserComputeIntensity", unsafe {
            entry!(self.ctx.table(), optixDenoiserComputeIntensity)(
                self.raw,
                stream.cu_stream(),
                image,
                out,
                self.scratch.cu_deviceptr(),
                self.scratch.len(),
            )
        })
    }

    /// # Safety
    ///
    /// `image` valid until the work completes on `stream`; `out` holds three floats.
    unsafe fn average_into(
        &self,
        stream: &Stream,
        image: &sys::OptixImage2D,
        out: sys::CUdeviceptr,
    ) -> Result<()> {
        // SAFETY: owned handle; scratch holds at least computeAverageColorSizeInBytes; forwarded
        // image and output contract.
        check("optixDenoiserComputeAverageColor", unsafe {
            entry!(self.ctx.table(), optixDenoiserComputeAverageColor)(
                self.raw,
                stream.cu_stream(),
                image,
                out,
                self.scratch.cu_deviceptr(),
                self.scratch.len(),
            )
        })
    }
}

fn check_options(o: &DenoiserOptions, width: u32, height: u32) -> Result<()> {
    if width == 0 || height == 0 {
        return Err(Error::Invalid("denoiser image size must be non-zero"));
    }
    if o.normal && !o.albedo {
        return Err(Error::Invalid("the normal guide requires the albedo guide"));
    }
    if let Some([w, h]) = o.tile
        && (w == 0 || h == 0)
    {
        return Err(Error::Invalid("tile size must be non-zero"));
    }
    Ok(())
}

/// The intensity and average-colour passes take 3- or 4-channel images only.
fn color_input(image: &Image<'_>) -> Result<()> {
    if image.format.channels() < 3 {
        Err(Error::Invalid(
            "intensity and average colour need a 3 or 4 channel image",
        ))
    } else {
        Ok(())
    }
}

impl Drop for Denoiser {
    fn drop(&mut self) {
        if let Some(destroy) = self.ctx.table().optixDenoiserDestroy {
            // The last owner may drop on a thread where the context is not current.
            let cuda = self.ctx.cuda();
            cuda.record_err(cuda.bind_to_thread());
            // SAFETY: owned handle; invocations must have completed (the `invoke` and `compute_*`
            // contracts).
            let code = unsafe { destroy(self.raw) };
            self.ctx.record("optixDenoiserDestroy", code);
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
            self.ctx.record("optixDenoiserDestroy", code);
        }
    }
}

/// One tile of [`split_image`] (`OptixUtilDenoiserImageTile`).
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    /// Input sub-image including overlap.
    pub input: sys::OptixImage2D,
    /// Output sub-image without overlap.
    pub output: sys::OptixImage2D,
    /// `optixDenoiserInvoke`'s `inputOffsetX`: overlap pixels left of the tile's own pixels.
    pub offset_x: u32,
    /// `optixDenoiserInvoke`'s `inputOffsetY`: overlap pixels above the tile's own pixels.
    pub offset_y: u32,
}

fn nth(tiles: &[Tile], t: usize) -> Result<Tile> {
    tiles
        .get(t)
        .copied()
        .ok_or(Error::Invalid("tile grids of the denoiser images differ"))
}

/// `optixUtilGetPixelStride`: the image's pixel stride, or its format's packed size when 0.
fn pixel_stride(image: &sys::OptixImage2D) -> Result<u32> {
    if image.pixelStrideInBytes != 0 {
        return Ok(image.pixelStrideInBytes);
    }
    let (channels, bytes) = match image.format {
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_HALF1 => (1, size_of::<u16>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_HALF2 => (2, size_of::<u16>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_HALF3 => (3, size_of::<u16>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_HALF4 => (4, size_of::<u16>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT1 => (1, size_of::<f32>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT2 => (2, size_of::<f32>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT3 => (3, size_of::<f32>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_FLOAT4 => (4, size_of::<f32>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_UCHAR3 => (3, size_of::<u8>()),
        sys::OptixPixelFormat_OPTIX_PIXEL_FORMAT_UCHAR4 => (4, size_of::<u8>()),
        _ => {
            return Err(Error::Invalid(
                "pixel stride must be given for internal-guide-layer or unknown pixel formats",
            ));
        }
    };
    Ok((channels * bytes) as u32)
}

/// `base + y * row + x * pixel` as a device address.
fn addr(base: sys::CUdeviceptr, y: i64, row: i64, x: i64, pixel: i64) -> Result<sys::CUdeviceptr> {
    // i128 cannot overflow for 32-bit strides and 33-bit coordinates.
    let bytes = i128::from(y) * i128::from(row) + i128::from(x) * i128::from(pixel);
    let bytes = u64::try_from(bytes).map_err(|_| Error::Invalid("tile lies before its image"))?;
    base.checked_add(bytes)
        .ok_or(Error::Invalid("tile address overflows"))
}

fn dim(v: i64) -> Result<u32> {
    u32::try_from(v).map_err(|_| Error::Invalid("tile dimension out of range"))
}

/// `optixUtilDenoiserSplitImage`: splits `input` into tiles of at most `tile_w x tile_h` pixels
/// plus `overlap` on inner sides, and `output` (the same or an integer multiple of the input
/// size) into the matching non-overlapping tiles. Pass each tile's offsets to
/// `optixDenoiserInvoke`; tiles may run concurrently when each has its own scratch.
pub fn split_image(
    input: &sys::OptixImage2D,
    output: &sys::OptixImage2D,
    overlap: u32,
    tile_w: u32,
    tile_h: u32,
) -> Result<Vec<Tile>> {
    if tile_w == 0 || tile_h == 0 {
        return Err(Error::Invalid("tile size must be non-zero"));
    }
    if input.width == 0 || input.height == 0 {
        return Err(Error::Invalid("cannot split an empty image"));
    }
    let in_pixel = i64::from(pixel_stride(input)?);
    let out_pixel = i64::from(pixel_stride(output)?);
    let (in_w, in_h) = (i64::from(input.width), i64::from(input.height));
    let (o, tw, th) = (i64::from(overlap), i64::from(tile_w), i64::from(tile_h));
    let (in_row, out_row) = (
        i64::from(input.rowStrideInBytes),
        i64::from(output.rowStrideInBytes),
    );
    let inp_w = (tw + 2 * o).min(in_w);
    let inp_h = (th + 2 * o).min(in_h);
    let up_x = i64::from(output.width) / in_w;
    let up_y = i64::from(output.height) / in_h;
    let mut tiles = Vec::new();
    let (mut inp_y, mut copied_y) = (0i64, 0i64);
    loop {
        let off_y = if inp_y == 0 {
            0
        } else {
            o.max(inp_h - (in_h - inp_y))
        };
        let copy_y = if inp_y == 0 {
            in_h.min(th + o)
        } else {
            th.min(in_h - copied_y)
        };
        let (mut inp_x, mut copied_x) = (0i64, 0i64);
        loop {
            let off_x = if inp_x == 0 {
                0
            } else {
                o.max(inp_w - (in_w - inp_x))
            };
            let copy_x = if inp_x == 0 {
                in_w.min(tw + o)
            } else {
                tw.min(in_w - copied_x)
            };
            let tile_in = sys::OptixImage2D {
                data: addr(input.data, inp_y - off_y, in_row, inp_x - off_x, in_pixel)?,
                width: dim(inp_w)?,
                height: dim(inp_h)?,
                rowStrideInBytes: input.rowStrideInBytes,
                pixelStrideInBytes: input.pixelStrideInBytes,
                format: input.format,
            };
            let tile_out = sys::OptixImage2D {
                data: addr(output.data, up_y * inp_y, out_row, up_x * inp_x, out_pixel)?,
                width: dim(up_x * copy_x)?,
                height: dim(up_y * copy_y)?,
                rowStrideInBytes: output.rowStrideInBytes,
                pixelStrideInBytes: output.pixelStrideInBytes,
                format: output.format,
            };
            tiles.push(Tile {
                input: tile_in,
                output: tile_out,
                offset_x: dim(off_x)?,
                offset_y: dim(off_y)?,
            });
            inp_x += if inp_x == 0 { tw + o } else { tw };
            copied_x += copy_x;
            if inp_x >= in_w {
                break;
            }
        }
        inp_y += if inp_y == 0 { th + o } else { th };
        copied_y += copy_y;
        if inp_y >= in_h {
            break;
        }
    }
    Ok(tiles)
}
