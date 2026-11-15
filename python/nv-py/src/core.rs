//! `nv_rs.core` — CUDA Context, Stream, and zero-copy DeviceBuffer (DLPack + `__cuda_array_interface__` + Buffer Protocol).

use std::ffi::c_void;
use std::sync::Arc;
use std::time::Instant;

use cuda_core::{Context, DeviceBuffer as CudaDeviceBuffer, Stream};
use nv_loader::diag::{self, Level};
use pyo3::buffer::PyBuffer;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyCapsule, PyDict, PyTuple};

use crate::dlpack::{
    DL_CUDA, DLManagedTensor, DLTENSOR_CAPSULE_NAME, USED_DLTENSOR_CAPSULE_NAME, format_dl_dtype,
    into_dlpack_managed, parse_dtype, pycapsule_dlpack_destructor,
};
use crate::errors::{ErrorKind, record_and_err};

/// CUDA driver context (`cuda_core::Context`).
#[pyclass(name = "Context", module = "nv_rs.core")]
#[derive(Clone)]
pub struct PyContext {
    pub inner: Arc<Context>,
    device_id: i32,
}

#[pymethods]
impl PyContext {
    /// Creates or retains the primary CUDA context on `device` (default `0`).
    #[new]
    #[pyo3(signature = (device=0))]
    fn new(device: usize) -> PyResult<Self> {
        let t0 = Instant::now();
        let inner = Context::new(device)
            .map_err(|e| record_and_err("core", "Context::new", e, ErrorKind::Cuda))?;
        diag::record_event(
            Level::Info,
            "core",
            "context_new",
            format!("device={device}"),
            t0.elapsed().as_nanos() as u64,
            true,
        );
        Ok(Self {
            inner,
            device_id: device as i32,
        })
    }

    /// Creates a new non-blocking CUDA stream bound to this context.
    fn new_stream(&self) -> PyResult<PyStream> {
        let stream = self
            .inner
            .new_stream()
            .map_err(|e| record_and_err("core", "new_stream", e, ErrorKind::Cuda))?;
        Ok(PyStream {
            inner: stream,
            device_id: self.device_id,
        })
    }

    /// Blocks until all work in this context has completed.
    fn synchronize(&self, py: Python<'_>) -> PyResult<()> {
        let ctx = self.inner.clone();
        py.detach(move || ctx.synchronize())
            .map_err(|e| record_and_err("core", "context_sync", e, ErrorKind::Cuda))
    }

    #[getter]
    fn device_id(&self) -> i32 {
        self.device_id
    }

    fn __repr__(&self) -> String {
        format!("Context(device_id={})", self.device_id)
    }
}

/// CUDA stream (`cuda_core::Stream`).
#[pyclass(name = "Stream", module = "nv_rs.core")]
#[derive(Clone)]
pub struct PyStream {
    pub inner: Arc<Stream>,
    pub device_id: i32,
}

#[pymethods]
impl PyStream {
    /// Creates a stream on a new or retained context for `device` (default `0`).
    #[new]
    #[pyo3(signature = (device=0))]
    fn new(device: usize) -> PyResult<Self> {
        let ctx = Context::new(device)
            .map_err(|e| record_and_err("core", "Stream::new", e, ErrorKind::Cuda))?;
        let inner = ctx
            .new_stream()
            .map_err(|e| record_and_err("core", "Stream::new", e, ErrorKind::Cuda))?;
        Ok(Self {
            inner,
            device_id: device as i32,
        })
    }

    /// Blocks until all preceding commands in this stream have completed.
    fn synchronize(&self, py: Python<'_>) -> PyResult<()> {
        let stream = self.inner.clone();
        py.detach(move || stream.synchronize())
            .map_err(|e| record_and_err("core", "stream_sync", e, ErrorKind::Cuda))
    }

    /// Raw `CUstream` handle integer for interop with PyTorch / CuPy.
    #[getter]
    fn raw_ptr(&self) -> u64 {
        self.inner.cu_stream() as u64
    }

    #[getter]
    fn device_id(&self) -> i32 {
        self.device_id
    }

    fn __repr__(&self) -> String {
        format!(
            "Stream(device_id={}, ptr=0x{:x})",
            self.device_id,
            self.raw_ptr()
        )
    }
}

/// Backing storage for [`PyDeviceBuffer`]: either owned `cuda_core::DeviceBuffer<u8>` or borrowed DLPack tensor.
enum Backing {
    Owned {
        buf: Arc<CudaDeviceBuffer<u8>>,
        nbytes: usize,
    },
    BorrowedDlPack {
        managed: *mut DLManagedTensor,
        ptr: u64,
        nbytes: usize,
    },
}

