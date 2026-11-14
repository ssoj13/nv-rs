//! Validated 2D image views, ROI/stride checks, and NPP image operations (color conversion,
//! resize, filtering, thresholding).
//!
//! **What.** [`ImageView`] and [`ImageViewMut`] borrow `cuda_core::DeviceBuffer<T>` with
//! explicit dimensions, channel count, and row byte stride (`step`), validating that `step`
//! covers every row, fits in `i32`, and that any [`Rect`] ROI lies within the image bounds.
//! Neighborhood filters ([`filter_box_8u_c1`], [`filter_gauss_8u_c1`], etc.) require an
//! interior [`ImageView::subview`] with sufficient [`BorderPad`] so out-of-bounds GPU reads
//! are impossible in safe Rust.
//! **Zero-alloc execution.** All wrappers pass the cached `NppStreamContext` from
//! [`crate::NppContext`] directly to `nppicc`, `nppig`, `nppif`, and `nppitc` with zero host
//! allocations.

use crate::{Error, NppContext, Result, check, sys};
use cuda_core::{DeviceBuffer, DeviceCopy};
use std::marker::PhantomData;

/// 2D image dimensions in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    /// Width in pixels (`> 0`).
    pub width: u32,
    /// Height in pixels (`> 0`).
    pub height: u32,
}

impl Size {
    /// Converts to `NppiSize`, rejecting dimensions exceeding `i32::MAX` or equal to zero.
    pub fn to_sys(self) -> Result<sys::NppiSize> {
        if self.width == 0 || self.height == 0 {
            return Err(Error::InvalidInput("image dimensions must be non-zero"));
        }
        let width =
            i32::try_from(self.width).map_err(|_| Error::InvalidInput("width exceeds i32::MAX"))?;
        let height = i32::try_from(self.height)
            .map_err(|_| Error::InvalidInput("height exceeds i32::MAX"))?;
        Ok(sys::NppiSize { width, height })
    }
}

/// 2D region of interest in pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    /// Leftmost pixel coordinate.
    pub x: u32,
    /// Topmost pixel coordinate.
    pub y: u32,
    /// ROI width in pixels (`> 0`).
    pub width: u32,
    /// ROI height in pixels (`> 0`).
    pub height: u32,
}

impl Rect {
    /// Full-image ROI starting at `(0, 0)`.
    pub const fn full(size: Size) -> Self {
        Self {
            x: 0,
            y: 0,
            width: size.width,
            height: size.height,
        }
    }

    /// Validates that this ROI is non-empty and lies within `bounds`.
    pub fn validate_within(self, bounds: Size) -> Result<sys::NppiRect> {
        if self.width == 0 || self.height == 0 {
            return Err(Error::InvalidInput("ROI dimensions must be non-zero"));
        }
        let right = self
            .x
            .checked_add(self.width)
            .ok_or(Error::InvalidInput("ROI x + width overflows"))?;
        let bottom = self
            .y
            .checked_add(self.height)
            .ok_or(Error::InvalidInput("ROI y + height overflows"))?;
        if right > bounds.width || bottom > bounds.height {
            return Err(Error::InvalidInput("ROI exceeds image bounds"));
        }
        Ok(sys::NppiRect {
            x: i32::try_from(self.x).map_err(|_| Error::InvalidInput("ROI x exceeds i32::MAX"))?,
            y: i32::try_from(self.y).map_err(|_| Error::InvalidInput("ROI y exceeds i32::MAX"))?,
            width: i32::try_from(self.width)
                .map_err(|_| Error::InvalidInput("ROI width exceeds i32::MAX"))?,
            height: i32::try_from(self.height)
                .map_err(|_| Error::InvalidInput("ROI height exceeds i32::MAX"))?,
        })
    }
}

