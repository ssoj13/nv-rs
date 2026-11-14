//! Safe wrapper over `nvjpeg-sys` for GPU-accelerated JPEG decoding, encoding, and metadata
//! parsing.
//!
//! **What.**
//! - [`Handle`] — owns `nvjpegHandle_t` bound to a `cuda_core::Context`; records any `Drop`
//!   teardown error on the CUDA context.
//! - [`JpegState`], [`PinnedBuffer`], [`NvjpegDeviceBuffer`] — decode state and reusable
//!   pinned/device scratch buffers.
//! - [`JpegStream`] — host-side JPEG header/metadata parser (dimensions, chroma subsampling,
//!   EXIF orientation, sample precision, encoding).
//! - [`EncoderParams`], [`EncoderState`] — JPEG compression parameters, encoding from planar
//!   or interleaved device buffers, and zero-alloc bitstream retrieval (`retrieve_bitstream_into`).
//! - [`ImagePlanes`] / [`ImagePlanesMut`] — validated planar/interleaved `nvjpegImage_t` views
//!   over `cuda_core::DeviceBuffer<u8>`.
//!
//! **Used by** `nvjpeg` GPU tests and imaging pipelines.

pub mod encode;
pub mod handle;

pub use encode::{EncoderParams, EncoderState};
pub use handle::{Handle, JpegState, JpegStream, NvjpegDeviceBuffer, PinnedBuffer};

pub use nvjpeg_sys as sys;

use cuda_core::{DeviceBuffer, DriverError};
use std::fmt;
use std::marker::PhantomData;

/// Raw `nvjpegStatus_t` wrapped in a `Copy` integer newtype so newer toolkit codes stay valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NvjpegStatus(pub u32);

impl NvjpegStatus {
    /// `NVJPEG_STATUS_SUCCESS` (`0`).
    pub const SUCCESS: Self = Self(sys::nvjpegStatus_t_NVJPEG_STATUS_SUCCESS);
}

impl fmt::Display for NvjpegStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NvjpegStatus({})", self.0)
    }
}

/// Errors returned by `nvjpeg`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `nvjpeg` could not be loaded from the CUDA Toolkit.
    #[error("nvJPEG unavailable: {0}")]
    Unavailable(String),
    /// An nvJPEG call returned a non-success status code.
    #[error("{call} failed with {code}")]
    Status {
        /// The nvJPEG entry point.
        call: &'static str,
        /// The raw status code.
        code: NvjpegStatus,
    },
    /// A CUDA driver call failed.
    #[error("CUDA: {0}")]
    Driver(#[from] DriverError),
    /// An argument, plane layout, or quality value failed validation before reaching nvJPEG.
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
}

/// Result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn ensure_loaded() -> Result<()> {
    sys::load().map(|_| ()).map_err(|e| Error::Unavailable(e.to_string()))
}

pub(crate) fn check(call: &'static str, code: sys::nvjpegStatus_t) -> Result<()> {
    if code == sys::nvjpegStatus_t_NVJPEG_STATUS_SUCCESS {
        Ok(())
    } else {
        Err(Error::Status { call, code: NvjpegStatus(code) })
    }
}

/// Decoder/handle backend (`nvjpegBackend_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Backend {
    /// `NVJPEG_BACKEND_DEFAULT`.
    #[default]
    Default,
    /// `NVJPEG_BACKEND_HYBRID`.
    Hybrid,
    /// `NVJPEG_BACKEND_GPU_HYBRID`.
    GpuHybrid,
    /// `NVJPEG_BACKEND_HARDWARE`.
    Hardware,
}

impl Backend {
    pub(crate) fn to_sys(self) -> sys::nvjpegBackend_t {
        match self {
            Self::Default => sys::nvjpegBackend_t_NVJPEG_BACKEND_DEFAULT,
            Self::Hybrid => sys::nvjpegBackend_t_NVJPEG_BACKEND_HYBRID,
            Self::GpuHybrid => sys::nvjpegBackend_t_NVJPEG_BACKEND_GPU_HYBRID,
            Self::Hardware => sys::nvjpegBackend_t_NVJPEG_BACKEND_HARDWARE,
        }
    }
}

