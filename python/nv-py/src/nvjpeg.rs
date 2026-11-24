//! `nv_rs.nvjpeg` — GPU-accelerated JPEG decoding, encoding, and header inspection.
//!
//! **What.** Exposes [`PyNvjpegDecoder`] (`NvjpegDecoder`) and [`PyNvjpegEncoder`]
//! (`NvjpegEncoder`) to Python under `nv_rs.nvjpeg`, operating on [`PyDeviceBuffer`]
//! (interleaved RGB/BGR or single-plane Y) and Python buffer-protocol inputs (`bytes`,
//! `bytearray`, `memoryview`, `numpy.ndarray`) with the GIL released via `py.detach`.
//!
//! **How.** A decode writes a new nv_rs-owned buffer whose producer stream is the decode stream,
//! under a write lease (`buf_util::enqueue_leased`), so a consumer on another stream either waits
//! (DLPack / CAI export) or is rejected until the decode completes. An encode reads its
//! `DeviceBuffer` input (owned or DLPack import, validated by `buf_util::check_buf`) under a read
//! lease. The decoder's single `JpegState` is reused across per-call streams only after the
//! previous stream has drained, because nvJPEG keeps per-decode buffers in the state.
//!
//! **Used by** `nv_rs` Python extension module (`python/nv-py/src/lib.rs`).

use std::sync::{Arc, Mutex};
use std::time::Instant;

use cuda_core::{DeviceBuffer as CudaDeviceBuffer, Stream};
use nv_loader::diag::{self, Level};
use nvjpeg::{
    Backend, ChromaSubsampling, EncoderParams, EncoderState, Handle, ImagePlanes, ImagePlanesMut,
    InputFormat, JpegState, OutputFormat,
};
use pyo3::buffer::PyBuffer;
use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::buf_util::{
    Access, check_buf, close_slot, closed as closed_err, drain_stream, enqueue_leased, is_closed,
    open, poisoned, typed_view,
};
use crate::core::{PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

py_err_mapper!(map_nvjpeg_err, "nvjpeg", nvjpeg::Error, {
    nvjpeg::Error::Unavailable(_) => ErrorKind::LibraryNotFound,
    nvjpeg::Error::InvalidInput(_) => ErrorKind::InvalidArgument,
    nvjpeg::Error::Status { .. } | nvjpeg::Error::Driver(_) => ErrorKind::Cuda,
});

py_parse_enum!(
    parse_backend,
    "nvjpeg",
    "parse_backend",
    Backend,
    "nvJPEG backend (expected default, hybrid, gpu_hybrid, or hardware)",
    {
        "default" => Backend::Default,
        "hybrid" => Backend::Hybrid,
        "gpu_hybrid" | "gpuhybrid" => Backend::GpuHybrid,
        "hardware" => Backend::Hardware,
    }
);

py_parse_enum!(
    parse_output_format,
    "nvjpeg",
    "parse_output_format",
    (OutputFormat, usize),
    "output format (expected rgbi, bgri, or y)",
    {
        "rgbi" => (OutputFormat::Rgbi, 3),
        "bgri" => (OutputFormat::Bgri, 3),
        "y" | "gray" => (OutputFormat::Y, 1),
    }
);

py_parse_enum!(
    parse_input_format,
    "nvjpeg",
    "parse_input_format",
    (InputFormat, usize),
    "input format (expected rgbi or bgri)",
    {
        "rgbi" => (InputFormat::Rgbi, 3),
        "bgri" => (InputFormat::Bgri, 3),
    }
);

py_parse_enum!(
    parse_subsampling,
    "nvjpeg",
    "parse_subsampling",
    ChromaSubsampling,
    "chroma subsampling (expected 444, 422, 420, 440, 411, 410, gray, or 410v)",
    {
        "444" | "yuv444" => ChromaSubsampling::Yuv444,
        "422" | "yuv422" => ChromaSubsampling::Yuv422,
        "420" | "yuv420" => ChromaSubsampling::Yuv420,
        "440" | "yuv440" => ChromaSubsampling::Yuv440,
        "411" | "yuv411" => ChromaSubsampling::Yuv411,
        "410" | "yuv410" => ChromaSubsampling::Yuv410,
        "gray" => ChromaSubsampling::Gray,
        "410v" | "yuv410v" => ChromaSubsampling::Yuv410v,
    }
);

fn subsampling_to_str(css: ChromaSubsampling) -> &'static str {
    match css {
        ChromaSubsampling::Yuv444 => "444",
        ChromaSubsampling::Yuv422 => "422",
        ChromaSubsampling::Yuv420 => "420",
        ChromaSubsampling::Yuv440 => "440",
        ChromaSubsampling::Yuv411 => "411",
        ChromaSubsampling::Yuv410 => "410",
        ChromaSubsampling::Gray => "gray",
        ChromaSubsampling::Yuv410v => "410v",
        ChromaSubsampling::Unknown => "unknown",
    }
}