/// Interpolation mode for [`resize_8u_c1`], [`resize_8u_c3`], [`resize_8u_c4`], and [`resize_32f_c3`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interpolation {
    /// Nearest-neighbor sampling (`NPPI_INTER_NN`).
    Nearest,
    /// Bilinear interpolation (`NPPI_INTER_LINEAR`).
    Linear,
    /// Bicubic interpolation (`NPPI_INTER_CUBIC`).
    Cubic,
    /// Supersampling (`NPPI_INTER_SUPER`).
    Super,
    /// Lanczos interpolation (`NPPI_INTER_LANCZOS`).
    Lanczos,
}

impl Interpolation {
    fn to_sys(self) -> i32 {
        match self {
            Self::Nearest => sys::NppiInterpolationMode_NPPI_INTER_NN,
            Self::Linear => sys::NppiInterpolationMode_NPPI_INTER_LINEAR,
            Self::Cubic => sys::NppiInterpolationMode_NPPI_INTER_CUBIC,
            Self::Super => sys::NppiInterpolationMode_NPPI_INTER_SUPER,
            Self::Lanczos => sys::NppiInterpolationMode_NPPI_INTER_LANCZOS,
        }
    }
}

/// Fixed Gaussian filter mask size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaskSize {
    /// 3x3 kernel (`NPP_MASK_SIZE_3_X_3`).
    X3x3,
    /// 5x5 kernel (`NPP_MASK_SIZE_5_X_5`).
    X5x5,
    /// 7x7 kernel (`NPP_MASK_SIZE_7_X_7`).
    X7x7,
}

impl MaskSize {
    fn to_sys(self) -> sys::NppiMaskSize {
        match self {
            Self::X3x3 => sys::NppiMaskSize_NPP_MASK_SIZE_3_X_3,
            Self::X5x5 => sys::NppiMaskSize_NPP_MASK_SIZE_5_X_5,
            Self::X7x7 => sys::NppiMaskSize_NPP_MASK_SIZE_7_X_7,
        }
    }
}

/// Pixel comparison operator for thresholding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmpOp {
    /// `pixel < threshold`
    Less,
    /// `pixel <= threshold`
    LessEq,
    /// `pixel == threshold`
    Eq,
    /// `pixel >= threshold`
    GreaterEq,
    /// `pixel > threshold`
    Greater,
}

impl CmpOp {
    fn to_sys(self) -> sys::NppCmpOp {
        match self {
            Self::Less => sys::NppCmpOp_NPP_CMP_LESS,
            Self::LessEq => sys::NppCmpOp_NPP_CMP_LESS_EQ,
            Self::Eq => sys::NppCmpOp_NPP_CMP_EQ,
            Self::GreaterEq => sys::NppCmpOp_NPP_CMP_GREATER_EQ,
            Self::Greater => sys::NppCmpOp_NPP_CMP_GREATER,
        }
    }
}

/// Validates image dimensions, channels, row stride (`step`), and backing buffer byte length.
pub fn validate_image_layout<T>(
    buf_elems: usize,
    size: Size,
    channels: u32,
    step: usize,
) -> Result<i32> {
    let _ = size.to_sys()?;
    if channels == 0 || channels > 4 {
        return Err(Error::InvalidInput("channel count must be 1..=4"));
    }
    let row_bytes = (size.width as usize)
        .checked_mul(channels as usize)
        .and_then(|n| n.checked_mul(size_of::<T>()))
        .ok_or(Error::InvalidInput("row byte size overflows"))?;
    if step < row_bytes {
        return Err(Error::InvalidInput(
            "row stride (step) is smaller than row byte width",
        ));
    }
    let step_i32 =
        i32::try_from(step).map_err(|_| Error::InvalidInput("row stride exceeds i32::MAX"))?;
    let required_bytes = (size.height as usize - 1)
        .checked_mul(step)
        .and_then(|n| n.checked_add(row_bytes))
        .ok_or(Error::InvalidInput("image byte size overflows"))?;
    let buf_bytes = buf_elems
        .checked_mul(size_of::<T>())
        .ok_or(Error::InvalidInput("buffer byte size overflows"))?;
    if buf_bytes < required_bytes {
        return Err(Error::InvalidInput(
            "device buffer is smaller than image layout requires",
        ));
    }
    Ok(step_i32)
}