/// Chroma subsampling format (`nvjpegChromaSubsampling_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChromaSubsampling {
    /// 4:4:4 (`NVJPEG_CSS_444`).
    Yuv444,
    /// 4:2:2 (`NVJPEG_CSS_422`).
    Yuv422,
    /// 4:2:0 (`NVJPEG_CSS_420`).
    Yuv420,
    /// 4:4:0 (`NVJPEG_CSS_440`).
    Yuv440,
    /// 4:1:1 (`NVJPEG_CSS_411`).
    Yuv411,
    /// 4:1:0 (`NVJPEG_CSS_410`).
    Yuv410,
    /// Grayscale (`NVJPEG_CSS_GRAY`).
    Gray,
    /// 4:1:0V (`NVJPEG_CSS_410V`).
    Yuv410v,
    /// Unknown (`NVJPEG_CSS_UNKNOWN`).
    Unknown,
}

impl ChromaSubsampling {
    pub(crate) fn from_sys(raw: sys::nvjpegChromaSubsampling_t) -> Self {
        match raw {
            sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_444 => Self::Yuv444,
            sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_422 => Self::Yuv422,
            sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_420 => Self::Yuv420,
            sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_440 => Self::Yuv440,
            sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_411 => Self::Yuv411,
            sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_410 => Self::Yuv410,
            sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_GRAY => Self::Gray,
            sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_410V => Self::Yuv410v,
            _ => Self::Unknown,
        }
    }

    pub(crate) fn to_sys(self) -> Result<sys::nvjpegChromaSubsampling_t> {
        match self {
            Self::Yuv444 => Ok(sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_444),
            Self::Yuv422 => Ok(sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_422),
            Self::Yuv420 => Ok(sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_420),
            Self::Yuv440 => Ok(sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_440),
            Self::Yuv411 => Ok(sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_411),
            Self::Yuv410 => Ok(sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_410),
            Self::Gray => Ok(sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_GRAY),
            Self::Yuv410v => Ok(sys::nvjpegChromaSubsampling_t_NVJPEG_CSS_410V),
            Self::Unknown => Err(Error::InvalidInput("cannot encode with Unknown chroma subsampling")),
        }
    }
}

/// EXIF orientation tag (`nvjpegExifOrientation_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExifOrientation {
    /// Unknown / absent.
    Unknown,
    /// Normal (1).
    Normal,
    /// Flip horizontal (2).
    FlipHorizontal,
    /// Rotate 180 (3).
    Rotate180,
    /// Flip vertical (4).
    FlipVertical,
    /// Transpose (5).
    Transpose,
    /// Rotate 90 CW (6).
    Rotate90,
    /// Transverse (7).
    Transverse,
    /// Rotate 270 CW (8).
    Rotate270,
}

impl ExifOrientation {
    pub(crate) fn from_sys(raw: sys::nvjpegExifOrientation_t) -> Self {
        match raw {
            sys::nvjpegExifOrientation_NVJPEG_ORIENTATION_NORMAL => Self::Normal,
            sys::nvjpegExifOrientation_NVJPEG_ORIENTATION_FLIP_HORIZONTAL => Self::FlipHorizontal,
            sys::nvjpegExifOrientation_NVJPEG_ORIENTATION_ROTATE_180 => Self::Rotate180,
            sys::nvjpegExifOrientation_NVJPEG_ORIENTATION_FLIP_VERTICAL => Self::FlipVertical,
            sys::nvjpegExifOrientation_NVJPEG_ORIENTATION_TRANSPOSE => Self::Transpose,
            sys::nvjpegExifOrientation_NVJPEG_ORIENTATION_ROTATE_90 => Self::Rotate90,
            sys::nvjpegExifOrientation_NVJPEG_ORIENTATION_TRANSVERSE => Self::Transverse,
            sys::nvjpegExifOrientation_NVJPEG_ORIENTATION_ROTATE_270 => Self::Rotate270,
            _ => Self::Unknown,
        }
    }
}

/// JPEG SOF encoding type (`nvjpegJpegEncoding_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JpegEncoding {
    /// Unknown.
    Unknown,
    /// Baseline DCT (`0xc0`).
    BaselineDct,
    /// Extended sequential DCT Huffman (`0xc1`).
    ExtendedSequentialDctHuffman,
    /// Progressive DCT Huffman (`0xc2`).
    ProgressiveDctHuffman,
    /// Lossless Huffman (`0xc3`).
    LosslessHuffman,
}

