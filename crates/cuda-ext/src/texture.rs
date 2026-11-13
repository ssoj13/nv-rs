//! Texture objects (filtered, cached reads) and surface objects (typed writes).
//!
//! **What.** [`TextureObject`] over an [`Array`], a [`MipmappedArray`], a linear device buffer
//! (`tex1Dfetch`) or a pitched 2D device
//! buffer; [`SurfaceObject`] over a surface-capable [`Array`]. Each object holds an `Arc` of its
//! storage, so the storage cannot be freed while a kernel may still read it through the handle.
//! [`TextureObject::handle`] / [`SurfaceObject::handle`] are the 64-bit values a kernel receives.
//! **Why.** Environment maps and OptiX material textures need hardware filtering and mipmaps;
//! device code samples them through texture intrinsics (cuda-oxide catalog, nv-rs M5).

use crate::array::{Array, ArrayFormat, MipmappedArray};
use crate::{invalid_value, teardown};
use cuda_core::sys;
use cuda_core::{Context, DriverError, IntoResult, RawDeviceBuffer};
use std::sync::Arc;

/// What happens to coordinates outside `[0, size)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressMode {
    /// Repeat (normalized coordinates only).
    Wrap,
    /// Clamp to the edge.
    Clamp,
    /// Mirror (normalized coordinates only).
    Mirror,
    /// Return the border colour.
    Border,
}

impl AddressMode {
    fn to_sys(self) -> sys::CUaddress_mode {
        match self {
            AddressMode::Wrap => sys::CUaddress_mode_enum_CU_TR_ADDRESS_MODE_WRAP,
            AddressMode::Clamp => sys::CUaddress_mode_enum_CU_TR_ADDRESS_MODE_CLAMP,
            AddressMode::Mirror => sys::CUaddress_mode_enum_CU_TR_ADDRESS_MODE_MIRROR,
            AddressMode::Border => sys::CUaddress_mode_enum_CU_TR_ADDRESS_MODE_BORDER,
        }
    }
}

/// Texel / mip-level filtering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterMode {
    /// Nearest texel.
    Point,
    /// Linear interpolation (float or normalized-integer reads only).
    Linear,
}

impl FilterMode {
    fn to_sys(self) -> sys::CUfilter_mode {
        match self {
            FilterMode::Point => sys::CUfilter_mode_enum_CU_TR_FILTER_MODE_POINT,
            FilterMode::Linear => sys::CUfilter_mode_enum_CU_TR_FILTER_MODE_LINEAR,
        }
    }
}

/// How a texture is read.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextureDesc {
    /// Per-axis address mode (x, y, z).
    pub address: [AddressMode; 3],
    /// Texel filter.
    pub filter: FilterMode,
    /// Filter between mip levels.
    pub mip_filter: FilterMode,
    /// Coordinates in `[0, 1)` instead of texels.
    pub normalized_coords: bool,
    /// Return integer formats as integers instead of normalized floats.
    pub read_as_integer: bool,
    /// Convert sRGB-encoded 8-bit data to linear on read.
    pub srgb: bool,
    /// Maximum anisotropy (1 disables).
    pub max_anisotropy: u32,
    /// Border colour for [`AddressMode::Border`].
    pub border_color: [f32; 4],
}

impl Default for TextureDesc {
    /// Bilinear, clamped, normalized coordinates, normalized-float reads.
    fn default() -> Self {
        Self {
            address: [AddressMode::Clamp; 3],
            filter: FilterMode::Linear,
            mip_filter: FilterMode::Linear,
            normalized_coords: true,
            read_as_integer: false,
            srgb: false,
            max_anisotropy: 1,
            border_color: [0.0; 4],
        }
    }
}

impl TextureDesc {
    fn to_sys(self, levels: u32) -> sys::CUDA_TEXTURE_DESC {
        // SAFETY: all-zero is the documented starting point.
        let mut d: sys::CUDA_TEXTURE_DESC = unsafe { std::mem::zeroed() };
        d.addressMode = self.address.map(AddressMode::to_sys);
        d.filterMode = self.filter.to_sys();
        d.mipmapFilterMode = self.mip_filter.to_sys();
        let mut flags = 0;
        if self.normalized_coords {
            flags |= sys::CU_TRSF_NORMALIZED_COORDINATES;
        }
        if self.read_as_integer {
            flags |= sys::CU_TRSF_READ_AS_INTEGER;
        }
        if self.srgb {
            flags |= sys::CU_TRSF_SRGB;
        }
        d.flags = flags;
        d.maxAnisotropy = self.max_anisotropy;
        d.maxMipmapLevelClamp = levels.saturating_sub(1) as f32;
        d.borderColor = self.border_color;
        d
    }
}

