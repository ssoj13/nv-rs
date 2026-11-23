//! [`Handle`], [`JpegState`], [`JpegStream`], [`PinnedBuffer`], and [`NvjpegDeviceBuffer`].
//!
//! **Ownership.** `Handle` holds `Arc<cuda_core::Context>` and destroys `nvjpegHandle_t` on
//! `Drop`, recording any teardown error on `Context::record_err`. All child handles hold
//! `Arc<Handle>` so the library handle always outlives them. Every constructor and `Drop` binds
//! the handle's CUDA context first.
//!
//! **Validation.** `decode` / `decode_jpeg` check the output planes against the geometry parsed
//! from the bitstream. `JpegStream::parse` always lets nvJPEG copy the bitstream, and pinned /
//! device scratch buffers attach to one `JpegState` at a time.

use crate::{
    Backend, ChromaSubsampling, DecodeGeometry, Error, ExifOrientation, ImageInfo, ImagePlanesMut,
    JpegEncoding, OutputFormat, Result, check, ensure_loaded, sys,
};
use cuda_core::{Context, DriverError, Stream};
use std::ffi::c_int;
use std::sync::atomic::{AtomicBool, Ordering};
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
        check("nvjpegCreateSimple", unsafe {
            sys::nvjpegCreateSimple(&mut raw)
        })?;
        Ok(Arc::new(Self {
            raw,
            cuda: cuda.clone(),
        }))
    }

    /// Creates a handle with an explicit [`Backend`] and `flags` (`nvjpegCreateEx`).
    pub fn with_backend(cuda: &Arc<Context>, backend: Backend, flags: u32) -> Result<Arc<Self>> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::nvjpegHandle_t = std::ptr::null_mut();
        // SAFETY: `cuda` is bound; null allocators select nvJPEG's default CUDA allocators.
        check("nvjpegCreateEx", unsafe {
            sys::nvjpegCreateEx(
                backend.to_sys(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                flags,
                &mut raw,
            )
        })?;
        Ok(Arc::new(Self {
            raw,
            cuda: cuda.clone(),
        }))
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
            components: usize::try_from(n_comp).map_err(|_| {
                Error::InvalidInput("nvjpegGetImageInfo returned negative component count")
            })?,
            subsampling: ChromaSubsampling::from_sys(subsampling),
            widths: [
                u32::try_from(widths[0]).map_err(|_| {
                    Error::InvalidInput("nvjpegGetImageInfo returned negative width")
                })?,
                u32::try_from(widths[1]).map_err(|_| {
                    Error::InvalidInput("nvjpegGetImageInfo returned negative width")
                })?,
                u32::try_from(widths[2]).map_err(|_| {
                    Error::InvalidInput("nvjpegGetImageInfo returned negative width")
                })?,
                u32::try_from(widths[3]).map_err(|_| {
                    Error::InvalidInput("nvjpegGetImageInfo returned negative width")
                })?,
            ],
            heights: [
                u32::try_from(heights[0]).map_err(|_| {
                    Error::InvalidInput("nvjpegGetImageInfo returned negative height")
                })?,
                u32::try_from(heights[1]).map_err(|_| {
                    Error::InvalidInput("nvjpegGetImageInfo returned negative height")
                })?,
                u32::try_from(heights[2]).map_err(|_| {
                    Error::InvalidInput("nvjpegGetImageInfo returned negative height")
                })?,
                u32::try_from(heights[3]).map_err(|_| {
                    Error::InvalidInput("nvjpegGetImageInfo returned negative height")
                })?,
            ],
        })
    }

    /// Binds the owning CUDA context before a `Drop` teardown, because nvJPEG destroy calls
    /// free CUDA memory and the dropping thread may have no (or another) current context.
    ///
    /// A bind failure is recorded on the context; the destroy call still runs so that one sticky
    /// error (which makes `bind_to_thread` fail) does not leak every later handle.
    pub(crate) fn bind_for_drop(&self) {
        if let Err(e) = self.cuda.bind_to_thread() {
            self.cuda.record_err(Err::<(), _>(e));
        }
    }

    /// Records a teardown error on the CUDA context during `Drop`.
    pub(crate) fn record_drop_err(&self, code: sys::nvjpegStatus_t) {
        if code != sys::nvjpegStatus_t_NVJPEG_STATUS_SUCCESS {
            self.cuda.record_err(Err::<(), _>(DriverError(
                cuda_core::sys::cudaError_enum_CUDA_ERROR_UNKNOWN,
            )));
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            self.bind_for_drop();
            // SAFETY: `self.raw` was created by `nvjpegCreate*` and is destroyed once here.
            let code = unsafe { sys::nvjpegDestroy(self.raw) };
            self.record_drop_err(code);
        }
    }
}