// SAFETY: CUDA device pointers and `DLManagedTensor` ownership are thread-safe to transfer/drop across threads.
unsafe impl Send for Backing {}
// SAFETY: Shared immutable references only expose the device pointer and byte length.
unsafe impl Sync for Backing {}

impl Drop for Backing {
    fn drop(&mut self) {
        match self {
            Backing::Owned { nbytes, .. } => {
                diag::record_gpu_free(*nbytes);
            }
            Backing::BorrowedDlPack { managed, .. } => {
                if !managed.is_null() {
                    // SAFETY: We own this `DLManagedTensor` extracted from `"dltensor"` capsule.
                    if let Some(del) = unsafe { (**managed).deleter } {
                        // SAFETY: Calling consumer deleter once on Drop.
                        unsafe { del(*managed) };
                    }
                    *managed = std::ptr::null_mut();
                }
            }
        }
    }
}

impl Backing {
    fn device_ptr(&self) -> u64 {
        match self {
            Backing::Owned { buf, .. } => buf.cu_deviceptr(),
            Backing::BorrowedDlPack { ptr, .. } => *ptr,
        }
    }

    fn nbytes(&self) -> usize {
        match self {
            Backing::Owned { nbytes, .. } | Backing::BorrowedDlPack { nbytes, .. } => *nbytes,
        }
    }

    fn as_owned_cuda(&self) -> Option<&Arc<CudaDeviceBuffer<u8>>> {
        match self {
            Backing::Owned { buf, .. } => Some(buf),
            Backing::BorrowedDlPack { .. } => None,
        }
    }
}

fn compute_row_major_strides(shape: &[i64]) -> Vec<i64> {
    if shape.is_empty() {
        return Vec::new();
    }
    let mut strides = vec![1i64; shape.len()];
    for i in (0..shape.len().saturating_sub(1)).rev() {
        strides[i] = strides[i + 1].saturating_mul(shape[i + 1].max(1));
    }
    strides
}

/// GPU device buffer supporting DLPack, `__cuda_array_interface__` v3, and Python Buffer Protocol.
#[pyclass(name = "DeviceBuffer", module = "nv_rs.core")]
#[derive(Clone)]
pub struct PyDeviceBuffer {
    backing: Arc<Backing>,
    device_id: i32,
    dtype: String,
    shape: Vec<i64>,
    strides: Vec<i64>,
}

impl PyDeviceBuffer {
    /// Wraps an already-allocated `cuda_core::DeviceBuffer<u8>` and tracks it in `diag`.
    pub fn from_owned_cuda(
        buf: CudaDeviceBuffer<u8>,
        device_id: i32,
        dtype: impl Into<String>,
        shape: Vec<i64>,
    ) -> Self {
        let nbytes = buf.len();
        diag::record_gpu_alloc(nbytes);
        let strides = compute_row_major_strides(&shape);
        Self {
            backing: Arc::new(Backing::Owned {
                buf: Arc::new(buf),
                nbytes,
            }),
            device_id,
            dtype: dtype.into(),
            shape,
            strides,
        }
    }

    /// Returns the underlying `cuda_core::DeviceBuffer<u8>` if owned by `nv_rs`.
    pub fn owned_cuda_buf(&self) -> Option<&Arc<CudaDeviceBuffer<u8>>> {
        self.backing.as_owned_cuda()
    }

    /// Raw CUDA device pointer (`CUdeviceptr`).
    pub fn raw_device_ptr(&self) -> u64 {
        self.backing.device_ptr()
    }

    /// Total byte size of the backing allocation (owned or borrowed DLPack).
    pub fn byte_len(&self) -> usize {
        self.backing.nbytes()
    }
}

#[pymethods]
impl PyDeviceBuffer {
    /// Allocates uninitialized device memory on `stream` for `shape` and `dtype` (default `"u8"`).
    #[staticmethod]
    #[pyo3(signature = (stream, shape, dtype="u8"))]
    fn alloc(stream: &PyStream, shape: Vec<i64>, dtype: &str) -> PyResult<Self> {
        let (_, _, itemsize) = parse_dtype(dtype).map_err(|e| {
            record_and_err("core", "DeviceBuffer::alloc", e, ErrorKind::InvalidArgument)
        })?;
        let numel = shape
            .iter()
            .try_fold(1usize, |acc, &dim| {
                if dim < 0 {
                    return Err("negative dimension in shape");
                }
                acc.checked_mul(dim as usize)
                    .ok_or("shape element count overflow")
            })
            .map_err(|e| {
                record_and_err("core", "DeviceBuffer::alloc", e, ErrorKind::InvalidArgument)
            })?;
        let nbytes = numel.checked_mul(itemsize).ok_or_else(|| {
            record_and_err(
                "core",
                "DeviceBuffer::alloc",
                "byte size overflow",
                ErrorKind::InvalidArgument,
            )
        })?;
        // SAFETY: Uninitialized GPU memory is immediately owned by `PyDeviceBuffer`; caller writes before reading.
        let buf = unsafe { CudaDeviceBuffer::<u8>::uninitialized_async(&stream.inner, nbytes) }
            .map_err(|e| record_and_err("core", "DeviceBuffer::alloc", e, ErrorKind::Cuda))?;
        Ok(Self::from_owned_cuda(buf, stream.device_id, dtype, shape))
    }

