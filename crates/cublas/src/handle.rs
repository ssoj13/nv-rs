//! [`Handle`]: owning cuBLAS context handle, stream binding, pointer/math modes, and workspace.
//!
//! **What.** Wraps `cublasHandle_t` bound to a `cuda_core::Context`, configures stream ordering,
//! host/device pointer mode, math mode, and caller-owned [`Scratch`] workspace, and destroys the
//! handle on `Drop` (keeping a failed `cublasDestroy_v2` in the `cuda_core_ext::teardown` sink).
//!
//! **Workspace rule.** [`Handle::set_workspace`] retains the scratch allocation it hands to
//! `cublasSetWorkspace_v2` until the workspace is replaced, cleared, or the handle is dropped, and
//! pins the handle to the scratch stream while it is bound: `cublasSetStream_v2` silently resets the
//! workspace to the default pool, and using a stream-ordered buffer on another stream is unsound, so
//! switching streams with a bound workspace is an error. [`Handle::clear_workspace`] restores the
//! default pool. Every stream bind makes the handle's context current, even on a stream-cache hit.
//!
//! **Why.** Centralizes handle lifecycle and ensures every cuBLAS call runs on the caller's
//! stream and context with zero hidden allocations.
//! **Used by** [`crate::gemm`] and external cuBLAS callers.

use crate::error::{check, teardown};
use crate::{Error, Result, Scratch, lt_sys, sys};
use cuda_core::{Context, DeviceBuffer, Stream};
use cuda_core_ext::same_context;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

/// Matrix transpose operation (`cublasOperation_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    /// Non-transposed (`CUBLAS_OP_N`).
    N,
    /// Transposed (`CUBLAS_OP_T`).
    T,
    /// Conjugate transposed (`CUBLAS_OP_C`).
    C,
}

impl Operation {
    /// Converts to `cublas_sys::cublasOperation_t`.
    #[inline]
    pub const fn raw(self) -> sys::cublasOperation_t {
        match self {
            Self::N => sys::cublasOperation_t_CUBLAS_OP_N,
            Self::T => sys::cublasOperation_t_CUBLAS_OP_T,
            Self::C => sys::cublasOperation_t_CUBLAS_OP_C,
        }
    }

    /// Converts to `i32` (`cublasOperation_t`) for `cublasLtMatmulDescSetAttribute`.
    #[inline]
    pub const fn raw_lt(self) -> i32 {
        // `c_int` alias: the bindgen enum repr is `i32` on MSVC and `u32` on Linux.
        self.raw() as std::ffi::c_int
    }
}

/// Scalar pointer location for `alpha`/`beta` (`cublasPointerMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointerMode {
    /// Scalars reside in host memory (`CUBLAS_POINTER_MODE_HOST`).
    Host,
    /// Scalars reside in device memory (`CUBLAS_POINTER_MODE_DEVICE`).
    Device,
}

impl PointerMode {
    /// Converts to `cublas_sys::cublasPointerMode_t`.
    #[inline]
    pub const fn raw(self) -> sys::cublasPointerMode_t {
        match self {
            Self::Host => sys::cublasPointerMode_t_CUBLAS_POINTER_MODE_HOST,
            Self::Device => sys::cublasPointerMode_t_CUBLAS_POINTER_MODE_DEVICE,
        }
    }

    /// Converts to `i32` (`cublasLtPointerMode_t`) for `cublasLtMatmulDescSetAttribute`.
    #[inline]
    pub const fn raw_lt(self) -> i32 {
        // `c_int` alias: the bindgen enum repr is `i32` on MSVC and `u32` on Linux.
        self.raw() as std::ffi::c_int
    }

    pub(crate) fn from_raw(raw: sys::cublasPointerMode_t) -> Result<Self> {
        match raw {
            sys::cublasPointerMode_t_CUBLAS_POINTER_MODE_HOST => Ok(Self::Host),
            sys::cublasPointerMode_t_CUBLAS_POINTER_MODE_DEVICE => Ok(Self::Device),
            _ => Err(Error::Invalid("unrecognized cublasPointerMode_t")),
        }
    }
}