impl JpegEncoding {
    pub(crate) fn from_sys(raw: sys::nvjpegJpegEncoding_t) -> Self {
        match raw {
            sys::nvjpegJpegEncoding_t_NVJPEG_ENCODING_BASELINE_DCT => Self::BaselineDct,
            sys::nvjpegJpegEncoding_t_NVJPEG_ENCODING_EXTENDED_SEQUENTIAL_DCT_HUFFMAN => Self::ExtendedSequentialDctHuffman,
            sys::nvjpegJpegEncoding_t_NVJPEG_ENCODING_PROGRESSIVE_DCT_HUFFMAN => Self::ProgressiveDctHuffman,
            sys::nvjpegJpegEncoding_t_NVJPEG_ENCODING_LOSSLESS_HUFFMAN => Self::LosslessHuffman,
            _ => Self::Unknown,
        }
    }

    pub(crate) fn to_sys(self) -> Result<sys::nvjpegJpegEncoding_t> {
        match self {
            Self::BaselineDct => Ok(sys::nvjpegJpegEncoding_t_NVJPEG_ENCODING_BASELINE_DCT),
            Self::ExtendedSequentialDctHuffman => Ok(sys::nvjpegJpegEncoding_t_NVJPEG_ENCODING_EXTENDED_SEQUENTIAL_DCT_HUFFMAN),
            Self::ProgressiveDctHuffman => Ok(sys::nvjpegJpegEncoding_t_NVJPEG_ENCODING_PROGRESSIVE_DCT_HUFFMAN),
            Self::LosslessHuffman => Ok(sys::nvjpegJpegEncoding_t_NVJPEG_ENCODING_LOSSLESS_HUFFMAN),
            Self::Unknown => Err(Error::InvalidInput("cannot encode with Unknown JPEG encoding")),
        }
    }
}

/// Decode output format (`nvjpegOutputFormat_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// Planar unchanged.
    Unchanged,
    /// Planar YUV.
    Yuv,
    /// Grayscale Y plane.
    Y,
    /// Planar RGB (`channel[0..3]`).
    Rgb,
    /// Planar BGR (`channel[0..3]`).
    Bgr,
    /// Interleaved RGB in `channel[0]`.
    Rgbi,
    /// Interleaved BGR in `channel[0]`.
    Bgri,
    /// Interleaved unchanged.
    UnchangedI,
}

impl OutputFormat {
    pub(crate) fn to_sys(self) -> sys::nvjpegOutputFormat_t {
        match self {
            Self::Unchanged => sys::nvjpegOutputFormat_t_NVJPEG_OUTPUT_UNCHANGED,
            Self::Yuv => sys::nvjpegOutputFormat_t_NVJPEG_OUTPUT_YUV,
            Self::Y => sys::nvjpegOutputFormat_t_NVJPEG_OUTPUT_Y,
            Self::Rgb => sys::nvjpegOutputFormat_t_NVJPEG_OUTPUT_RGB,
            Self::Bgr => sys::nvjpegOutputFormat_t_NVJPEG_OUTPUT_BGR,
            Self::Rgbi => sys::nvjpegOutputFormat_t_NVJPEG_OUTPUT_RGBI,
            Self::Bgri => sys::nvjpegOutputFormat_t_NVJPEG_OUTPUT_BGRI,
            Self::UnchangedI => sys::nvjpegOutputFormat_t_NVJPEG_OUTPUT_UNCHANGEDI,
        }
    }
}

/// Encode input format (`nvjpegInputFormat_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputFormat {
    /// Planar YUV.
    Yuv,
    /// Planar RGB (`channel[0..3]`).
    Rgb,
    /// Planar BGR (`channel[0..3]`).
    Bgr,
    /// Interleaved RGB in `channel[0]`.
    Rgbi,
    /// Interleaved BGR in `channel[0]`.
    Bgri,
}

impl InputFormat {
    pub(crate) fn to_sys(self) -> sys::nvjpegInputFormat_t {
        match self {
            Self::Yuv => sys::nvjpegInputFormat_t_NVJPEG_INPUT_YUV,
            Self::Rgb => sys::nvjpegInputFormat_t_NVJPEG_INPUT_RGB,
            Self::Bgr => sys::nvjpegInputFormat_t_NVJPEG_INPUT_BGR,
            Self::Rgbi => sys::nvjpegInputFormat_t_NVJPEG_INPUT_RGBI,
            Self::Bgri => sys::nvjpegInputFormat_t_NVJPEG_INPUT_BGRI,
        }
    }
}