    /// Uploads host bytes / `bytearray` / `memoryview` / `numpy.ndarray` (any dtype) to GPU via Python Buffer Protocol.
    #[staticmethod]
    #[pyo3(signature = (stream, data, shape=None, dtype=None))]
    fn from_buffer(
        py: Python<'_>,
        stream: &PyStream,
        data: &Bound<'_, PyAny>,
        shape: Option<Vec<i64>>,
        dtype: Option<&str>,
    ) -> PyResult<Self> {
        let inferred_shape = if shape.is_none() {
            data.getattr("shape")
                .ok()
                .and_then(|s| s.extract::<Vec<i64>>().ok())
        } else {
            None
        };
        let inferred_dtype = if dtype.is_none() {
            data.getattr("dtype")
                .ok()
                .and_then(|d| d.getattr("str").ok())
                .and_then(|s| s.extract::<String>().ok())
                .and_then(|typestr| {
                    // Normalize numpy typestr to our canonical short dtype if supported.
                    match typestr.as_str() {
                        "|u1" => Some("u8".to_owned()),
                        "|i1" => Some("i8".to_owned()),
                        "<u2" => Some("u16".to_owned()),
                        "<i2" => Some("i16".to_owned()),
                        "<u4" => Some("u32".to_owned()),
                        "<i4" => Some("i32".to_owned()),
                        "<u8" => Some("u64".to_owned()),
                        "<i8" => Some("i64".to_owned()),
                        "<f2" => Some("f16".to_owned()),
                        "<f4" => Some("f32".to_owned()),
                        "<f8" => Some("f64".to_owned()),
                        "|b1" => Some("bool".to_owned()),
                        _ => None,
                    }
                })
        } else {
            None
        };

        // Cast any buffer (including non-u8 numpy arrays like float32) to a contiguous byte view.
        let byte_view = match PyBuffer::<u8>::get(data) {
            Ok(b) => b,
            Err(_) => {
                let builtins = py.import("builtins")?;
                let mv = builtins.call_method1("memoryview", (data,))?;
                let byte_mv = mv.call_method1("cast", ("B",))?;
                PyBuffer::<u8>::get(&byte_mv).map_err(|e| {
                    record_and_err("core", "from_buffer", e, ErrorKind::InvalidArgument)
                })?
            }
        };

        let nbytes = byte_view.len_bytes();
        let mut host = vec![0u8; nbytes];
        byte_view
            .copy_to_slice(py, &mut host)
            .map_err(|e| record_and_err("core", "from_buffer", e, ErrorKind::InvalidArgument))?;

        let resolved_dtype = dtype
            .map(ToOwned::to_owned)
            .or(inferred_dtype)
            .unwrap_or_else(|| "u8".to_owned());
        let (_, _, itemsize) = parse_dtype(&resolved_dtype)
            .map_err(|e| record_and_err("core", "from_buffer", e, ErrorKind::InvalidArgument))?;
        let resolved_shape = shape
            .or(inferred_shape)
            .unwrap_or_else(|| vec![(nbytes / itemsize.max(1)) as i64]);

        let s_inner = stream.inner.clone();
        let dev_buf = py
            .detach(move || CudaDeviceBuffer::from_host(&s_inner, &host))
            .map_err(|e| record_and_err("core", "from_buffer", e, ErrorKind::Cuda))?;
        Ok(Self::from_owned_cuda(
            dev_buf,
            stream.device_id,
            resolved_dtype,
            resolved_shape,
        ))
    }