/// Valid border padding (in pixels) around an [`ImageView`] inside its backing buffer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BorderPad {
    /// Valid pixels to the left of the ROI.
    pub left: u32,
    /// Valid pixels above the ROI.
    pub top: u32,
    /// Valid pixels to the right of the ROI.
    pub right: u32,
    /// Valid pixels below the ROI.
    pub bottom: u32,
}

impl BorderPad {
    /// Checks that the view has at least `(l, t, r, b)` valid border pixels on each side.
    pub fn check_at_least(self, l: u32, t: u32, r: u32, b: u32) -> Result<()> {
        if self.left < l || self.top < t || self.right < r || self.bottom < b {
            return Err(Error::InvalidInput(
                "source ImageView lacks required border padding for neighborhood filter; use subview()",
            ));
        }
        Ok(())
    }
}

/// Borrowed read-only 2D device image view with validated stride, bounds, and border padding.
pub struct ImageView<'a, T: DeviceCopy> {
    ptr: u64,
    step: i32,
    size: Size,
    channels: u32,
    pad: BorderPad,
    _borrow: PhantomData<&'a DeviceBuffer<T>>,
}

impl<'a, T: DeviceCopy> ImageView<'a, T> {
    /// Creates a tightly packed image view (`step = width * channels * size_of::<T>()`).
    pub fn packed(buf: &'a DeviceBuffer<T>, size: Size, channels: u32) -> Result<Self> {
        let step = (size.width as usize)
            .checked_mul(channels as usize)
            .and_then(|n| n.checked_mul(size_of::<T>()))
            .ok_or(Error::InvalidInput("row byte size overflows"))?;
        Self::with_step(buf, size, channels, step)
    }

    /// Creates an image view with explicit byte stride `step` between consecutive rows.
    pub fn with_step(
        buf: &'a DeviceBuffer<T>,
        size: Size,
        channels: u32,
        step: usize,
    ) -> Result<Self> {
        let step_i32 = validate_image_layout::<T>(buf.len(), size, channels, step)?;
        Ok(Self {
            ptr: buf.cu_deviceptr(),
            step: step_i32,
            size,
            channels,
            pad: BorderPad::default(),
            _borrow: PhantomData,
        })
    }

    /// Returns a sub-view covering `rect`, accumulating valid border padding from the parent
    /// view so neighborhood filters ([`filter_box_8u_c1`], [`filter_gauss_8u_c1`], etc.) can
    /// safely read border pixels without out-of-bounds GPU access.
    pub fn subview(&self, rect: Rect) -> Result<Self> {
        let _ = rect.validate_within(self.size)?;
        let byte_off = (rect.y as u64) * (self.step as u64)
            + (rect.x as u64) * (self.channels as u64) * (size_of::<T>() as u64);
        let pad = BorderPad {
            left: self.pad.left + rect.x,
            top: self.pad.top + rect.y,
            right: self.pad.right + (self.size.width - rect.x - rect.width),
            bottom: self.pad.bottom + (self.size.height - rect.y - rect.height),
        };
        Ok(Self {
            ptr: self.ptr + byte_off,
            step: self.step,
            size: Size {
                width: rect.width,
                height: rect.height,
            },
            channels: self.channels,
            pad,
            _borrow: PhantomData,
        })
    }

    /// Image dimensions.
    pub const fn size(&self) -> Size {
        self.size
    }

    /// Row stride in bytes.
    pub const fn step(&self) -> i32 {
        self.step
    }

    /// Number of channels per pixel.
    pub const fn channels(&self) -> u32 {
        self.channels
    }

    /// Valid border padding available around this view.
    pub const fn border_pad(&self) -> BorderPad {
        self.pad
    }

