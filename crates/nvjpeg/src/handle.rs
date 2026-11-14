//! [`Handle`], [`JpegState`], [`JpegStream`], [`PinnedBuffer`], and [`NvjpegDeviceBuffer`].
//!
//! **Ownership.** `Handle` holds `Arc<cuda_core::Context>` and destroys `nvjpegHandle_t` on
//! `Drop`, recording any teardown error on `Context::record_err`. All child handles hold
//! `Arc<Handle>` so the library handle always outlives them.

use crate::{
    Backend, ChromaSubsampling, Error, ExifOrientation, ImageInfo, ImagePlanesMut, JpegEncoding, OutputFormat, Result, check, ensure_loaded, sys,
};
use cuda_core::{Context, DriverError, Stream};
use std::ffi::c_int;
use std::sync::{Arc, Mutex};

/// Top-level nvJPEG library handle (`nvjpegHandle_t`).
pub struct Handle {
    raw: sys::nvjpegHandle_t,
    cuda: Arc<Context>,
}

// SAFETY: `nvjpegHandle_t` is thread-safe per NVIDIA docs when each thread uses its own state.
unsafe impl Send for Handle {}
// SAFETY: as above.
unsafe impl Sync for Handle {}

impl Handle {
    /// Creates a handle with the default backend and allocators (`nvjpegCreateSimple`).
    pub fn new(cuda: &Arc<Context>) -> Result<Arc<Self>> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::nvjpegHandle_t = std::ptr::null_mut();
        // SAFETY: `cuda` is bound and `raw` is a valid out-pointer.
        check("nvjpegCreateSimple", unsafe { sys::nvjpegCreateSimple(&mut raw) })?;
        Ok(Arc::new(Self { raw, cuda: cuda.clone() }))
    }

    /// Creates a handle with an explicit [`Backend`] and `flags` (`nvjpegCreateEx`).
    pub fn with_backend(cuda: &Arc<Context>, backend: Backend, flags: u32) -> Result<Arc<Self>> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::nvjpegHandle_t = std::ptr::null_mut();
        // SAFETY: `cuda` is bound; null allocators select nvJPEG's default CUDA allocators.
        check("nvjpegCreateEx", unsafe {
            sys::nvjpegCreateEx(backend.to_sys(), std::ptr::null_mut(), std::ptr::null_mut(), flags, &mut raw)
        })?;
        Ok(Arc::new(Self { raw, cuda: cuda.clone() }))
    }

    /// Returns the raw `nvjpegHandle_t`.
    pub fn raw(&self) -> sys::nvjpegHandle_t {
        self.raw
    }

    /// Returns the owning CUDA context.
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// Parses image dimensions, component count, and chroma subsampling without decoding
    /// (`nvjpegGetImageInfo`).
    pub fn image_info(&self, data: &[u8]) -> Result<ImageInfo> {
        if data.is_empty() {
            return Err(Error::InvalidInput("JPEG bitstream is empty"));
        }
        let mut n_comp: c_int = 0;
        let mut subsampling: sys::nvjpegChromaSubsampling_t = 0;
        let mut widths = [0 as c_int; 4];
        let mut heights = [0 as c_int; 4];
        // SAFETY: `self.raw` is live; `data` is valid for `data.len()` bytes; output arrays have 4 elements.
        check("nvjpegGetImageInfo", unsafe {
            sys::nvjpegGetImageInfo(
                self.raw,
                data.as_ptr(),
                data.len(),
                &mut n_comp,
                &mut subsampling,
                widths.as_mut_ptr(),
                heights.as_mut_ptr(),
            )
        })?;
        Ok(ImageInfo {
            components: usize::try_from(n_comp).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative component count"))?,
            subsampling: ChromaSubsampling::from_sys(subsampling),
            widths: [
                u32::try_from(widths[0]).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative width"))?,
                u32::try_from(widths[1]).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative width"))?,
                u32::try_from(widths[2]).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative width"))?,
                u32::try_from(widths[3]).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative width"))?,
            ],
            heights: [
                u32::try_from(heights[0]).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative height"))?,
                u32::try_from(heights[1]).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative height"))?,
                u32::try_from(heights[2]).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative height"))?,
                u32::try_from(heights[3]).map_err(|_| Error::InvalidInput("nvjpegGetImageInfo returned negative height"))?,
            ],
        })
    }

    /// Records a teardown error on the CUDA context during `Drop`.
    pub(crate) fn record_drop_err(&self, code: sys::nvjpegStatus_t) {
        if code != sys::nvjpegStatus_t_NVJPEG_STATUS_SUCCESS {
            self.cuda.record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN)));
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `nvjpegCreate*` and is destroyed once here.
            let code = unsafe { sys::nvjpegDestroy(self.raw) };
            self.record_drop_err(code);
        }
    }
}