    /// Zero-copy import from any object implementing `__dlpack__` (PyTorch, CuPy, JAX) or a `"dltensor"` capsule.
    #[staticmethod]
    #[pyo3(signature = (obj, stream=None))]
    fn from_dlpack(
        py: Python<'_>,
        obj: &Bound<'_, PyAny>,
        stream: Option<&PyStream>,
    ) -> PyResult<Self> {
        let capsule_bound: Bound<'_, PyAny> = if obj.hasattr("__dlpack__")? {
            if let Some(s) = stream {
                obj.call_method1("__dlpack__", (s.raw_ptr(),))?
            } else {
                obj.call_method0("__dlpack__")?
            }
        } else {
            obj.clone()
        };

        let cap = capsule_bound.cast::<PyCapsule>().map_err(|_| {
            record_and_err(
                "core",
                "from_dlpack",
                "expected PyCapsule or object with __dlpack__",
                ErrorKind::InvalidArgument,
            )
        })?;

        // SAFETY: Check capsule name and extract `DLManagedTensor*` per DLPack protocol.
        let raw_ptr = unsafe {
            pyo3::ffi::PyCapsule_GetPointer(cap.as_ptr(), DLTENSOR_CAPSULE_NAME.as_ptr().cast())
        };
        if raw_ptr.is_null() {
            return Err(record_and_err(
                "core",
                "from_dlpack",
                "invalid or already-consumed DLPack capsule",
                ErrorKind::InvalidArgument,
            ));
        }

        // Rename capsule to `"used_dltensor"` so Python GC won't double-free it.
        // SAFETY: `cap.as_ptr()` is a valid PyCapsule and `USED_DLTENSOR_CAPSULE_NAME` is static NUL-terminated.
        let rc = unsafe {
            pyo3::ffi::PyCapsule_SetName(cap.as_ptr(), USED_DLTENSOR_CAPSULE_NAME.as_ptr().cast())
        };
        if rc != 0 {
            return Err(PyErr::fetch(py));
        }

        let managed = raw_ptr.cast::<DLManagedTensor>();
        // SAFETY: Valid `DLManagedTensor` transferred to us by DLPack producer.
        let dl = unsafe { &(*managed).dl_tensor };
        if dl.device.device_type != DL_CUDA {
            // Free immediately before returning error so we don't leak the transferred capsule.
            // SAFETY: `managed` belongs to us now.
            if let Some(del) = unsafe { (*managed).deleter } {
                // SAFETY: Calling producer deleter once on error path.
                unsafe { del(managed) };
            }
            return Err(record_and_err(
                "core",
                "from_dlpack",
                format!(
                    "expected CUDA DLPack device (type={DL_CUDA}), got {}",
                    dl.device.device_type
                ),
                ErrorKind::InvalidArgument,
            ));
        }

        let dtype_str = format_dl_dtype(dl.dtype).map_err(|e| {
            // SAFETY: Reading deleter function pointer from a valid `DLManagedTensor`.
            if let Some(del) = unsafe { (*managed).deleter } {
                // SAFETY: Calling producer deleter once on error path.
                unsafe { del(managed) };
            }
            record_and_err("core", "from_dlpack", e, ErrorKind::InvalidArgument)
        })?;
        let (_, _, itemsize) = parse_dtype(dtype_str).unwrap_or((dl.dtype, "|u1", 1));

        let ndim = dl.ndim.max(0) as usize;
        let shape: Vec<i64> = if ndim > 0 && !dl.shape.is_null() {
            // SAFETY: `dl.shape` has `ndim` elements when `ndim > 0` and non-null.
            unsafe { std::slice::from_raw_parts(dl.shape, ndim) }.to_vec()
        } else {
            Vec::new()
        };
        let strides: Vec<i64> = if ndim > 0 && !dl.strides.is_null() {
            // SAFETY: `dl.strides` has `ndim` elements when non-null.
            unsafe { std::slice::from_raw_parts(dl.strides, ndim) }.to_vec()
        } else {
            compute_row_major_strides(&shape)
        };
        let numel: usize = shape.iter().map(|&d| d.max(0) as usize).product();
        let nbytes = numel.saturating_mul(itemsize);
        let ptr = (dl.data as usize as u64).saturating_add(dl.byte_offset);
        let device_id = dl.device.device_id;

        diag::record_event(
            Level::Trace,
            "core",
            "from_dlpack",
            format!("ptr=0x{ptr:x} nbytes={nbytes} dtype={dtype_str}"),
            0,
            true,
        );