fn copy_py_bytes(py: Python<'_>, data: &Bound<'_, PyAny>, op: &str) -> PyResult<Vec<u8>> {
    let view = PyBuffer::<u8>::get(data)
        .map_err(|e| record_and_err("nvjpeg", op, e, ErrorKind::InvalidArgument))?;
    let nbytes = view.len_bytes();
    let mut host = vec![0u8; nbytes];
    view.copy_to_slice(py, &mut host)
        .map_err(|e| record_and_err("nvjpeg", op, e, ErrorKind::InvalidArgument))?;
    Ok(host)
}

/// Resolves the image size: both `width` and `height` given, or both inferred from an
/// `(H, W, channels)` `shape` attribute.
///
/// Giving only one of them is rejected (it would mix an explicit and an inferred dimension). An
/// input without a `shape` attribute needs both; any other error reading `shape` propagates.
fn resolve_image_dims(
    image: &Bound<'_, PyAny>,
    width: Option<u32>,
    height: Option<u32>,
    expected_channels: usize,
    op: &str,
) -> PyResult<(u32, u32)> {
    let invalid = |msg: String| record_and_err("nvjpeg", op, msg, ErrorKind::InvalidArgument);
    let (w, h) = match (width, height) {
        (Some(w), Some(h)) => (w, h),
        (None, None) => {
            let shape = match image.getattr("shape") {
                Ok(shape) => shape.extract::<Vec<i64>>().map_err(|e| {
                    invalid(format!("cannot read the input's shape as integers: {e}"))
                })?,
                Err(e) if e.is_instance_of::<PyAttributeError>(image.py()) => {
                    return Err(invalid(
                        "width and height are required for an input without a shape".to_owned(),
                    ));
                }
                Err(e) => return Err(e),
            };
            match shape.as_slice() {
                [h, w, c] if *c == expected_channels as i64 => {
                    let dim = |v: i64, what: &str| {
                        u32::try_from(v)
                            .map_err(|_| invalid(format!("{what} {v} out of range for u32")))
                    };
                    (dim(*w, "width")?, dim(*h, "height")?)
                }
                _ => {
                    return Err(invalid(format!(
                        "input shape {shape:?} is not (H, W, {expected_channels}); pass width and height"
                    )));
                }
            }
        }
        _ => {
            return Err(invalid(
                "pass both width and height, or neither to infer them from the shape".to_owned(),
            ));
        }
    };
    if w == 0 || h == 0 {
        return Err(invalid("width and height must be > 0".to_owned()));
    }
    Ok((w, h))
}

/// Decode state plus the stream its last decode was queued on.
///
/// The library handle lives here (declared last, so it drops after `state`) so that `close()`
/// releases it together with the decode state.
struct DecoderState {
    state: JpegState,
    /// nvJPEG keeps per-decode buffers in `state`; a decode on another stream waits for this one
    /// to drain first so it cannot overwrite them under queued work.
    last_stream: Arc<Stream>,
    handle: Arc<Handle>,
}

/// GPU-accelerated JPEG decoder bound to a CUDA context and default stream.
#[pyclass(name = "NvjpegDecoder", module = "nv_rs.nvjpeg")]
pub struct PyNvjpegDecoder {
    state: Arc<Mutex<Option<DecoderState>>>,
    stream: PyStream,
}

#[pymethods]
impl PyNvjpegDecoder {
    /// Creates a new `NvjpegDecoder` on `stream`'s CUDA context, GIL-free.
    #[new]
    #[pyo3(signature = (stream, backend="default"))]
    fn new(py: Python<'_>, stream: &PyStream, backend: &str) -> PyResult<Self> {
        let parsed_backend = parse_backend(backend)?;
        let s = stream.inner.clone();
        let (handle, state) = py
            .detach(move || -> nvjpeg::Result<(Arc<Handle>, JpegState)> {
                let ctx = s.context();
                let handle = if parsed_backend == Backend::Default {
                    Handle::new(ctx)
                } else {
                    Handle::with_backend(ctx, parsed_backend, 0)
                }?;
                let state = JpegState::new(&handle)?;
                Ok((handle, state))
            })
            .map_err(|e| map_nvjpeg_err("NvjpegDecoder::new", e))?;

        Ok(Self {
            state: Arc::new(Mutex::new(Some(DecoderState {
                state,
                last_stream: stream.inner.clone(),
                handle,
            }))),
            stream: stream.clone(),
        })
    }