    fn as_ptr(&self) -> *const T {
        self.ptr as *const T
    }
}

/// Borrowed mutable 2D device image view with validated stride and bounds.
pub struct ImageViewMut<'a, T: DeviceCopy> {
    ptr: u64,
    step: i32,
    size: Size,
    channels: u32,
    _borrow: PhantomData<&'a mut DeviceBuffer<T>>,
}

impl<'a, T: DeviceCopy> ImageViewMut<'a, T> {
    /// Creates a tightly packed mutable image view.
    pub fn packed(buf: &'a mut DeviceBuffer<T>, size: Size, channels: u32) -> Result<Self> {
        let step = (size.width as usize)
            .checked_mul(channels as usize)
            .and_then(|n| n.checked_mul(size_of::<T>()))
            .ok_or(Error::InvalidInput("row byte size overflows"))?;
        Self::with_step(buf, size, channels, step)
    }

    /// Creates a mutable image view with explicit byte stride `step`.
    pub fn with_step(
        buf: &'a mut DeviceBuffer<T>,
        size: Size,
        channels: u32,
        step: usize,
    ) -> Result<Self> {
        let step_i32 = validate_image_layout::<T>(buf.len(), size, channels, step)?;
        Ok(Self {
            ptr: buf.cu_deviceptr(),
            step: step_i32,
            size,
            channels,
            _borrow: PhantomData,
        })
    }

    /// Returns a mutable sub-view covering `rect`.
    pub fn subview_mut(&mut self, rect: Rect) -> Result<ImageViewMut<'_, T>> {
        let _ = rect.validate_within(self.size)?;
        let byte_off = (rect.y as u64) * (self.step as u64)
            + (rect.x as u64) * (self.channels as u64) * (size_of::<T>() as u64);
        Ok(ImageViewMut {
            ptr: self.ptr + byte_off,
            step: self.step,
            size: Size {
                width: rect.width,
                height: rect.height,
            },
            channels: self.channels,
            _borrow: PhantomData,
        })
    }

    /// Image dimensions.
    pub const fn size(&self) -> Size {
        self.size
    }

    /// Row stride in bytes.
    pub const fn step(&self) -> i32 {
        self.step
    }

    /// Number of channels per pixel.
    pub const fn channels(&self) -> u32 {
        self.channels
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr as *mut T
    }
}

fn check_pair_roi<S: DeviceCopy, D: DeviceCopy>(
    src: &ImageView<'_, S>,
    dst: &ImageViewMut<'_, D>,
    expected_src_ch: u32,
    expected_dst_ch: u32,
) -> Result<sys::NppiSize> {
    if src.channels != expected_src_ch || dst.channels != expected_dst_ch {
        return Err(Error::InvalidInput("image channel count mismatch"));
    }
    if src.size != dst.size {
        return Err(Error::InvalidInput(
            "source and destination dimensions must match",
        ));
    }
    src.size.to_sys()
}

/// Converts packed 8-bit RGB (`C3`) to 8-bit grayscale (`C1`) using `nppiRGBToGray_8u_C3C1R_Ctx`.
pub fn rgb_to_gray_8u(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    dst: &mut ImageViewMut<'_, u8>,
) -> Result<()> {
    sys::nppicc::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let roi = check_pair_roi(src, dst, 3, 1)?;
    ctx.bind()?;
    // SAFETY: `src` and `dst` are validated device image views with matching ROI and channel counts.
    check("nppiRGBToGray_8u_C3C1R_Ctx", unsafe {
        sys::nppiRGBToGray_8u_C3C1R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            ctx.raw(),
        )
    })
}