/// Math acceleration mode (`cublasMath_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MathMode {
    /// Default math (`CUBLAS_DEFAULT_MATH`).
    Default,
    /// Allow Tensor Core operations (`CUBLAS_TENSOR_OP_MATH`).
    TensorOp,
    /// Pedantic precision without tensor down-conversion (`CUBLAS_PEDANTIC_MATH`).
    Pedantic,
    /// Allow TF32 Tensor Core acceleration for `f32` (`CUBLAS_TF32_TENSOR_OP_MATH`).
    Tf32TensorOp,
    /// Raw `cublasMath_t` value for newer toolkit flags.
    Raw(sys::cublasMath_t),
}

impl MathMode {
    /// Converts to `cublas_sys::cublasMath_t`.
    #[inline]
    pub const fn raw(self) -> sys::cublasMath_t {
        match self {
            Self::Default => sys::cublasMath_t_CUBLAS_DEFAULT_MATH,
            Self::TensorOp => sys::cublasMath_t_CUBLAS_TENSOR_OP_MATH,
            Self::Pedantic => sys::cublasMath_t_CUBLAS_PEDANTIC_MATH,
            Self::Tf32TensorOp => sys::cublasMath_t_CUBLAS_TF32_TENSOR_OP_MATH,
            Self::Raw(v) => v,
        }
    }

    pub(crate) fn from_raw(raw: sys::cublasMath_t) -> Self {
        match raw {
            sys::cublasMath_t_CUBLAS_DEFAULT_MATH => Self::Default,
            sys::cublasMath_t_CUBLAS_TENSOR_OP_MATH => Self::TensorOp,
            sys::cublasMath_t_CUBLAS_PEDANTIC_MATH => Self::Pedantic,
            sys::cublasMath_t_CUBLAS_TF32_TENSOR_OP_MATH => Self::Tf32TensorOp,
            other => Self::Raw(other),
        }
    }
}

/// Element data type (`cudaDataType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    /// 16-bit IEEE float (`CUDA_R_16F`).
    F16,
    /// 16-bit brain float (`CUDA_R_16BF`).
    Bf16,
    /// 32-bit IEEE float (`CUDA_R_32F`).
    F32,
    /// 64-bit IEEE float (`CUDA_R_64F`).
    F64,
    /// Complex 32-bit float pair (`CUDA_C_32F`).
    C32F,
    /// Complex 64-bit float pair (`CUDA_C_64F`).
    C64F,
    /// Signed 8-bit integer (`CUDA_R_8I`).
    I8,
    /// Signed 32-bit integer (`CUDA_R_32I`).
    I32,
    /// Unsigned 8-bit integer (`CUDA_R_8U`).
    U8,
    /// Unsigned 32-bit integer (`CUDA_R_32U`).
    U32,
}

impl DataType {
    /// Converts to `cublas_sys::cudaDataType_t`.
    #[inline]
    pub const fn raw(self) -> sys::cudaDataType_t {
        match self {
            Self::F16 => sys::cudaDataType_t_CUDA_R_16F,
            Self::Bf16 => sys::cudaDataType_t_CUDA_R_16BF,
            Self::F32 => sys::cudaDataType_t_CUDA_R_32F,
            Self::F64 => sys::cudaDataType_t_CUDA_R_64F,
            Self::C32F => sys::cudaDataType_t_CUDA_C_32F,
            Self::C64F => sys::cudaDataType_t_CUDA_C_64F,
            Self::I8 => sys::cudaDataType_t_CUDA_R_8I,
            Self::I32 => sys::cudaDataType_t_CUDA_R_32I,
            Self::U8 => sys::cudaDataType_t_CUDA_R_8U,
            Self::U32 => sys::cudaDataType_t_CUDA_R_32U,
        }
    }