/// Claims exclusive attachment of a scratch buffer to one [`JpegState`].
///
/// nvJPEG lists its pinned and device buffers among the objects to "instantiate separately for
/// each thread", so one buffer must never back two states that may decode concurrently. The
/// claim is taken under the buffer's `Mutex`, the same lock `resize` holds, so a resize can never
/// overlap a decode through an attached state.
fn claim_attachment<T>(raw: &Mutex<T>, attached: &AtomicBool, what: &'static str) -> Result<()> {
    let _guard = raw.lock().unwrap_or_else(|e| e.into_inner());
    attached
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .map(|_| ())
        .map_err(|_| Error::InvalidInput(what))
}

/// Reusable pinned host buffer (`nvjpegBufferPinned_t`).
///
/// A buffer can be attached to one [`JpegState`] at a time; it is released when that state is
/// dropped or attaches another buffer. Resize it before attaching.
pub struct PinnedBuffer {
    raw: Mutex<sys::nvjpegBufferPinned_t>,
    attached: AtomicBool,
    handle: Arc<Handle>,
}

// SAFETY: `PinnedBuffer` owns its `nvjpegBufferPinned_t` behind a `Mutex`.
unsafe impl Send for PinnedBuffer {}
// SAFETY: `resize` runs under the `Mutex` and is refused while the buffer is attached; attachment
// is exclusive (`claim_attachment`), so nvJPEG never uses the buffer from two states or threads.
unsafe impl Sync for PinnedBuffer {}

impl PinnedBuffer {
    /// Allocates a new pinned buffer handle (`nvjpegBufferPinnedCreate`).
    pub fn new(handle: &Arc<Handle>) -> Result<Arc<Self>> {
        handle.cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegBufferPinned_t = std::ptr::null_mut();
        // SAFETY: `handle` is live on the bound context; null allocator uses default pinned allocator.
        check("nvjpegBufferPinnedCreate", unsafe {
            sys::nvjpegBufferPinnedCreate(handle.raw(), std::ptr::null_mut(), &mut raw)
        })?;
        Ok(Arc::new(Self {
            raw: Mutex::new(raw),
            attached: AtomicBool::new(false),
            handle: handle.clone(),
        }))
    }

    /// Pre-allocates or resizes the pinned buffer to `size` bytes (`nvjpegBufferPinnedResize`).
    ///
    /// Fails with [`Error::InvalidInput`] while the buffer is attached to a [`JpegState`].
    pub fn resize(&self, size: usize, stream: &Stream) -> Result<()> {
        self.handle.cuda().bind_to_thread()?;
        let raw = self.raw.lock().unwrap_or_else(|e| e.into_inner());
        if self.attached.load(Ordering::Acquire) {
            return Err(Error::InvalidInput(
                "resize a PinnedBuffer before attaching it to a JpegState",
            ));
        }
        // SAFETY: `raw` and `stream.cu_stream()` are valid on the bound context, and no state
        // uses the buffer (not attached, and attaching waits for this lock).
        check("nvjpegBufferPinnedResize", unsafe {
            sys::nvjpegBufferPinnedResize(*raw, size, stream.cu_stream())
        })
    }