/// Converts packed 8-bit RGB (`C3`) to YCbCr (`C3`) using `nppiRGBToYCbCr_8u_C3R_Ctx`.
pub fn rgb_to_ycbcr_8u(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    dst: &mut ImageViewMut<'_, u8>,
) -> Result<()> {
    sys::nppicc::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let roi = check_pair_roi(src, dst, 3, 3)?;
    ctx.bind()?;
    // SAFETY: `src` and `dst` are validated 3-channel device image views with matching ROI.
    check("nppiRGBToYCbCr_8u_C3R_Ctx", unsafe {
        sys::nppiRGBToYCbCr_8u_C3R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            ctx.raw(),
        )
    })
}

/// Converts packed 8-bit YCbCr (`C3`) to RGB (`C3`) using `nppiYCbCrToRGB_8u_C3R_Ctx`.
pub fn ycbcr_to_rgb_8u(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    dst: &mut ImageViewMut<'_, u8>,
) -> Result<()> {
    sys::nppicc::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let roi = check_pair_roi(src, dst, 3, 3)?;
    ctx.bind()?;
    // SAFETY: `src` and `dst` are validated 3-channel device image views with matching ROI.
    check("nppiYCbCrToRGB_8u_C3R_Ctx", unsafe {
        sys::nppiYCbCrToRGB_8u_C3R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            ctx.raw(),
        )
    })
}

fn validate_resize_args<T: DeviceCopy>(
    src: &ImageView<'_, T>,
    src_roi: Rect,
    dst: &ImageViewMut<'_, T>,
    dst_roi: Rect,
    channels: u32,
) -> Result<(sys::NppiSize, sys::NppiRect, sys::NppiSize, sys::NppiRect)> {
    if src.channels != channels || dst.channels != channels {
        return Err(Error::InvalidInput("resize channel count mismatch"));
    }
    let src_sz = src.size.to_sys()?;
    let dst_sz = dst.size.to_sys()?;
    let s_roi = src_roi.validate_within(src.size)?;
    let d_roi = dst_roi.validate_within(dst.size)?;
    Ok((src_sz, s_roi, dst_sz, d_roi))
}

/// Resizes a 1-channel 8-bit image (`nppiResize_8u_C1R_Ctx`).
pub fn resize_8u_c1(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    src_roi: Rect,
    dst: &mut ImageViewMut<'_, u8>,
    dst_roi: Rect,
    interp: Interpolation,
) -> Result<()> {
    sys::nppig::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let (src_sz, s_roi, dst_sz, d_roi) = validate_resize_args(src, src_roi, dst, dst_roi, 1)?;
    ctx.bind()?;
    // SAFETY: source/destination pointers, strides, sizes, and ROIs are validated above.
    check("nppiResize_8u_C1R_Ctx", unsafe {
        sys::nppiResize_8u_C1R_Ctx(
            src.as_ptr(),
            src.step(),
            src_sz,
            s_roi,
            dst.as_mut_ptr(),
            dst.step(),
            dst_sz,
            d_roi,
            interp.to_sys(),
            ctx.raw(),
        )
    })
}

/// Resizes a 3-channel 8-bit image (`nppiResize_8u_C3R_Ctx`).
pub fn resize_8u_c3(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    src_roi: Rect,
    dst: &mut ImageViewMut<'_, u8>,
    dst_roi: Rect,
    interp: Interpolation,
) -> Result<()> {
    sys::nppig::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let (src_sz, s_roi, dst_sz, d_roi) = validate_resize_args(src, src_roi, dst, dst_roi, 3)?;
    ctx.bind()?;
    // SAFETY: source/destination pointers, strides, sizes, and ROIs are validated above.
    check("nppiResize_8u_C3R_Ctx", unsafe {
        sys::nppiResize_8u_C3R_Ctx(
            src.as_ptr(),
            src.step(),
            src_sz,
            s_roi,
            dst.as_mut_ptr(),
            dst.step(),
            dst_sz,
            d_roi,
            interp.to_sys(),
            ctx.raw(),
        )
    })
}

