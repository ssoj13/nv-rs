//! `nv_rs.nvcomp` — GPU batched compression/decompression over zero-copy `DeviceBuffer`.
//!
//! **How.** A `CompressionManager` runs on the stream it was created with: the wrapper's staging
//! and scratch buffers belong to that stream, so per-call streams are not offered. `compress`
//! reads a `DeviceBuffer` (`u8`, owned or DLPack import, validated by `buf_util::check_buf`)
//! under a read lease; both directions block until the stream completes (the wrapper reads the
//! per-chunk sizes back), and a decompressed buffer records the manager's stream as its producer.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use nv_loader::diag::{self, Level};
use nvcomp::{CompressedBuffer, CompressionManager, Format, FormatKind};
use pyo3::prelude::*;

use crate::buf_util::{
    Access, check_buf, close_slot, closed as closed_err, drain_stream, enqueue_leased, is_closed,
    open, poisoned, typed_view,
};
use crate::core::{PyDeviceBuffer, PyStream};
use crate::errors::{ErrorKind, record_and_err};

py_parse_enum!(
    parse_format_kind,
    "nvcomp",
    "parse_format",
    FormatKind,
    "nvCOMP format (expected lz4, snappy, zstd, cascaded, gdeflate, or bitcomp)",
    {
        "lz4" => FormatKind::Lz4,
        "snappy" => FormatKind::Snappy,
        "zstd" => FormatKind::Zstd,
        "cascaded" => FormatKind::Cascaded,
        "gdeflate" => FormatKind::Gdeflate,
        "bitcomp" => FormatKind::Bitcomp,
    }
);

/// Canonical Python name of `kind` (the inverse of [`parse_format_kind`]).
fn format_kind_name(kind: FormatKind) -> &'static str {
    match kind {
        FormatKind::Lz4 => "lz4",
        FormatKind::Snappy => "snappy",
        FormatKind::Zstd => "zstd",
        FormatKind::Cascaded => "cascaded",
        FormatKind::Gdeflate => "gdeflate",
        FormatKind::Bitcomp => "bitcomp",
    }
}

py_err_mapper!(map_nvcomp_err, "nvcomp", nvcomp::Error, {
    nvcomp::Error::Load(_) | nvcomp::Error::Call(_) => ErrorKind::LibraryNotFound,
    nvcomp::Error::Invalid(_) | nvcomp::Error::Unsupported(_) | nvcomp::Error::Budget { .. } => {
        ErrorKind::InvalidArgument
    },
    nvcomp::Error::Driver(_) | nvcomp::Error::Status { .. } | nvcomp::Error::BitcompStatus { .. } => {
        ErrorKind::Cuda
    },
});

/// Self-describing GPU compressed payload produced by [`PyCompressionManager::compress`].
#[pyclass(name = "CompressedBuffer", module = "nv_rs.nvcomp")]
pub struct PyCompressedBuffer {
    pub inner: CompressedBuffer,
    format_name: &'static str,
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
        self.format_name
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
    inner: Arc<Mutex<Option<CompressionManager>>>,
    stream: PyStream,
    format_name: &'static str,
}

#[pymethods]
impl PyCompressionManager {
    /// Creates a manager for `format` on `stream` (every call runs there), GIL-free.
    #[new]
    #[pyo3(signature = (stream, format="lz4", chunk_size=65536, budget=None))]
    fn new(
        py: Python<'_>,
        stream: &PyStream,
        format: &str,
        chunk_size: usize,
        budget: Option<usize>,
    ) -> PyResult<Self> {
        let kind = parse_format_kind(format)?;
        let fmt = Format::default_for(kind);
        let s = stream.inner.clone();
        let mgr = py
            .detach(move || CompressionManager::new(&s, fmt, chunk_size, budget))
            .map_err(|e| map_nvcomp_err("CompressionManager::new", e))?;
        Ok(Self {
            inner: Arc::new(Mutex::new(Some(mgr))),
            stream: stream.clone(),
            format_name: format_kind_name(kind),
        })
    }