    pub(crate) fn raw(&self) -> sys::nvjpegBufferPinned_t {
        *self.raw.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn claim(&self) -> Result<()> {
        claim_attachment(
            &self.raw,
            &self.attached,
            "PinnedBuffer is already attached to another JpegState",
        )
    }

    fn release(&self) {
        self.attached.store(false, Ordering::Release);
    }
}

impl Drop for PinnedBuffer {
    fn drop(&mut self) {
        let raw = *self.raw.get_mut().unwrap_or_else(|e| e.into_inner());
        if !raw.is_null() {
            self.handle.bind_for_drop();
            // SAFETY: `raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegBufferPinnedDestroy(raw) };
            self.handle.record_drop_err(code);
        }
    }
}

/// Reusable device buffer (`nvjpegBufferDevice_t`).
///
/// A buffer can be attached to one [`JpegState`] at a time; it is released when that state is
/// dropped or attaches another buffer. Resize it before attaching.
pub struct NvjpegDeviceBuffer {
    raw: Mutex<sys::nvjpegBufferDevice_t>,
    attached: AtomicBool,
    handle: Arc<Handle>,
}

// SAFETY: `NvjpegDeviceBuffer` owns its `nvjpegBufferDevice_t` behind a `Mutex`.
unsafe impl Send for NvjpegDeviceBuffer {}
// SAFETY: `resize` runs under the `Mutex` and is refused while the buffer is attached; attachment
// is exclusive (`claim_attachment`), so nvJPEG never uses the buffer from two states or threads.
unsafe impl Sync for NvjpegDeviceBuffer {}

impl NvjpegDeviceBuffer {
    /// Allocates a new device buffer handle (`nvjpegBufferDeviceCreate`).
    pub fn new(handle: &Arc<Handle>) -> Result<Arc<Self>> {
        handle.cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegBufferDevice_t = std::ptr::null_mut();
        // SAFETY: `handle` is live on the bound context; null allocator uses default device allocator.
        check("nvjpegBufferDeviceCreate", unsafe {
            sys::nvjpegBufferDeviceCreate(handle.raw(), std::ptr::null_mut(), &mut raw)
        })?;
        Ok(Arc::new(Self {
            raw: Mutex::new(raw),
            attached: AtomicBool::new(false),
            handle: handle.clone(),
        }))
    }

    /// Pre-allocates or resizes the device buffer to `size` bytes (`nvjpegBufferDeviceResize`).
    ///
    /// Fails with [`Error::InvalidInput`] while the buffer is attached to a [`JpegState`].
    pub fn resize(&self, size: usize, stream: &Stream) -> Result<()> {
        self.handle.cuda().bind_to_thread()?;
        let raw = self.raw.lock().unwrap_or_else(|e| e.into_inner());
        if self.attached.load(Ordering::Acquire) {
            return Err(Error::InvalidInput(
                "resize an NvjpegDeviceBuffer before attaching it to a JpegState",
            ));
        }
        // SAFETY: `raw` and `stream.cu_stream()` are valid on the bound context, and no state
        // uses the buffer (not attached, and attaching waits for this lock).
        check("nvjpegBufferDeviceResize", unsafe {
            sys::nvjpegBufferDeviceResize(*raw, size, stream.cu_stream())
        })
    }

    pub(crate) fn raw(&self) -> sys::nvjpegBufferDevice_t {
        *self.raw.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn claim(&self) -> Result<()> {
        claim_attachment(
            &self.raw,
            &self.attached,
            "NvjpegDeviceBuffer is already attached to another JpegState",
        )
    }

    fn release(&self) {
        self.attached.store(false, Ordering::Release);
    }
}

impl Drop for NvjpegDeviceBuffer {
    fn drop(&mut self) {
        let raw = *self.raw.get_mut().unwrap_or_else(|e| e.into_inner());
        if !raw.is_null() {
            self.handle.bind_for_drop();
            // SAFETY: `raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegBufferDeviceDestroy(raw) };
            self.handle.record_drop_err(code);
        }
    }
}

/// Decoupled JPEG decoder implementation (`nvjpegJpegDecoder_t`) for API v2 decoding.
pub struct JpegDecoder {
    raw: sys::nvjpegJpegDecoder_t,
    handle: Arc<Handle>,
}

// SAFETY: `nvjpegJpegDecoder_t` is thread-safe once created for a live `nvjpegHandle_t`.
unsafe impl Send for JpegDecoder {}
// SAFETY: as above.
unsafe impl Sync for JpegDecoder {}

impl JpegDecoder {
    /// Creates a decoupled decoder for `backend` (`nvjpegDecoderCreate`).
    pub fn new(handle: &Arc<Handle>, backend: Backend) -> Result<Arc<Self>> {
        handle.cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegJpegDecoder_t = std::ptr::null_mut();
        // SAFETY: `handle` is live on the bound CUDA context and `raw` is a valid out-pointer.
        check("nvjpegDecoderCreate", unsafe {
            sys::nvjpegDecoderCreate(handle.raw(), backend.to_sys(), &mut raw)
        })?;
        Ok(Arc::new(Self {
            raw,
            handle: handle.clone(),
        }))
    }