/// Reusable pinned host buffer (`nvjpegBufferPinned_t`).
pub struct PinnedBuffer {
    raw: Mutex<sys::nvjpegBufferPinned_t>,
    handle: Arc<Handle>,
}

// SAFETY: `PinnedBuffer` owns its `nvjpegBufferPinned_t` behind a `Mutex`.
unsafe impl Send for PinnedBuffer {}
// SAFETY: `Mutex` serializes access to `nvjpegBufferPinned_t`.
unsafe impl Sync for PinnedBuffer {}

impl PinnedBuffer {
    /// Allocates a new pinned buffer handle (`nvjpegBufferPinnedCreate`).
    pub fn new(handle: &Arc<Handle>) -> Result<Arc<Self>> {
        let mut raw: sys::nvjpegBufferPinned_t = std::ptr::null_mut();
        // SAFETY: `handle` is live; null allocator uses default pinned allocator.
        check("nvjpegBufferPinnedCreate", unsafe { sys::nvjpegBufferPinnedCreate(handle.raw(), std::ptr::null_mut(), &mut raw) })?;
        Ok(Arc::new(Self { raw: Mutex::new(raw), handle: handle.clone() }))
    }

    /// Pre-allocates or resizes the pinned buffer to `size` bytes (`nvjpegBufferPinnedResize`).
    pub fn resize(&self, size: usize, stream: &Stream) -> Result<()> {
        self.handle.cuda().bind_to_thread()?;
        let raw = self.raw.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: `raw` and `stream.cu_stream()` are valid on the bound context.
        check("nvjpegBufferPinnedResize", unsafe { sys::nvjpegBufferPinnedResize(*raw, size, stream.cu_stream()) })
    }

