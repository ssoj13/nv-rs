//! `nv_rs.nvjpeg` — GPU-accelerated JPEG decoding, encoding, and header inspection.
//!
//! **What.** Exposes [`PyNvjpegDecoder`] (`NvjpegDecoder`) and [`PyNvjpegEncoder`]
//! (`NvjpegEncoder`) to Python under `nv_rs.nvjpeg`, operating on [`PyDeviceBuffer`]
//! (interleaved RGB/BGR or single-plane Y) and Python buffer-protocol inputs (`bytes`,
//! `bytearray`, `memoryview`, `numpy.ndarray`) with the GIL released via `py.detach`.
//!
//! **Used by** `nv_rs` Python extension module (`python/nv-py/src/lib.rs`).

use std::sync::{Arc, Mutex};
use std::time::Instant;

use cuda_core::DeviceBuffer as CudaDeviceBuffer;
use nv_loader::diag::{self, Level};
use nvjpeg::{
    Backend, ChromaSubsampling, EncoderParams, EncoderState, Handle, ImagePlanes, ImagePlanesMut,
    InputFormat, JpegState, OutputFormat,
};
use pyo3::buffer::PyBuffer;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

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

fn resolve_image_dims(
    image: &Bound<'_, PyAny>,
    width: Option<u32>,
    height: Option<u32>,
    expected_channels: usize,
    op: &str,
) -> PyResult<(u32, u32)> {
    if let (Some(w), Some(h)) = (width, height) {
        if w == 0 || h == 0 {
            return Err(record_and_err(
                "nvjpeg",
                op,
                "width and height must be > 0",
                ErrorKind::InvalidArgument,
            ));
        }
        return Ok((w, h));
    }

    let shape = image
        .getattr("shape")
        .ok()
        .and_then(|s| s.extract::<Vec<i64>>().ok());

    let (inferred_w, inferred_h) = match shape.as_deref() {
        Some([h, w, c]) if *h > 0 && *w > 0 && *c == expected_channels as i64 => {
            let w_u32 = u32::try_from(*w).map_err(|_| {
                record_and_err(
                    "nvjpeg",
                    op,
                    "width overflows u32",
                    ErrorKind::InvalidArgument,
                )
            })?;
            let h_u32 = u32::try_from(*h).map_err(|_| {
                record_and_err(
                    "nvjpeg",
                    op,
                    "height overflows u32",
                    ErrorKind::InvalidArgument,
                )
            })?;
            (Some(w_u32), Some(h_u32))
        }
        _ => (None, None),
    };

    let w = width.or(inferred_w).ok_or_else(|| {
        record_and_err(
            "nvjpeg",
            op,
            "width and height are required when input shape is not (H, W, 3)",
            ErrorKind::InvalidArgument,
        )
    })?;
    let h = height.or(inferred_h).ok_or_else(|| {
        record_and_err(
            "nvjpeg",
            op,
            "width and height are required when input shape is not (H, W, 3)",
            ErrorKind::InvalidArgument,
        )
    })?;
    if w == 0 || h == 0 {
        return Err(record_and_err(
            "nvjpeg",
            op,
            "width and height must be > 0",
            ErrorKind::InvalidArgument,
        ));
    }
    Ok((w, h))
}

/// GPU-accelerated JPEG decoder bound to a CUDA context and default stream.
#[pyclass(name = "NvjpegDecoder", module = "nv_rs.nvjpeg")]
pub struct PyNvjpegDecoder {
    handle: Arc<Handle>,
    state: Arc<Mutex<JpegState>>,
    stream: PyStream,
}