    pub(crate) fn raw(&self) -> sys::nvjpegJpegDecoder_t {
        self.raw
    }

    pub(crate) fn handle(&self) -> &Arc<Handle> {
        &self.handle
    }
}

impl Drop for JpegDecoder {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            self.handle.bind_for_drop();
            // SAFETY: `self.raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegDecoderDestroy(self.raw) };
            self.handle.record_drop_err(code);
        }
    }
}

/// Decode parameters (`nvjpegDecodeParams_t`) for decoupled API v2 decoding.
pub struct DecodeParams {
    raw: sys::nvjpegDecodeParams_t,
    handle: Arc<Handle>,
    output_format: Option<OutputFormat>,
}

// SAFETY: `nvjpegDecodeParams_t` is owned exclusively; `&mut self` serializes mutation.
unsafe impl Send for DecodeParams {}

impl DecodeParams {
    /// Creates decode parameters (`nvjpegDecodeParamsCreate`).
    pub fn new(handle: &Arc<Handle>) -> Result<Self> {
        handle.cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegDecodeParams_t = std::ptr::null_mut();
        // SAFETY: `handle` is live on the bound context and `raw` is a valid out-pointer.
        check("nvjpegDecodeParamsCreate", unsafe {
            sys::nvjpegDecodeParamsCreate(handle.raw(), &mut raw)
        })?;
        Ok(Self {
            raw,
            handle: handle.clone(),
            output_format: None,
        })
    }

    /// Sets the target output pixel format (`nvjpegDecodeParamsSetOutputFormat`).
    pub fn set_output_format(&mut self, format: OutputFormat) -> Result<()> {
        // SAFETY: `self.raw` is live and `format.to_sys()` is a valid `nvjpegOutputFormat_t`.
        check("nvjpegDecodeParamsSetOutputFormat", unsafe {
            sys::nvjpegDecodeParamsSetOutputFormat(self.raw, format.to_sys())
        })?;
        self.output_format = Some(format);
        Ok(())
    }

    pub(crate) fn raw(&self) -> sys::nvjpegDecodeParams_t {
        self.raw
    }
}

impl Drop for DecodeParams {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            self.handle.bind_for_drop();
            // SAFETY: `self.raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegDecodeParamsDestroy(self.raw) };
            self.handle.record_drop_err(code);
        }
    }
}

/// JPEG decode state (`nvjpegJpegState_t`).
pub struct JpegState {
    raw: sys::nvjpegJpegState_t,
    handle: Arc<Handle>,
    decoder: Option<Arc<JpegDecoder>>,
    _pinned: Option<Arc<PinnedBuffer>>,
    _device: Option<Arc<NvjpegDeviceBuffer>>,
}

// SAFETY: `nvjpegJpegState_t` is owned exclusively; `&mut self` serializes decode calls.
unsafe impl Send for JpegState {}

impl JpegState {
    /// Creates a new legacy v1 decode state (`nvjpegJpegStateCreate`) for [`Self::decode`].
    pub fn new(handle: &Arc<Handle>) -> Result<Self> {
        handle.cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegJpegState_t = std::ptr::null_mut();
        // SAFETY: `handle` is live on the bound context and `raw` is a valid out-pointer.
        check("nvjpegJpegStateCreate", unsafe {
            sys::nvjpegJpegStateCreate(handle.raw(), &mut raw)
        })?;
        Ok(Self {
            raw,
            handle: handle.clone(),
            decoder: None,
            _pinned: None,
            _device: None,
        })
    }

    /// Creates a decoupled v2 decoder state (`nvjpegDecoderStateCreate`) that supports
    /// [`Self::attach_pinned`], [`Self::attach_device`], and [`Self::decode_jpeg`].
    pub fn with_decoder(decoder: &Arc<JpegDecoder>) -> Result<Self> {
        decoder.handle().cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegJpegState_t = std::ptr::null_mut();
        // SAFETY: `decoder` and its owning `handle` are live on the bound context and `raw` is a
        // valid out-pointer.
        check("nvjpegDecoderStateCreate", unsafe {
            sys::nvjpegDecoderStateCreate(decoder.handle().raw(), decoder.raw(), &mut raw)
        })?;
        Ok(Self {
            raw,
            handle: decoder.handle().clone(),
            decoder: Some(decoder.clone()),
            _pinned: None,
            _device: None,
        })
    }