    /// Converts to `cublaslt_sys::cudaDataType_t`.
    #[inline]
    pub const fn raw_lt(self) -> lt_sys::cudaDataType_t {
        self.raw() as lt_sys::cudaDataType_t
    }

    /// Size of one element in bytes.
    #[inline]
    pub const fn element_bytes(self) -> usize {
        match self {
            Self::I8 | Self::U8 => 1,
            Self::F16 | Self::Bf16 => 2,
            Self::F32 | Self::I32 | Self::U32 => 4,
            Self::F64 | Self::C32F => 8,
            Self::C64F => 16,
        }
    }
}

/// Compute precision mode (`cublasComputeType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComputeType {
    /// 16-bit float (`CUBLAS_COMPUTE_16F`).
    F16,
    /// 16-bit float pedantic (`CUBLAS_COMPUTE_16F_PEDANTIC`).
    F16Pedantic,
    /// 32-bit float (`CUBLAS_COMPUTE_32F`).
    F32,
    /// 32-bit float pedantic (`CUBLAS_COMPUTE_32F_PEDANTIC`).
    F32Pedantic,
    /// 32-bit float allowing 16F/TF32 tensor down-conversion (`CUBLAS_COMPUTE_32F_FAST_16F`).
    F32Fast16F,
    /// 32-bit float allowing BF16/TF32 tensor down-conversion (`CUBLAS_COMPUTE_32F_FAST_16BF`).
    F32Fast16BF,
    /// 32-bit float allowing TF32 tensor down-conversion (`CUBLAS_COMPUTE_32F_FAST_TF32`).
    F32FastTf32,
    /// 64-bit float (`CUBLAS_COMPUTE_64F`).
    F64,
    /// 64-bit float pedantic (`CUBLAS_COMPUTE_64F_PEDANTIC`).
    F64Pedantic,
    /// 32-bit signed integer (`CUBLAS_COMPUTE_32I`).
    I32,
    /// 32-bit signed integer pedantic (`CUBLAS_COMPUTE_32I_PEDANTIC`).
    I32Pedantic,
}

impl ComputeType {
    /// Converts to `cublas_sys::cublasComputeType_t`.
    #[inline]
    pub const fn raw(self) -> sys::cublasComputeType_t {
        match self {
            Self::F16 => sys::cublasComputeType_t_CUBLAS_COMPUTE_16F,
            Self::F16Pedantic => sys::cublasComputeType_t_CUBLAS_COMPUTE_16F_PEDANTIC,
            Self::F32 => sys::cublasComputeType_t_CUBLAS_COMPUTE_32F,
            Self::F32Pedantic => sys::cublasComputeType_t_CUBLAS_COMPUTE_32F_PEDANTIC,
            Self::F32Fast16F => sys::cublasComputeType_t_CUBLAS_COMPUTE_32F_FAST_16F,
            Self::F32Fast16BF => sys::cublasComputeType_t_CUBLAS_COMPUTE_32F_FAST_16BF,
            Self::F32FastTf32 => sys::cublasComputeType_t_CUBLAS_COMPUTE_32F_FAST_TF32,
            Self::F64 => sys::cublasComputeType_t_CUBLAS_COMPUTE_64F,
            Self::F64Pedantic => sys::cublasComputeType_t_CUBLAS_COMPUTE_64F_PEDANTIC,
            Self::I32 => sys::cublasComputeType_t_CUBLAS_COMPUTE_32I,
            Self::I32Pedantic => sys::cublasComputeType_t_CUBLAS_COMPUTE_32I_PEDANTIC,
        }
    }

    /// Converts to `cublaslt_sys::cublasComputeType_t`.
    #[inline]
    pub const fn raw_lt(self) -> lt_sys::cublasComputeType_t {
        self.raw() as lt_sys::cublasComputeType_t
    }
}

