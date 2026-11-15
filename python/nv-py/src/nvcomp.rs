//! `nv_rs.nvcomp` — GPU batched compression/decompression over zero-copy `DeviceBuffer`.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use nv_loader::diag::{self, Level};
use nvcomp::{CompressedBuffer, CompressionManager, Format, FormatKind};
use pyo3::prelude::*;

use crate::core::{PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

fn parse_format_kind(name: &str) -> PyResult<FormatKind> {
    match name.trim().to_ascii_lowercase().as_str() {
        "lz4" => Ok(FormatKind::Lz4),
        "snappy" => Ok(FormatKind::Snappy),
        "zstd" => Ok(FormatKind::Zstd),
        "cascaded" => Ok(FormatKind::Cascaded),
        "gdeflate" => Ok(FormatKind::Gdeflate),
        "bitcomp" => Ok(FormatKind::Bitcomp),
        other => Err(record_and_err(
            "nvcomp",
            "parse_format",
            format!(
                "unsupported nvCOMP format `{other}`; expected lz4, snappy, zstd, cascaded, gdeflate, or bitcomp"
            ),
            ErrorKind::InvalidArgument,
        )),
    }
}

fn map_nvcomp_err(op: &str, err: nvcomp::Error) -> PyErr {
    let kind = match &err {
        nvcomp::Error::Load(_) => ErrorKind::LibraryNotFound,
        nvcomp::Error::Invalid(_) | nvcomp::Error::Unsupported(_) => ErrorKind::InvalidArgument,
        _ => ErrorKind::Cuda,
    };
    record_and_err("nvcomp", op, err, kind)
}

/// Self-describing GPU compressed payload produced by [`PyCompressionManager::compress`].
#[pyclass(name = "CompressedBuffer", module = "nv_rs.nvcomp")]
pub struct PyCompressedBuffer {
    pub inner: CompressedBuffer,
    format_name: String,
    device_id: i32,
}

#[pymethods]
impl PyCompressedBuffer {
    /// Actual compressed payload bytes across all chunks (excluding slot padding).
    #[getter]
    fn compressed_bytes(&self) -> usize {
        self.inner
            .layout
            .compressed_ranges
            .iter()
            .map(|r| r.len())
            .sum()
    }

    /// Total GPU device buffer capacity allocated for chunk slots.
    #[getter]
    fn allocated_bytes(&self) -> usize {
        self.inner.data.len()
    }

    #[getter]
    fn uncompressed_bytes(&self) -> usize {
        self.inner.layout.uncompressed_bytes
    }

    #[getter]
    fn ratio(&self) -> f64 {
        let comp = self.compressed_bytes();
        if comp == 0 {
            0.0
        } else {
            self.inner.layout.uncompressed_bytes as f64 / comp as f64
        }
    }

    #[getter]
    fn format(&self) -> &str {
        &self.format_name
    }

    #[getter]
    fn device_id(&self) -> i32 {
        self.device_id
    }

    fn __repr__(&self) -> String {
        format!(
            "CompressedBuffer(format='{}', uncompressed={}, compressed={}, ratio={:.2}x)",
            self.format_name,
            self.uncompressed_bytes(),
            self.compressed_bytes(),
            self.ratio()
        )
    }
}

/// Reusable GPU compression/decompression manager bound to a CUDA stream.
#[pyclass(name = "CompressionManager", module = "nv_rs.nvcomp")]
pub struct PyCompressionManager {
    inner: Arc<Mutex<CompressionManager>>,
    stream: PyStream,
    format_name: String,
}

#[pymethods]
impl PyCompressionManager {
    #[new]
    #[pyo3(signature = (stream, format="lz4", chunk_size=65536, budget=None))]
    fn new(
        stream: &PyStream,
        format: &str,
        chunk_size: usize,
        budget: Option<usize>,
    ) -> PyResult<Self> {
        let kind = parse_format_kind(format)?;
        let fmt = Format::default_for(kind);
        let mgr = CompressionManager::new(&stream.inner, fmt, chunk_size, budget)
            .map_err(|e| map_nvcomp_err("CompressionManager::new", e))?;
        Ok(Self {
            inner: Arc::new(Mutex::new(mgr)),
            stream: stream.clone(),
            format_name: format.to_ascii_lowercase(),
        })
    }

    /// Compresses a `DeviceBuffer` asynchronously on `stream` (or the manager's stream), GIL-free.
    #[pyo3(signature = (input, stream=None))]
    fn compress(
        &self,
        py: Python<'_>,
        input: &PyDeviceBuffer,
        stream: Option<&PyStream>,
    ) -> PyResult<PyCompressedBuffer> {
        let s = stream.unwrap_or(&self.stream).clone();
        let dev_in = input
            .owned_cuda_buf()
            .ok_or_else(|| {
                record_and_err(
                    "nvcomp",
                    "compress",
                    "compress requires an nv_rs-owned DeviceBuffer (use DeviceBuffer.from_buffer)",
                    ErrorKind::InvalidArgument,
                )
            })?
            .clone();
        let mgr_arc = self.inner.clone();
        let fmt_name = self.format_name.clone();
        let t0 = Instant::now();
        let compressed = py
            .detach(move || {
                let mut mgr = mgr_arc
                    .lock()
                    .map_err(|_| nvcomp::Error::Invalid("manager mutex poisoned"))?;
                mgr.compress_async(&s.inner, &dev_in)
            })
            .map_err(|e| map_nvcomp_err("compress", e))?;

        diag::record_event(
            Level::Info,
            "nvcomp",
            "compress",
            format!(
                "fmt={} {}->{} B",
                fmt_name,
                compressed.layout.uncompressed_bytes,
                compressed.data.len()
            ),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(PyCompressedBuffer {
            inner: compressed,
            format_name: fmt_name,
            device_id: s.device_id,
        })
    }

    /// Decompresses a `CompressedBuffer` into a new `DeviceBuffer` (supports DLPack), GIL-free.
    #[pyo3(signature = (compressed, stream=None))]
    fn decompress(
        &self,
        py: Python<'_>,
        compressed: &PyCompressedBuffer,
        stream: Option<&PyStream>,
    ) -> PyResult<PyDeviceBuffer> {
        let s = stream.unwrap_or(&self.stream).clone();
        let mgr_arc = self.inner.clone();
        let t0 = Instant::now();
        // Borrow `compressed.inner` across `py.detach` by cloning layout/stream? `decompress_to_buffer_async` takes `&CompressedBuffer`.
        // Since `&PyCompressedBuffer` stays pinned under GIL while Python holds it, we can pass a reference inside `detach`?
        // No: `detach` requires `Send + Sync` / `'static`? No, `py.detach` only requires `Ungil` (Send)!
        let comp_ref = &compressed.inner;
        let dev_out = py
            .detach(|| {
                let mut mgr = mgr_arc
                    .lock()
                    .map_err(|_| nvcomp::Error::Invalid("manager mutex poisoned"))?;
                mgr.decompress_to_buffer_async(&s.inner, comp_ref)
            })
            .map_err(|e| map_nvcomp_err("decompress", e))?;

        let nbytes = dev_out.len();
        diag::record_event(
            Level::Info,
            "nvcomp",
            "decompress",
            format!("fmt={} -> {nbytes} B", compressed.format_name),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(PyDeviceBuffer::from_owned_cuda(
            dev_out,
            s.device_id,
            "u8",
            vec![nbytes as i64],
        ))
    }
}

/// Registers the `nv_rs.nvcomp` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(py, "nvcomp")?;
    m.add_class::<PyCompressedBuffer>()?;
    m.add_class::<PyCompressionManager>()?;
    parent.add_submodule(&m)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("nv_rs.nvcomp", m)?;
    Ok(())
}