    /// Attaches a reusable pinned buffer (`nvjpegStateAttachPinnedBuffer`) to a decoder-bound
    /// state and retains an `Arc` so the buffer cannot be dropped while attached.
    ///
    /// The buffer is attached exclusively: attaching one already attached to another live
    /// `JpegState` fails with [`Error::InvalidInput`]. A previously attached buffer is released.
    pub fn attach_pinned(&mut self, buf: &Arc<PinnedBuffer>) -> Result<()> {
        if self.decoder.is_none() {
            return Err(Error::InvalidInput(
                "attach_pinned requires a JpegState created with JpegState::with_decoder",
            ));
        }
        if self._pinned.as_ref().is_some_and(|p| Arc::ptr_eq(p, buf)) {
            return Ok(());
        }
        buf.claim()?;
        // SAFETY: both `self.raw` (created via `nvjpegDecoderStateCreate`) and `buf.raw()` belong
        // to a live `nvjpegHandle_t`, and `buf` is now attached to this state only.
        let status = unsafe { sys::nvjpegStateAttachPinnedBuffer(self.raw, buf.raw()) };
        if let Err(e) = check("nvjpegStateAttachPinnedBuffer", status) {
            buf.release();
            return Err(e);
        }
        if let Some(old) = self._pinned.replace(buf.clone()) {
            old.release();
        }
        Ok(())
    }

    /// Attaches a reusable device buffer (`nvjpegStateAttachDeviceBuffer`) to a decoder-bound
    /// state and retains an `Arc` so the buffer cannot be dropped while attached.
    ///
    /// The buffer is attached exclusively: attaching one already attached to another live
    /// `JpegState` fails with [`Error::InvalidInput`]. A previously attached buffer is released.
    pub fn attach_device(&mut self, buf: &Arc<NvjpegDeviceBuffer>) -> Result<()> {
        if self.decoder.is_none() {
            return Err(Error::InvalidInput(
                "attach_device requires a JpegState created with JpegState::with_decoder",
            ));
        }
        if self._device.as_ref().is_some_and(|d| Arc::ptr_eq(d, buf)) {
            return Ok(());
        }
        buf.claim()?;
        // SAFETY: both `self.raw` (created via `nvjpegDecoderStateCreate`) and `buf.raw()` belong
        // to a live `nvjpegHandle_t`, and `buf` is now attached to this state only.
        let status = unsafe { sys::nvjpegStateAttachDeviceBuffer(self.raw, buf.raw()) };
        if let Err(e) = check("nvjpegStateAttachDeviceBuffer", status) {
            buf.release();
            return Err(e);
        }
        if let Some(old) = self._device.replace(buf.clone()) {
            old.release();
        }
        Ok(())
    }

    /// Decodes a single JPEG bitstream into `dst` on `stream` (`nvjpegDecode`).
    pub fn decode(
        &mut self,
        data: &[u8],
        format: OutputFormat,
        dst: &mut ImagePlanesMut<'_>,
        stream: &Stream,
    ) -> Result<()> {
        if self.decoder.is_some() {
            return Err(Error::InvalidInput(
                "use decode_jpeg for a decoder-bound JpegState",
            ));
        }
        if data.is_empty() {
            return Err(Error::InvalidInput("JPEG bitstream is empty"));
        }
        let geom = DecodeGeometry::from_info(&self.handle.image_info(data)?)?;
        format.validate_layout(dst.layout(), &geom)?;
        self.handle.cuda().bind_to_thread()?;
        // SAFETY: `data` is valid for `data.len()` bytes, `dst` has device planes that cover the
        // image `data` decodes to in `format` (checked against its parsed geometry), and
        // `stream` is valid on the bound context.
        check("nvjpegDecode", unsafe {
            sys::nvjpegDecode(
                self.handle.raw(),
                self.raw,
                data.as_ptr(),
                data.len(),
                format.to_sys(),
                dst.as_mut_raw(),
                stream.cu_stream(),
            )
        })
    }