#[pymethods]
impl PyNvjpegDecoder {
    /// Creates a new `NvjpegDecoder` on `stream`'s CUDA context.
    #[new]
    #[pyo3(signature = (stream, backend="default"))]
    fn new(stream: &PyStream, backend: &str) -> PyResult<Self> {
        let parsed_backend = parse_backend(backend)?;
        let ctx = stream.inner.context();
        let handle = if parsed_backend == Backend::Default {
            Handle::new(ctx)
        } else {
            Handle::with_backend(ctx, parsed_backend, 0)
        }
        .map_err(|e| map_nvjpeg_err("NvjpegDecoder::new", e))?;

        let state = JpegState::new(&handle).map_err(|e| map_nvjpeg_err("NvjpegDecoder::new", e))?;

        Ok(Self {
            handle,
            state: Arc::new(Mutex::new(state)),
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
        let handle = self.handle.clone();
        let t0 = Instant::now();
        let info = py
            .detach(move || handle.image_info(&host))
            .map_err(|e| map_nvjpeg_err("decode_info", e))?;

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
        let s = stream.unwrap_or(&self.stream).clone();
        let handle = self.handle.clone();
        let state_arc = self.state.clone();
        let t0 = Instant::now();

        let (dev_buf, w, h) = py
            .detach(
                move || -> nvjpeg::Result<(CudaDeviceBuffer<u8>, u32, u32)> {
                    let info = handle.image_info(&host)?;
                    let w = info.widths[0];
                    let h = info.heights[0];
                    let pitch = (w as usize)
                        .checked_mul(bpp)
                        .ok_or(nvjpeg::Error::InvalidInput("row bytes overflow"))?;
                    let nbytes = pitch
                        .checked_mul(h as usize)
                        .ok_or(nvjpeg::Error::InvalidInput("image byte size overflow"))?;

                    // SAFETY: Uninitialized GPU memory is immediately written by `nvjpegDecode` before any read.
                    let mut dev_buf =
                        unsafe { CudaDeviceBuffer::<u8>::uninitialized_async(&s.inner, nbytes) }?;

                    let mut planes = if bpp == 1 {
                        ImagePlanesMut::single_y(&mut dev_buf, w, h, pitch)?
                    } else {
                        ImagePlanesMut::interleaved(&mut dev_buf, w, h, bpp as u32, pitch)?
                    };

                    let mut state = state_arc
                        .lock()
                        .map_err(|_| nvjpeg::Error::InvalidInput("decoder state mutex poisoned"))?;
                    state.decode(&host, fmt, &mut planes, &s.inner)?;
                    Ok((dev_buf, w, h))
                },
            )
            .map_err(|e| map_nvjpeg_err("decode", e))?;

        let nbytes = dev_buf.len();
        diag::record_event(
            Level::Info,
            "nvjpeg",
            "decode",
            format!("{w}x{h} fmt={output_format} {nbytes} B"),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        let shape = if bpp == 1 {
            vec![h as i64, w as i64]
        } else {
            vec![h as i64, w as i64, bpp as i64]
        };
        Ok(PyDeviceBuffer::from_owned_cuda(
            dev_buf,
            s.device_id,
            "u8",
            shape,
        ))
    }
}

struct EncoderInner {
    params: EncoderParams,
    state: EncoderState,
}

/// GPU-accelerated JPEG encoder bound to a CUDA context and default stream.
#[pyclass(name = "NvjpegEncoder", module = "nv_rs.nvjpeg")]
pub struct PyNvjpegEncoder {
    inner: Arc<Mutex<EncoderInner>>,
    stream: PyStream,
}

#[pymethods]
impl PyNvjpegEncoder {
    /// Creates a new `NvjpegEncoder` on `stream`'s CUDA context.
    #[new]
    #[pyo3(signature = (stream, quality=90, subsampling=None))]
    fn new(stream: &PyStream, quality: u8, subsampling: Option<&str>) -> PyResult<Self> {
        let handle = Handle::new(stream.inner.context())
            .map_err(|e| map_nvjpeg_err("NvjpegEncoder::new", e))?;
        let mut params = EncoderParams::new(&handle, &stream.inner)
            .map_err(|e| map_nvjpeg_err("NvjpegEncoder::new", e))?;
        params
            .set_quality(quality, &stream.inner)
            .map_err(|e| map_nvjpeg_err("NvjpegEncoder::new", e))?;
        let css = match subsampling {
            Some(sub) => parse_subsampling(sub)?,
            None => ChromaSubsampling::Yuv420,
        };
        params
            .set_sampling_factors(css, &stream.inner)
            .map_err(|e| map_nvjpeg_err("NvjpegEncoder::new", e))?;
        let state = EncoderState::new(&handle, &stream.inner)
            .map_err(|e| map_nvjpeg_err("NvjpegEncoder::new", e))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(EncoderInner { params, state })),
            stream: stream.clone(),
        })
    }

    /// Encodes a `DeviceBuffer` or host buffer (`bytes`/`bytearray`/`memoryview`/`ndarray`) to JPEG `bytes`, GIL-free.
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
        let s = stream.unwrap_or(&self.stream).clone();

        enum EncodeInput {
            Device(Arc<CudaDeviceBuffer<u8>>),
            Host(Vec<u8>),
        }

        let input = if let Ok(py_buf) = image.extract::<PyRef<'_, PyDeviceBuffer>>() {
            let dev = py_buf
                .owned_cuda_buf()
                .ok_or_else(|| {
                    record_and_err(
                        "nvjpeg",
                        "encode",
                        "encode requires an nv_rs-owned DeviceBuffer (use DeviceBuffer.from_buffer)",
                        ErrorKind::InvalidArgument,
                    )
                })?
                .clone();
            EncodeInput::Device(dev)
        } else {
            let host = copy_py_bytes(py, image, "encode")?;
            EncodeInput::Host(host)
        };

        let inner_arc = self.inner.clone();
        let t0 = Instant::now();

        let jpeg = py
            .detach(move || -> nvjpeg::Result<Vec<u8>> {
                let uploaded;
                let dev_ref: &CudaDeviceBuffer<u8> = match &input {
                    EncodeInput::Device(buf) => buf.as_ref(),
                    EncodeInput::Host(host) => {
                        uploaded = CudaDeviceBuffer::from_host(&s.inner, host)?;
                        &uploaded
                    }
                };

                let pitch = (w as usize)
                    .checked_mul(bpp)
                    .ok_or(nvjpeg::Error::InvalidInput("row bytes overflow"))?;
                let planes = ImagePlanes::interleaved(dev_ref, w, h, bpp as u32, pitch)?;

                let mut inner = inner_arc
                    .lock()
                    .map_err(|_| nvjpeg::Error::InvalidInput("encoder state mutex poisoned"))?;
                if let Some(q) = quality {
                    inner.params.set_quality(q, &s.inner)?;
                }
                if let Some(sub) = css {
                    inner.params.set_sampling_factors(sub, &s.inner)?;
                }
                let EncoderInner { params, state } = &mut *inner;
                state.encode_image(params, &planes, fmt, w, h, &s.inner)?;
                state.retrieve_bitstream(&s.inner)
            })
            .map_err(|e| map_nvjpeg_err("encode", e))?;

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
}

/// Registers the `nv_rs.nvjpeg` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "nvjpeg", |m| {
        m.add_class::<PyNvjpegDecoder>()?;
        m.add_class::<PyNvjpegEncoder>()?;
    })
}