/// Algorithm selector for `cublasGemmEx` (`cublasGemmAlgo_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GemmAlgo {
    /// Default heuristic (`CUBLAS_GEMM_DEFAULT`).
    Default,
    /// Default Tensor Core heuristic (`CUBLAS_GEMM_DEFAULT_TENSOR_OP`).
    DefaultTensorOp,
    /// Explicit raw `cublasGemmAlgo_t` value.
    Raw(sys::cublasGemmAlgo_t),
}

impl GemmAlgo {
    /// Converts to `cublas_sys::cublasGemmAlgo_t`.
    #[inline]
    pub const fn raw(self) -> sys::cublasGemmAlgo_t {
        match self {
            Self::Default => sys::cublasGemmAlgo_t_CUBLAS_GEMM_DEFAULT,
            Self::DefaultTensorOp => sys::cublasGemmAlgo_t_CUBLAS_GEMM_DEFAULT_TENSOR_OP,
            Self::Raw(v) => v,
        }
    }
}

/// Owning cuBLAS context handle (`cublasHandle_t`).
///
/// Bound to a single `cuda_core::Context` and destroyed on drop (recording any teardown error on
/// the context).
pub struct Handle {
    raw: sys::cublasHandle_t,
    cuda: Arc<Context>,
    pointer_mode: Cell<PointerMode>,
    last_raw_stream: Cell<*mut std::ffi::c_void>,
    /// Workspace currently installed with `cublasSetWorkspace_v2`; `None` means cuBLAS uses its
    /// default pool. Fields drop after `Drop::drop` destroys `raw`.
    workspace: RefCell<Option<BoundWorkspace>>,
}

/// User workspace installed on a [`Handle`], retained so it outlives every call that may use it.
struct BoundWorkspace {
    /// Installed allocation (`None` for an empty, zero-byte workspace). Never read: holding it is
    /// the point; its stream-ordered free runs on `stream` after the cuBLAS work queued there.
    _buf: Option<Arc<DeviceBuffer<u8>>>,
    /// Stream the handle is pinned to while this workspace is installed.
    stream: Arc<Stream>,
}

// SAFETY: `cublasHandle_t` has no thread-local state once created; moving ownership between threads
// is safe as long as concurrent calls on the same handle are externally synchronized (`!Sync`).
unsafe impl Send for Handle {}