    pub(crate) fn raw(&self) -> sys::nvjpegBufferPinned_t {
        *self.raw.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl Drop for PinnedBuffer {
    fn drop(&mut self) {
        let raw = *self.raw.get_mut().unwrap_or_else(|e| e.into_inner());
        if !raw.is_null() {
            // SAFETY: `raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegBufferPinnedDestroy(raw) };
            self.handle.record_drop_err(code);
        }
    }
}

/// Reusable device buffer (`nvjpegBufferDevice_t`).
pub struct NvjpegDeviceBuffer {
    raw: Mutex<sys::nvjpegBufferDevice_t>,
    handle: Arc<Handle>,
}

// SAFETY: `NvjpegDeviceBuffer` owns its `nvjpegBufferDevice_t` behind a `Mutex`.
unsafe impl Send for NvjpegDeviceBuffer {}
// SAFETY: `Mutex` serializes access to `nvjpegBufferDevice_t`.
unsafe impl Sync for NvjpegDeviceBuffer {}

impl NvjpegDeviceBuffer {
    /// Allocates a new device buffer handle (`nvjpegBufferDeviceCreate`).
    pub fn new(handle: &Arc<Handle>) -> Result<Arc<Self>> {
        let mut raw: sys::nvjpegBufferDevice_t = std::ptr::null_mut();
        // SAFETY: `handle` is live; null allocator uses default device allocator.
        check("nvjpegBufferDeviceCreate", unsafe { sys::nvjpegBufferDeviceCreate(handle.raw(), std::ptr::null_mut(), &mut raw) })?;
        Ok(Arc::new(Self { raw: Mutex::new(raw), handle: handle.clone() }))
    }

    /// Pre-allocates or resizes the device buffer to `size` bytes (`nvjpegBufferDeviceResize`).
    pub fn resize(&self, size: usize, stream: &Stream) -> Result<()> {
        self.handle.cuda().bind_to_thread()?;
        let raw = self.raw.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: `raw` and `stream.cu_stream()` are valid on the bound context.
        check("nvjpegBufferDeviceResize", unsafe { sys::nvjpegBufferDeviceResize(*raw, size, stream.cu_stream()) })
    }

    pub(crate) fn raw(&self) -> sys::nvjpegBufferDevice_t {
        *self.raw.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl Drop for NvjpegDeviceBuffer {
    fn drop(&mut self) {
        let raw = *self.raw.get_mut().unwrap_or_else(|e| e.into_inner());
        if !raw.is_null() {
            // SAFETY: `raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegBufferDeviceDestroy(raw) };
            self.handle.record_drop_err(code);
        }
    }
}

/// JPEG decode state (`nvjpegJpegState_t`).
pub struct JpegState {
    raw: sys::nvjpegJpegState_t,
    handle: Arc<Handle>,
    _pinned: Option<Arc<PinnedBuffer>>,
    _device: Option<Arc<NvjpegDeviceBuffer>>,
}

// SAFETY: `nvjpegJpegState_t` is owned exclusively; `&mut self` serializes decode calls.
unsafe impl Send for JpegState {}

impl JpegState {
    /// Creates a new decode state (`nvjpegJpegStateCreate`).
    pub fn new(handle: &Arc<Handle>) -> Result<Self> {
        let mut raw: sys::nvjpegJpegState_t = std::ptr::null_mut();
        // SAFETY: `handle` is live and `raw` is a valid out-pointer.
        check("nvjpegJpegStateCreate", unsafe { sys::nvjpegJpegStateCreate(handle.raw(), &mut raw) })?;
        Ok(Self { raw, handle: handle.clone(), _pinned: None, _device: None })
    }

    /// Attaches a reusable pinned buffer (`nvjpegStateAttachPinnedBuffer`) and retains an `Arc`
    /// so the buffer cannot be dropped while attached.
    pub fn attach_pinned(&mut self, buf: &Arc<PinnedBuffer>) -> Result<()> {
        // SAFETY: both `self.raw` and `buf.raw()` belong to live `nvjpegHandle_t`.
        check("nvjpegStateAttachPinnedBuffer", unsafe { sys::nvjpegStateAttachPinnedBuffer(self.raw, buf.raw()) })?;
        self._pinned = Some(buf.clone());
        Ok(())
    }

    /// Attaches a reusable device buffer (`nvjpegStateAttachDeviceBuffer`) and retains an `Arc`
    /// so the buffer cannot be dropped while attached.
    pub fn attach_device(&mut self, buf: &Arc<NvjpegDeviceBuffer>) -> Result<()> {
        // SAFETY: both `self.raw` and `buf.raw()` belong to live `nvjpegHandle_t`.
        check("nvjpegStateAttachDeviceBuffer", unsafe { sys::nvjpegStateAttachDeviceBuffer(self.raw, buf.raw()) })?;
        self._device = Some(buf.clone());
        Ok(())
    }

    /// Decodes a single JPEG bitstream into `dst` on `stream` (`nvjpegDecode`).
    pub fn decode(&mut self, data: &[u8], format: OutputFormat, dst: &mut ImagePlanesMut<'_>, stream: &Stream) -> Result<()> {
        if data.is_empty() {
            return Err(Error::InvalidInput("JPEG bitstream is empty"));
        }
        format.validate_layout(dst.layout())?;
        self.handle.cuda().bind_to_thread()?;
        // SAFETY: `data` is valid for `data.len()` bytes, `dst` has validated device plane pointers
        // matching `format`, and `stream` is valid on the bound context.
        check("nvjpegDecode", unsafe {
            sys::nvjpegDecode(self.handle.raw(), self.raw, data.as_ptr(), data.len(), format.to_sys(), dst.as_mut_raw(), stream.cu_stream())
        })
    }
}

impl Drop for JpegState {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegJpegStateDestroy(self.raw) };
            self.handle.record_drop_err(code);
        }
    }
}

/// Host-side parsed JPEG stream (`nvjpegJpegStream_t`) for metadata and EXIF inspection.
pub struct JpegStream {
    raw: sys::nvjpegJpegStream_t,
    handle: Arc<Handle>,
}

// SAFETY: `nvjpegJpegStream_t` is host-side state owned exclusively by `JpegStream`.
unsafe impl Send for JpegStream {}

impl JpegStream {
    /// Creates a new stream parser (`nvjpegJpegStreamCreate`).
    pub fn new(handle: &Arc<Handle>) -> Result<Self> {
        let mut raw: sys::nvjpegJpegStream_t = std::ptr::null_mut();
        // SAFETY: `handle` is live and `raw` is a valid out-pointer.
        check("nvjpegJpegStreamCreate", unsafe { sys::nvjpegJpegStreamCreate(handle.raw(), &mut raw) })?;
        Ok(Self { raw, handle: handle.clone() })
    }