    /// Decodes a parsed [`JpegStream`] using the decoupled v2 decoder (`nvjpegDecodeJpeg`).
    pub fn decode_jpeg(
        &mut self,
        jpeg_stream: &JpegStream,
        params: &DecodeParams,
        dst: &mut ImagePlanesMut<'_>,
        stream: &Stream,
    ) -> Result<()> {
        let decoder = self.decoder.as_ref().ok_or(Error::InvalidInput(
            "decode_jpeg requires a JpegState created with JpegState::with_decoder",
        ))?;
        let format = params
            .output_format
            .ok_or(Error::InvalidInput("DecodeParams output format not set"))?;
        if !jpeg_stream.decodable {
            return Err(Error::InvalidInput(
                "decode_jpeg requires a JpegStream filled by a successful JpegStream::parse",
            ));
        }
        format.validate_layout(dst.layout(), &jpeg_stream.geometry()?)?;
        self.handle.cuda().bind_to_thread()?;
        // SAFETY: `self.raw` is a decoder-bound state, `jpeg_stream` holds its own copy of a fully
        // parsed bitstream, `params` has the checked output format, `dst` has device planes that
        // cover the stream's image in that format, and `stream` is bound.
        check("nvjpegDecodeJpeg", unsafe {
            sys::nvjpegDecodeJpeg(
                self.handle.raw(),
                decoder.raw(),
                self.raw,
                jpeg_stream.raw(),
                dst.as_mut_raw(),
                params.raw(),
                stream.cu_stream(),
            )
        })
    }
}

impl Drop for JpegState {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            self.handle.bind_for_drop();
            // SAFETY: `self.raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegJpegStateDestroy(self.raw) };
            self.handle.record_drop_err(code);
        }
        if let Some(p) = &self._pinned {
            p.release();
        }
        if let Some(d) = &self._device {
            d.release();
        }
    }
}

/// Host-side parsed JPEG stream (`nvjpegJpegStream_t`) for metadata and EXIF inspection.
pub struct JpegStream {
    raw: sys::nvjpegJpegStream_t,
    handle: Arc<Handle>,
    /// `true` only after a successful [`Self::parse`], which copies the whole bitstream into
    /// nvJPEG's internal storage; a header-only parse is never decoded.
    pub(crate) decodable: bool,
}

// SAFETY: `nvjpegJpegStream_t` is host-side state owned exclusively by `JpegStream`.
unsafe impl Send for JpegStream {}

impl JpegStream {
    /// Creates a new stream parser (`nvjpegJpegStreamCreate`).
    pub fn new(handle: &Arc<Handle>) -> Result<Self> {
        handle.cuda().bind_to_thread()?;
        let mut raw: sys::nvjpegJpegStream_t = std::ptr::null_mut();
        // SAFETY: `handle` is live on the bound context and `raw` is a valid out-pointer.
        check("nvjpegJpegStreamCreate", unsafe {
            sys::nvjpegJpegStreamCreate(handle.raw(), &mut raw)
        })?;
        Ok(Self {
            raw,
            handle: handle.clone(),
            decodable: false,
        })
    }

    /// Parses a JPEG bitstream (`nvjpegJpegStreamParse`).
    ///
    /// `save_stream` must be `true`. nvJPEG documents that with `save_stream == 0` the stream
    /// "will just save the pointers" to `data` and reads them again during decode, which would
    /// let a later [`JpegState::decode_jpeg`] read `data` after this borrow ended. `false` is
    /// therefore rejected with [`Error::InvalidInput`].
    pub fn parse(&mut self, data: &[u8], save_metadata: bool, save_stream: bool) -> Result<()> {
        self.decodable = false;
        if data.is_empty() {
            return Err(Error::InvalidInput("JPEG bitstream is empty"));
        }
        if !save_stream {
            return Err(Error::InvalidInput(
                "JpegStream::parse requires save_stream = true: nvJPEG would keep a pointer to the borrowed bitstream",
            ));
        }
        // SAFETY: `self.handle` and `self.raw` are live, `data` is valid for `data.len()` bytes,
        // and `save_stream = 1` makes nvJPEG copy the bitstream instead of retaining `data`.
        check("nvjpegJpegStreamParse", unsafe {
            sys::nvjpegJpegStreamParse(
                self.handle.raw(),
                data.as_ptr(),
                data.len(),
                c_int::from(save_metadata),
                1,
                self.raw,
            )
        })?;
        self.decodable = true;
        Ok(())
    }