/// What a texture reads from (kept alive by the texture).
#[expect(dead_code, reason = "held only to keep the storage alive while the texture handle exists")]
enum Source {
    Array(Arc<Array>),
    Mipmapped(Arc<MipmappedArray>),
    Pitched(RawDeviceBuffer),
    Linear(RawDeviceBuffer),
}

/// A texture object.
pub struct TextureObject {
    raw: sys::CUtexObject,
    ctx: Arc<Context>,
    _source: Source,
}

// SAFETY: a CUtexObject is an immutable 64-bit driver handle.
unsafe impl Send for TextureObject {}
// SAFETY: as above.
unsafe impl Sync for TextureObject {}

impl TextureObject {
    /// A texture over a whole array.
    pub fn from_array(array: &Arc<Array>, desc: TextureDesc) -> Result<Self, DriverError> {
        // SAFETY: zeroed resource descriptor, then the array.
        let mut res: sys::CUDA_RESOURCE_DESC = unsafe { std::mem::zeroed() };
        res.resType = sys::CUresourcetype_enum_CU_RESOURCE_TYPE_ARRAY;
        res.res.array.hArray = array.cu_array();
        Self::create(array.context(), &res, desc.to_sys(1), Source::Array(array.clone()))
    }

    /// A texture over every level of a mipmapped array.
    pub fn from_mipmapped(array: &Arc<MipmappedArray>, desc: TextureDesc) -> Result<Self, DriverError> {
        // SAFETY: zeroed resource descriptor, then the mipmapped array.
        let mut res: sys::CUDA_RESOURCE_DESC = unsafe { std::mem::zeroed() };
        res.resType = sys::CUresourcetype_enum_CU_RESOURCE_TYPE_MIPMAPPED_ARRAY;
        res.res.mipmap.hMipmappedArray = array.cu_mipmapped_array();
        Self::create(array.context(), &res, desc.to_sys(array.levels()), Source::Mipmapped(array.clone()))
    }

    /// A 2D texture over a pitched device buffer (`pitch_bytes` per row; the driver requires the
    /// pitch to meet the device's texture-pitch alignment).
    pub fn from_pitched(
        buffer: &RawDeviceBuffer,
        width: usize,
        height: usize,
        pitch_bytes: usize,
        format: ArrayFormat,
        channels: u32,
        desc: TextureDesc,
    ) -> Result<Self, DriverError> {
        let row = width * format.bytes() * channels as usize;
        if !matches!(channels, 1 | 2 | 4) || pitch_bytes < row || pitch_bytes.checked_mul(height).is_none_or(|n| n > buffer.len_bytes()) {
            return Err(invalid_value());
        }
        // SAFETY: zeroed resource descriptor, then the pitched range checked above.
        let mut res: sys::CUDA_RESOURCE_DESC = unsafe { std::mem::zeroed() };
        res.resType = sys::CUresourcetype_enum_CU_RESOURCE_TYPE_PITCH2D;
        res.res.pitch2D.devPtr = buffer.cu_deviceptr();
        res.res.pitch2D.format = format.to_sys();
        res.res.pitch2D.numChannels = channels;
        res.res.pitch2D.width = width;
        res.res.pitch2D.height = height;
        res.res.pitch2D.pitchInBytes = pitch_bytes;
        Self::create(buffer.context(), &res, desc.to_sys(1), Source::Pitched(buffer.clone()))
    }