        Ok(Self {
            backing: Arc::new(Backing::BorrowedDlPack {
                managed,
                ptr,
                nbytes,
            }),
            device_id,
            dtype: dtype_str.to_owned(),
            shape,
            strides,
        })
    }

    /// Exports this buffer as a zero-copy DLPack `"dltensor"` `PyCapsule`.
    #[pyo3(signature = (stream=None))]
    fn __dlpack__<'py>(
        &self,
        py: Python<'py>,
        stream: Option<u64>,
    ) -> PyResult<Bound<'py, PyCapsule>> {
        let _ = stream;
        let (dl_dtype, _, _) = parse_dtype(&self.dtype)
            .map_err(|e| record_and_err("core", "__dlpack__", e, ErrorKind::InvalidArgument))?;
        let managed = into_dlpack_managed(
            self.backing.clone(),
            self.backing.device_ptr(),
            self.device_id,
            dl_dtype,
            self.shape.clone(),
            self.strides.clone(),
        );
        // SAFETY: `managed` is a valid heap-allocated `DLManagedTensor`; `pycapsule_dlpack_destructor` frees it if unconsumed.
        let cap_ptr = unsafe {
            pyo3::ffi::PyCapsule_New(
                managed.cast::<c_void>(),
                DLTENSOR_CAPSULE_NAME.as_ptr().cast(),
                Some(pycapsule_dlpack_destructor),
            )
        };
        if cap_ptr.is_null() {
            // SAFETY: Capsule creation failed, reclaim `managed`.
            unsafe { crate::dlpack::dlpack_deleter::<Arc<Backing>>(managed) };
            return Err(PyErr::fetch(py));
        }
        // SAFETY: `cap_ptr` is a newly created owned `PyObject*` reference to a `PyCapsule`.
        let bound = unsafe { Bound::from_owned_ptr(py, cap_ptr) };
        Ok(bound.cast_into::<PyCapsule>()?)
    }

    /// DLPack protocol device tuple `(kDLCUDA, device_id)`.
    fn __dlpack_device__(&self) -> (i32, i32) {
        (DL_CUDA, self.device_id)
    }

    /// `__cuda_array_interface__` v3 dictionary for zero-copy CuPy / Numba / PyTorch interop.
    #[getter]
    fn __cuda_array_interface__(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let (_, typestr, _) = parse_dtype(&self.dtype).map_err(|e| {
            record_and_err(
                "core",
                "__cuda_array_interface__",
                e,
                ErrorKind::InvalidArgument,
            )
        })?;
        let d = PyDict::new(py);
        let shape_tuple = PyTuple::new(py, self.shape.iter().copied())?;
        let data_tuple = PyTuple::new(
            py,
            [
                self.backing.device_ptr().into_pyobject(py)?.into_any(),
                false.into_pyobject(py)?.to_owned().into_any(),
            ],
        )?;
        d.set_item("shape", shape_tuple)?;
        d.set_item("typestr", typestr)?;
        d.set_item("data", data_tuple)?;
        d.set_item("version", 3)?;
        Ok(d.unbind())
    }

    /// Downloads device memory to host `bytes`, synchronizing `stream`.
    fn to_bytes<'py>(&self, py: Python<'py>, stream: &PyStream) -> PyResult<Bound<'py, PyBytes>> {
        let owned = self
            .owned_cuda_buf()
            .ok_or_else(|| {
                record_and_err(
                    "core",
                    "to_bytes",
                    "to_bytes currently requires an nv_rs-owned DeviceBuffer",
                    ErrorKind::InvalidArgument,
                )
            })?
            .clone();
        let s_inner = stream.inner.clone();
        let host = py
            .detach(move || {
                s_inner.synchronize()?;
                owned.to_host_vec(&s_inner)
            })
            .map_err(|e| record_and_err("core", "to_bytes", e, ErrorKind::Cuda))?;
        Ok(PyBytes::new(py, &host))
    }

    #[getter]
    fn ptr(&self) -> u64 {
        self.backing.device_ptr()
    }

    #[getter]
    fn nbytes(&self) -> usize {
        self.backing.nbytes()
    }

    #[getter]
    fn shape(&self) -> Vec<i64> {
        self.shape.clone()
    }

    #[getter]
    fn dtype(&self) -> &str {
        &self.dtype
    }

    #[getter]
    fn device_id(&self) -> i32 {
        self.device_id
    }

    fn __repr__(&self) -> String {
        format!(
            "DeviceBuffer(ptr=0x{:x}, nbytes={}, dtype='{}', shape={:?}, device_id={})",
            self.ptr(),
            self.nbytes(),
            self.dtype,
            self.shape,
            self.device_id
        )
    }
}

/// Registers the `nv_rs.core` submodule.
pub fn register(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(py, "core")?;
    m.add_class::<PyContext>()?;
    m.add_class::<PyStream>()?;
    m.add_class::<PyDeviceBuffer>()?;
    parent.add_submodule(&m)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("nv_rs.core", m)?;
    Ok(())
}