/// Resizes a 4-channel 8-bit image (`nppiResize_8u_C4R_Ctx`).
pub fn resize_8u_c4(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    src_roi: Rect,
    dst: &mut ImageViewMut<'_, u8>,
    dst_roi: Rect,
    interp: Interpolation,
) -> Result<()> {
    sys::nppig::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let (src_sz, s_roi, dst_sz, d_roi) = validate_resize_args(src, src_roi, dst, dst_roi, 4)?;
    ctx.bind()?;
    // SAFETY: source/destination pointers, strides, sizes, and ROIs are validated above.
    check("nppiResize_8u_C4R_Ctx", unsafe {
        sys::nppiResize_8u_C4R_Ctx(
            src.as_ptr(),
            src.step(),
            src_sz,
            s_roi,
            dst.as_mut_ptr(),
            dst.step(),
            dst_sz,
            d_roi,
            interp.to_sys(),
            ctx.raw(),
        )
    })
}

/// Resizes a 3-channel 32-bit float image (`nppiResize_32f_C3R_Ctx`).
pub fn resize_32f_c3(
    ctx: &NppContext,
    src: &ImageView<'_, f32>,
    src_roi: Rect,
    dst: &mut ImageViewMut<'_, f32>,
    dst_roi: Rect,
    interp: Interpolation,
) -> Result<()> {
    sys::nppig::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let (src_sz, s_roi, dst_sz, d_roi) = validate_resize_args(src, src_roi, dst, dst_roi, 3)?;
    ctx.bind()?;
    // SAFETY: source/destination pointers, strides, sizes, and ROIs are validated above.
    check("nppiResize_32f_C3R_Ctx", unsafe {
        sys::nppiResize_32f_C3R_Ctx(
            src.as_ptr(),
            src.step(),
            src_sz,
            s_roi,
            dst.as_mut_ptr(),
            dst.step(),
            dst_sz,
            d_roi,
            interp.to_sys(),
            ctx.raw(),
        )
    })
}

fn validate_box_args<T: DeviceCopy>(
    src: &ImageView<'_, T>,
    dst: &ImageViewMut<'_, T>,
    channels: u32,
    mask: Size,
    anchor: (i32, i32),
) -> Result<(sys::NppiSize, sys::NppiSize, sys::NppiPoint)> {
    let roi = check_pair_roi(src, dst, channels, channels)?;
    let mask_sz = mask.to_sys()?;
    if anchor.0 < 0 || anchor.1 < 0 || anchor.0 >= mask_sz.width || anchor.1 >= mask_sz.height {
        return Err(Error::InvalidInput("filter anchor lies outside mask"));
    }
    let pad_l = anchor.0 as u32;
    let pad_t = anchor.1 as u32;
    let pad_r = (mask_sz.width - 1 - anchor.0) as u32;
    let pad_b = (mask_sz.height - 1 - anchor.1) as u32;
    src.border_pad()
        .check_at_least(pad_l, pad_t, pad_r, pad_b)?;
    Ok((
        roi,
        mask_sz,
        sys::NppiPoint {
            x: anchor.0,
            y: anchor.1,
        },
    ))
}

fn validate_gauss_args<T: DeviceCopy>(
    src: &ImageView<'_, T>,
    dst: &ImageViewMut<'_, T>,
    channels: u32,
    mask: MaskSize,
) -> Result<sys::NppiSize> {
    let roi = check_pair_roi(src, dst, channels, channels)?;
    let r = match mask {
        MaskSize::X3x3 => 1,
        MaskSize::X5x5 => 2,
        MaskSize::X7x7 => 3,
    };
    src.border_pad().check_at_least(r, r, r, r)?;
    Ok(roi)
}