impl Handle {
    /// Creates a new cuBLAS handle on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        cuda.bind_to_thread()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `cuda` is bound and alive; `raw` is a valid out-pointer.
        check("cublasCreate_v2", unsafe { sys::cublasCreate_v2(&mut raw) })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            pointer_mode: Cell::new(PointerMode::Host),
            last_raw_stream: Cell::new(std::ptr::null_mut()),
            workspace: RefCell::new(None),
        })
    }

    /// Returns the raw `cublasHandle_t`.
    #[inline]
    pub fn raw(&self) -> sys::cublasHandle_t {
        self.raw
    }

    /// Returns the owning CUDA context.
    #[inline]
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// Binds `stream` to this handle and makes the handle's context current on this thread.
    ///
    /// Fails if `stream` belongs to a different CUDA context, or if a workspace bound with
    /// [`Self::set_workspace`] belongs to another stream (call [`Self::clear_workspace`] first).
    pub fn set_stream(&self, stream: &Stream) -> Result<()> {
        if !same_context(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than the cuBLAS handle",
            ));
        }
        // SAFETY: `stream` is a live stream of `self.cuda` (checked above); safe callers pass it
        // again to every enqueueing call, which rebinds it before use.
        unsafe { self.bind_stream(stream.cu_stream().cast()) }
    }

    /// Binds an externally owned raw `CUstream` (e.g. from CubeCL) to this handle, caching the
    /// last bound pointer to avoid redundant driver calls on repeated launches.
    ///
    /// # Safety
    /// - `raw_stream` must be either null (default stream) or a live `CUstream` created in
    ///   `self.cuda()` that remains valid for all subsequent cuBLAS calls until rebound.
    pub unsafe fn set_raw_stream(&self, raw_stream: *mut std::ffi::c_void) -> Result<()> {
        // SAFETY: forwarded caller contract on `raw_stream`.
        unsafe { self.bind_stream(raw_stream) }
    }

    /// Makes `self.cuda` current, then binds `raw_stream` unless it is already bound.
    ///
    /// # Safety
    /// `raw_stream` must be null or a live `CUstream` of `self.cuda`.
    unsafe fn bind_stream(&self, raw_stream: *mut std::ffi::c_void) -> Result<()> {
        // Bind before the cache check: the cache skips only `cublasSetStream_v2`, while cuBLAS
        // enqueues on whatever context is current on the calling thread.
        self.cuda.bind_to_thread()?;
        if self.last_raw_stream.get() == raw_stream {
            return Ok(());
        }
        // `cublasSetStream_v2` would silently drop the workspace to the default pool, and the
        // stream-ordered workspace must not be used on another stream.
        if self.workspace.borrow().is_some() {
            return Err(Error::Invalid(
                "cuBLAS workspace is bound to another stream; call clear_workspace first",
            ));
        }
        // SAFETY: caller guarantees `raw_stream` belongs to `self.cuda` and stays alive.
        check("cublasSetStream_v2", unsafe {
            sys::cublasSetStream_v2(self.raw, raw_stream.cast())
        })?;
        self.last_raw_stream.set(raw_stream);
        Ok(())
    }

    /// Sets whether scalar `alpha`/`beta` arguments live on host or device memory.
    pub fn set_pointer_mode(&self, mode: PointerMode) -> Result<()> {
        self.cuda.bind_to_thread()?;
        // SAFETY: `self.raw` is a live cuBLAS handle.
        check("cublasSetPointerMode_v2", unsafe {
            sys::cublasSetPointerMode_v2(self.raw, mode.raw())
        })?;
        self.pointer_mode.set(mode);
        Ok(())
    }

    /// Ensures the handle is in [`PointerMode::Host`] before passing host scalar pointers.
    #[inline]
    pub(crate) fn require_host_pointer_mode(&self) -> Result<()> {
        if self.pointer_mode.get() != PointerMode::Host {
            return Err(Error::Invalid("safe GEMM requires PointerMode::Host"));
        }
        Ok(())
    }

    /// Queries the current pointer mode.
    pub fn pointer_mode(&self) -> Result<PointerMode> {
        self.cuda.bind_to_thread()?;
        let mut mode = 0;
        // SAFETY: `self.raw` is live and `mode` is a valid out-pointer.
        check("cublasGetPointerMode_v2", unsafe {
            sys::cublasGetPointerMode_v2(self.raw, &mut mode)
        })?;
        PointerMode::from_raw(mode)
    }

    /// Sets the math acceleration mode.
    pub fn set_math_mode(&self, mode: MathMode) -> Result<()> {
        self.cuda.bind_to_thread()?;
        // SAFETY: `self.raw` is a live cuBLAS handle.
        check("cublasSetMathMode", unsafe {
            sys::cublasSetMathMode(self.raw, mode.raw())
        })
    }

    /// Queries the current math acceleration mode.
    pub fn math_mode(&self) -> Result<MathMode> {
        self.cuda.bind_to_thread()?;
        let mut mode = 0;
        // SAFETY: `self.raw` is live and `mode` is a valid out-pointer.
        check("cublasGetMathMode", unsafe {
            sys::cublasGetMathMode(self.raw, &mut mode)
        })?;
        Ok(MathMode::from_raw(mode))
    }

    /// Binds a caller-owned [`Scratch`] workspace of `bytes` (admitted against `budget`) to this
    /// handle and binds `scratch.stream()` as the active stream.
    ///
    /// The handle retains the allocation until the workspace is replaced, cleared, or the handle
    /// is dropped, so later growth or drop of `scratch` cannot free memory cuBLAS still uses. While
    /// bound, the handle stays on `scratch.stream()`: [`Self::set_stream`] with another stream fails.
    /// `bytes == 0` installs an empty workspace (cuBLAS then uses no workspace at all).
    pub fn set_workspace(&self, scratch: &mut Scratch, bytes: usize, budget: usize) -> Result<()> {
        let stream = scratch.stream().clone();
        if !same_context(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "scratch stream belongs to a different CUDA context than the cuBLAS handle",
            ));
        }
        let (ptr, cap) = scratch.reserve(&stream, bytes, Some(budget))?;
        // `reserve(0)` reports the current buffer; `bytes == 0` asks for no workspace at all.
        let (ptr, cap) = if bytes == 0 { (0, 0) } else { (ptr, cap) };
        let buf = if cap == 0 {
            None
        } else {
            scratch.shared_buffer()
        };
        self.cuda.bind_to_thread()?;
        let raw_s: *mut std::ffi::c_void = stream.cu_stream().cast();
        let stream_reset = self.last_raw_stream.get() != raw_s;
        if stream_reset {
            // SAFETY: `self.raw` is live and `stream` belongs to `self.cuda` (checked above). This
            // also resets any previous workspace to the default pool, so cuBLAS stops using it.
            check("cublasSetStream_v2", unsafe {
                sys::cublasSetStream_v2(self.raw, stream.cu_stream())
            })?;
            self.last_raw_stream.set(raw_s);
        }
        // SAFETY: `self.raw` is live; `ptr` is valid for `cap` device bytes on `stream` (or null/0)
        // and `buf` keeps that allocation alive for as long as it stays installed.
        let set = check("cublasSetWorkspace_v2", unsafe {
            sys::cublasSetWorkspace_v2(self.raw, ptr as usize as *mut std::ffi::c_void, cap)
        });
        if set.is_ok() {
            *self.workspace.borrow_mut() = Some(BoundWorkspace { _buf: buf, stream });
        } else if stream_reset {
            // After a stream reset cuBLAS is back on its default pool and no longer refers to the
            // previous workspace; without a reset the previous one stays installed and retained.
            *self.workspace.borrow_mut() = None;
        }
        set
    }

    /// Detaches any user-supplied workspace and restores cuBLAS's default workspace pool.
    ///
    /// `cublasSetWorkspace_v2(NULL, 0)` would instead disable workspace use entirely; rebinding the
    /// current stream with `cublasSetStream_v2` is the documented way back to the default pool.
    /// The released allocation is freed stream-ordered after the work already queued on it.
    pub fn clear_workspace(&self) -> Result<()> {
        self.cuda.bind_to_thread()?;
        let mut slot = self.workspace.borrow_mut();
        let Some(ws) = slot.as_ref() else {
            return Ok(());
        };
        // SAFETY: `self.raw` is live and `ws.stream` is the retained stream the handle is pinned
        // to while the workspace is installed, so this keeps the bound stream unchanged.
        check("cublasSetStream_v2", unsafe {
            sys::cublasSetStream_v2(self.raw, ws.stream.cu_stream())
        })?;
        *slot = None;
        Ok(())
    }

    /// Returns the cuBLAS library version reported by `cublasGetVersion_v2`.
    pub fn version(&self) -> Result<i32> {
        self.cuda.bind_to_thread()?;
        let mut v = 0;
        // SAFETY: `self.raw` is live and `v` is a valid out-pointer.
        check("cublasGetVersion_v2", unsafe {
            sys::cublasGetVersion_v2(self.raw, &mut v)
        })?;
        Ok(v)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // A failed bind leaves its driver error on `self.cuda` and skips the native call.
            if cuda_core_ext::teardown::bind(&self.cuda) {
                // SAFETY: `self.raw` was created by `cublasCreate_v2` and is destroyed once.
                let code = unsafe { sys::cublasDestroy_v2(self.raw) };
                teardown(code, "cublasDestroy_v2");
            }
            self.raw = std::ptr::null_mut();
        }
    }
}
