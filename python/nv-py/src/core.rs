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

/// RAII guard that holds unconsumed ownership of a [`DLManagedTensor`].
///
/// If dropped before ownership is transferred via [`DlManagedGuard::into_raw`],
/// it cleanly invokes the producer's `deleter` function exactly once without leaking.
struct DlManagedGuard {
    managed: *mut DLManagedTensor,
}

impl DlManagedGuard {
    fn new(managed: *mut DLManagedTensor) -> Self {
        Self { managed }
    }

    fn into_raw(mut self) -> *mut DLManagedTensor {
        let ptr = self.managed;
        self.managed = std::ptr::null_mut();
        ptr
    }
}

impl Drop for DlManagedGuard {
    fn drop(&mut self) {
        if !self.managed.is_null() {
            // SAFETY: We have exclusive unconsumed ownership of `self.managed`.
            // Producer deleter is invoked cleanly once on error/early exit.
            unsafe {
                if let Some(del) = (*self.managed).deleter {
                    del(self.managed);
                }
            }
            self.managed = std::ptr::null_mut();
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

/// Validates that dimensions in `shape` are non-negative, computes element count and expected byte count
/// using checked arithmetic against `itemsize`.
fn validate_shape_and_bytes(shape: &[i64], itemsize: usize) -> Result<(usize, usize), String> {
    if itemsize == 0 {
        return Err("dtype itemsize must be greater than 0".to_owned());
    }
    let numel = shape.iter().try_fold(1usize, |acc, &dim| {
        if dim < 0 {
            return Err(format!("negative dimension in shape: {dim}"));
        }
        let udim = usize::try_from(dim).map_err(|_| format!("dimension out of range: {dim}"))?;
        acc.checked_mul(udim)
            .ok_or_else(|| "shape element count overflow".to_owned())
    })?;
    let nbytes = numel
        .checked_mul(itemsize)
        .ok_or_else(|| "byte size overflow for shape and dtype".to_owned())?;
    Ok((numel, nbytes))
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
    stream: Option<Arc<Stream>>,
}

impl PyDeviceBuffer {
    /// Wraps an already-allocated `cuda_core::DeviceBuffer<u8>` and tracks it in `diag`.
    pub fn from_owned_cuda(
        buf: CudaDeviceBuffer<u8>,
        device_id: i32,
        dtype: impl Into<String>,
        shape: Vec<i64>,
    ) -> Self {
        Self::from_owned_cuda_with_stream(buf, device_id, dtype, shape, None)
    }

    /// Wraps an already-allocated `cuda_core::DeviceBuffer<u8>` with an associated stream.
    pub fn from_owned_cuda_with_stream(
        buf: CudaDeviceBuffer<u8>,
        device_id: i32,
        dtype: impl Into<String>,
        shape: Vec<i64>,
        stream: Option<Arc<Stream>>,
    ) -> Self {
        let dtype_str = dtype.into();
        let (_, _, itemsize) =
            parse_dtype(&dtype_str).unwrap_or_else(|e| panic!("from_owned_cuda: {e}"));
        let (_, expected_bytes) = validate_shape_and_bytes(&shape, itemsize)
            .unwrap_or_else(|e| panic!("from_owned_cuda: {e}"));
        assert_eq!(
            buf.len(),
            expected_bytes,
            "CudaDeviceBuffer length ({}) does not match shape {:?} and dtype '{}' (expected {} bytes)",
            buf.len(),
            shape,
            dtype_str,
            expected_bytes
        );
        let nbytes = buf.len();
        diag::record_gpu_alloc(nbytes);
        let strides = compute_row_major_strides(&shape);
        Self {
            backing: Arc::new(Backing::Owned {
                buf: Arc::new(buf),
                nbytes,
            }),
            device_id,
            dtype: dtype_str,
            shape,
            strides,
            stream,
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

    /// Returns true if the buffer's layout is row-major (C-contiguous).
    pub fn is_c_contiguous(&self) -> bool {
        if self.shape.is_empty() {
            return true;
        }
        let expected = compute_row_major_strides(&self.shape);
        self.strides == expected
    }

    /// Helper to synchronize the producer stream when exporting to a consumer.
    fn sync_for_consumer_stream(
        &self,
        py: Python<'_>,
        consumer_stream: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<()> {
        let Some(ref prod_stream) = self.stream else {
            return Ok(());
        };

        let mut do_sync = true;
        if let Some(stream_obj) = consumer_stream {
            if stream_obj.is_none() {
                // stream=None in DLPack: synchronize producer stream
                do_sync = true;
            } else if let Ok(val) = stream_obj.extract::<i64>() {
                if val == -1 {
                    // DLPack spec: sentinel -1 explicitly requests no synchronization
                    do_sync = false;
                } else if val >= 0 {
                    let prod_handle = prod_stream.cu_stream() as i64;
                    if prod_handle == val {
                        do_sync = false;
                    }
                }
            } else if let Ok(py_stream) = stream_obj.extract::<PyRef<'_, PyStream>>() {
                let prod_handle = prod_stream.cu_stream() as u64;
                let cons_handle = py_stream.inner.cu_stream() as u64;
                if prod_handle == cons_handle {
                    do_sync = false;
                }
            }
        }

        if do_sync {
            let s = prod_stream.clone();
            py.detach(move || s.synchronize())
                .map_err(|e| record_and_err("core", "__dlpack__", e, ErrorKind::Cuda))?;
        }
        Ok(())
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
        let (_, nbytes) = validate_shape_and_bytes(&shape, itemsize).map_err(|e| {
            record_and_err("core", "DeviceBuffer::alloc", e, ErrorKind::InvalidArgument)
        })?;
        // SAFETY: Uninitialized GPU memory is immediately owned by `PyDeviceBuffer`; caller writes before reading.
        let buf = unsafe { CudaDeviceBuffer::<u8>::uninitialized_async(&stream.inner, nbytes) }
            .map_err(|e| record_and_err("core", "DeviceBuffer::alloc", e, ErrorKind::Cuda))?;
        Ok(Self::from_owned_cuda_with_stream(
            buf,
            stream.device_id,
            dtype,
            shape,
            Some(stream.inner.clone()),
        ))
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

        let resolved_shape = if let Some(s) = shape {
            let (_, expected_bytes) = validate_shape_and_bytes(&s, itemsize).map_err(|e| {
                record_and_err("core", "from_buffer", e, ErrorKind::InvalidArgument)
            })?;
            if expected_bytes != nbytes {
                return Err(record_and_err(
                    "core",
                    "from_buffer",
                    format!(
                        "buffer byte length ({nbytes}) does not match explicit shape {s:?} with dtype '{resolved_dtype}' (expected {expected_bytes} bytes)"
                    ),
                    ErrorKind::InvalidArgument,
                ));
            }
            s
        } else if let Some(s) = inferred_shape {
            let (_, expected_bytes) = validate_shape_and_bytes(&s, itemsize).map_err(|e| {
                record_and_err("core", "from_buffer", e, ErrorKind::InvalidArgument)
            })?;
            if expected_bytes != nbytes {
                return Err(record_and_err(
                    "core",
                    "from_buffer",
                    format!(
                        "buffer byte length ({nbytes}) does not match inferred shape {s:?} with dtype '{resolved_dtype}' (expected {expected_bytes} bytes)"
                    ),
                    ErrorKind::InvalidArgument,
                ));
            }
            s
        } else {
            if nbytes % itemsize != 0 {
                return Err(record_and_err(
                    "core",
                    "from_buffer",
                    format!(
                        "buffer byte length ({nbytes}) is not a multiple of dtype '{resolved_dtype}' itemsize ({itemsize})"
                    ),
                    ErrorKind::InvalidArgument,
                ));
            }
            if nbytes == 0 {
                vec![0]
            } else {
                vec![(nbytes / itemsize) as i64]
            }
        };

        let s_inner = stream.inner.clone();
        let dev_buf = py
            .detach(move || CudaDeviceBuffer::from_host(&s_inner, &host))
            .map_err(|e| record_and_err("core", "from_buffer", e, ErrorKind::Cuda))?;
        Ok(Self::from_owned_cuda_with_stream(
            dev_buf,
            stream.device_id,
            resolved_dtype,
            resolved_shape,
            Some(stream.inner.clone()),
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
        // Wrap in RAII guard immediately after taking ownership of the renamed capsule.
        let guard = DlManagedGuard::new(managed);

        // SAFETY: `managed` is non-null and points to a valid `DLManagedTensor` transferred to us.
        // Copy all scalar fields upfront before any validation or potential error exit.
        let (
            device_type,
            device_id,
            dtype_raw,
            data_ptr,
            byte_offset,
            ndim_raw,
            shape_raw,
            strides_raw,
        ) = unsafe {
            let dl = &(*managed).dl_tensor;
            (
                dl.device.device_type,
                dl.device.device_id,
                dl.dtype,
                dl.data,
                dl.byte_offset,
                dl.ndim,
                dl.shape,
                dl.strides,
            )
        };

        if device_type != DL_CUDA {
            return Err(record_and_err(
                "core",
                "from_dlpack",
                format!("expected CUDA DLPack device (type={DL_CUDA}), got {device_type}"),
                ErrorKind::InvalidArgument,
            ));
        }

        let dtype_str = format_dl_dtype(dtype_raw)
            .map_err(|e| record_and_err("core", "from_dlpack", e, ErrorKind::InvalidArgument))?;
        let (_, _, itemsize) = parse_dtype(dtype_str)
            .map_err(|e| record_and_err("core", "from_dlpack", e, ErrorKind::InvalidArgument))?;

        if ndim_raw < 0 {
            return Err(record_and_err(
                "core",
                "from_dlpack",
                format!("negative ndim in DLPack tensor: {ndim_raw}"),
                ErrorKind::InvalidArgument,
            ));
        }
        let ndim = ndim_raw as usize;
        let shape: Vec<i64> = if ndim > 0 {
            if shape_raw.is_null() {
                return Err(record_and_err(
                    "core",
                    "from_dlpack",
                    "null shape pointer with positive ndim in DLPack tensor",
                    ErrorKind::InvalidArgument,
                ));
            }
            // SAFETY: `shape_raw` is non-null and points to `ndim` elements of i64.
            unsafe { std::slice::from_raw_parts(shape_raw, ndim) }.to_vec()
        } else {
            Vec::new()
        };

        let (_numel, nbytes) = validate_shape_and_bytes(&shape, itemsize)
            .map_err(|e| record_and_err("core", "from_dlpack", e, ErrorKind::InvalidArgument))?;

        let strides: Vec<i64> = if ndim > 0 && !strides_raw.is_null() {
            // SAFETY: `strides_raw` is non-null and points to `ndim` elements of i64.
            unsafe { std::slice::from_raw_parts(strides_raw, ndim) }.to_vec()
        } else {
            compute_row_major_strides(&shape)
        };

        if data_ptr.is_null() && nbytes > 0 {
            return Err(record_and_err(
                "core",
                "from_dlpack",
                "null data pointer in non-empty DLPack tensor",
                ErrorKind::InvalidArgument,
            ));
        }
        let base_ptr = data_ptr as usize as u64;
        let ptr = base_ptr.checked_add(byte_offset).ok_or_else(|| {
            record_and_err(
                "core",
                "from_dlpack",
                "pointer overflow with byte_offset in DLPack tensor",
                ErrorKind::InvalidArgument,
            )
        })?;

        // Transfer ownership of `managed` out of the guard to `Backing::BorrowedDlPack`.
        let managed = guard.into_raw();

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
            stream: stream.map(|s| s.inner.clone()),
        })
    }

    /// Exports this buffer as a zero-copy DLPack `"dltensor"` `PyCapsule`.
    #[pyo3(signature = (stream=None))]
    fn __dlpack__<'py>(
        &self,
        py: Python<'py>,
        stream: Option<&Bound<'py, PyAny>>,
    ) -> PyResult<Bound<'py, PyCapsule>> {
        self.sync_for_consumer_stream(py, stream)?;

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
        let (_, typestr, itemsize) = parse_dtype(&self.dtype).map_err(|e| {
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

        // Export byte strides if non-empty, or None for scalar (0-d array).
        if self.shape.is_empty() {
            d.set_item("strides", py.None())?;
        } else {
            let byte_strides: Result<Vec<i64>, _> = self
                .strides
                .iter()
                .map(|&s| {
                    s.checked_mul(itemsize as i64)
                        .ok_or_else(|| "stride overflow when converting to bytes".to_owned())
                })
                .collect();
            let byte_strides = byte_strides.map_err(|e| {
                record_and_err(
                    "core",
                    "__cuda_array_interface__",
                    e,
                    ErrorKind::InvalidArgument,
                )
            })?;
            let strides_tuple = PyTuple::new(py, byte_strides)?;
            d.set_item("strides", strides_tuple)?;
        }

        // Export stream metadata: integer stream handle or None.
        if let Some(ref s) = self.stream {
            d.set_item("stream", s.cu_stream() as u64)?;
        } else {
            d.set_item("stream", py.None())?;
        }

        Ok(d.unbind())
    }

    /// Downloads device memory to host `bytes`, synchronizing `stream`.
    fn to_bytes<'py>(&self, py: Python<'py>, stream: &PyStream) -> PyResult<Bound<'py, PyBytes>> {
        if !self.is_c_contiguous() {
            return Err(record_and_err(
                "core",
                "to_bytes",
                "to_bytes requires a contiguous buffer; non-contiguous layout is not supported",
                ErrorKind::InvalidArgument,
            ));
        }
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
    fn strides(&self) -> Vec<i64> {
        self.strides.clone()
    }

    #[getter]
    fn is_contiguous(&self) -> bool {
        self.is_c_contiguous()
    }

    #[getter]
    fn dtype(&self) -> &str {
        &self.dtype
    }

    #[getter]
    fn device_id(&self) -> i32 {
        self.device_id
    }

    #[getter]
    fn stream(&self) -> Option<PyStream> {
        self.stream.as_ref().map(|s| PyStream {
            inner: s.clone(),
            device_id: self.device_id,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_validate_shape_and_bytes_scalar() {
        let (numel, nbytes) = validate_shape_and_bytes(&[], 4).expect("scalar shape is valid");
        assert_eq!(numel, 1);
        assert_eq!(nbytes, 4);
    }

    #[test]
    fn test_validate_shape_and_bytes_zero_dim() {
        let (numel, nbytes) = validate_shape_and_bytes(&[0], 4).expect("zero-dim shape is valid");
        assert_eq!(numel, 0);
        assert_eq!(nbytes, 0);

        let (numel2, nbytes2) =
            validate_shape_and_bytes(&[10, 0, 5], 2).expect("zero-dim multi-dim shape is valid");
        assert_eq!(numel2, 0);
        assert_eq!(nbytes2, 0);
    }

    #[test]
    fn test_validate_shape_and_bytes_1d_and_nd() {
        let (numel, nbytes) = validate_shape_and_bytes(&[1024], 4).expect("1d f32 shape is valid");
        assert_eq!(numel, 1024);
        assert_eq!(nbytes, 4096);

        let (numel, nbytes) =
            validate_shape_and_bytes(&[2, 3, 4], 8).expect("3d u64 shape is valid");
        assert_eq!(numel, 24);
        assert_eq!(nbytes, 192);
    }

    #[test]
    fn test_validate_shape_and_bytes_rejects_negative() {
        let err = validate_shape_and_bytes(&[-1], 4).expect_err("must reject negative dimension");
        assert!(err.contains("negative dimension"), "got: {err}");

        let err2 = validate_shape_and_bytes(&[10, -5, 2], 4)
            .expect_err("must reject negative dimension in 3D");
        assert!(err2.contains("negative dimension"), "got: {err2}");
    }

    #[test]
    fn test_validate_shape_and_bytes_rejects_overflow() {
        let err = validate_shape_and_bytes(&[i64::MAX, 3], 1)
            .expect_err("must reject element count overflow");
        assert!(err.contains("overflow"), "got: {err}");

        let err2 =
            validate_shape_and_bytes(&[i64::MAX], 4).expect_err("must reject byte size overflow");
        assert!(err2.contains("overflow"), "got: {err2}");
    }

    #[test]
    fn test_validate_shape_and_bytes_rejects_zero_itemsize() {
        let err = validate_shape_and_bytes(&[10], 0).expect_err("must reject zero itemsize");
        assert!(err.contains("itemsize"), "got: {err}");
    }

    #[test]
    fn test_compute_row_major_strides() {
        assert_eq!(compute_row_major_strides(&[]), Vec::<i64>::new());
        assert_eq!(compute_row_major_strides(&[10]), vec![1]);
        assert_eq!(compute_row_major_strides(&[2, 3]), vec![3, 1]);
        assert_eq!(compute_row_major_strides(&[2, 3, 4]), vec![12, 4, 1]);
    }

    static DELETER_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "C" fn test_mock_deleter(self_: *mut DLManagedTensor) {
        DELETER_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
        if !self_.is_null() {
            // SAFETY: `self_` was allocated via Box::into_raw in test.
            let _ = unsafe { Box::from_raw(self_) };
        }
    }

    #[test]
    fn test_dl_managed_guard_drop_invokes_deleter() {
        DELETER_CALL_COUNT.store(0, Ordering::SeqCst);
        let tensor = Box::new(DLManagedTensor {
            dl_tensor: crate::dlpack::DLTensor {
                data: std::ptr::null_mut(),
                device: crate::dlpack::DLDevice {
                    device_type: crate::dlpack::DL_CPU,
                    device_id: 0,
                },
                ndim: 0,
                dtype: crate::dlpack::DLDataType {
                    code: 1,
                    bits: 8,
                    lanes: 1,
                },
                shape: std::ptr::null_mut(),
                strides: std::ptr::null_mut(),
                byte_offset: 0,
            },
            manager_ctx: std::ptr::null_mut(),
            deleter: Some(test_mock_deleter),
        });
        let raw = Box::into_raw(tensor);
        {
            let _guard = DlManagedGuard::new(raw);
            // _guard dropped at end of scope on simulated error
        }
        assert_eq!(
            DELETER_CALL_COUNT.load(Ordering::SeqCst),
            1,
            "deleter must be invoked exactly once on drop"
        );
    }

    #[test]
    fn test_dl_managed_guard_into_raw_suppresses_drop_deleter() {
        DELETER_CALL_COUNT.store(0, Ordering::SeqCst);
        let tensor = Box::new(DLManagedTensor {
            dl_tensor: crate::dlpack::DLTensor {
                data: std::ptr::null_mut(),
                device: crate::dlpack::DLDevice {
                    device_type: crate::dlpack::DL_CUDA,
                    device_id: 0,
                },
                ndim: 0,
                dtype: crate::dlpack::DLDataType {
                    code: 1,
                    bits: 8,
                    lanes: 1,
                },
                shape: std::ptr::null_mut(),
                strides: std::ptr::null_mut(),
                byte_offset: 0,
            },
            manager_ctx: std::ptr::null_mut(),
            deleter: Some(test_mock_deleter),
        });
        let raw = Box::into_raw(tensor);
        let retrieved = {
            let guard = DlManagedGuard::new(raw);
            guard.into_raw()
        };
        assert_eq!(
            DELETER_CALL_COUNT.load(Ordering::SeqCst),
            0,
            "deleter must not be invoked when into_raw transfers ownership"
        );
        // SAFETY: manually cleanup raw pointer
        unsafe {
            test_mock_deleter(retrieved);
        }
        assert_eq!(DELETER_CALL_COUNT.load(Ordering::SeqCst), 1);
    }
}