    /// Parses a JPEG bitstream (`nvjpegJpegStreamParse`).
    pub fn parse(&mut self, data: &[u8], save_metadata: bool, save_stream: bool) -> Result<()> {
        if data.is_empty() {
            return Err(Error::InvalidInput("JPEG bitstream is empty"));
        }
        // SAFETY: `self.handle` and `self.raw` are live and `data` is valid for `data.len()` bytes.
        check("nvjpegJpegStreamParse", unsafe {
            sys::nvjpegJpegStreamParse(self.handle.raw(), data.as_ptr(), data.len(), c_int::from(save_metadata), c_int::from(save_stream), self.raw)
        })
    }

    /// Parses only the JPEG header (`nvjpegJpegStreamParseHeader`).
    pub fn parse_header(&mut self, data: &[u8]) -> Result<()> {
        if data.is_empty() {
            return Err(Error::InvalidInput("JPEG bitstream is empty"));
        }
        // SAFETY: `self.handle` and `self.raw` are live and `data` is valid for `data.len()` bytes.
        check("nvjpegJpegStreamParseHeader", unsafe {
            sys::nvjpegJpegStreamParseHeader(self.handle.raw(), data.as_ptr(), data.len(), self.raw)
        })
    }

    /// Frame `(width, height)` (`nvjpegJpegStreamGetFrameDimensions`).
    pub fn dimensions(&self) -> Result<(u32, u32)> {
        let (mut w, mut h) = (0u32, 0u32);
        // SAFETY: `self.raw` is parsed and `w`, `h` are valid out-pointers.
        check("nvjpegJpegStreamGetFrameDimensions", unsafe { sys::nvjpegJpegStreamGetFrameDimensions(self.raw, &mut w, &mut h) })?;
        Ok((w, h))
    }

    /// Number of color components (`nvjpegJpegStreamGetComponentsNum`).
    pub fn components_num(&self) -> Result<u32> {
        let mut n = 0u32;
        // SAFETY: `self.raw` is parsed and `n` is a valid out-pointer.
        check("nvjpegJpegStreamGetComponentsNum", unsafe { sys::nvjpegJpegStreamGetComponentsNum(self.raw, &mut n) })?;
        Ok(n)
    }

    /// Chroma subsampling (`nvjpegJpegStreamGetChromaSubsampling`).
    pub fn chroma_subsampling(&self) -> Result<ChromaSubsampling> {
        let mut css: sys::nvjpegChromaSubsampling_t = 0;
        // SAFETY: `self.raw` is parsed and `css` is a valid out-pointer.
        check("nvjpegJpegStreamGetChromaSubsampling", unsafe { sys::nvjpegJpegStreamGetChromaSubsampling(self.raw, &mut css) })?;
        Ok(ChromaSubsampling::from_sys(css))
    }

    /// EXIF orientation (`nvjpegJpegStreamGetExifOrientation`).
    pub fn exif_orientation(&self) -> Result<ExifOrientation> {
        let mut ori: sys::nvjpegExifOrientation_t = 0;
        // SAFETY: `self.raw` is parsed and `ori` is a valid out-pointer.
        check("nvjpegJpegStreamGetExifOrientation", unsafe { sys::nvjpegJpegStreamGetExifOrientation(self.raw, &mut ori) })?;
        Ok(ExifOrientation::from_sys(ori))
    }

    /// Sample bit precision (`nvjpegJpegStreamGetSamplePrecision`).
    pub fn sample_precision(&self) -> Result<u32> {
        let mut prec = 0u32;
        // SAFETY: `self.raw` is parsed and `prec` is a valid out-pointer.
        check("nvjpegJpegStreamGetSamplePrecision", unsafe { sys::nvjpegJpegStreamGetSamplePrecision(self.raw, &mut prec) })?;
        Ok(prec)
    }

    /// JPEG SOF encoding type (`nvjpegJpegStreamGetJpegEncoding`).
    pub fn jpeg_encoding(&self) -> Result<JpegEncoding> {
        let mut enc: sys::nvjpegJpegEncoding_t = 0;
        // SAFETY: `self.raw` is parsed and `enc` is a valid out-pointer.
        check("nvjpegJpegStreamGetJpegEncoding", unsafe { sys::nvjpegJpegStreamGetJpegEncoding(self.raw, &mut enc) })?;
        Ok(JpegEncoding::from_sys(enc))
    }
}

impl Drop for JpegStream {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegJpegStreamDestroy(self.raw) };
            self.handle.record_drop_err(code);
        }
    }
}