/// Image metadata returned by `nvjpegGetImageInfo`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    /// Number of encoded components (`1` or `3`).
    pub components: usize,
    /// Chroma subsampling.
    pub subsampling: ChromaSubsampling,
    /// Width of each component (`0` for unused components).
    pub widths: [u32; 4],
    /// Height of each component (`0` for unused components).
    pub heights: [u32; 4],
}

/// Validates a plane descriptor's pitch and backing buffer length.
pub fn validate_plane(buf_len: usize, pitch: usize, min_row_bytes: usize, height: u32) -> Result<()> {
    if min_row_bytes == 0 || height == 0 {
        return Err(Error::InvalidInput("plane dimensions must be non-zero"));
    }
    if pitch < min_row_bytes {
        return Err(Error::InvalidInput("plane pitch is smaller than row byte width"));
    }
    let needed = (height as usize - 1)
        .checked_mul(pitch)
        .and_then(|n| n.checked_add(min_row_bytes))
        .ok_or(Error::InvalidInput("plane byte size overflows"))?;
    if buf_len < needed {
        return Err(Error::InvalidInput("device buffer is smaller than plane layout requires"));
    }
    Ok(())
}

/// Internal layout kind tracked on [`ImagePlanes`] / [`ImagePlanesMut`] so nvJPEG format
/// mismatches (e.g. passing a single plane to a planar RGB decode/encode) are rejected before FFI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlaneLayout {
    Interleaved { width: u32, height: u32, bpp: u32 },
    Planar3 { width: u32, height: u32, plane_dims: [(u32, u32); 3] },
    SingleY { width: u32, height: u32 },
}

impl OutputFormat {
    pub(crate) fn validate_layout(self, layout: PlaneLayout) -> Result<()> {
        match (self, layout) {
            (Self::Rgbi | Self::Bgri, PlaneLayout::Interleaved { bpp: 3, .. }) => Ok(()),
            (Self::UnchangedI, PlaneLayout::Interleaved { .. }) => Ok(()),
            (Self::Rgb | Self::Bgr, PlaneLayout::Planar3 { width, height, plane_dims })
                if plane_dims.iter().all(|&(pw, ph)| pw >= width && ph >= height) =>
            {
                Ok(())
            }
            (Self::Yuv | Self::Unchanged, PlaneLayout::Planar3 { width, height, plane_dims })
                if plane_dims[0].0 >= width
                    && plane_dims[0].1 >= height
                    && plane_dims[1].0 >= width.div_ceil(2)
                    && plane_dims[1].1 >= height.div_ceil(2)
                    && plane_dims[2].0 >= width.div_ceil(2)
                    && plane_dims[2].1 >= height.div_ceil(2) =>
            {
                Ok(())
            }
            (Self::Y, PlaneLayout::SingleY { .. }) => Ok(()),
            (Self::Y, PlaneLayout::Planar3 { width, height, plane_dims }) if plane_dims[0].0 >= width && plane_dims[0].1 >= height => Ok(()),
            _ => Err(Error::InvalidInput("output format or plane dimensions do not match ImagePlanesMut layout")),
        }
    }
}

impl InputFormat {
    pub(crate) fn validate_layout(self, layout: PlaneLayout, width: u32, height: u32) -> Result<()> {
        match (self, layout) {
            (Self::Rgbi | Self::Bgri, PlaneLayout::Interleaved { width: w, height: h, bpp: 3 }) if w == width && h == height => Ok(()),
            (Self::Rgb | Self::Bgr, PlaneLayout::Planar3 { width: w, height: h, plane_dims })
                if w == width && h == height && plane_dims.iter().all(|&(pw, ph)| pw >= width && ph >= height) =>
            {
                Ok(())
            }
            (Self::Yuv, PlaneLayout::Planar3 { width: w, height: h, plane_dims })
                if w == width
                    && h == height
                    && plane_dims[0].0 >= width
                    && plane_dims[0].1 >= height
                    && plane_dims[1].0 >= width.div_ceil(2)
                    && plane_dims[1].1 >= height.div_ceil(2)
                    && plane_dims[2].0 >= width.div_ceil(2)
                    && plane_dims[2].1 >= height.div_ceil(2) =>
            {
                Ok(())
            }
            _ => Err(Error::InvalidInput("input format or dimensions do not match ImagePlanes layout")),
        }
    }
}