    /// Parses JPEG header metadata without decoding: returns `(width, height, components, subsampling)`.
    fn decode_info(
        &self,
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
    ) -> PyResult<(u32, u32, usize, &'static str)> {
        let host = copy_py_bytes(py, data, "decode_info")?;
        let state_arc = self.state.clone();
        let t0 = Instant::now();
        let info = py.detach(move || -> PyResult<_> {
            let handle = {
                let mut slot = state_arc
                    .lock()
                    .map_err(|_| poisoned("nvjpeg", "decode_info"))?;
                open(&mut slot, "nvjpeg", "decode_info")?.handle.clone()
            };
            handle
                .image_info(&host)
                .map_err(|e| map_nvjpeg_err("decode_info", e))
        })?;

        let w = info.widths[0];
        let h = info.heights[0];
        let css = subsampling_to_str(info.subsampling);
        diag::record_event(
            Level::Info,
            "nvjpeg",
            "decode_info",
            format!("{w}x{h} comps={} css={css}", info.components),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok((w, h, info.components, css))
    }

    /// Decodes JPEG bytes into a GPU `DeviceBuffer` (interleaved RGB/BGR or Y), GIL-free.
    ///
    /// The result's producer stream is the decode stream (`stream`, or the decoder's own).
    #[pyo3(signature = (data, stream=None, output_format="rgbi"))]
    fn decode(
        &self,
        py: Python<'_>,
        data: &Bound<'_, PyAny>,
        stream: Option<&PyStream>,
        output_format: &str,
    ) -> PyResult<PyDeviceBuffer> {
        let (fmt, bpp) = parse_output_format(output_format)?;
        let host = copy_py_bytes(py, data, "decode")?;
        let s = stream.unwrap_or(&self.stream).inner.clone();
        let state_arc = self.state.clone();
        let t0 = Instant::now();

        let (out, w, h) = py.detach(move || -> PyResult<(PyDeviceBuffer, u32, u32)> {
            let map = |e| map_nvjpeg_err("decode", e);
            let invalid =
                |msg: &str| record_and_err("nvjpeg", "decode", msg, ErrorKind::InvalidArgument);
            let mut slot = state_arc
                .lock()
                .map_err(|_| invalid("decoder state mutex poisoned"))?;
            let guard = open(&mut slot, "nvjpeg", "decode")?;
            let info = guard.handle.image_info(&host).map_err(map)?;
            let w = info.widths[0];
            let h = info.heights[0];
            let pitch = (w as usize)
                .checked_mul(bpp)
                .ok_or_else(|| invalid("row bytes overflow"))?;
            let nbytes = pitch
                .checked_mul(h as usize)
                .ok_or_else(|| invalid("image byte size overflow"))?;
            let shape = if bpp == 1 {
                vec![h as i64, w as i64]
            } else {
                vec![h as i64, w as i64, bpp as i64]
            };

            // SAFETY: the decode below writes every byte before the buffer is handed to Python;
            // if it fails, the buffer is dropped unread.
            let dev_buf = unsafe { CudaDeviceBuffer::<u8>::uninitialized_async(&s, nbytes) }
                .map_err(|e| record_and_err("nvjpeg", "decode", e, ErrorKind::Cuda))?;
            let out = PyDeviceBuffer::from_owned_cuda(dev_buf, "u8", shape, &s)?;
            let mut view = typed_view::<u8>("nvjpeg", "decode", "output", &out, nbytes)?;
            let mut planes = if bpp == 1 {
                ImagePlanesMut::single_y(&mut view, w, h, pitch)
            } else {
                ImagePlanesMut::interleaved(&mut view, w, h, bpp as u32, pitch)
            }
            .map_err(map)?;

            if guard.last_stream.cu_stream() != s.cu_stream() {
                guard
                    .last_stream
                    .synchronize()
                    .map_err(|e| record_and_err("nvjpeg", "decode", e, ErrorKind::Cuda))?;
                guard.last_stream = s.clone();
            }
            enqueue_leased("nvjpeg", "decode", &s, [(&out, Access::Write)], || {
                guard.state.decode(&host, fmt, &mut planes, &s).map_err(map)
            })?;
            Ok((out, w, h))
        })?;

        diag::record_event(
            Level::Info,
            "nvjpeg",
            "decode",
            format!("{w}x{h} fmt={output_format} {} B", out.byte_len()),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(out)
    }

    /// Releases the nvJPEG handle and decode state now instead of at garbage collection.
    ///
    /// Waits for in-flight calls and for the last decode's stream, then frees the native
    /// objects. Idempotent; every later call on this object raises `NvInvalidArgumentError`.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        close_slot(py, &self.state, |st| drain_stream("nvjpeg", &st.last_stream))
    }

    /// `True` once `close()` has released the native decoder.
    #[getter]
    fn is_closed(&self) -> bool {
        is_closed(&self.state)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if is_closed(&slf.state) {
            return Err(closed_err("nvjpeg", "__enter__"));
        }
        Ok(slf)
    }

    /// Closes the decoder; exceptions raised in the `with` body are not suppressed.
    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py)?;
        Ok(false)
    }
}