/// Applies a box filter to an interior ROI of a 1-channel 8-bit image (`nppiFilterBox_8u_C1R_Ctx`).
///
/// `src` must be an interior [`ImageView::subview`] with enough border padding for `mask` / `anchor`.
pub fn filter_box_8u_c1(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    dst: &mut ImageViewMut<'_, u8>,
    mask: Size,
    anchor: (i32, i32),
) -> Result<()> {
    sys::nppif::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let (roi, mask_sz, pt) = validate_box_args(src, dst, 1, mask, anchor)?;
    ctx.bind()?;
    // SAFETY: validated device views, border padding, mask size, and anchor within mask.
    check("nppiFilterBox_8u_C1R_Ctx", unsafe {
        sys::nppiFilterBox_8u_C1R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            mask_sz,
            pt,
            ctx.raw(),
        )
    })
}

/// Applies a box filter to an interior ROI of a 3-channel 8-bit image (`nppiFilterBox_8u_C3R_Ctx`).
pub fn filter_box_8u_c3(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    dst: &mut ImageViewMut<'_, u8>,
    mask: Size,
    anchor: (i32, i32),
) -> Result<()> {
    sys::nppif::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let (roi, mask_sz, pt) = validate_box_args(src, dst, 3, mask, anchor)?;
    ctx.bind()?;
    // SAFETY: validated 3-channel device views, border padding, mask size, and anchor within mask.
    check("nppiFilterBox_8u_C3R_Ctx", unsafe {
        sys::nppiFilterBox_8u_C3R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            mask_sz,
            pt,
            ctx.raw(),
        )
    })
}

/// Applies a Gaussian filter to an interior ROI of a 1-channel 8-bit image (`nppiFilterGauss_8u_C1R_Ctx`).
pub fn filter_gauss_8u_c1(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    dst: &mut ImageViewMut<'_, u8>,
    mask: MaskSize,
) -> Result<()> {
    sys::nppif::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let roi = validate_gauss_args(src, dst, 1, mask)?;
    ctx.bind()?;
    // SAFETY: validated 1-channel device views, border padding, and fixed enum mask size.
    check("nppiFilterGauss_8u_C1R_Ctx", unsafe {
        sys::nppiFilterGauss_8u_C1R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            mask.to_sys(),
            ctx.raw(),
        )
    })
}

/// Applies a Gaussian filter to an interior ROI of a 3-channel 8-bit image (`nppiFilterGauss_8u_C3R_Ctx`).
pub fn filter_gauss_8u_c3(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    dst: &mut ImageViewMut<'_, u8>,
    mask: MaskSize,
) -> Result<()> {
    sys::nppif::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let roi = validate_gauss_args(src, dst, 3, mask)?;
    ctx.bind()?;
    // SAFETY: validated 3-channel device views, border padding, and fixed enum mask size.
    check("nppiFilterGauss_8u_C3R_Ctx", unsafe {
        sys::nppiFilterGauss_8u_C3R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            mask.to_sys(),
            ctx.raw(),
        )
    })
}

fn validate_threshold_op(op: CmpOp) -> Result<sys::NppCmpOp> {
    match op {
        CmpOp::Less | CmpOp::Greater => Ok(op.to_sys()),
        CmpOp::LessEq | CmpOp::Eq | CmpOp::GreaterEq => Err(Error::InvalidInput(
            "nppiThreshold_Val only supports CmpOp::Less and CmpOp::Greater",
        )),
    }
}

/// Thresholds a 1-channel 8-bit image (`nppiThreshold_Val_8u_C1R_Ctx`): pixels where
/// `src OP threshold` is true are set to `value`, otherwise copied from `src`.
///
/// NPP only supports [`CmpOp::Less`] and [`CmpOp::Greater`] for `nppiThreshold_Val`.
pub fn threshold_val_8u_c1(
    ctx: &NppContext,
    src: &ImageView<'_, u8>,
    dst: &mut ImageViewMut<'_, u8>,
    threshold: u8,
    value: u8,
    op: CmpOp,
) -> Result<()> {
    sys::nppitc::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let roi = check_pair_roi(src, dst, 1, 1)?;
    let cmp = validate_threshold_op(op)?;
    ctx.bind()?;
    // SAFETY: validated 1-channel `u8` device views and supported comparison operator.
    check("nppiThreshold_Val_8u_C1R_Ctx", unsafe {
        sys::nppiThreshold_Val_8u_C1R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            threshold,
            value,
            cmp,
            ctx.raw(),
        )
    })
}