/// Read-only device image planes (`nvjpegImage_t`) for encoding.
pub struct ImagePlanes<'a> {
    raw: sys::nvjpegImage_t,
    layout: PlaneLayout,
    _borrow: PhantomData<&'a DeviceBuffer<u8>>,
}

impl<'a> ImagePlanes<'a> {
    /// Single interleaved plane (`channel[0]`), e.g. `RGBI` or `BGRI` (`bpp = 3`).
    pub fn interleaved(buf: &'a DeviceBuffer<u8>, width: u32, height: u32, bpp: u32, pitch: usize) -> Result<Self> {
        let min_row = (width as usize).checked_mul(bpp as usize).ok_or(Error::InvalidInput("row bytes overflow"))?;
        validate_plane(buf.len(), pitch, min_row, height)?;
        let mut raw = sys::nvjpegImage_t { channel: [std::ptr::null_mut(); 4], pitch: [0; 4] };
        raw.channel[0] = buf.cu_deviceptr() as *mut u8;
        raw.pitch[0] = pitch;
        Ok(Self { raw, layout: PlaneLayout::Interleaved { width, height, bpp }, _borrow: PhantomData })
    }

    /// Planar 3-channel image (`channel[0..3]`), e.g. `RGB`, `BGR`, or `YUV`.
    pub fn planar3(planes: [(&'a DeviceBuffer<u8>, usize, u32, u32); 3], width: u32, height: u32) -> Result<Self> {
        let mut raw = sys::nvjpegImage_t { channel: [std::ptr::null_mut(); 4], pitch: [0; 4] };
        let mut plane_dims = [(0u32, 0u32); 3];
        for (i, (buf, pitch, w, h)) in planes.into_iter().enumerate() {
            validate_plane(buf.len(), pitch, w as usize, h)?;
            raw.channel[i] = buf.cu_deviceptr() as *mut u8;
            raw.pitch[i] = pitch;
            plane_dims[i] = (w, h);
        }
        Ok(Self { raw, layout: PlaneLayout::Planar3 { width, height, plane_dims }, _borrow: PhantomData })
    }

    pub(crate) fn layout(&self) -> PlaneLayout {
        self.layout
    }

    pub(crate) fn as_raw(&self) -> &sys::nvjpegImage_t {
        &self.raw
    }
}

/// Mutable device image planes (`nvjpegImage_t`) for decoding.
pub struct ImagePlanesMut<'a> {
    raw: sys::nvjpegImage_t,
    layout: PlaneLayout,
    _borrow: PhantomData<&'a mut DeviceBuffer<u8>>,
}

impl<'a> ImagePlanesMut<'a> {
    /// Single interleaved output plane (`channel[0]`), e.g. `RGBI` or `BGRI`.
    pub fn interleaved(buf: &'a mut DeviceBuffer<u8>, width: u32, height: u32, bpp: u32, pitch: usize) -> Result<Self> {
        let min_row = (width as usize).checked_mul(bpp as usize).ok_or(Error::InvalidInput("row bytes overflow"))?;
        validate_plane(buf.len(), pitch, min_row, height)?;
        let mut raw = sys::nvjpegImage_t { channel: [std::ptr::null_mut(); 4], pitch: [0; 4] };
        raw.channel[0] = buf.cu_deviceptr() as *mut u8;
        raw.pitch[0] = pitch;
        Ok(Self { raw, layout: PlaneLayout::Interleaved { width, height, bpp }, _borrow: PhantomData })
    }

    /// Single luminance output plane (`channel[0]`) for [`OutputFormat::Y`].
    pub fn single_y(buf: &'a mut DeviceBuffer<u8>, width: u32, height: u32, pitch: usize) -> Result<Self> {
        validate_plane(buf.len(), pitch, width as usize, height)?;
        let mut raw = sys::nvjpegImage_t { channel: [std::ptr::null_mut(); 4], pitch: [0; 4] };
        raw.channel[0] = buf.cu_deviceptr() as *mut u8;
        raw.pitch[0] = pitch;
        Ok(Self { raw, layout: PlaneLayout::SingleY { width, height }, _borrow: PhantomData })
    }