    /// Compresses a `DeviceBuffer` (`u8`) on the manager's stream, GIL-free; returns when done.
    fn compress(&self, py: Python<'_>, input: &PyDeviceBuffer) -> PyResult<PyCompressedBuffer> {
        let s = self.stream.inner.clone();
        check_buf(
            "nvcomp",
            "compress",
            "input",
            input,
            &["u8"],
            0,
            s.context(),
        )?;
        let view = typed_view::<u8>("nvcomp", "compress", "input", input, input.byte_len())?;
        let mgr_arc = self.inner.clone();
        let t0 = Instant::now();
        let compressed = py.detach(move || -> PyResult<CompressedBuffer> {
            let mut slot = mgr_arc.lock().map_err(|_| poisoned("nvcomp", "compress"))?;
            let mgr = open(&mut slot, "nvcomp", "compress")?;
            let mut out = None;
            enqueue_leased("nvcomp", "compress", &s, [(input, Access::Read)], || {
                out = Some(
                    mgr.compress_async(&s, &view)
                        .map_err(|e| map_nvcomp_err("compress", e))?,
                );
                Ok(())
            })?;
            out.ok_or_else(|| {
                record_and_err(
                    "nvcomp",
                    "compress",
                    "compression produced no output",
                    ErrorKind::Runtime,
                )
            })
        })?;

        diag::record_event(
            Level::Info,
            "nvcomp",
            "compress",
            format!(
                "fmt={} {}->{} B",
                self.format_name,
                compressed.layout.uncompressed_bytes,
                compressed.data.len()
            ),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        Ok(PyCompressedBuffer {
            inner: compressed,
            format_name: self.format_name,
            device_id: self.stream.device_id,
        })
    }

    /// Decompresses a `CompressedBuffer` into a new `DeviceBuffer` (`u8`, supports DLPack) on the
    /// manager's stream, GIL-free; returns when done.
    fn decompress(
        &self,
        py: Python<'_>,
        compressed: &PyCompressedBuffer,
    ) -> PyResult<PyDeviceBuffer> {
        let s = self.stream.inner.clone();
        let mgr_arc = self.inner.clone();
        let t0 = Instant::now();
        // `py.detach` needs only `Send` captures: `&CompressedBuffer` is one, and the GIL-held
        // borrow of `compressed` outlives the call.
        let comp_ref = &compressed.inner;
        let dev_out = py.detach(|| -> PyResult<_> {
            let mut slot = mgr_arc
                .lock()
                .map_err(|_| poisoned("nvcomp", "decompress"))?;
            let mgr = open(&mut slot, "nvcomp", "decompress")?;
            mgr.decompress_to_buffer_async(&s, comp_ref)
                .map_err(|e| map_nvcomp_err("decompress", e))
        })?;

        let nbytes = dev_out.len();
        diag::record_event(
            Level::Info,
            "nvcomp",
            "decompress",
            format!("fmt={} -> {nbytes} B", compressed.format_name),
            t0.elapsed().as_nanos() as u64,
            true,
        );

        let len = i64::try_from(nbytes).map_err(|_| {
            record_and_err(
                "nvcomp",
                "decompress",
                format!("decompressed size {nbytes} does not fit i64"),
                ErrorKind::InvalidArgument,
            )
        })?;
        PyDeviceBuffer::from_owned_cuda(dev_out, "u8", vec![len], &s)
    }

    /// Releases the native manager now instead of at garbage collection.
    ///
    /// Waits for in-flight calls and for work queued on the manager's stream, then frees the
    /// native objects. Idempotent; every later call on this object raises
    /// `NvInvalidArgumentError`.
    fn close(&self, py: Python<'_>) -> PyResult<()> {
        let stream = self.stream.inner.clone();
        close_slot(py, &self.inner, move |_| drain_stream("nvcomp", &stream))
    }

    /// `True` once `close()` has released the native manager.
    #[getter]
    fn is_closed(&self) -> bool {
        is_closed(&self.inner)
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if is_closed(&slf.inner) {
            return Err(closed_err("nvcomp", "__enter__"));
        }
        Ok(slf)
    }

    /// Closes the manager; exceptions raised in the `with` body are not suppressed.
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

/// Registers the `nv_rs.nvcomp` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    py_register_submodule!(py, parent, "nvcomp", |m| {
        m.add_class::<PyCompressedBuffer>()?;
        m.add_class::<PyCompressionManager>()?;
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_kind_name_round_trips_through_the_parser() {
        for name in ["lz4", "snappy", "zstd", "cascaded", "gdeflate", "bitcomp"] {
            let kind = parse_format_kind(&format!(" {} ", name.to_ascii_uppercase()))
                .expect("known format");
            assert_eq!(format_kind_name(kind), name);
        }
    }
}