/// Thresholds a 1-channel 32-bit float image (`nppiThreshold_Val_32f_C1R_Ctx`).
///
/// NPP only supports [`CmpOp::Less`] and [`CmpOp::Greater`] for `nppiThreshold_Val`.
pub fn threshold_val_32f_c1(
    ctx: &NppContext,
    src: &ImageView<'_, f32>,
    dst: &mut ImageViewMut<'_, f32>,
    threshold: f32,
    value: f32,
    op: CmpOp,
) -> Result<()> {
    sys::nppitc::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let roi = check_pair_roi(src, dst, 1, 1)?;
    let cmp = validate_threshold_op(op)?;
    ctx.bind()?;
    // SAFETY: validated 1-channel `f32` device views and supported comparison operator.
    check("nppiThreshold_Val_32f_C1R_Ctx", unsafe {
        sys::nppiThreshold_Val_32f_C1R_Ctx(
            src.as_ptr(),
            src.step(),
            dst.as_mut_ptr(),
            dst.step(),
            roi,
            threshold,
            value,
            cmp,
            ctx.raw(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_and_rect_validation_rejects_invalid_bounds() {
        assert!(matches!(
            Size {
                width: 0,
                height: 10
            }
            .to_sys(),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            Size {
                width: u32::MAX,
                height: 1
            }
            .to_sys(),
            Err(Error::InvalidInput(_))
        ));

        let bounds = Size {
            width: 64,
            height: 64,
        };
        assert!(Rect::full(bounds).validate_within(bounds).is_ok());
        assert!(matches!(
            Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 10
            }
            .validate_within(bounds),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            Rect {
                x: 32,
                y: 0,
                width: 33,
                height: 10
            }
            .validate_within(bounds),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            Rect {
                x: 0,
                y: 60,
                width: 10,
                height: 5
            }
            .validate_within(bounds),
            Err(Error::InvalidInput(_))
        ));
    }

    #[test]
    fn image_layout_validation_checks_stride_and_buffer_len() {
        let size = Size {
            width: 16,
            height: 8,
        };
        // 16 * 3 = 48 bytes per row; 8 rows * 48 = 384 bytes.
        assert_eq!(
            validate_image_layout::<u8>(384, size, 3, 48).expect("exact packed"),
            48
        );
        assert!(matches!(
            validate_image_layout::<u8>(384, size, 3, 47),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_image_layout::<u8>(383, size, 3, 48),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_image_layout::<u8>(1024, size, 0, 64),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_image_layout::<u8>(1024, size, 5, 128),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_image_layout::<u8>(usize::MAX, size, 3, (i32::MAX as usize) + 1),
            Err(Error::InvalidInput(_))
        ));
    }

    #[test]
    fn border_pad_validation_enforces_neighborhood_margins() {
        let pad = BorderPad {
            left: 1,
            top: 1,
            right: 1,
            bottom: 1,
        };
        assert!(pad.check_at_least(1, 1, 1, 1).is_ok());
        assert!(matches!(
            pad.check_at_least(2, 1, 1, 1),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            BorderPad::default().check_at_least(1, 1, 1, 1),
            Err(Error::InvalidInput(_))
        ));
    }

    #[test]
    fn threshold_op_validation_rejects_unsupported_modes() {
        assert!(validate_threshold_op(CmpOp::Less).is_ok());
        assert!(validate_threshold_op(CmpOp::Greater).is_ok());
        assert!(matches!(
            validate_threshold_op(CmpOp::LessEq),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_threshold_op(CmpOp::Eq),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_threshold_op(CmpOp::GreaterEq),
            Err(Error::InvalidInput(_))
        ));
    }
}
