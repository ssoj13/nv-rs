//! `nv_rs.core` — CUDA Context, Stream, and zero-copy DeviceBuffer (DLPack + `__cuda_array_interface__` + Buffer Protocol).
//!
//! **Invariants every `PyDeviceBuffer` upholds** (the other binding modules rely on them):
//! - the layout is dense row-major: `from_dlpack` rejects any other strides (size-1 dimensions
//!   and empty tensors excepted) and stores canonical row-major strides, so `(ptr, nbytes)` is
//!   the exact span the elements occupy;
//! - `dtype` is the canonical short name from `dlpack::DTYPE_TABLE`;
//! - [`PyDeviceBuffer::storage`] is the `cuda_core::RawDeviceBuffer` that completion leases
//!   (`buf_util::enqueue_leased`) attach to: the owned allocation, or a foreign view whose owner
//!   keeps a DLPack producer's tensor alive (its deleter runs after the last view and lease);
//! - nv_rs-owned buffers always record the stream their contents are produced on, which
//!   `__dlpack__`, `__cuda_array_interface__` and `to_bytes` wait for.
//!
//! Argument validation for native calls lives in `buf_util::check_buf`.

use std::ffi::c_void;
use std::sync::Arc;
use std::time::Instant;

use cuda_core::{
    Context, DeviceAllocation, DeviceBuffer as CudaDeviceBuffer, RawDeviceBuffer, Stream,
};
use nv_loader::diag::{self, Level};
use pyo3::buffer::PyBuffer;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyCapsule, PyDict, PyTuple};