struct EncoderInner {
    params: EncoderParams,
    state: EncoderState,
    /// Constructor defaults, applied to every call that does not override them.
    default_quality: u8,
    default_css: ChromaSubsampling,
    /// Values currently set on `params`.
    quality: u8,
    css: ChromaSubsampling,
}

/// GPU-accelerated JPEG encoder bound to a CUDA context and default stream.
#[pyclass(name = "NvjpegEncoder", module = "nv_rs.nvjpeg")]
pub struct PyNvjpegEncoder {
    inner: Arc<Mutex<Option<EncoderInner>>>,
    stream: PyStream,
}

#[pymethods]
impl PyNvjpegEncoder {
    /// Creates a new `NvjpegEncoder` on `stream`'s CUDA context, GIL-free.
    ///
    /// `quality` and `subsampling` are the defaults of every `encode` call.
    #[new]
    #[pyo3(signature = (stream, quality=90, subsampling=None))]
    fn new(
        py: Python<'_>,
        stream: &PyStream,
        quality: u8,
        subsampling: Option<&str>,
    ) -> PyResult<Self> {
        let css = match subsampling {
            Some(sub) => parse_subsampling(sub)?,
            None => ChromaSubsampling::Yuv420,
        };
        let s = stream.inner.clone();
        let (params, state) = py
            .detach(move || -> nvjpeg::Result<(EncoderParams, EncoderState)> {
                let handle = Handle::new(s.context())?;
                let mut params = EncoderParams::new(&handle, &s)?;
                params.set_quality(quality, &s)?;
                params.set_sampling_factors(css, &s)?;
                let state = EncoderState::new(&handle, &s)?;
                Ok((params, state))
            })
            .map_err(|e| map_nvjpeg_err("NvjpegEncoder::new", e))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(Some(EncoderInner {
                params,
                state,
                default_quality: quality,
                default_css: css,
                quality,
                css,
            }))),
            stream: stream.clone(),
        })
    }

    /// Encodes a `DeviceBuffer` (`u8`, owned or DLPack import) or host buffer
    /// (`bytes`/`bytearray`/`memoryview`/`ndarray`) to JPEG `bytes`, GIL-free.
    ///
    /// `width` and `height` are given together or inferred together from an `(H, W, 3)` shape.
    /// `quality` and `subsampling` apply to this call only (the constructor's values otherwise).
    #[pyo3(signature = (image, width=None, height=None, quality=None, subsampling=None, input_format="rgbi", stream=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "Python keyword-argument API for JPEG encoding options"
    )]
    fn encode<'py>(
        &self,
        py: Python<'py>,
        image: &Bound<'_, PyAny>,
        width: Option<u32>,
        height: Option<u32>,
        quality: Option<u8>,
        subsampling: Option<&str>,
        input_format: &str,
        stream: Option<&PyStream>,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let (fmt, bpp) = parse_input_format(input_format)?;
        let (w, h) = resolve_image_dims(image, width, height, bpp, "encode")?;
        let css = subsampling.map(parse_subsampling).transpose()?;
        let s = stream.unwrap_or(&self.stream).inner.clone();
        let invalid =
            |msg: &str| record_and_err("nvjpeg", "encode", msg, ErrorKind::InvalidArgument);
        let pitch = (w as usize)
            .checked_mul(bpp)
            .ok_or_else(|| invalid("row bytes overflow"))?;
        let need = pitch
            .checked_mul(h as usize)
            .ok_or_else(|| invalid("image byte size overflow"))?;

        enum EncodeInput<'a> {
            /// The Python buffer (leased while the encode reads it) and a typed view of it.
            Device(&'a PyDeviceBuffer, CudaDeviceBuffer<u8>),
            Host(Vec<u8>),
        }

        // A failed extraction only means "not a DeviceBuffer": the host path then requires the
        // buffer protocol and reports its own error.
        let dev_arg = image.extract::<PyRef<'_, PyDeviceBuffer>>();
        let input = match &dev_arg {
            Ok(buf) => {
                let buf: &PyDeviceBuffer = buf;
                check_buf("nvjpeg", "encode", "image", buf, &["u8"], need, s.context())?;
                EncodeInput::Device(
                    buf,
                    typed_view::<u8>("nvjpeg", "encode", "image", buf, need)?,
                )
            }
            Err(_) => EncodeInput::Host(copy_py_bytes(py, image, "encode")?),
        };

        let inner_arc = self.inner.clone();
        let t0 = Instant::now();

        let jpeg = py.detach(move || -> PyResult<Vec<u8>> {
            let map = |e| map_nvjpeg_err("encode", e);
            let uploaded;
            let (src, leased): (&CudaDeviceBuffer<u8>, Option<&PyDeviceBuffer>) = match &input {
                EncodeInput::Device(buf, view) => (view, Some(*buf)),
                EncodeInput::Host(host) => {
                    uploaded = CudaDeviceBuffer::from_host(&s, host)
                        .map_err(|e| record_and_err("nvjpeg", "encode", e, ErrorKind::Cuda))?;
                    (&uploaded, None)
                }
            };
            let planes = ImagePlanes::interleaved(src, w, h, bpp as u32, pitch).map_err(map)?;

            let mut slot = inner_arc
                .lock()
                .map_err(|_| invalid("encoder state mutex poisoned"))?;
            let inner = open(&mut slot, "nvjpeg", "encode")?;
            let q = quality.unwrap_or(inner.default_quality);
            if q != inner.quality {
                inner.params.set_quality(q, &s).map_err(map)?;
                inner.quality = q;
            }
            let sub = css.unwrap_or(inner.default_css);
            if sub != inner.css {
                inner.params.set_sampling_factors(sub, &s).map_err(map)?;
                inner.css = sub;
            }
            let (params, state) = (&inner.params, &mut inner.state);
            let mut encode = || {
                state
                    .encode_image(params, &planes, fmt, w, h, &s)
                    .map_err(map)
            };
            match leased {
                Some(buf) => enqueue_leased("nvjpeg", "encode", &s, [(buf, Access::Read)], encode)?,
                None => encode()?,
            }
            state.retrieve_bitstream(&s).map_err(map)
        })?;

        diag::record_event(
            Level::Info,
            "nvjpeg",
            "encode",
            format!("{w}x{h} fmt={input_format} -> {} B", jpeg.len()),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(PyBytes::new(py, &jpeg))
    }

    /// Releases the nvJPEG encoder state now instead of at garbage collection.
    ///
    /// Waits for in-flight calls and for work queued on the encoder's stream, then frees the
    /// native objects. Idempotent; every later call on this object raises
    /// `NvInvalidArgumentError`.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        let stream = self.stream.inner.clone();
        close_slot(py, &self.inner, move |_| drain_stream("nvjpeg", &stream))
    }

    /// `True` once `close()` has released the native encoder.
    #[getter]
    fn is_closed(&self) -> bool {
        is_closed(&self.inner)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if is_closed(&slf.inner) {
            return Err(closed_err("nvjpeg", "__enter__"));
        }
        Ok(slf)
    }

    /// Closes the encoder; exceptions raised in the `with` body are not suppressed.
    #[pyo3(signature = (exc_type=None, exc_val=None, exc_tb=None))]
    fn __exit__(
        &self,
        py: Python<'_>,
        exc_type: Option<&Bound<'_, PyAny>>,
        exc_val: Option<&Bound<'_, PyAny>>,
        exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<bool> {
        let _ = (exc_type, exc_val, exc_tb);
        self.close(py)?;
        Ok(false)
    }
}

/// Registers the `nv_rs.nvjpeg` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "nvjpeg", |m| {
        m.add_class::<PyNvjpegDecoder>()?;
        m.add_class::<PyNvjpegEncoder>()?;
    })
}