    /// Planar 3-channel output image (`channel[0..3]`), e.g. `RGB`, `BGR`, or `YUV`.
    pub fn planar3(planes: [(&'a mut DeviceBuffer<u8>, usize, u32, u32); 3], width: u32, height: u32) -> Result<Self> {
        let mut raw = sys::nvjpegImage_t { channel: [std::ptr::null_mut(); 4], pitch: [0; 4] };
        let mut plane_dims = [(0u32, 0u32); 3];
        for (i, (buf, pitch, w, h)) in planes.into_iter().enumerate() {
            validate_plane(buf.len(), pitch, w as usize, h)?;
            raw.channel[i] = buf.cu_deviceptr() as *mut u8;
            raw.pitch[i] = pitch;
            plane_dims[i] = (w, h);
        }
        Ok(Self { raw, layout: PlaneLayout::Planar3 { width, height, plane_dims }, _borrow: PhantomData })
    }

    pub(crate) fn layout(&self) -> PlaneLayout {
        self.layout
    }

    pub(crate) fn as_mut_raw(&mut self) -> &mut sys::nvjpegImage_t {
        &mut self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_validation_rejects_bad_pitch_and_short_buffer() {
        assert!(validate_plane(64 * 64 * 3, 64 * 3, 64 * 3, 64).is_ok());
        assert!(matches!(validate_plane(64 * 64 * 3, 64 * 3 - 1, 64 * 3, 64), Err(Error::InvalidInput(_))));
        assert!(matches!(validate_plane(64 * 64 * 3 - 1, 64 * 3, 64 * 3, 64), Err(Error::InvalidInput(_))));
        assert!(matches!(validate_plane(1024, 64, 0, 16), Err(Error::InvalidInput(_))));
        assert!(matches!(validate_plane(1024, 64, 64, 0), Err(Error::InvalidInput(_))));
    }

    #[test]
    fn layout_validation_rejects_planar_interleaved_and_undersized_planes() {
        let interleaved = PlaneLayout::Interleaved { width: 64, height: 64, bpp: 3 };
        let planar = PlaneLayout::Planar3 { width: 64, height: 64, plane_dims: [(64, 64); 3] };
        let planar_undersized = PlaneLayout::Planar3 { width: 64, height: 64, plane_dims: [(1, 1); 3] };
        let planar_yuv420 = PlaneLayout::Planar3 { width: 64, height: 64, plane_dims: [(64, 64), (32, 32), (32, 32)] };
        let single_y = PlaneLayout::SingleY { width: 64, height: 64 };

        assert!(OutputFormat::Rgbi.validate_layout(interleaved).is_ok());
        assert!(OutputFormat::Rgb.validate_layout(interleaved).is_err());
        assert!(OutputFormat::Rgb.validate_layout(planar).is_ok());
        assert!(OutputFormat::Rgb.validate_layout(planar_undersized).is_err());
        assert!(OutputFormat::Yuv.validate_layout(planar_yuv420).is_ok());
        assert!(OutputFormat::Rgb.validate_layout(planar_yuv420).is_err());
        assert!(OutputFormat::Rgbi.validate_layout(planar).is_err());
        assert!(OutputFormat::Y.validate_layout(single_y).is_ok());
        assert!(OutputFormat::Rgb.validate_layout(single_y).is_err());

        assert!(InputFormat::Rgbi.validate_layout(interleaved, 64, 64).is_ok());
        assert!(InputFormat::Rgbi.validate_layout(interleaved, 32, 64).is_err());
        assert!(InputFormat::Rgb.validate_layout(interleaved, 64, 64).is_err());
        assert!(InputFormat::Rgb.validate_layout(planar, 64, 64).is_ok());
        assert!(InputFormat::Rgb.validate_layout(planar_undersized, 64, 64).is_err());
        assert!(InputFormat::Yuv.validate_layout(planar_yuv420, 64, 64).is_ok());
        assert!(InputFormat::Yuv.validate_layout(planar_undersized, 64, 64).is_err());
    }

    #[test]
    fn subsampling_and_encoding_reject_unknown_for_encode() {
        assert!(matches!(ChromaSubsampling::Unknown.to_sys(), Err(Error::InvalidInput(_))));
        assert!(matches!(JpegEncoding::Unknown.to_sys(), Err(Error::InvalidInput(_))));
        assert_eq!(NvjpegStatus::SUCCESS.to_string(), "NvjpegStatus(0)");
    }
}