use crate::buf_util::{Access, check_buf, enqueue_leased};
use crate::dlpack::{
    DL_CUDA, DLManagedTensor, DLTENSOR_CAPSULE_NAME, USED_DLTENSOR_CAPSULE_NAME, find_dtype_entry,
    format_dl_dtype, into_dlpack_managed, parse_dtype, pycapsule_dlpack_destructor,
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

/// Producer-owned tensor memory adopted by [`PyDeviceBuffer::from_dlpack`].
///
/// It is the owner of a `RawDeviceBuffer::foreign` view, so a completion lease taken on that
/// view keeps the producer's tensor alive until the queued work completes. The producer's
/// deleter runs in `Drop`, after the last view and the last lease are gone; that can happen on
/// `cuda-core`'s retirement worker without the GIL, which DLPack deleters must tolerate
/// (NumPy, CuPy and PyTorch acquire the GIL themselves where they need it).
struct DlPackOwner {
    managed: *mut DLManagedTensor,
    ptr: u64,
    nbytes: usize,
    context: Arc<Context>,
}

// SAFETY: the producer transferred exclusive ownership of `managed` to us; it is touched only
// once, by `Drop`, and DLPack deleters may be invoked from any thread.
unsafe impl Send for DlPackOwner {}
// SAFETY: shared references read only the immutable `ptr`, `nbytes` and `context` fields.
unsafe impl Sync for DlPackOwner {}

impl Drop for DlPackOwner {
    fn drop(&mut self) {
        // SAFETY: `managed` is the non-null `DLManagedTensor` taken from a `"dltensor"` capsule;
        // its deleter is called exactly once, here.
        if let Some(del) = unsafe { (*self.managed).deleter } {
            // SAFETY: see above; ownership ends with this call.
            unsafe { del(self.managed) };
        }
    }
}

// SAFETY: `ptr..ptr + nbytes` is the producer's tensor extent on `context`'s device (overflow is
// rejected before construction) and stays valid until `Drop` runs the producer's deleter.
unsafe impl DeviceAllocation for DlPackOwner {
    fn device_ptr(&self) -> cuda_bindings::CUdeviceptr {
        self.ptr
    }
    fn len_bytes(&self) -> usize {
        self.nbytes
    }
    fn context(&self) -> Arc<Context> {
        self.context.clone()
    }
}

/// Backing storage for [`PyDeviceBuffer`]: either owned `cuda_core::DeviceBuffer<u8>` or borrowed DLPack tensor.
enum Backing {
    Owned {
        buf: Arc<CudaDeviceBuffer<u8>>,
        nbytes: usize,
    },
    /// Foreign view over a [`DlPackOwner`].
    BorrowedDlPack { raw: RawDeviceBuffer },
}

impl Drop for Backing {
    fn drop(&mut self) {
        if let Backing::Owned { nbytes, .. } = self {
            diag::record_gpu_free(*nbytes);
        }
    }
}

impl Backing {
    fn device_ptr(&self) -> u64 {
        match self {
            Backing::Owned { buf, .. } => buf.cu_deviceptr(),
            Backing::BorrowedDlPack { raw } => raw.cu_deviceptr(),
        }
    }

    fn nbytes(&self) -> usize {
        match self {
            Backing::Owned { nbytes, .. } => *nbytes,
            Backing::BorrowedDlPack { raw } => raw.len_bytes(),
        }
    }

    fn storage(&self) -> &RawDeviceBuffer {
        match self {
            Backing::Owned { buf, .. } => buf.storage(),
            Backing::BorrowedDlPack { raw } => raw,
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

/// Checks that element `strides` describe the dense row-major layout of `shape`.
///
/// Size-1 dimensions may carry any stride (they are never stepped over) and an empty tensor
/// addresses no element, so both are accepted; zero, negative or permuted strides are not,
/// because every native binding treats `(ptr, nbytes)` as one dense span.
fn check_c_contiguous(shape: &[i64], strides: &[i64]) -> Result<(), String> {
    if shape.contains(&0) {
        return Ok(());
    }
    let mut expected = 1i64;
    for (&dim, &stride) in shape.iter().zip(strides).rev() {
        if dim != 1 && stride != expected {
            return Err(format!(
                "non-contiguous DLPack tensor (shape {shape:?}, element strides {strides:?}); \
                 only dense row-major tensors are accepted, call .contiguous() first"
            ));
        }
        expected = expected
            .checked_mul(dim)
            .ok_or_else(|| "stride overflow in DLPack tensor".to_owned())?;
    }
    Ok(())
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

/// Checks that an owned allocation of `len` bytes holds `shape` elements of `dtype` and returns
/// the canonical dtype name; split from [`PyDeviceBuffer::from_owned_cuda`] so it is testable
/// without a GPU.
fn check_owned_layout(len: usize, dtype: &str, shape: &[i64]) -> Result<&'static str, String> {
    let entry = find_dtype_entry(dtype).ok_or_else(|| format!("unsupported dtype `{dtype}`"))?;
    let (_, expected) = validate_shape_and_bytes(shape, entry.itemsize)?;
    if len != expected {
        return Err(format!(
            "device buffer length ({len} bytes) does not match shape {shape:?} and dtype \
             '{dtype}' (expected {expected} bytes)"
        ));
    }
    Ok(entry.canonical)
}

impl PyDeviceBuffer {
    /// Wraps an nv_rs-owned allocation whose contents are produced by work queued on `stream`.
    ///
    /// `stream` becomes the buffer's producer stream: `__dlpack__`, `__cuda_array_interface__`
    /// and `to_bytes` wait for it, so pass the stream the filling work was queued on. Fails with
    /// `NvInvalidArgumentError` when `buf.len()` does not match `shape` x `dtype`, or when `buf`
    /// and `stream` belong to different CUDA contexts. The device id is taken from `stream`.
    pub fn from_owned_cuda(
        buf: CudaDeviceBuffer<u8>,
        dtype: &str,
        shape: Vec<i64>,
        stream: &Arc<Stream>,
    ) -> PyResult<Self> {
        let invalid =
            |e: String| record_and_err("core", "from_owned_cuda", e, ErrorKind::InvalidArgument);
        let canonical = check_owned_layout(buf.len(), dtype, &shape).map_err(invalid)?;
        if buf.storage().context().as_ref() != stream.context().as_ref() {
            return Err(invalid(
                "device buffer and producer stream belong to different CUDA contexts".to_owned(),
            ));
        }
        let ordinal = stream.context().ordinal();
        let device_id = i32::try_from(ordinal)
            .map_err(|_| invalid(format!("device ordinal {ordinal} does not fit i32")))?;
        let nbytes = buf.len();
        diag::record_gpu_alloc(nbytes);
        let strides = compute_row_major_strides(&shape);
        Ok(Self {
            backing: Arc::new(Backing::Owned {
                buf: Arc::new(buf),
                nbytes,
            }),
            device_id,
            dtype: canonical.to_owned(),
            shape,
            strides,
            stream: Some(stream.clone()),
        })
    }

    /// Returns the underlying `cuda_core::DeviceBuffer<u8>` if owned by `nv_rs`.
    pub fn owned_cuda_buf(&self) -> Option<&Arc<CudaDeviceBuffer<u8>>> {
        self.backing.as_owned_cuda()
    }

    /// Allocation identity and access tracker for completion leases (`buf_util::enqueue_leased`);
    /// covers owned buffers and DLPack imports alike.
    pub fn storage(&self) -> &RawDeviceBuffer {
        self.backing.storage()
    }

    /// Canonical dtype name (`dlpack::DTYPE_TABLE`).
    pub fn dtype_name(&self) -> &str {
        &self.dtype
    }

    /// Element count (`1` for a 0-d buffer).
    pub fn numel(&self) -> usize {
        // Shapes are validated at construction, so every dimension is non-negative.
        self.shape.iter().map(|&d| d as usize).product()
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
        Self::from_owned_cuda(buf, dtype, shape, &stream.inner)
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
                // NumPy typestrs are aliases in `DTYPE_TABLE`; unknown ones stay uninferred.
                .and_then(|typestr| find_dtype_entry(&typestr).map(|e| e.canonical.to_owned()))
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
        Self::from_owned_cuda(dev_buf, &resolved_dtype, resolved_shape, &stream.inner)
    }

    /// Zero-copy import from any object implementing `__dlpack__` (PyTorch, CuPy, JAX) or a `"dltensor"` capsule.
    ///
    /// Only dense row-major tensors on the CUDA device of `stream` (when given) are accepted.
    #[staticmethod]
    #[pyo3(signature = (obj, stream=None))]
    fn from_dlpack(
        py: Python<'_>,
        obj: &Bound<'_, PyAny>,
        stream: Option<&PyStream>,
    ) -> PyResult<Self> {
        let capsule_bound: Bound<'_, PyAny> = if obj.hasattr("__dlpack__")? {
            if let Some(s) = stream {
                // `stream` is keyword-only in the Array API / DLPack >= 0.8 signature.
                let kwargs = PyDict::new(py);
                kwargs.set_item("stream", s.raw_ptr())?;
                obj.call_method("__dlpack__", (), Some(&kwargs))?
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

        // Null strides mean row-major (DLPack); explicit ones must describe the same layout.
        let strides = compute_row_major_strides(&shape);
        if ndim > 0 && !strides_raw.is_null() {
            // SAFETY: `strides_raw` is non-null and points to `ndim` elements of i64.
            let given = unsafe { std::slice::from_raw_parts(strides_raw, ndim) };
            check_c_contiguous(&shape, given).map_err(|e| {
                record_and_err("core", "from_dlpack", e, ErrorKind::InvalidArgument)
            })?;
        }

        if let Some(s) = stream
            && s.device_id != device_id
        {
            return Err(record_and_err(
                "core",
                "from_dlpack",
                format!(
                    "DLPack tensor is on CUDA device {device_id}, but `stream` is on device {}",
                    s.device_id
                ),
                ErrorKind::InvalidArgument,
            ));
        }

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
        if ptr.checked_add(nbytes as u64).is_none() {
            return Err(record_and_err(
                "core",
                "from_dlpack",
                "DLPack tensor extent overflows the device address space",
                ErrorKind::InvalidArgument,
            ));
        }

        let ordinal = usize::try_from(device_id).map_err(|_| {
            record_and_err(
                "core",
                "from_dlpack",
                format!("negative CUDA device id in DLPack tensor: {device_id}"),
                ErrorKind::InvalidArgument,
            )
        })?;
        // The tensor's device context: leases check it against the stream of every native call.
        let context = Context::new(ordinal)
            .map_err(|e| record_and_err("core", "from_dlpack", e, ErrorKind::Cuda))?;

        // Transfer ownership of `managed` out of the guard to the lease-aware owner.
        let owner = Arc::new(DlPackOwner {
            managed: guard.into_raw(),
            ptr,
            nbytes,
            context,
        });

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
                raw: RawDeviceBuffer::foreign(owner, nbytes),
            }),
            device_id,
            dtype: dtype_str.to_owned(),
            shape,
            strides,
            stream: stream.map(|s| s.inner.clone()),
        })
    }

    /// Exports this buffer as a zero-copy DLPack `"dltensor"` `PyCapsule`.
    ///
    /// Accepts the Array API keywords: `max_version` (the unversioned legacy capsule is always
    /// returned, which consumers accept), `dl_device` (must be this buffer's CUDA device) and
    /// `copy` (`True` raises `BufferError`: exports are zero-copy only).
    #[pyo3(signature = (stream=None, max_version=None, dl_device=None, copy=None))]
    fn __dlpack__<'py>(
        &self,
        py: Python<'py>,
        stream: Option<&Bound<'py, PyAny>>,
        max_version: Option<&Bound<'py, PyAny>>,
        dl_device: Option<(i32, i32)>,
        copy: Option<bool>,
    ) -> PyResult<Bound<'py, PyCapsule>> {
        let _ = max_version;
        if let Some(dev) = dl_device
            && dev != (DL_CUDA, self.device_id)
        {
            return Err(record_and_err(
                "core",
                "__dlpack__",
                format!(
                    "dl_device {dev:?} differs from this buffer's device ({DL_CUDA}, {}); \
                     cross-device export is not supported",
                    self.device_id
                ),
                ErrorKind::Buffer,
            ));
        }
        if copy == Some(true) {
            return Err(record_and_err(
                "core",
                "__dlpack__",
                "copy=True is not supported: nv_rs exports DLPack tensors zero-copy only",
                ErrorKind::Buffer,
            ));
        }
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
    /// @py-return: dict[str, Any]
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

    /// Downloads device memory (owned buffer or DLPack import) to host `bytes` on `stream`,
    /// after the buffer's producer stream and `stream` have finished their queued work.
    fn to_bytes<'py>(&self, py: Python<'py>, stream: &PyStream) -> PyResult<Bound<'py, PyBytes>> {
        let (ptr, nbytes) = check_buf(
            "core",
            "to_bytes",
            "self",
            self,
            &[],
            0,
            stream.inner.context(),
        )?;
        let cuda_err =
            |e: cuda_core::DriverError| record_and_err("core", "to_bytes", e, ErrorKind::Cuda);
        let s_inner = stream.inner.clone();
        let host = py.detach(|| -> PyResult<Vec<u8>> {
            // The contents may still be in flight on the producer stream (the copy below is
            // ordered only after work already queued on `stream`).
            if let Some(p) = &self.stream
                && p.cu_stream() != s_inner.cu_stream()
            {
                p.synchronize().map_err(cuda_err)?;
            }
            let mut host = vec![0u8; nbytes];
            if nbytes == 0 {
                return Ok(host);
            }
            let queued =
                enqueue_leased("core", "to_bytes", &s_inner, [(self, Access::Read)], || {
                    // SAFETY: `host` holds `nbytes` writable bytes and outlives the copy (synchronized
                    // below); `ptr..ptr + nbytes` is this buffer's dense span (`check_buf`).
                    unsafe {
                        cuda_core::memory::memcpy_dtoh_async(
                            host.as_mut_ptr(),
                            ptr,
                            nbytes,
                            s_inner.cu_stream(),
                        )
                    }
                    .map_err(cuda_err)
                });
            // Synchronize even when `queued` failed: the copy may have been queued before a
            // lease bookkeeping error, and `host` must outlive it.
            if let Err(e) = s_inner.synchronize() {
                // The driver has not confirmed that it stopped writing `host`: never free it.
                std::mem::forget(host);
                return Err(cuda_err(e));
            }
            queued?;
            Ok(host)
        })?;
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
    py_register_submodule!(py, parent, "core", |m| {
        m.add_class::<PyContext>()?;
        m.add_class::<PyStream>()?;
        m.add_class::<PyDeviceBuffer>()?;
    })
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

    #[test]
    fn test_check_c_contiguous_accepts_row_major_and_wildcards() {
        assert!(check_c_contiguous(&[2, 3], &[3, 1]).is_ok());
        assert!(check_c_contiguous(&[], &[]).is_ok());
        // Size-1 dimensions carry arbitrary strides (PyTorch emits these for `unsqueeze`).
        assert!(check_c_contiguous(&[3, 1], &[1, 7]).is_ok());
        assert!(check_c_contiguous(&[1, 4], &[0, 1]).is_ok());
        // An empty tensor addresses no element.
        assert!(check_c_contiguous(&[0, 5], &[1, 0]).is_ok());
    }

    #[test]
    fn test_check_c_contiguous_rejects_strided_layouts() {
        // P1: transposed view, zero-stride broadcast (`expand`) and negative strides.
        for (shape, strides) in [
            (vec![3i64, 2], vec![1i64, 3]),
            (vec![1000], vec![0]),
            (vec![4], vec![-1]),
            (vec![2, 3], vec![6, 2]),
        ] {
            let err = check_c_contiguous(&shape, &strides).expect_err("strided layout accepted");
            assert!(err.contains("non-contiguous"), "got: {err}");
        }
    }

    #[test]
    fn test_check_owned_layout_reports_errors_instead_of_panicking() {
        // P6: these inputs used to reach `panic!` / `assert_eq!` in `from_owned_cuda_with_stream`.
        assert_eq!(check_owned_layout(24, "float32", &[2, 3]), Ok("f32"));
        assert!(check_owned_layout(4, "complex64", &[1]).is_err());
        assert!(check_owned_layout(4, "f32", &[-1]).is_err());
        let err = check_owned_layout(20, "f32", &[2, 3]).expect_err("length mismatch accepted");
        assert!(err.contains("expected 24 bytes"), "got: {err}");
    }

    #[test]
    fn test_numpy_typestr_inference_matches_previous_table() {
        // P13: `from_buffer` infers dtypes through `DTYPE_TABLE` aliases instead of a copy.
        for (typestr, canonical) in [
            ("|u1", "u8"),
            ("|i1", "i8"),
            ("<u2", "u16"),
            ("<i2", "i16"),
            ("<u4", "u32"),
            ("<i4", "i32"),
            ("<u8", "u64"),
            ("<i8", "i64"),
            ("<f2", "f16"),
            ("<f4", "f32"),
            ("<f8", "f64"),
            ("|b1", "bool"),
        ] {
            assert_eq!(
                find_dtype_entry(typestr).map(|e| e.canonical),
                Some(canonical)
            );
        }
        // Byte-swapped and void typestrs stay uninferred rather than reinterpreted.
        assert!(find_dtype_entry(">f4").is_none());
        assert!(find_dtype_entry("<V2").is_none());
    }

    static LEASED_DELETES: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "C" fn leased_deleter(self_: *mut DLManagedTensor) {
        LEASED_DELETES.fetch_add(1, Ordering::SeqCst);
        // SAFETY: `self_` was allocated via `Box::into_raw` in the test below.
        drop(unsafe { Box::from_raw(self_) });
    }

    #[test]
    #[ignore = "GPU: needs a CUDA driver"]
    fn gpu_dlpack_import_deleter_waits_for_leased_work() {
        // LEASE-API for DLPack imports: the producer's deleter (which lets e.g. PyTorch's caching
        // allocator reuse the block) must not run while leased work on the tensor is queued.
        let ctx = Context::new(0).expect("cuda context");
        let a = ctx.new_stream().expect("stream");
        let tensor = Box::new(DLManagedTensor {
            dl_tensor: crate::dlpack::DLTensor {
                data: std::ptr::null_mut(),
                device: crate::dlpack::DLDevice {
                    device_type: DL_CUDA,
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
            deleter: Some(leased_deleter),
        });
        // The enqueue below queues no device access, so a placeholder address is never touched.
        let owner = Arc::new(DlPackOwner {
            managed: Box::into_raw(tensor),
            ptr: 0x1000,
            nbytes: 16,
            context: ctx.clone(),
        });
        let buf = PyDeviceBuffer {
            backing: Arc::new(Backing::BorrowedDlPack {
                raw: RawDeviceBuffer::foreign(owner, 16),
            }),
            device_id: 0,
            dtype: "u8".to_owned(),
            shape: vec![16],
            strides: vec![1],
            stream: None,
        };
        a.launch_host_function(|| std::thread::sleep(std::time::Duration::from_millis(200)))
            .expect("busy");
        enqueue_leased("test", "noop", &a, [(&buf, Access::Read)], || Ok(())).expect("lease");
        drop(buf);
        std::thread::sleep(std::time::Duration::from_millis(30));
        assert_eq!(
            LEASED_DELETES.load(Ordering::SeqCst),
            0,
            "deleter ran while leased work was queued"
        );
        a.synchronize().expect("sync");
        let t0 = Instant::now();
        while LEASED_DELETES.load(Ordering::SeqCst) == 0 && t0.elapsed().as_secs() < 2 {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        assert_eq!(
            LEASED_DELETES.load(Ordering::SeqCst),
            1,
            "deleter must run once after completion"
        );
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