    /// A 1D texture over a whole linear device buffer (`CU_RESOURCE_TYPE_LINEAR`), read with
    /// `tex1Dfetch` (integer texel index; no filtering, no normalized coordinates, no mipmaps).
    /// The buffer must hold a whole number of `format` x `channels` elements, at most the
    /// device's `MAXIMUM_TEXTURE1D_LINEAR_WIDTH`, and start on its `TEXTURE_ALIGNMENT`.
    pub fn from_linear(buffer: &RawDeviceBuffer, format: ArrayFormat, channels: u32, desc: TextureDesc) -> Result<Self, DriverError> {
        let elem = format.bytes() * channels as usize;
        let ctx = buffer.context();
        let attr = |a| -> Result<u64, DriverError> {
            ctx.bind_to_thread()?;
            let mut v = 0;
            // SAFETY: valid attribute and out-pointer; the device belongs to `ctx`.
            unsafe { sys::cuDeviceGetAttribute(&mut v, a, ctx.cu_device()) }.result()?;
            u64::try_from(v).map_err(|_| invalid_value())
        };
        let max = attr(sys::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH)?;
        let align = attr(sys::CUdevice_attribute_enum_CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT)?.max(1);
        if !matches!(channels, 1 | 2 | 4) || buffer.len_bytes() == 0 || !buffer.len_bytes().is_multiple_of(elem) {
            return Err(invalid_value());
        }
        if (buffer.len_bytes() / elem) as u64 > max || !buffer.cu_deviceptr().is_multiple_of(align) {
            return Err(invalid_value());
        }
        // Linear textures take only element fetches: normalized coordinates and filtering do not
        // apply, and the driver rejects them.
        if desc.normalized_coords || desc.filter == FilterMode::Linear {
            return Err(invalid_value());
        }
        // SAFETY: zeroed resource descriptor, then the linear range checked above.
        let mut res: sys::CUDA_RESOURCE_DESC = unsafe { std::mem::zeroed() };
        res.resType = sys::CUresourcetype_enum_CU_RESOURCE_TYPE_LINEAR;
        res.res.linear.devPtr = buffer.cu_deviceptr();
        res.res.linear.format = format.to_sys();
        res.res.linear.numChannels = channels;
        res.res.linear.sizeInBytes = buffer.len_bytes();
        Self::create(ctx, &res, desc.to_sys(1), Source::Linear(buffer.clone()))
    }

    fn create(ctx: &Arc<Context>, res: &sys::CUDA_RESOURCE_DESC, tex: sys::CUDA_TEXTURE_DESC, source: Source) -> Result<Self, DriverError> {
        ctx.bind_to_thread()?;
        let mut raw: sys::CUtexObject = 0;
        // SAFETY: valid descriptors; no resource view; the storage is kept alive by `source`.
        unsafe { sys::cuTexObjectCreate(&mut raw, res, &tex, std::ptr::null()) }.result()?;
        Ok(Self { raw, ctx: ctx.clone(), _source: source })
    }

    /// The handle a kernel receives.
    pub fn handle(&self) -> u64 {
        self.raw
    }
}

impl Drop for TextureObject {
    fn drop(&mut self) {
        let raw = self.raw;
        // SAFETY: owned handle; work using it must have completed (the caller's stream contract).
        teardown(&self.ctx, || unsafe { sys::cuTexObjectDestroy(raw) });
    }
}

/// A surface object over a surface-capable array.
pub struct SurfaceObject {
    raw: sys::CUsurfObject,
    ctx: Arc<Context>,
    _array: Arc<Array>,
}

// SAFETY: a CUsurfObject is an immutable 64-bit driver handle.
unsafe impl Send for SurfaceObject {}
// SAFETY: as above.
unsafe impl Sync for SurfaceObject {}

impl SurfaceObject {
    /// A surface over `array`, which must have been created with `ArrayDesc::surface`.
    pub fn new(array: &Arc<Array>) -> Result<Self, DriverError> {
        if !array.desc().surface {
            return Err(invalid_value());
        }
        let ctx = array.context();
        ctx.bind_to_thread()?;
        // SAFETY: zeroed resource descriptor, then the array.
        let mut res: sys::CUDA_RESOURCE_DESC = unsafe { std::mem::zeroed() };
        res.resType = sys::CUresourcetype_enum_CU_RESOURCE_TYPE_ARRAY;
        res.res.array.hArray = array.cu_array();
        let mut raw: sys::CUsurfObject = 0;
        // SAFETY: valid descriptor; the array is kept alive by `_array`.
        unsafe { sys::cuSurfObjectCreate(&mut raw, &res) }.result()?;
        Ok(Self { raw, ctx: ctx.clone(), _array: array.clone() })
    }

    /// The handle a kernel receives.
    pub fn handle(&self) -> u64 {
        self.raw
    }
}

impl Drop for SurfaceObject {
    fn drop(&mut self) {
        let raw = self.raw;
        // SAFETY: owned handle; work using it must have completed (the caller's stream contract).
        teardown(&self.ctx, || unsafe { sys::cuSurfObjectDestroy(raw) });
    }
}