    /// Parses only the JPEG header (`nvjpegJpegStreamParseHeader`).
    ///
    /// The stream then answers metadata queries but cannot be passed to
    /// [`JpegState::decode_jpeg`] until [`Self::parse`] succeeds.
    pub fn parse_header(&mut self, data: &[u8]) -> Result<()> {
        self.decodable = false;
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
        check("nvjpegJpegStreamGetFrameDimensions", unsafe {
            sys::nvjpegJpegStreamGetFrameDimensions(self.raw, &mut w, &mut h)
        })?;
        Ok((w, h))
    }

    /// Number of color components (`nvjpegJpegStreamGetComponentsNum`).
    pub fn components_num(&self) -> Result<u32> {
        let mut n = 0u32;
        // SAFETY: `self.raw` is parsed and `n` is a valid out-pointer.
        check("nvjpegJpegStreamGetComponentsNum", unsafe {
            sys::nvjpegJpegStreamGetComponentsNum(self.raw, &mut n)
        })?;
        Ok(n)
    }

    /// Chroma subsampling (`nvjpegJpegStreamGetChromaSubsampling`).
    pub fn chroma_subsampling(&self) -> Result<ChromaSubsampling> {
        let mut css: sys::nvjpegChromaSubsampling_t = 0;
        // SAFETY: `self.raw` is parsed and `css` is a valid out-pointer.
        check("nvjpegJpegStreamGetChromaSubsampling", unsafe {
            sys::nvjpegJpegStreamGetChromaSubsampling(self.raw, &mut css)
        })?;
        Ok(ChromaSubsampling::from_sys(css))
    }

    /// EXIF orientation (`nvjpegJpegStreamGetExifOrientation`).
    pub fn exif_orientation(&self) -> Result<ExifOrientation> {
        let mut ori: sys::nvjpegExifOrientation_t = 0;
        // SAFETY: `self.raw` is parsed and `ori` is a valid out-pointer.
        check("nvjpegJpegStreamGetExifOrientation", unsafe {
            sys::nvjpegJpegStreamGetExifOrientation(self.raw, &mut ori)
        })?;
        Ok(ExifOrientation::from_sys(ori))
    }

    /// Sample bit precision (`nvjpegJpegStreamGetSamplePrecision`).
    pub fn sample_precision(&self) -> Result<u32> {
        let mut prec = 0u32;
        // SAFETY: `self.raw` is parsed and `prec` is a valid out-pointer.
        check("nvjpegJpegStreamGetSamplePrecision", unsafe {
            sys::nvjpegJpegStreamGetSamplePrecision(self.raw, &mut prec)
        })?;
        Ok(prec)
    }

    /// JPEG SOF encoding type (`nvjpegJpegStreamGetJpegEncoding`).
    pub fn jpeg_encoding(&self) -> Result<JpegEncoding> {
        let mut enc: sys::nvjpegJpegEncoding_t = 0;
        // SAFETY: `self.raw` is parsed and `enc` is a valid out-pointer.
        check("nvjpegJpegStreamGetJpegEncoding", unsafe {
            sys::nvjpegJpegStreamGetJpegEncoding(self.raw, &mut enc)
        })?;
        Ok(JpegEncoding::from_sys(enc))
    }

    /// Per-component dimensions of the parsed frame
    /// (`nvjpegJpegStreamGetComponentsNum` + `nvjpegJpegStreamGetComponentDimensions`).
    pub(crate) fn geometry(&self) -> Result<DecodeGeometry> {
        let components = usize::try_from(self.components_num()?)
            .map_err(|_| Error::InvalidInput("JPEG component count exceeds usize"))?;
        let mut dims = [(0u32, 0u32); 4];
        for (component, d) in (0u32..).zip(dims.iter_mut()).take(components) {
            let (mut w, mut h) = (0u32, 0u32);
            // SAFETY: `self.raw` is parsed, `component < components_num`, and `w`, `h` are valid
            // out-pointers.
            check("nvjpegJpegStreamGetComponentDimensions", unsafe {
                sys::nvjpegJpegStreamGetComponentDimensions(self.raw, component, &mut w, &mut h)
            })?;
            *d = (w, h);
        }
        DecodeGeometry::new(components, dims)
    }

    pub(crate) fn raw(&self) -> sys::nvjpegJpegStream_t {
        self.raw
    }
}

impl Drop for JpegStream {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            self.handle.bind_for_drop();
            // SAFETY: `self.raw` is owned by `self` and `self.handle` is still alive.
            let code = unsafe { sys::nvjpegJpegStreamDestroy(self.raw) };
            self.handle.record_drop_err(code);
        }
    }
}
