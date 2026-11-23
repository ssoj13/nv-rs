//! Safe cuBLASLt API: [`LtHandle`], [`MatmulDesc`], [`MatrixLayout`], [`Preference`], heuristics,
//! and workspace-budgeted matmul.
//!
//! **What.** Wraps `cublasLtHandle_t`, `cublasLtMatmulDesc_t`, `cublasLtMatrixLayout_t`,
//! `cublasLtMatmulPreference_t`, heuristic algorithm selection, and `cublasLtMatmul` with
//! caller-owned [`Scratch`] workspace and explicit byte budgets.
//! **Why.** cuBLASLt exposes row/column/tensor-core layouts, epilogues (ReLU, GELU, bias), and
//! explicit workspace sizing without hidden internal allocations.
//! **Used by** `cublas` callers needing tensor-core layouts, fused epilogues, or explicit
//! heuristic workspace control.

use crate::error::{check_lt, teardown_lt};
use crate::gemm::batches_disjoint;
use crate::{
    ComputeType, DataType, Error, Operation, PointerMode, Result, Scratch, Status, lt_sys,
};
use cuda_core::{Context, DeviceBuffer, Stream};
use std::ffi::{CString, c_void};
use std::path::Path;
use std::sync::Arc;

/// Library version property selector (`libraryPropertyType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LibraryProperty {
    /// Major version (`MAJOR_VERSION`).
    MajorVersion,
    /// Minor version (`MINOR_VERSION`).
    MinorVersion,
    /// Patch level (`PATCH_LEVEL`).
    PatchLevel,
    /// Raw `libraryPropertyType_t` value.
    Raw(lt_sys::libraryPropertyType),
}

impl LibraryProperty {
    /// Converts to `cublaslt_sys::libraryPropertyType`.
    #[inline]
    pub const fn raw(self) -> lt_sys::libraryPropertyType {
        match self {
            Self::MajorVersion => lt_sys::libraryPropertyType_t_MAJOR_VERSION,
            Self::MinorVersion => lt_sys::libraryPropertyType_t_MINOR_VERSION,
            Self::PatchLevel => lt_sys::libraryPropertyType_t_PATCH_LEVEL,
            Self::Raw(v) => v,
        }
    }
}

/// Callback signature for `cublasLtLoggerSetCallback`.
pub type LoggerCallback = unsafe extern "C" fn(
    log_level: std::os::raw::c_int,
    function_name: *const std::os::raw::c_char,
    message: *const std::os::raw::c_char,
);

/// Returns the cuBLASLt library version (`cublasLtGetVersion`).
pub fn version() -> Result<usize> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `cublasLtGetVersion` takes no arguments and returns a `usize` version number.
    unsafe { lt_sys::cublasLtGetVersion() }.map_err(|e| Error::Unavailable(e.to_string()))
}

/// Returns the CUDA Runtime version cuBLASLt was built against (`cublasLtGetCudartVersion`).
pub fn cudart_version() -> Result<usize> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `cublasLtGetCudartVersion` takes no arguments and returns a `usize` version number.
    unsafe { lt_sys::cublasLtGetCudartVersion() }.map_err(|e| Error::Unavailable(e.to_string()))
}

/// Queries a cuBLASLt version property (`cublasLtGetProperty`).
pub fn get_property(prop: LibraryProperty) -> Result<i32> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let mut value = 0i32;
    // SAFETY: `value` is a valid out-pointer for `c_int`.
    check_lt("cublasLtGetProperty", unsafe {
        lt_sys::cublasLtGetProperty(prop.raw(), &mut value)
    })?;
    Ok(value)
}

/// Disables CPU instruction sets by bitmask and returns the resulting active mask
/// (`cublasLtDisableCpuInstructionsSetMask`).
pub fn disable_cpu_instructions_set_mask(mask: u32) -> Result<u32> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `cublasLtDisableCpuInstructionsSetMask` is a pure process-wide mask setter.
    unsafe { lt_sys::cublasLtDisableCpuInstructionsSetMask(mask) }
        .map_err(|e| Error::Unavailable(e.to_string()))
}

/// Returns the current cuBLASLt heuristics cache capacity (`cublasLtHeuristicsCacheGetCapacity`).
pub fn heuristics_cache_capacity() -> Result<usize> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let mut capacity = 0usize;
    // SAFETY: `capacity` is a valid out-pointer for `usize`.
    check_lt("cublasLtHeuristicsCacheGetCapacity", unsafe {
        lt_sys::cublasLtHeuristicsCacheGetCapacity(&mut capacity)
    })?;
    Ok(capacity)
}

/// Sets the cuBLASLt heuristics cache capacity (`cublasLtHeuristicsCacheSetCapacity`).
pub fn set_heuristics_cache_capacity(capacity: usize) -> Result<()> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `capacity` is a plain `usize` value.
    check_lt("cublasLtHeuristicsCacheSetCapacity", unsafe {
        lt_sys::cublasLtHeuristicsCacheSetCapacity(capacity)
    })
}

/// Registers (or clears with `None`) the cuBLASLt logger callback (`cublasLtLoggerSetCallback`).
pub fn logger_set_callback(callback: Option<LoggerCallback>) -> Result<()> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `callback` is either `None` or a valid `extern "C"` function pointer.
    check_lt("cublasLtLoggerSetCallback", unsafe {
        lt_sys::cublasLtLoggerSetCallback(callback)
    })
}

/// Sets the cuBLASLt logger output `FILE*` stream (`cublasLtLoggerSetFile`).
///
/// # Safety
///
/// `file` must be a valid, open C `FILE*` stream that outlives any subsequent cuBLASLt logging.
pub unsafe fn logger_set_file(file: *mut lt_sys::FILE) -> Result<()> {
    if file.is_null() {
        return Err(Error::Invalid("logger file pointer must be non-null"));
    }
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: caller guarantees `file` is a valid, open C `FILE*` stream.
    check_lt("cublasLtLoggerSetFile", unsafe {
        lt_sys::cublasLtLoggerSetFile(file)
    })
}

/// Opens a log file for cuBLASLt logging (`cublasLtLoggerOpenFile`).
pub fn logger_open_file(path: impl AsRef<Path>) -> Result<()> {
    let path_str = path
        .as_ref()
        .to_str()
        .ok_or(Error::Invalid("logger path is not valid UTF-8"))?;
    let c_path = CString::new(path_str)
        .map_err(|_| Error::Invalid("logger path contains interior NUL byte"))?;
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `c_path` is a valid NUL-terminated C string.
    check_lt("cublasLtLoggerOpenFile", unsafe {
        lt_sys::cublasLtLoggerOpenFile(c_path.as_ptr())
    })
}

/// Sets the cuBLASLt logger verbosity level (`cublasLtLoggerSetLevel`).
pub fn logger_set_level(level: i32) -> Result<()> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `level` is a plain `c_int` value.
    check_lt("cublasLtLoggerSetLevel", unsafe {
        lt_sys::cublasLtLoggerSetLevel(level)
    })
}

/// Sets the cuBLASLt logger category mask (`cublasLtLoggerSetMask`).
pub fn logger_set_mask(mask: i32) -> Result<()> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `mask` is a plain `c_int` value.
    check_lt("cublasLtLoggerSetMask", unsafe {
        lt_sys::cublasLtLoggerSetMask(mask)
    })
}

/// Force-disables cuBLASLt logging for the current process (`cublasLtLoggerForceDisable`).
pub fn logger_force_disable() -> Result<()> {
    lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    // SAFETY: `cublasLtLoggerForceDisable` takes no arguments.
    check_lt("cublasLtLoggerForceDisable", unsafe {
        lt_sys::cublasLtLoggerForceDisable()
    })
}

fn op_from_lt(raw: i32) -> Result<Operation> {
    if raw == Operation::N.raw_lt() {
        Ok(Operation::N)
    } else if raw == Operation::T.raw_lt() {
        Ok(Operation::T)
    } else if raw == Operation::C.raw_lt() {
        Ok(Operation::C)
    } else {
        Err(Error::Invalid("unrecognized cublasOperation_t"))
    }
}

fn pointer_mode_from_lt(raw: i32) -> Result<PointerMode> {
    if raw == PointerMode::Host.raw_lt() {
        Ok(PointerMode::Host)
    } else if raw == PointerMode::Device.raw_lt() {
        Ok(PointerMode::Device)
    } else {
        Err(Error::Invalid("unrecognized cublasLtPointerMode_t"))
    }
}

/// Matrix memory layout order (`cublasLtOrder_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Order {
    /// Column-major (`CUBLASLT_ORDER_COL`).
    Col,
    /// Row-major (`CUBLASLT_ORDER_ROW`).
    Row,
    /// 32-column interleaved (`CUBLASLT_ORDER_COL32`).
    Col32,
    /// 4x4x2x8 interleaved (`CUBLASLT_ORDER_COL4_4R2_8C`).
    Col44r28c,
    /// 32x2x4x4 interleaved (`CUBLASLT_ORDER_COL32_2R_4R4`).
    Col322r4r4,
}

impl Order {
    /// Converts to `cublaslt_sys::cublasLtOrder_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtOrder_t {
        match self {
            Self::Col => lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_COL,
            Self::Row => lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_ROW,
            Self::Col32 => lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_COL32,
            Self::Col44r28c => lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_COL4_4R2_8C,
            Self::Col322r4r4 => lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_COL32_2R_4R4,
        }
    }

    /// Converts from `cublaslt_sys::cublasLtOrder_t`.
    #[inline]
    pub fn from_raw(raw: lt_sys::cublasLtOrder_t) -> Result<Self> {
        match raw {
            lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_COL => Ok(Self::Col),
            lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_ROW => Ok(Self::Row),
            lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_COL32 => Ok(Self::Col32),
            lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_COL4_4R2_8C => Ok(Self::Col44r28c),
            lt_sys::cublasLtOrder_t_CUBLASLT_ORDER_COL32_2R_4R4 => Ok(Self::Col322r4r4),
            _ => Err(Error::Invalid("unrecognized cublasLtOrder_t")),
        }
    }
}

/// Fused epilogue operation (`cublasLtEpilogue_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Epilogue {
    /// Standard matmul without extra epilogue (`CUBLASLT_EPILOGUE_DEFAULT`).
    Default,
    /// Apply ReLU (`CUBLASLT_EPILOGUE_RELU`).
    Relu,
    /// Add bias vector (`CUBLASLT_EPILOGUE_BIAS`).
    Bias,
    /// Add bias and apply ReLU (`CUBLASLT_EPILOGUE_RELU_BIAS`).
    ReluBias,
    /// Apply GELU (`CUBLASLT_EPILOGUE_GELU`).
    Gelu,
    /// Add bias and apply GELU (`CUBLASLT_EPILOGUE_GELU_BIAS`).
    GeluBias,
    /// Raw `cublasLtEpilogue_t` value.
    Raw(lt_sys::cublasLtEpilogue_t),
}

impl Epilogue {
    /// Converts to `cublaslt_sys::cublasLtEpilogue_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtEpilogue_t {
        match self {
            Self::Default => lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_DEFAULT,
            Self::Relu => lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_RELU,
            Self::Bias => lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_BIAS,
            Self::ReluBias => lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_RELU_BIAS,
            Self::Gelu => lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_GELU,
            Self::GeluBias => lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_GELU_BIAS,
            Self::Raw(v) => v,
        }
    }

    /// Converts from `cublaslt_sys::cublasLtEpilogue_t`.
    #[inline]
    pub const fn from_raw(raw: lt_sys::cublasLtEpilogue_t) -> Self {
        match raw {
            lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_DEFAULT => Self::Default,
            lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_RELU => Self::Relu,
            lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_BIAS => Self::Bias,
            lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_RELU_BIAS => Self::ReluBias,
            lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_GELU => Self::Gelu,
            lt_sys::cublasLtEpilogue_t_CUBLASLT_EPILOGUE_GELU_BIAS => Self::GeluBias,
            other => Self::Raw(other),
        }
    }
}

/// Owning cuBLASLt handle (`cublasLtHandle_t`).
#[must_use]
pub struct LtHandle {
    raw: lt_sys::cublasLtHandle_t,
    cuda: Arc<Context>,
}

// SAFETY: `cublasLtHandle_t` can be moved across threads; concurrent use requires external sync (`!Sync`).
unsafe impl Send for LtHandle {}

impl LtHandle {
    /// Creates a new cuBLASLt handle on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        cuda.bind_to_thread()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `cuda` is bound and alive; `raw` is a valid out-pointer.
        check_lt("cublasLtCreate", unsafe {
            lt_sys::cublasLtCreate(&mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
        })
    }

    /// Returns the raw `cublasLtHandle_t`.
    #[inline]
    pub fn raw(&self) -> lt_sys::cublasLtHandle_t {
        self.raw
    }

    /// Returns the owning CUDA context.
    #[inline]
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// Queries the single best heuristic algorithm for the given matmul descriptors.
    pub fn matmul_heuristic(
        &self,
        desc: &MatmulDesc,
        a: &MatrixLayout,
        b: &MatrixLayout,
        c: &MatrixLayout,
        d: &MatrixLayout,
        pref: &Preference,
    ) -> Result<HeuristicResult> {
        self.cuda.bind_to_thread()?;
        let mut result = HeuristicResult::zeroed();
        let mut count = 0i32;
        // SAFETY: all handles and descriptors are live and non-null; `result` receives 1 entry.
        check_lt("cublasLtMatmulAlgoGetHeuristic", unsafe {
            lt_sys::cublasLtMatmulAlgoGetHeuristic(
                self.raw,
                desc.raw,
                a.raw,
                b.raw,
                c.raw,
                d.raw,
                pref.raw,
                1,
                &mut result.raw,
                &mut count,
            )
        })?;
        if count <= 0 || result.raw.state != lt_sys::cublasStatus_t_CUBLAS_STATUS_SUCCESS {
            return Err(Error::Invalid(
                "cublasLtMatmulAlgoGetHeuristic found no supported algorithm",
            ));
        }
        Ok(result)
    }

    /// Queries up to `out.capacity()` heuristic algorithms into `out` (or resizes `out` to
    /// `max_results`), returning the number of valid entries written.
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatmulAlgoGetHeuristic requires desc, 4 layouts, pref, max_results, and output vec"
    )]
    pub fn matmul_heuristics(
        &self,
        desc: &MatmulDesc,
        a: &MatrixLayout,
        b: &MatrixLayout,
        c: &MatrixLayout,
        d: &MatrixLayout,
        pref: &Preference,
        max_results: usize,
        out: &mut Vec<HeuristicResult>,
    ) -> Result<usize> {
        if max_results == 0 {
            out.clear();
            return Ok(0);
        }
        self.cuda.bind_to_thread()?;
        let req = i32::try_from(max_results)
            .map_err(|_| Error::Invalid("max_results exceeds i32::MAX"))?;
        out.resize(max_results, HeuristicResult::zeroed());
        let mut count = 0i32;
        // SAFETY: `HeuristicResult` is `#[repr(transparent)]` over `cublasLtMatmulHeuristicResult_t`,
        // so `out.as_mut_ptr().cast()` is valid for `max_results` elements.
        check_lt("cublasLtMatmulAlgoGetHeuristic", unsafe {
            lt_sys::cublasLtMatmulAlgoGetHeuristic(
                self.raw,
                desc.raw,
                a.raw,
                b.raw,
                c.raw,
                d.raw,
                pref.raw,
                req,
                out.as_mut_ptr().cast(),
                &mut count,
            )
        })?;
        let n = (count.max(0) as usize).min(max_results);
        out.truncate(n);
        out.retain(HeuristicResult::is_valid);
        Ok(out.len())
    }

    /// Executes `D = alpha * op(A) * op(B) + beta * C` using `heuristic` and a caller-owned
    /// [`Scratch`] workspace bounded by `budget`.
    ///
    /// # Safety
    ///
    /// `alpha` and `beta` must point to valid scale values matching `desc`'s scale type and
    /// pointer mode. `a`, `b`, `c`, and `d` must hold at least the byte extents described by
    /// `a_desc`, `b_desc`, `c_desc`, and `d_desc`.
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatmul requires descriptors, buffers, algo, scratch, and stream"
    )]
    pub unsafe fn matmul_with_scratch(
        &self,
        stream: &Stream,
        desc: &MatmulDesc,
        alpha: *const c_void,
        a: &DeviceBuffer<u8>,
        a_desc: &MatrixLayout,
        b: &DeviceBuffer<u8>,
        b_desc: &MatrixLayout,
        beta: *const c_void,
        c: &DeviceBuffer<u8>,
        c_desc: &MatrixLayout,
        d: &mut DeviceBuffer<u8>,
        d_desc: &MatrixLayout,
        heuristic: &HeuristicResult,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<()> {
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than LtHandle",
            ));
        }
        if !std::ptr::eq(stream, &**scratch.stream()) {
            return Err(Error::Invalid(
                "scratch stream does not match matmul execution stream",
            ));
        }
        if alpha.is_null() || beta.is_null() {
            return Err(Error::Invalid("alpha and beta pointers must be non-null"));
        }
        let ws_bytes = heuristic.workspace_size();
        let (ws_ptr, _) = scratch.reserve(stream, ws_bytes, budget)?;
        self.cuda.bind_to_thread()?;
        // SAFETY: caller guarantees `alpha`/`beta` and matrix buffer extents match the descriptors;
        // `ws_ptr` has at least `ws_bytes` on `stream`.
        check_lt("cublasLtMatmul", unsafe {
            lt_sys::cublasLtMatmul(
                self.raw,
                desc.raw,
                alpha,
                a.cu_deviceptr() as usize as *const c_void,
                a_desc.raw,
                b.cu_deviceptr() as usize as *const c_void,
                b_desc.raw,
                beta,
                c.cu_deviceptr() as usize as *const c_void,
                c_desc.raw,
                d.cu_deviceptr() as usize as *mut c_void,
                d_desc.raw,
                &heuristic.raw.algo,
                ws_ptr,
                ws_bytes,
                stream.cu_stream(),
            )
        })
    }

    /// Convenience `f32` matmul with host scalars `alpha` and `beta` and caller-owned [`Scratch`].
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatmul convenience wrapper"
    )]
    pub fn matmul_f32(
        &self,
        stream: &Stream,
        desc: &MatmulDesc,
        alpha: f32,
        a: &DeviceBuffer<f32>,
        a_desc: &MatrixLayout,
        b: &DeviceBuffer<f32>,
        b_desc: &MatrixLayout,
        beta: f32,
        c: &DeviceBuffer<f32>,
        c_desc: &MatrixLayout,
        d: &mut DeviceBuffer<f32>,
        d_desc: &MatrixLayout,
        heuristic: &HeuristicResult,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<()> {
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than LtHandle",
            ));
        }
        if !std::ptr::eq(stream, &**scratch.stream()) {
            return Err(Error::Invalid(
                "scratch stream does not match matmul execution stream",
            ));
        }
        if desc.pointer_mode != PointerMode::Host || desc.scale != DataType::F32 {
            return Err(Error::Invalid(
                "matmul_f32 requires PointerMode::Host and DataType::F32 scale",
            ));
        }
        if a_desc.dtype != DataType::F32
            || b_desc.dtype != DataType::F32
            || c_desc.dtype != DataType::F32
            || d_desc.dtype != DataType::F32
        {
            return Err(Error::Invalid(
                "matmul_f32 requires DataType::F32 for all matrix layouts",
            ));
        }
        if a.len() < a_desc.min_elems()?
            || b.len() < b_desc.min_elems()?
            || c.len() < c_desc.min_elems()?
            || d.len() < d_desc.min_elems()?
        {
            return Err(Error::Invalid(
                "device buffer smaller than required MatrixLayout extent",
            ));
        }
        d_desc.check_output_batch()?;
        let ws_bytes = heuristic.workspace_size();
        let (ws_ptr, _) = scratch.reserve(stream, ws_bytes, budget)?;
        self.cuda.bind_to_thread()?;
        // SAFETY: `alpha` and `beta` are valid host `f32` scalars; descriptors and `f32` device
        // buffers are owned and live on `stream`.
        check_lt("cublasLtMatmul", unsafe {
            lt_sys::cublasLtMatmul(
                self.raw,
                desc.raw,
                (&raw const alpha).cast(),
                a.cu_deviceptr() as usize as *const c_void,
                a_desc.raw,
                b.cu_deviceptr() as usize as *const c_void,
                b_desc.raw,
                (&raw const beta).cast(),
                c.cu_deviceptr() as usize as *const c_void,
                c_desc.raw,
                d.cu_deviceptr() as usize as *mut c_void,
                d_desc.raw,
                &heuristic.raw.algo,
                ws_ptr,
                ws_bytes,
                stream.cu_stream(),
            )
        })
    }

    /// Queries the single best heuristic algorithm for a specific CUDA stream
    /// (`cublasLtMatmulAlgoGetHeuristicForStream`).
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatmulAlgoGetHeuristicForStream requires stream, desc, 4 layouts, and pref"
    )]
    pub fn matmul_heuristic_for_stream(
        &self,
        stream: &Stream,
        desc: &MatmulDesc,
        a: &MatrixLayout,
        b: &MatrixLayout,
        c: &MatrixLayout,
        d: &MatrixLayout,
        pref: &Preference,
    ) -> Result<HeuristicResult> {
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than LtHandle",
            ));
        }
        self.cuda.bind_to_thread()?;
        let mut result = HeuristicResult::zeroed();
        let mut count = 0i32;
        // SAFETY: all handles, descriptors, and `stream` are live on the same context; `result` receives 1 entry.
        check_lt("cublasLtMatmulAlgoGetHeuristicForStream", unsafe {
            lt_sys::cublasLtMatmulAlgoGetHeuristicForStream(
                self.raw,
                desc.raw,
                a.raw,
                b.raw,
                c.raw,
                d.raw,
                pref.raw,
                1,
                &mut result.raw,
                &mut count,
                stream.cu_stream(),
            )
        })?;
        if count <= 0 || !result.is_valid() {
            return Err(Error::Invalid(
                "cublasLtMatmulAlgoGetHeuristicForStream found no supported algorithm",
            ));
        }
        Ok(result)
    }

    /// Queries up to `max_results` heuristic algorithms for a specific CUDA stream into `out`
    /// (`cublasLtMatmulAlgoGetHeuristicForStream`).
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatmulAlgoGetHeuristicForStream requires stream, desc, 4 layouts, pref, max_results, and out"
    )]
    pub fn matmul_heuristics_for_stream(
        &self,
        stream: &Stream,
        desc: &MatmulDesc,
        a: &MatrixLayout,
        b: &MatrixLayout,
        c: &MatrixLayout,
        d: &MatrixLayout,
        pref: &Preference,
        max_results: usize,
        out: &mut Vec<HeuristicResult>,
    ) -> Result<usize> {
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than LtHandle",
            ));
        }
        if max_results == 0 {
            out.clear();
            return Ok(0);
        }
        self.cuda.bind_to_thread()?;
        let req = i32::try_from(max_results)
            .map_err(|_| Error::Invalid("max_results exceeds i32::MAX"))?;
        out.resize(max_results, HeuristicResult::zeroed());
        let mut count = 0i32;
        // SAFETY: `HeuristicResult` is `#[repr(transparent)]` over `cublasLtMatmulHeuristicResult_t`,
        // so `out.as_mut_ptr().cast()` is valid for `max_results` elements.
        check_lt("cublasLtMatmulAlgoGetHeuristicForStream", unsafe {
            lt_sys::cublasLtMatmulAlgoGetHeuristicForStream(
                self.raw,
                desc.raw,
                a.raw,
                b.raw,
                c.raw,
                d.raw,
                pref.raw,
                req,
                out.as_mut_ptr().cast(),
                &mut count,
                stream.cu_stream(),
            )
        })?;
        let n = (count.max(0) as usize).min(max_results);
        out.truncate(n);
        out.retain(HeuristicResult::is_valid);
        Ok(out.len())
    }

    /// Lists up to `max_results` supported algorithm IDs for the given type signature into `out`
    /// (`cublasLtMatmulAlgoGetIds`).
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatmulAlgoGetIds requires compute, scale, 4 matrix types, max_results, and out"
    )]
    pub fn matmul_algo_ids(
        &self,
        compute: ComputeType,
        scale: DataType,
        a_type: DataType,
        b_type: DataType,
        c_type: DataType,
        d_type: DataType,
        max_results: usize,
        out: &mut Vec<i32>,
    ) -> Result<usize> {
        if max_results == 0 {
            out.clear();
            return Ok(0);
        }
        self.cuda.bind_to_thread()?;
        let req = i32::try_from(max_results)
            .map_err(|_| Error::Invalid("max_results exceeds i32::MAX"))?;
        out.resize(max_results, 0);
        let mut count = 0i32;
        // SAFETY: `out` has `max_results` `i32` slots and `self.raw` is a live handle.
        check_lt("cublasLtMatmulAlgoGetIds", unsafe {
            lt_sys::cublasLtMatmulAlgoGetIds(
                self.raw,
                compute.raw_lt(),
                scale.raw_lt(),
                a_type.raw_lt(),
                b_type.raw_lt(),
                c_type.raw_lt(),
                d_type.raw_lt(),
                req,
                out.as_mut_ptr(),
                &mut count,
            )
        })?;
        let n = (count.max(0) as usize).min(max_results);
        out.truncate(n);
        Ok(out.len())
    }

    /// Initializes a [`MatmulAlgo`] descriptor for `algo_id` (`cublasLtMatmulAlgoInit`).
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatmulAlgoInit requires compute, scale, 4 matrix types, and algo_id"
    )]
    pub fn matmul_algo_init(
        &self,
        compute: ComputeType,
        scale: DataType,
        a_type: DataType,
        b_type: DataType,
        c_type: DataType,
        d_type: DataType,
        algo_id: i32,
    ) -> Result<MatmulAlgo> {
        self.cuda.bind_to_thread()?;
        let mut algo = MatmulAlgo::zeroed();
        // SAFETY: `self.raw` is live and `algo.raw` is a valid out-pointer.
        check_lt("cublasLtMatmulAlgoInit", unsafe {
            lt_sys::cublasLtMatmulAlgoInit(
                self.raw,
                compute.raw_lt(),
                scale.raw_lt(),
                a_type.raw_lt(),
                b_type.raw_lt(),
                c_type.raw_lt(),
                d_type.raw_lt(),
                algo_id,
                &mut algo.raw,
            )
        })?;
        Ok(algo)
    }

    /// Validates an explicitly configured [`MatmulAlgo`] against the problem descriptors
    /// (`cublasLtMatmulAlgoCheck`).
    pub fn matmul_algo_check(
        &self,
        desc: &MatmulDesc,
        a: &MatrixLayout,
        b: &MatrixLayout,
        c: &MatrixLayout,
        d: &MatrixLayout,
        algo: &MatmulAlgo,
    ) -> Result<HeuristicResult> {
        self.cuda.bind_to_thread()?;
        let mut result = HeuristicResult::zeroed();
        // SAFETY: all handles and descriptors are live; `result.raw` is a valid out-pointer.
        check_lt("cublasLtMatmulAlgoCheck", unsafe {
            lt_sys::cublasLtMatmulAlgoCheck(
                self.raw,
                desc.raw,
                a.raw,
                b.raw,
                c.raw,
                d.raw,
                &algo.raw,
                &mut result.raw,
            )
        })?;
        // `cublasLtMatmulAlgoCheck` never updates `result.algo` (`cublasLt.h`); carry the checked
        // algo so the result is directly usable with `matmul_f32` and its checked workspace size.
        result.raw.algo = algo.raw;
        Ok(result)
    }

    /// Validates an explicitly configured [`MatmulAlgo`] for a specific CUDA stream
    /// (`cublasLtMatmulAlgoCheckForStream`).
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatmulAlgoCheckForStream requires stream, desc, 4 layouts, and algo"
    )]
    pub fn matmul_algo_check_for_stream(
        &self,
        stream: &Stream,
        desc: &MatmulDesc,
        a: &MatrixLayout,
        b: &MatrixLayout,
        c: &MatrixLayout,
        d: &MatrixLayout,
        algo: &MatmulAlgo,
    ) -> Result<HeuristicResult> {
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than LtHandle",
            ));
        }
        self.cuda.bind_to_thread()?;
        let mut result = HeuristicResult::zeroed();
        // SAFETY: all handles, descriptors, and `stream` are live on the same context.
        check_lt("cublasLtMatmulAlgoCheckForStream", unsafe {
            lt_sys::cublasLtMatmulAlgoCheckForStream(
                self.raw,
                desc.raw,
                a.raw,
                b.raw,
                c.raw,
                d.raw,
                &algo.raw,
                &mut result.raw,
                stream.cu_stream(),
            )
        })?;
        // `cublasLtMatmulAlgoCheckForStream` never updates `result.algo`; see `matmul_algo_check`.
        result.raw.algo = algo.raw;
        Ok(result)
    }

    /// Executes `C = alpha * op(A) + beta * op(B)` (`cublasLtMatrixTransform`).
    ///
    /// # Safety
    ///
    /// `alpha` and `beta` must point to valid scale values matching `desc`'s scale type and
    /// pointer mode. `a`, optional `b`, and `c` must hold at least the byte extents described by
    /// `a_desc`, `b_desc`, and `c_desc`.
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatrixTransform requires stream, desc, scales, 3 buffers/layouts"
    )]
    pub unsafe fn matrix_transform(
        &self,
        stream: &Stream,
        desc: &MatrixTransformDesc,
        alpha: *const c_void,
        a: &DeviceBuffer<u8>,
        a_desc: &MatrixLayout,
        beta: *const c_void,
        b: Option<(&DeviceBuffer<u8>, &MatrixLayout)>,
        c: &mut DeviceBuffer<u8>,
        c_desc: &MatrixLayout,
    ) -> Result<()> {
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than LtHandle",
            ));
        }
        if alpha.is_null() || beta.is_null() {
            return Err(Error::Invalid("alpha and beta pointers must be non-null"));
        }
        let (b_ptr, b_raw_desc) = match b {
            Some((buf, layout)) => (buf.cu_deviceptr() as usize as *const c_void, layout.raw),
            None => (std::ptr::null(), std::ptr::null_mut()),
        };
        self.cuda.bind_to_thread()?;
        // SAFETY: caller guarantees `alpha`/`beta` and matrix buffer extents match the descriptors.
        check_lt("cublasLtMatrixTransform", unsafe {
            lt_sys::cublasLtMatrixTransform(
                self.raw,
                desc.raw,
                alpha,
                a.cu_deviceptr() as usize as *const c_void,
                a_desc.raw,
                beta,
                b_ptr,
                b_raw_desc,
                c.cu_deviceptr() as usize as *mut c_void,
                c_desc.raw,
                stream.cu_stream(),
            )
        })
    }

    /// Convenience `f32` matrix transform `C = alpha * op(A) + beta * op(B)` with host scalars.
    #[expect(
        clippy::too_many_arguments,
        reason = "cublasLtMatrixTransform convenience wrapper"
    )]
    pub fn matrix_transform_f32(
        &self,
        stream: &Stream,
        desc: &MatrixTransformDesc,
        alpha: f32,
        a: &DeviceBuffer<f32>,
        a_desc: &MatrixLayout,
        beta: f32,
        b: Option<(&DeviceBuffer<f32>, &MatrixLayout)>,
        c: &mut DeviceBuffer<f32>,
        c_desc: &MatrixLayout,
    ) -> Result<()> {
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than LtHandle",
            ));
        }
        if desc.pointer_mode != PointerMode::Host || desc.scale != DataType::F32 {
            return Err(Error::Invalid(
                "matrix_transform_f32 requires PointerMode::Host and DataType::F32 scale",
            ));
        }
        if a_desc.dtype != DataType::F32 || c_desc.dtype != DataType::F32 {
            return Err(Error::Invalid(
                "matrix_transform_f32 requires DataType::F32 for A and C layouts",
            ));
        }
        if a.len() < a_desc.min_elems()? || c.len() < c_desc.min_elems()? {
            return Err(Error::Invalid(
                "device buffer smaller than required MatrixLayout extent",
            ));
        }
        c_desc.check_output_batch()?;
        let (b_ptr, b_raw_desc) = match b {
            Some((buf, layout)) => {
                if layout.dtype != DataType::F32 {
                    return Err(Error::Invalid(
                        "matrix_transform_f32 requires DataType::F32 for B layout",
                    ));
                }
                if buf.len() < layout.min_elems()? {
                    return Err(Error::Invalid(
                        "B device buffer smaller than required MatrixLayout extent",
                    ));
                }
                (buf.cu_deviceptr() as usize as *const c_void, layout.raw)
            }
            None => (std::ptr::null(), std::ptr::null_mut()),
        };
        self.cuda.bind_to_thread()?;
        // SAFETY: `alpha` and `beta` are valid host `f32` scalars; descriptors and `f32` device
        // buffers are owned and live on `stream`.
        check_lt("cublasLtMatrixTransform", unsafe {
            lt_sys::cublasLtMatrixTransform(
                self.raw,
                desc.raw,
                (&raw const alpha).cast(),
                a.cu_deviceptr() as usize as *const c_void,
                a_desc.raw,
                (&raw const beta).cast(),
                b_ptr,
                b_raw_desc,
                c.cu_deviceptr() as usize as *mut c_void,
                c_desc.raw,
                stream.cu_stream(),
            )
        })
    }
}

impl Drop for LtHandle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            if self.cuda.bind_to_thread().is_ok() {
                // SAFETY: `self.raw` was created by `cublasLtCreate` and is destroyed once.
                let code = unsafe { lt_sys::cublasLtDestroy(self.raw) };
                teardown_lt(&self.cuda, code);
            } else {
                teardown_lt(
                    &self.cuda,
                    lt_sys::cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
                );
            }
            self.raw = std::ptr::null_mut();
        }
    }
}

/// Attribute selector for [`MatmulDesc`] (`cublasLtMatmulDescAttributes_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MatmulDescAttr {
    /// Compute type (`CUBLASLT_MATMUL_DESC_COMPUTE_TYPE`).
    ComputeType,
    /// Scale type (`CUBLASLT_MATMUL_DESC_SCALE_TYPE`).
    ScaleType,
    /// Pointer mode (`CUBLASLT_MATMUL_DESC_POINTER_MODE`).
    PointerMode,
    /// Matrix A op (`CUBLASLT_MATMUL_DESC_TRANSA`).
    TransA,
    /// Matrix B op (`CUBLASLT_MATMUL_DESC_TRANSB`).
    TransB,
    /// Matrix C op (`CUBLASLT_MATMUL_DESC_TRANSC`).
    TransC,
    /// Fill mode (`CUBLASLT_MATMUL_DESC_FILL_MODE`).
    FillMode,
    /// Epilogue (`CUBLASLT_MATMUL_DESC_EPILOGUE`).
    Epilogue,
    /// Bias pointer (`CUBLASLT_MATMUL_DESC_BIAS_POINTER`).
    BiasPointer,
    /// Bias batch stride (`CUBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE`).
    BiasBatchStride,
    /// Epilogue aux pointer (`CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER`).
    EpilogueAuxPointer,
    /// Epilogue aux leading dimension (`CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD`).
    EpilogueAuxLd,
    /// Epilogue aux batch stride (`CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE`).
    EpilogueAuxBatchStride,
    /// Alpha vector batch stride (`CUBLASLT_MATMUL_DESC_ALPHA_VECTOR_BATCH_STRIDE`).
    AlphaVectorBatchStride,
    /// SM count target (`CUBLASLT_MATMUL_DESC_SM_COUNT_TARGET`).
    SmCountTarget,
    /// A scale pointer (`CUBLASLT_MATMUL_DESC_A_SCALE_POINTER`).
    AScalePointer,
    /// B scale pointer (`CUBLASLT_MATMUL_DESC_B_SCALE_POINTER`).
    BScalePointer,
    /// C scale pointer (`CUBLASLT_MATMUL_DESC_C_SCALE_POINTER`).
    CScalePointer,
    /// D scale pointer (`CUBLASLT_MATMUL_DESC_D_SCALE_POINTER`).
    DScalePointer,
    /// Amax D pointer (`CUBLASLT_MATMUL_DESC_AMAX_D_POINTER`).
    AmaxDPointer,
    /// Epilogue aux data type (`CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_DATA_TYPE`).
    EpilogueAuxDataType,
    /// Epilogue aux scale pointer (`CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER`).
    EpilogueAuxScalePointer,
    /// Epilogue aux amax pointer (`CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_AMAX_POINTER`).
    EpilogueAuxAmaxPointer,
    /// Fast accumulation (`CUBLASLT_MATMUL_DESC_FAST_ACCUM`).
    FastAccum,
    /// Bias data type (`CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE`).
    BiasDataType,
    /// A scale mode (`CUBLASLT_MATMUL_DESC_A_SCALE_MODE`).
    AScaleMode,
    /// B scale mode (`CUBLASLT_MATMUL_DESC_B_SCALE_MODE`).
    BScaleMode,
    /// C scale mode (`CUBLASLT_MATMUL_DESC_C_SCALE_MODE`).
    CScaleMode,
    /// D scale mode (`CUBLASLT_MATMUL_DESC_D_SCALE_MODE`).
    DScaleMode,
    /// Epilogue aux scale mode (`CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_MODE`).
    EpilogueAuxScaleMode,
    /// D out scale pointer (`CUBLASLT_MATMUL_DESC_D_OUT_SCALE_POINTER`).
    DOutScalePointer,
    /// D out scale mode (`CUBLASLT_MATMUL_DESC_D_OUT_SCALE_MODE`).
    DOutScaleMode,
    /// Emulation descriptor (`CUBLASLT_MATMUL_DESC_EMULATION_DESCRIPTOR`).
    EmulationDescriptor,
    /// Alpha batch stride (`CUBLASLT_MATMUL_DESC_ALPHA_BATCH_STRIDE`).
    AlphaBatchStride,
    /// Beta batch stride (`CUBLASLT_MATMUL_DESC_BETA_BATCH_STRIDE`).
    BetaBatchStride,
    /// Raw `cublasLtMatmulDescAttributes_t` value.
    Raw(lt_sys::cublasLtMatmulDescAttributes_t),
}

impl MatmulDescAttr {
    /// Converts to `cublaslt_sys::cublasLtMatmulDescAttributes_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtMatmulDescAttributes_t {
        match self {
            Self::ComputeType => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_COMPUTE_TYPE
            }
            Self::ScaleType => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_SCALE_TYPE
            }
            Self::PointerMode => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_POINTER_MODE
            }
            Self::TransA => lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_TRANSA,
            Self::TransB => lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_TRANSB,
            Self::TransC => lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_TRANSC,
            Self::FillMode => lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_FILL_MODE,
            Self::Epilogue => lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE,
            Self::BiasPointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_BIAS_POINTER
            }
            Self::BiasBatchStride => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE
            }
            Self::EpilogueAuxPointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER
            }
            Self::EpilogueAuxLd => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD
            }
            Self::EpilogueAuxBatchStride => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE
            }
            Self::AlphaVectorBatchStride => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_ALPHA_VECTOR_BATCH_STRIDE
            }
            Self::SmCountTarget => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_SM_COUNT_TARGET
            }
            Self::AScalePointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_A_SCALE_POINTER
            }
            Self::BScalePointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_B_SCALE_POINTER
            }
            Self::CScalePointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_C_SCALE_POINTER
            }
            Self::DScalePointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_D_SCALE_POINTER
            }
            Self::AmaxDPointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_AMAX_D_POINTER
            }
            Self::EpilogueAuxDataType => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_DATA_TYPE
            }
            Self::EpilogueAuxScalePointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER
            }
            Self::EpilogueAuxAmaxPointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_AMAX_POINTER
            }
            Self::FastAccum => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_FAST_ACCUM
            }
            Self::BiasDataType => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE
            }
            Self::AScaleMode => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_A_SCALE_MODE
            }
            Self::BScaleMode => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_B_SCALE_MODE
            }
            Self::CScaleMode => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_C_SCALE_MODE
            }
            Self::DScaleMode => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_D_SCALE_MODE
            }
            Self::EpilogueAuxScaleMode => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_MODE
            }
            Self::DOutScalePointer => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_D_OUT_SCALE_POINTER
            }
            Self::DOutScaleMode => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_D_OUT_SCALE_MODE
            }
            Self::EmulationDescriptor => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EMULATION_DESCRIPTOR
            }
            Self::AlphaBatchStride => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_ALPHA_BATCH_STRIDE
            }
            Self::BetaBatchStride => {
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_BETA_BATCH_STRIDE
            }
            Self::Raw(v) => v,
        }
    }
}

/// Matmul operation descriptor (`cublasLtMatmulDesc_t`).
#[must_use]
pub struct MatmulDesc {
    raw: lt_sys::cublasLtMatmulDesc_t,
    cuda: Arc<Context>,
    scale: DataType,
    pointer_mode: PointerMode,
}

// SAFETY: descriptor owns no thread-local state; concurrent mutation requires external sync (`!Sync`).
unsafe impl Send for MatmulDesc {}

impl MatmulDesc {
    /// Creates a new matmul descriptor with `compute` precision and `scale` scalar data type.
    pub fn new(cuda: &Arc<Context>, compute: ComputeType, scale: DataType) -> Result<Self> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check_lt("cublasLtMatmulDescCreate", unsafe {
            lt_sys::cublasLtMatmulDescCreate(&mut raw, compute.raw_lt(), scale.raw_lt())
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            scale,
            pointer_mode: PointerMode::Host,
        })
    }

    /// Writes raw attribute bytes (`cublasLtMatmulDescSetAttribute`).
    ///
    /// # Safety
    ///
    /// cuBLASLt trusts these bytes during every later matmul with this descriptor:
    /// - do not change `PointerMode` or `ScaleType` here: [`LtHandle::matmul_f32`] validates its
    ///   host `f32` scalars against values cached by [`Self::new`] and [`Self::set_pointer_mode`];
    /// - every pointer attribute (bias, epilogue aux, scale, amax, D-out scale) must be null or a
    ///   device allocation of `self`'s context covering the extent the descriptor implies, alive
    ///   until all matmuls using it complete; `EmulationDescriptor` must name a live descriptor;
    /// - scale modes and alpha/beta batch strides must match the buffers those pointers address.
    pub unsafe fn set_attribute_bytes(&mut self, attr: MatmulDescAttr, buf: &[u8]) -> Result<()> {
        self.set_attr(attr, buf)
    }

    /// Writes an attribute whose value the typed setters have already validated.
    fn set_attr(&mut self, attr: MatmulDescAttr, buf: &[u8]) -> Result<()> {
        // SAFETY: `self.raw` is live and `buf` is valid for `buf.len()` bytes.
        check_lt("cublasLtMatmulDescSetAttribute", unsafe {
            lt_sys::cublasLtMatmulDescSetAttribute(
                self.raw,
                attr.raw(),
                buf.as_ptr().cast(),
                buf.len(),
            )
        })
    }

    /// Reads raw attribute bytes (`cublasLtMatmulDescGetAttribute`), returning bytes written.
    pub fn get_attribute_bytes(&self, attr: MatmulDescAttr, buf: &mut [u8]) -> Result<usize> {
        let mut written = 0usize;
        // SAFETY: `self.raw` is live, `buf` is writable for `buf.len()` bytes, `written` is valid.
        check_lt("cublasLtMatmulDescGetAttribute", unsafe {
            lt_sys::cublasLtMatmulDescGetAttribute(
                self.raw,
                attr.raw(),
                buf.as_mut_ptr().cast(),
                buf.len(),
                &mut written,
            )
        })?;
        Ok(written)
    }

    /// Sets the transpose operation for matrix A (`CUBLASLT_MATMUL_DESC_TRANSA`).
    pub fn set_trans_a(&mut self, op: Operation) -> Result<()> {
        let v = op.raw_lt();
        self.set_attr(MatmulDescAttr::TransA, &v.to_ne_bytes())
    }

    /// Queries the transpose operation for matrix A (`CUBLASLT_MATMUL_DESC_TRANSA`).
    pub fn trans_a(&self) -> Result<Operation> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.get_attribute_bytes(MatmulDescAttr::TransA, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected TransA attribute size"));
        }
        op_from_lt(i32::from_ne_bytes(buf))
    }

    /// Sets the transpose operation for matrix B (`CUBLASLT_MATMUL_DESC_TRANSB`).
    pub fn set_trans_b(&mut self, op: Operation) -> Result<()> {
        let v = op.raw_lt();
        self.set_attr(MatmulDescAttr::TransB, &v.to_ne_bytes())
    }

    /// Queries the transpose operation for matrix B (`CUBLASLT_MATMUL_DESC_TRANSB`).
    pub fn trans_b(&self) -> Result<Operation> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.get_attribute_bytes(MatmulDescAttr::TransB, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected TransB attribute size"));
        }
        op_from_lt(i32::from_ne_bytes(buf))
    }

    /// Sets the transpose operation for matrix C (`CUBLASLT_MATMUL_DESC_TRANSC`).
    pub fn set_trans_c(&mut self, op: Operation) -> Result<()> {
        let v = op.raw_lt();
        self.set_attr(MatmulDescAttr::TransC, &v.to_ne_bytes())
    }

    /// Sets the scalar pointer mode (`CUBLASLT_MATMUL_DESC_POINTER_MODE`).
    pub fn set_pointer_mode(&mut self, mode: PointerMode) -> Result<()> {
        let v = mode.raw_lt();
        self.set_attr(MatmulDescAttr::PointerMode, &v.to_ne_bytes())?;
        self.pointer_mode = mode;
        Ok(())
    }

    /// Queries the scalar pointer mode (`CUBLASLT_MATMUL_DESC_POINTER_MODE`).
    pub fn pointer_mode(&self) -> Result<PointerMode> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.get_attribute_bytes(MatmulDescAttr::PointerMode, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected PointerMode attribute size"));
        }
        pointer_mode_from_lt(i32::from_ne_bytes(buf))
    }

    /// Sets the fused epilogue (`CUBLASLT_MATMUL_DESC_EPILOGUE`).
    pub fn set_epilogue(&mut self, epilogue: Epilogue) -> Result<()> {
        let v = epilogue.raw();
        self.set_attr(MatmulDescAttr::Epilogue, &v.to_ne_bytes())
    }

    /// Queries the fused epilogue (`CUBLASLT_MATMUL_DESC_EPILOGUE`).
    pub fn epilogue(&self) -> Result<Epilogue> {
        let mut buf = [0u8; size_of::<lt_sys::cublasLtEpilogue_t>()];
        let n = self.get_attribute_bytes(MatmulDescAttr::Epilogue, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected Epilogue attribute size"));
        }
        Ok(Epilogue::from_raw(
            lt_sys::cublasLtEpilogue_t::from_ne_bytes(buf),
        ))
    }

    /// Sets the target SM count hint (`CUBLASLT_MATMUL_DESC_SM_COUNT_TARGET`).
    pub fn set_sm_count_target(&mut self, sm_count: i32) -> Result<()> {
        self.set_attr(MatmulDescAttr::SmCountTarget, &sm_count.to_ne_bytes())
    }

    /// Enables or disables fast FP8 accumulation (`CUBLASLT_MATMUL_DESC_FAST_ACCUM`).
    pub fn set_fast_accum(&mut self, fast_accum: bool) -> Result<()> {
        let v: i8 = i8::from(fast_accum);
        self.set_attr(MatmulDescAttr::FastAccum, &v.to_ne_bytes())
    }

    /// Sets the device bias vector pointer (`CUBLASLT_MATMUL_DESC_BIAS_POINTER`).
    ///
    /// # Safety
    ///
    /// `bias_ptr` must be null or point to a valid device buffer outliving matmul execution.
    pub unsafe fn set_bias_pointer(&mut self, bias_ptr: *const c_void) -> Result<()> {
        // SAFETY: `bias_ptr` is passed as a pointer-sized attribute value.
        check_lt("cublasLtMatmulDescSetAttribute", unsafe {
            lt_sys::cublasLtMatmulDescSetAttribute(
                self.raw,
                MatmulDescAttr::BiasPointer.raw(),
                (&raw const bias_ptr).cast(),
                size_of_val(&bias_ptr),
            )
        })
    }

    /// Sets the bias vector element data type (`CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE`).
    pub fn set_bias_data_type(&mut self, dtype: DataType) -> Result<()> {
        let v = dtype.raw_lt();
        self.set_attr(MatmulDescAttr::BiasDataType, &v.to_ne_bytes())
    }

    /// Sets the bias batch stride in elements (`CUBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE`).
    pub fn set_bias_batch_stride(&mut self, stride: i64) -> Result<()> {
        self.set_attr(MatmulDescAttr::BiasBatchStride, &stride.to_ne_bytes())
    }

    /// Attaches an [`EmulationDesc`] (`CUBLASLT_MATMUL_DESC_EMULATION_DESCRIPTOR`).
    pub fn set_emulation_desc(&mut self, emulation: &EmulationDesc) -> Result<()> {
        let v = emulation.raw;
        // SAFETY: `v` is a live `cublasLtEmulationDesc_t` handle.
        check_lt("cublasLtMatmulDescSetAttribute", unsafe {
            lt_sys::cublasLtMatmulDescSetAttribute(
                self.raw,
                MatmulDescAttr::EmulationDescriptor.raw(),
                (&raw const v).cast(),
                size_of_val(&v),
            )
        })
    }
}

impl Drop for MatmulDesc {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cublasLtMatmulDescCreate` and is destroyed once.
            let code = unsafe { lt_sys::cublasLtMatmulDescDestroy(self.raw) };
            teardown_lt(&self.cuda, code);
            self.raw = std::ptr::null_mut();
        }
    }
}

/// Matrix batching mode (`cublasLtBatchMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BatchMode {
    /// Strided batching (`CUBLASLT_BATCH_MODE_STRIDED`).
    Strided,
    /// Pointer-array batching (`CUBLASLT_BATCH_MODE_POINTER_ARRAY`).
    PointerArray,
    /// Grouped batching (`CUBLASLT_BATCH_MODE_GROUPED`).
    Grouped,
    /// Raw `cublasLtBatchMode_t` value.
    Raw(lt_sys::cublasLtBatchMode_t),
}

impl BatchMode {
    /// Converts to `cublaslt_sys::cublasLtBatchMode_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtBatchMode_t {
        match self {
            Self::Strided => lt_sys::cublasLtBatchMode_t_CUBLASLT_BATCH_MODE_STRIDED,
            Self::PointerArray => lt_sys::cublasLtBatchMode_t_CUBLASLT_BATCH_MODE_POINTER_ARRAY,
            Self::Grouped => lt_sys::cublasLtBatchMode_t_CUBLASLT_BATCH_MODE_GROUPED,
            Self::Raw(v) => v,
        }
    }
}

/// Attribute selector for [`MatrixLayout`] (`cublasLtMatrixLayoutAttribute_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MatrixLayoutAttr {
    /// Element data type (`CUBLASLT_MATRIX_LAYOUT_TYPE`).
    Type,
    /// Memory order (`CUBLASLT_MATRIX_LAYOUT_ORDER`).
    Order,
    /// Number of rows (`CUBLASLT_MATRIX_LAYOUT_ROWS`).
    Rows,
    /// Number of columns (`CUBLASLT_MATRIX_LAYOUT_COLS`).
    Cols,
    /// Leading dimension (`CUBLASLT_MATRIX_LAYOUT_LD`).
    Ld,
    /// Batch count (`CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT`).
    BatchCount,
    /// Strided batch offset (`CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET`).
    StridedBatchOffset,
    /// Planar complex plane offset (`CUBLASLT_MATRIX_LAYOUT_PLANE_OFFSET`).
    PlaneOffset,
    /// Batch mode (`CUBLASLT_MATRIX_LAYOUT_BATCH_MODE`).
    BatchMode,
    /// Grouped rows array (`CUBLASLT_GROUPED_MATRIX_LAYOUT_ROWS_ARRAY`).
    GroupedRowsArray,
    /// Grouped cols array (`CUBLASLT_GROUPED_MATRIX_LAYOUT_COLS_ARRAY`).
    GroupedColsArray,
    /// Grouped ld array (`CUBLASLT_GROUPED_MATRIX_LAYOUT_LD_ARRAY`).
    GroupedLdArray,
    /// Grouped rows/cols integer width (`CUBLASLT_GROUPED_MATRIX_LAYOUT_ROWS_COLS_ARRAY_INTEGER_WIDTH`).
    GroupedRowsColsIntegerWidth,
    /// Grouped ld integer width (`CUBLASLT_GROUPED_MATRIX_LAYOUT_LD_ARRAY_INTEGER_WIDTH`).
    GroupedLdIntegerWidth,
    /// Raw `cublasLtMatrixLayoutAttribute_t` value.
    Raw(lt_sys::cublasLtMatrixLayoutAttribute_t),
}

impl MatrixLayoutAttr {
    /// Converts to `cublaslt_sys::cublasLtMatrixLayoutAttribute_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtMatrixLayoutAttribute_t {
        match self {
            Self::Type => lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_TYPE,
            Self::Order => lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_ORDER,
            Self::Rows => lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_ROWS,
            Self::Cols => lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_COLS,
            Self::Ld => lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_LD,
            Self::BatchCount => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT
            }
            Self::StridedBatchOffset => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET
            }
            Self::PlaneOffset => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_PLANE_OFFSET
            }
            Self::BatchMode => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_BATCH_MODE
            }
            Self::GroupedRowsArray => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_GROUPED_MATRIX_LAYOUT_ROWS_ARRAY
            }
            Self::GroupedColsArray => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_GROUPED_MATRIX_LAYOUT_COLS_ARRAY
            }
            Self::GroupedLdArray => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_GROUPED_MATRIX_LAYOUT_LD_ARRAY
            }
            Self::GroupedRowsColsIntegerWidth => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_GROUPED_MATRIX_LAYOUT_ROWS_COLS_ARRAY_INTEGER_WIDTH
            }
            Self::GroupedLdIntegerWidth => {
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_GROUPED_MATRIX_LAYOUT_LD_ARRAY_INTEGER_WIDTH
            }
            Self::Raw(v) => v,
        }
    }
}

/// Matrix layout descriptor (`cublasLtMatrixLayout_t`).
#[must_use]
pub struct MatrixLayout {
    raw: lt_sys::cublasLtMatrixLayout_t,
    cuda: Arc<Context>,
    dtype: DataType,
    rows: u64,
    cols: u64,
    ld: i64,
    order: Order,
    batch_count: i32,
    strided_offset: i64,
    /// `false` once the layout addresses pointer arrays (grouped layout or non-strided batch
    /// mode) instead of one dense buffer; [`Self::min_elems`] then has no meaning.
    dense: bool,
    /// Created by [`Self::new_grouped`]: `rows`/`cols`/`ld` are placeholders.
    grouped: bool,
}

/// Validates the documented minimum leading dimension for `order` (`cublasLt.h`,
/// `cublasLtOrder_t` and `CUBLASLT_MATRIX_LAYOUT_LD`: "large enough so that matrix memory
/// locations are not overlapping"); [`MatrixLayout::min_elems`] is only an upper bound under it.
fn validate_ld(order: Order, rows: u64, cols: u64, ld: i64) -> Result<()> {
    let ld = u64::try_from(ld).map_err(|_| Error::Invalid("negative leading dimension"))?;
    let min = match order {
        Order::Col => Some(rows),
        Order::Row => Some(cols),
        Order::Col32 => rows.checked_mul(32),
        Order::Col44r28c => rows.div_ceil(8).checked_mul(32 * 8),
        Order::Col322r4r4 => rows.div_ceil(32).checked_mul(32 * 32),
    }
    .ok_or(Error::Invalid("matrix layout leading dimension overflow"))?;
    if ld < min {
        return Err(Error::Invalid(
            "leading dimension is smaller than the matrix order requires",
        ));
    }
    Ok(())
}

// SAFETY: layout descriptor owns no thread-local state (`!Sync`).
unsafe impl Send for MatrixLayout {}

impl MatrixLayout {
    /// Creates a matrix layout descriptor for `rows x cols` with leading dimension `ld`.
    pub fn new(
        cuda: &Arc<Context>,
        dtype: DataType,
        rows: u64,
        cols: u64,
        ld: i64,
    ) -> Result<Self> {
        if rows == 0 || cols == 0 || ld <= 0 {
            return Err(Error::Invalid(
                "matrix layout requires positive rows, cols, and ld",
            ));
        }
        validate_ld(Order::Col, rows, cols, ld)?;
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check_lt("cublasLtMatrixLayoutCreate", unsafe {
            lt_sys::cublasLtMatrixLayoutCreate(&mut raw, dtype.raw_lt(), rows, cols, ld)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            dtype,
            rows,
            cols,
            ld,
            order: Order::Col,
            batch_count: 1,
            strided_offset: 0,
            dense: true,
            grouped: false,
        })
    }

    /// Creates a grouped matrix layout descriptor (`cublasLtGroupedMatrixLayoutCreate`).
    ///
    /// # Safety
    ///
    /// `rows_array`, `cols_array`, and `ld_array` must be valid pointers to `group_count`
    /// elements matching the configured integer width.
    pub unsafe fn new_grouped(
        cuda: &Arc<Context>,
        dtype: DataType,
        group_count: i32,
        rows_array: *const c_void,
        cols_array: *const c_void,
        ld_array: *const c_void,
    ) -> Result<Self> {
        if group_count <= 0 {
            return Err(Error::Invalid(
                "grouped matrix layout requires positive group_count",
            ));
        }
        if rows_array.is_null() || cols_array.is_null() || ld_array.is_null() {
            return Err(Error::Invalid(
                "grouped matrix layout arrays must be non-null",
            ));
        }
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: caller guarantees array pointers hold `group_count` elements; `raw` is valid.
        check_lt("cublasLtGroupedMatrixLayoutCreate", unsafe {
            lt_sys::cublasLtGroupedMatrixLayoutCreate(
                &mut raw,
                dtype.raw_lt(),
                group_count,
                rows_array,
                cols_array,
                ld_array,
            )
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            dtype,
            rows: 1,
            cols: 1,
            ld: 1,
            order: Order::Col,
            batch_count: group_count,
            strided_offset: 0,
            dense: false,
            grouped: true,
        })
    }

    /// Writes raw attribute bytes (`cublasLtMatrixLayoutSetAttribute`).
    ///
    /// # Safety
    ///
    /// Do not change `Type`, `Order`, `Rows`, `Cols`, `Ld`, `BatchCount`, `StridedBatchOffset` or
    /// `BatchMode` here: [`LtHandle::matmul_f32`] and [`LtHandle::matrix_transform_f32`] validate
    /// buffers against the values cached by the typed constructors and setters. Any other value
    /// (for example `PlaneOffset` or grouped arrays) must keep every buffer later bound to this
    /// layout valid for the extent cuBLASLt derives from it.
    pub unsafe fn set_attribute_bytes(
        &mut self,
        attr: MatrixLayoutAttr,
        buf: &[u8],
    ) -> Result<()> {
        self.set_attr(attr, buf)
    }

    /// Writes an attribute whose value the typed setters have already validated.
    fn set_attr(&mut self, attr: MatrixLayoutAttr, buf: &[u8]) -> Result<()> {
        // SAFETY: `self.raw` is live and `buf` is valid for `buf.len()` bytes.
        check_lt("cublasLtMatrixLayoutSetAttribute", unsafe {
            lt_sys::cublasLtMatrixLayoutSetAttribute(
                self.raw,
                attr.raw(),
                buf.as_ptr().cast(),
                buf.len(),
            )
        })
    }

    /// Reads raw attribute bytes (`cublasLtMatrixLayoutGetAttribute`), returning bytes written.
    pub fn get_attribute_bytes(&self, attr: MatrixLayoutAttr, buf: &mut [u8]) -> Result<usize> {
        let mut written = 0usize;
        // SAFETY: `self.raw` is live, `buf` is writable for `buf.len()` bytes, `written` is valid.
        check_lt("cublasLtMatrixLayoutGetAttribute", unsafe {
            lt_sys::cublasLtMatrixLayoutGetAttribute(
                self.raw,
                attr.raw(),
                buf.as_mut_ptr().cast(),
                buf.len(),
                &mut written,
            )
        })?;
        Ok(written)
    }

    /// Sets the matrix memory order (`CUBLASLT_MATRIX_LAYOUT_ORDER`).
    pub fn set_order(&mut self, order: Order) -> Result<()> {
        if !self.grouped {
            validate_ld(order, self.rows, self.cols, self.ld)?;
        }
        let v = order.raw();
        self.set_attr(MatrixLayoutAttr::Order, &v.to_ne_bytes())?;
        self.order = order;
        Ok(())
    }

    /// Queries the matrix memory order (`CUBLASLT_MATRIX_LAYOUT_ORDER`).
    pub fn order(&self) -> Result<Order> {
        let mut buf = [0u8; size_of::<lt_sys::cublasLtOrder_t>()];
        let n = self.get_attribute_bytes(MatrixLayoutAttr::Order, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected Order attribute size"));
        }
        Order::from_raw(lt_sys::cublasLtOrder_t::from_ne_bytes(buf))
    }

    /// Queries the row count (`CUBLASLT_MATRIX_LAYOUT_ROWS`).
    pub fn rows(&self) -> Result<u64> {
        let mut buf = [0u8; size_of::<u64>()];
        let n = self.get_attribute_bytes(MatrixLayoutAttr::Rows, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected Rows attribute size"));
        }
        Ok(u64::from_ne_bytes(buf))
    }

    /// Queries the column count (`CUBLASLT_MATRIX_LAYOUT_COLS`).
    pub fn cols(&self) -> Result<u64> {
        let mut buf = [0u8; size_of::<u64>()];
        let n = self.get_attribute_bytes(MatrixLayoutAttr::Cols, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected Cols attribute size"));
        }
        Ok(u64::from_ne_bytes(buf))
    }

    /// Queries the leading dimension (`CUBLASLT_MATRIX_LAYOUT_LD`).
    pub fn ld(&self) -> Result<i64> {
        let mut buf = [0u8; size_of::<i64>()];
        let n = self.get_attribute_bytes(MatrixLayoutAttr::Ld, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected Ld attribute size"));
        }
        Ok(i64::from_ne_bytes(buf))
    }

    /// Sets batched count and strided batch offset in elements.
    pub fn set_batch(&mut self, batch_count: i32, strided_offset: i64) -> Result<()> {
        if batch_count <= 0 || strided_offset < 0 {
            return Err(Error::Invalid(
                "batch_count must be positive and strided_offset non-negative",
            ));
        }
        self.set_attr(MatrixLayoutAttr::BatchCount, &batch_count.to_ne_bytes())?;
        self.set_attr(MatrixLayoutAttr::StridedBatchOffset, &strided_offset.to_ne_bytes())?;
        self.batch_count = batch_count;
        self.strided_offset = strided_offset;
        Ok(())
    }

    /// Queries the batch count (`CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT`).
    pub fn batch_count(&self) -> Result<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.get_attribute_bytes(MatrixLayoutAttr::BatchCount, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected BatchCount attribute size"));
        }
        Ok(i32::from_ne_bytes(buf))
    }

    /// Queries the strided batch offset (`CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET`).
    pub fn strided_batch_offset(&self) -> Result<i64> {
        let mut buf = [0u8; size_of::<i64>()];
        let n = self.get_attribute_bytes(MatrixLayoutAttr::StridedBatchOffset, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid(
                "unexpected StridedBatchOffset attribute size",
            ));
        }
        Ok(i64::from_ne_bytes(buf))
    }

    /// Sets the planar complex plane offset in elements (`CUBLASLT_MATRIX_LAYOUT_PLANE_OFFSET`).
    pub fn set_plane_offset(&mut self, offset: i64) -> Result<()> {
        self.set_attr(MatrixLayoutAttr::PlaneOffset, &offset.to_ne_bytes())
    }

    /// Sets the batching mode (`CUBLASLT_MATRIX_LAYOUT_BATCH_MODE`).
    ///
    /// Any mode other than strided makes cuBLASLt read the bound buffer as a device array of
    /// matrix pointers, so the typed `f32` entry points reject such layouts.
    pub fn set_batch_mode(&mut self, mode: BatchMode) -> Result<()> {
        let v = mode.raw();
        self.set_attr(MatrixLayoutAttr::BatchMode, &v.to_ne_bytes())?;
        self.dense = !self.grouped && v == BatchMode::Strided.raw();
        Ok(())
    }

    /// Rejects an output layout whose batch matrices overlap (`batch_count > 1` with a
    /// `strided_offset` that makes concurrent batches write the same element).
    pub(crate) fn check_output_batch(&self) -> Result<()> {
        let batch = u64::try_from(self.batch_count)
            .map_err(|_| Error::Invalid("negative matrix layout batch count"))?;
        let ld =
            u64::try_from(self.ld).map_err(|_| Error::Invalid("negative leading dimension"))?;
        let stride = u64::try_from(self.strided_offset)
            .map_err(|_| Error::Invalid("negative strided batch offset"))?;
        let disjoint = match self.order {
            Order::Col => batches_disjoint(self.rows, self.cols, ld, stride, batch),
            Order::Row => batches_disjoint(self.cols, self.rows, ld, stride, batch),
            // Tiled orders: `ld` is the stride between 32-column groups (`cublasLtOrder_t`).
            Order::Col32 | Order::Col44r28c | Order::Col322r4r4 => {
                batch <= 1
                    || self
                        .cols
                        .div_ceil(32)
                        .checked_mul(ld)
                        .is_some_and(|span| stride >= span)
            }
        };
        if !disjoint {
            return Err(Error::Invalid(
                "output layout strided_offset makes batch matrices overlap",
            ));
        }
        Ok(())
    }

    /// Minimum required element count for buffers bound to this layout.
    ///
    /// Fails for grouped and pointer-array layouts: their buffers are arrays of device pointers,
    /// not dense matrices.
    pub fn min_elems(&self) -> Result<usize> {
        if !self.dense {
            return Err(Error::Invalid(
                "grouped or pointer-array matrix layout has no dense element extent",
            ));
        }
        let ld =
            u64::try_from(self.ld).map_err(|_| Error::Invalid("negative leading dimension"))?;
        let matrix_span = match self.order {
            Order::Row => self.rows.checked_mul(ld),
            Order::Col | Order::Col32 | Order::Col44r28c | Order::Col322r4r4 => {
                self.cols.checked_mul(ld)
            }
        }
        .ok_or(Error::Invalid("matrix layout element count overflow"))?;
        let batch_span = ((self.batch_count - 1) as u64)
            .checked_mul(self.strided_offset as u64)
            .ok_or(Error::Invalid("matrix layout batch offset overflow"))?;
        let total = matrix_span
            .checked_add(batch_span)
            .ok_or(Error::Invalid("matrix layout total extent overflow"))?;
        usize::try_from(total).map_err(|_| Error::Invalid("matrix layout extent exceeds usize"))
    }
}

impl Drop for MatrixLayout {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cublasLtMatrixLayoutCreate` and is destroyed once.
            let code = unsafe { lt_sys::cublasLtMatrixLayoutDestroy(self.raw) };
            teardown_lt(&self.cuda, code);
            self.raw = std::ptr::null_mut();
        }
    }
}

/// Split-K reduction scheme (`cublasLtReductionScheme_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReductionScheme {
    /// No reduction (`CUBLASLT_REDUCTION_SCHEME_NONE`).
    None,
    /// In-place reduction (`CUBLASLT_REDUCTION_SCHEME_INPLACE`).
    Inplace,
    /// Compute-type workspace reduction (`CUBLASLT_REDUCTION_SCHEME_COMPUTE_TYPE`).
    ComputeType,
    /// Output-type workspace reduction (`CUBLASLT_REDUCTION_SCHEME_OUTPUT_TYPE`).
    OutputType,
    /// All schemes mask (`CUBLASLT_REDUCTION_SCHEME_MASK`).
    Mask,
    /// Raw `cublasLtReductionScheme_t` value.
    Raw(lt_sys::cublasLtReductionScheme_t),
}

impl ReductionScheme {
    /// Converts to `cublaslt_sys::cublasLtReductionScheme_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtReductionScheme_t {
        match self {
            Self::None => lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_NONE,
            Self::Inplace => lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_INPLACE,
            Self::ComputeType => {
                lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_COMPUTE_TYPE
            }
            Self::OutputType => {
                lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_OUTPUT_TYPE
            }
            Self::Mask => lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_MASK,
            Self::Raw(v) => v,
        }
    }

    /// Converts from `cublaslt_sys::cublasLtReductionScheme_t`.
    #[inline]
    pub const fn from_raw(raw: lt_sys::cublasLtReductionScheme_t) -> Self {
        match raw {
            lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_NONE => Self::None,
            lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_INPLACE => Self::Inplace,
            lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_COMPUTE_TYPE => {
                Self::ComputeType
            }
            lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_OUTPUT_TYPE => {
                Self::OutputType
            }
            lt_sys::cublasLtReductionScheme_t_CUBLASLT_REDUCTION_SCHEME_MASK => Self::Mask,
            other => Self::Raw(other),
        }
    }
}

/// Heuristic search mode (`cublasLtMatmulSearch_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SearchMode {
    /// Best fit (`CUBLASLT_SEARCH_BEST_FIT`).
    BestFit,
    /// Limited by algorithm ID (`CUBLASLT_SEARCH_LIMITED_BY_ALGO_ID`).
    LimitedByAlgoId,
    /// Raw `cublasLtMatmulSearch_t` value.
    Raw(lt_sys::cublasLtMatmulSearch_t),
}

impl SearchMode {
    /// Converts to `cublaslt_sys::cublasLtMatmulSearch_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtMatmulSearch_t {
        match self {
            Self::BestFit => lt_sys::cublasLtMatmulSearch_t_CUBLASLT_SEARCH_BEST_FIT,
            Self::LimitedByAlgoId => {
                lt_sys::cublasLtMatmulSearch_t_CUBLASLT_SEARCH_LIMITED_BY_ALGO_ID
            }
            Self::Raw(v) => v,
        }
    }
}

/// Attribute selector for [`Preference`] (`cublasLtMatmulPreferenceAttributes_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PreferenceAttr {
    /// Search mode (`CUBLASLT_MATMUL_PREF_SEARCH_MODE`).
    SearchMode,
    /// Max workspace bytes (`CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES`).
    MaxWorkspaceBytes,
    /// Reduction scheme mask (`CUBLASLT_MATMUL_PREF_REDUCTION_SCHEME_MASK`).
    ReductionSchemeMask,
    /// Min alignment for A (`CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_A_BYTES`).
    MinAlignmentABytes,
    /// Min alignment for B (`CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_B_BYTES`).
    MinAlignmentBBytes,
    /// Min alignment for C (`CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_C_BYTES`).
    MinAlignmentCBytes,
    /// Min alignment for D (`CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_D_BYTES`).
    MinAlignmentDBytes,
    /// Max waves count (`CUBLASLT_MATMUL_PREF_MAX_WAVES_COUNT`).
    MaxWavesCount,
    /// Numerical implementation mask (`CUBLASLT_MATMUL_PREF_IMPL_MASK`).
    ImplMask,
    /// Raw `cublasLtMatmulPreferenceAttributes_t` value.
    Raw(lt_sys::cublasLtMatmulPreferenceAttributes_t),
}

impl PreferenceAttr {
    /// Converts to `cublaslt_sys::cublasLtMatmulPreferenceAttributes_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtMatmulPreferenceAttributes_t {
        match self {
            Self::SearchMode => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_SEARCH_MODE
            }
            Self::MaxWorkspaceBytes => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES
            }
            Self::ReductionSchemeMask => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_REDUCTION_SCHEME_MASK
            }
            Self::MinAlignmentABytes => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_A_BYTES
            }
            Self::MinAlignmentBBytes => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_B_BYTES
            }
            Self::MinAlignmentCBytes => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_C_BYTES
            }
            Self::MinAlignmentDBytes => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_MIN_ALIGNMENT_D_BYTES
            }
            Self::MaxWavesCount => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_MAX_WAVES_COUNT
            }
            Self::ImplMask => {
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_IMPL_MASK
            }
            Self::Raw(v) => v,
        }
    }
}

/// Heuristic search preference descriptor (`cublasLtMatmulPreference_t`).
#[must_use]
pub struct Preference {
    raw: lt_sys::cublasLtMatmulPreference_t,
    cuda: Arc<Context>,
}

// SAFETY: preference descriptor owns no thread-local state (`!Sync`).
unsafe impl Send for Preference {}

impl Preference {
    /// Creates a new matmul preference descriptor.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check_lt("cublasLtMatmulPreferenceCreate", unsafe {
            lt_sys::cublasLtMatmulPreferenceCreate(&mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
        })
    }

    /// Writes raw attribute bytes (`cublasLtMatmulPreferenceSetAttribute`).
    pub fn set_attribute_bytes(&mut self, attr: PreferenceAttr, buf: &[u8]) -> Result<()> {
        // SAFETY: `self.raw` is live and `buf` is valid for `buf.len()` bytes.
        check_lt("cublasLtMatmulPreferenceSetAttribute", unsafe {
            lt_sys::cublasLtMatmulPreferenceSetAttribute(
                self.raw,
                attr.raw(),
                buf.as_ptr().cast(),
                buf.len(),
            )
        })
    }

    /// Reads raw attribute bytes (`cublasLtMatmulPreferenceGetAttribute`), returning bytes written.
    pub fn get_attribute_bytes(&self, attr: PreferenceAttr, buf: &mut [u8]) -> Result<usize> {
        let mut written = 0usize;
        // SAFETY: `self.raw` is live, `buf` is writable for `buf.len()` bytes, `written` is valid.
        check_lt("cublasLtMatmulPreferenceGetAttribute", unsafe {
            lt_sys::cublasLtMatmulPreferenceGetAttribute(
                self.raw,
                attr.raw(),
                buf.as_mut_ptr().cast(),
                buf.len(),
                &mut written,
            )
        })?;
        Ok(written)
    }

    /// Sets the maximum workspace size in bytes (`CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES`).
    pub fn set_max_workspace_bytes(&mut self, bytes: u64) -> Result<()> {
        self.set_attribute_bytes(PreferenceAttr::MaxWorkspaceBytes, &bytes.to_ne_bytes())
    }

    /// Queries the maximum workspace size in bytes (`CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES`).
    pub fn max_workspace_bytes(&self) -> Result<u64> {
        let mut buf = [0u8; size_of::<u64>()];
        let n = self.get_attribute_bytes(PreferenceAttr::MaxWorkspaceBytes, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid(
                "unexpected MaxWorkspaceBytes attribute size",
            ));
        }
        Ok(u64::from_ne_bytes(buf))
    }

    /// Sets the heuristic search mode (`CUBLASLT_MATMUL_PREF_SEARCH_MODE`).
    pub fn set_search_mode(&mut self, mode: SearchMode) -> Result<()> {
        let v = mode.raw();
        self.set_attribute_bytes(PreferenceAttr::SearchMode, &v.to_ne_bytes())
    }

    /// Sets the allowed reduction scheme bitmask (`CUBLASLT_MATMUL_PREF_REDUCTION_SCHEME_MASK`).
    pub fn set_reduction_scheme_mask(&mut self, mask: u32) -> Result<()> {
        self.set_attribute_bytes(PreferenceAttr::ReductionSchemeMask, &mask.to_ne_bytes())
    }

    /// Sets minimum alignment requirements in bytes for A, B, C, and D.
    pub fn set_min_alignment_bytes(&mut self, a: u32, b: u32, c: u32, d: u32) -> Result<()> {
        self.set_attribute_bytes(PreferenceAttr::MinAlignmentABytes, &a.to_ne_bytes())?;
        self.set_attribute_bytes(PreferenceAttr::MinAlignmentBBytes, &b.to_ne_bytes())?;
        self.set_attribute_bytes(PreferenceAttr::MinAlignmentCBytes, &c.to_ne_bytes())?;
        self.set_attribute_bytes(PreferenceAttr::MinAlignmentDBytes, &d.to_ne_bytes())
    }

    /// Sets the maximum GPU waves count (`CUBLASLT_MATMUL_PREF_MAX_WAVES_COUNT`).
    pub fn set_max_waves_count(&mut self, max_waves: f32) -> Result<()> {
        self.set_attribute_bytes(PreferenceAttr::MaxWavesCount, &max_waves.to_ne_bytes())
    }

    /// Sets the numerical implementation mask (`CUBLASLT_MATMUL_PREF_IMPL_MASK`).
    pub fn set_impl_mask(&mut self, mask: u64) -> Result<()> {
        self.set_attribute_bytes(PreferenceAttr::ImplMask, &mask.to_ne_bytes())
    }
}

impl Drop for Preference {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cublasLtMatmulPreferenceCreate` and is destroyed once.
            let code = unsafe { lt_sys::cublasLtMatmulPreferenceDestroy(self.raw) };
            teardown_lt(&self.cuda, code);
            self.raw = std::ptr::null_mut();
        }
    }
}

/// Attribute selector for [`MatrixTransformDesc`] (`cublasLtMatrixTransformDescAttributes_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MatrixTransformDescAttr {
    /// Scale data type (`CUBLASLT_MATRIX_TRANSFORM_DESC_SCALE_TYPE`).
    ScaleType,
    /// Pointer mode (`CUBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE`).
    PointerMode,
    /// Matrix A op (`CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSA`).
    TransA,
    /// Matrix B op (`CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSB`).
    TransB,
    /// Raw `cublasLtMatrixTransformDescAttributes_t` value.
    Raw(lt_sys::cublasLtMatrixTransformDescAttributes_t),
}

impl MatrixTransformDescAttr {
    /// Converts to `cublaslt_sys::cublasLtMatrixTransformDescAttributes_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtMatrixTransformDescAttributes_t {
        match self {
            Self::ScaleType => {
                lt_sys::cublasLtMatrixTransformDescAttributes_t_CUBLASLT_MATRIX_TRANSFORM_DESC_SCALE_TYPE
            }
            Self::PointerMode => {
                lt_sys::cublasLtMatrixTransformDescAttributes_t_CUBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE
            }
            Self::TransA => {
                lt_sys::cublasLtMatrixTransformDescAttributes_t_CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSA
            }
            Self::TransB => {
                lt_sys::cublasLtMatrixTransformDescAttributes_t_CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSB
            }
            Self::Raw(v) => v,
        }
    }
}

/// Matrix transform operation descriptor (`cublasLtMatrixTransformDesc_t`).
#[must_use]
pub struct MatrixTransformDesc {
    raw: lt_sys::cublasLtMatrixTransformDesc_t,
    cuda: Arc<Context>,
    scale: DataType,
    pointer_mode: PointerMode,
}

// SAFETY: transform descriptor owns no thread-local state (`!Sync`).
unsafe impl Send for MatrixTransformDesc {}

impl MatrixTransformDesc {
    /// Creates a new matrix transform descriptor with `scale` scalar data type
    /// (`cublasLtMatrixTransformDescCreate`).
    pub fn new(cuda: &Arc<Context>, scale: DataType) -> Result<Self> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check_lt("cublasLtMatrixTransformDescCreate", unsafe {
            lt_sys::cublasLtMatrixTransformDescCreate(&mut raw, scale.raw_lt())
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            scale,
            pointer_mode: PointerMode::Host,
        })
    }

    /// Writes raw attribute bytes (`cublasLtMatrixTransformDescSetAttribute`).
    ///
    /// # Safety
    ///
    /// Do not change `PointerMode` or `ScaleType` here: [`LtHandle::matrix_transform_f32`]
    /// validates its host `f32` scalars against the values cached by [`Self::new`] and
    /// [`Self::set_pointer_mode`]. Use the typed setters instead.
    pub unsafe fn set_attribute_bytes(
        &mut self,
        attr: MatrixTransformDescAttr,
        buf: &[u8],
    ) -> Result<()> {
        self.set_attr(attr, buf)
    }

    /// Writes an attribute whose value the typed setters have already validated.
    fn set_attr(&mut self, attr: MatrixTransformDescAttr, buf: &[u8]) -> Result<()> {
        // SAFETY: `self.raw` is live and `buf` is valid for `buf.len()` bytes.
        check_lt("cublasLtMatrixTransformDescSetAttribute", unsafe {
            lt_sys::cublasLtMatrixTransformDescSetAttribute(
                self.raw,
                attr.raw(),
                buf.as_ptr().cast(),
                buf.len(),
            )
        })
    }

    /// Reads raw attribute bytes (`cublasLtMatrixTransformDescGetAttribute`), returning bytes written.
    pub fn get_attribute_bytes(
        &self,
        attr: MatrixTransformDescAttr,
        buf: &mut [u8],
    ) -> Result<usize> {
        let mut written = 0usize;
        // SAFETY: `self.raw` is live, `buf` is writable for `buf.len()` bytes, `written` is valid.
        check_lt("cublasLtMatrixTransformDescGetAttribute", unsafe {
            lt_sys::cublasLtMatrixTransformDescGetAttribute(
                self.raw,
                attr.raw(),
                buf.as_mut_ptr().cast(),
                buf.len(),
                &mut written,
            )
        })?;
        Ok(written)
    }

    /// Sets the scalar pointer mode (`CUBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE`).
    pub fn set_pointer_mode(&mut self, mode: PointerMode) -> Result<()> {
        let v = mode.raw_lt();
        self.set_attr(MatrixTransformDescAttr::PointerMode, &v.to_ne_bytes())?;
        self.pointer_mode = mode;
        Ok(())
    }

    /// Queries the scalar pointer mode (`CUBLASLT_MATRIX_TRANSFORM_DESC_POINTER_MODE`).
    pub fn pointer_mode(&self) -> Result<PointerMode> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.get_attribute_bytes(MatrixTransformDescAttr::PointerMode, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected PointerMode attribute size"));
        }
        pointer_mode_from_lt(i32::from_ne_bytes(buf))
    }

    /// Sets the transpose operation for matrix A (`CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSA`).
    pub fn set_trans_a(&mut self, op: Operation) -> Result<()> {
        let v = op.raw_lt();
        self.set_attr(MatrixTransformDescAttr::TransA, &v.to_ne_bytes())
    }

    /// Queries the transpose operation for matrix A (`CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSA`).
    pub fn trans_a(&self) -> Result<Operation> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.get_attribute_bytes(MatrixTransformDescAttr::TransA, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected TransA attribute size"));
        }
        op_from_lt(i32::from_ne_bytes(buf))
    }

    /// Sets the transpose operation for matrix B (`CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSB`).
    pub fn set_trans_b(&mut self, op: Operation) -> Result<()> {
        let v = op.raw_lt();
        self.set_attr(MatrixTransformDescAttr::TransB, &v.to_ne_bytes())
    }

    /// Queries the transpose operation for matrix B (`CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSB`).
    pub fn trans_b(&self) -> Result<Operation> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.get_attribute_bytes(MatrixTransformDescAttr::TransB, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected TransB attribute size"));
        }
        op_from_lt(i32::from_ne_bytes(buf))
    }
}

impl Drop for MatrixTransformDesc {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cublasLtMatrixTransformDescCreate` and is destroyed once.
            let code = unsafe { lt_sys::cublasLtMatrixTransformDescDestroy(self.raw) };
            teardown_lt(&self.cuda, code);
            self.raw = std::ptr::null_mut();
        }
    }
}

/// Attribute selector for [`EmulationDesc`] (`cublasLtEmulationDescAttributes_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EmulationDescAttr {
    /// Emulation strategy (`CUBLASLT_EMULATION_DESC_STRATEGY`).
    Strategy,
    /// Special values support (`CUBLASLT_EMULATION_DESC_SPECIAL_VALUES_SUPPORT`).
    SpecialValuesSupport,
    /// Fixed-point mantissa control (`CUBLASLT_EMULATION_DESC_FIXEDPOINT_MANTISSA_CONTROL`).
    FixedpointMantissaControl,
    /// Fixed-point max mantissa bit count (`CUBLASLT_EMULATION_DESC_FIXEDPOINT_MAX_MANTISSA_BIT_COUNT`).
    FixedpointMaxMantissaBitCount,
    /// Fixed-point mantissa bit offset (`CUBLASLT_EMULATION_DESC_FIXEDPOINT_MANTISSA_BIT_OFFSET`).
    FixedpointMantissaBitOffset,
    /// Fixed-point mantissa bit count pointer (`CUBLASLT_EMULATION_DESC_FIXEDPOINT_MANTISSA_BIT_COUNT_POINTER`).
    FixedpointMantissaBitCountPointer,
    /// Raw `cublasLtEmulationDescAttributes_t` value.
    Raw(lt_sys::cublasLtEmulationDescAttributes_t),
}

impl EmulationDescAttr {
    /// Converts to `cublaslt_sys::cublasLtEmulationDescAttributes_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtEmulationDescAttributes_t {
        match self {
            Self::Strategy => {
                lt_sys::cublasLtEmulationDescAttributes_t_CUBLASLT_EMULATION_DESC_STRATEGY
            }
            Self::SpecialValuesSupport => {
                lt_sys::cublasLtEmulationDescAttributes_t_CUBLASLT_EMULATION_DESC_SPECIAL_VALUES_SUPPORT
            }
            Self::FixedpointMantissaControl => {
                lt_sys::cublasLtEmulationDescAttributes_t_CUBLASLT_EMULATION_DESC_FIXEDPOINT_MANTISSA_CONTROL
            }
            Self::FixedpointMaxMantissaBitCount => {
                lt_sys::cublasLtEmulationDescAttributes_t_CUBLASLT_EMULATION_DESC_FIXEDPOINT_MAX_MANTISSA_BIT_COUNT
            }
            Self::FixedpointMantissaBitOffset => {
                lt_sys::cublasLtEmulationDescAttributes_t_CUBLASLT_EMULATION_DESC_FIXEDPOINT_MANTISSA_BIT_OFFSET
            }
            Self::FixedpointMantissaBitCountPointer => {
                lt_sys::cublasLtEmulationDescAttributes_t_CUBLASLT_EMULATION_DESC_FIXEDPOINT_MANTISSA_BIT_COUNT_POINTER
            }
            Self::Raw(v) => v,
        }
    }
}

/// Floating-point emulation descriptor (`cublasLtEmulationDesc_t`).
#[must_use]
pub struct EmulationDesc {
    raw: lt_sys::cublasLtEmulationDesc_t,
    cuda: Arc<Context>,
}

// SAFETY: emulation descriptor owns no thread-local state (`!Sync`).
unsafe impl Send for EmulationDesc {}

impl EmulationDesc {
    /// Creates a new emulation descriptor (`cublasLtEmulationDescCreate`).
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check_lt("cublasLtEmulationDescCreate", unsafe {
            lt_sys::cublasLtEmulationDescCreate(&mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
        })
    }

    /// Writes raw attribute bytes (`cublasLtEmulationDescSetAttribute`).
    ///
    /// # Safety
    ///
    /// `FixedpointMantissaBitCountPointer` (or any pointer-valued `Raw` attribute) must be null or
    /// a writable device allocation of the matmul's context, alive until every matmul whose
    /// descriptor carries this emulation descriptor completes: cuBLASLt writes through it.
    pub unsafe fn set_attribute_bytes(
        &mut self,
        attr: EmulationDescAttr,
        buf: &[u8],
    ) -> Result<()> {
        self.set_attr(attr, buf)
    }

    /// Writes an attribute value. Safe for 4-byte values: cuBLASLt rejects a buffer whose size
    /// differs from the attribute's storage (`cublasLt.h`, `*SetAttribute` retval
    /// `CUBLAS_STATUS_INVALID_VALUE`), so a 4-byte write can never set an 8-byte pointer attribute.
    fn set_attr(&mut self, attr: EmulationDescAttr, buf: &[u8]) -> Result<()> {
        // SAFETY: `self.raw` is live and `buf` is valid for `buf.len()` bytes.
        check_lt("cublasLtEmulationDescSetAttribute", unsafe {
            lt_sys::cublasLtEmulationDescSetAttribute(
                self.raw,
                attr.raw(),
                buf.as_ptr().cast(),
                buf.len(),
            )
        })
    }

    /// Reads raw attribute bytes (`cublasLtEmulationDescGetAttribute`), returning bytes written.
    pub fn get_attribute_bytes(&self, attr: EmulationDescAttr, buf: &mut [u8]) -> Result<usize> {
        let mut written = 0usize;
        // SAFETY: `self.raw` is live, `buf` is writable for `buf.len()` bytes, `written` is valid.
        check_lt("cublasLtEmulationDescGetAttribute", unsafe {
            lt_sys::cublasLtEmulationDescGetAttribute(
                self.raw,
                attr.raw(),
                buf.as_mut_ptr().cast(),
                buf.len(),
                &mut written,
            )
        })?;
        Ok(written)
    }

    /// Writes an `i32` emulation attribute.
    pub fn set_attribute_i32(&mut self, attr: EmulationDescAttr, value: i32) -> Result<()> {
        self.set_attr(attr, &value.to_ne_bytes())
    }

    /// Reads an `i32` emulation attribute.
    pub fn attribute_i32(&self, attr: EmulationDescAttr) -> Result<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.get_attribute_bytes(attr, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected emulation i32 attribute size"));
        }
        Ok(i32::from_ne_bytes(buf))
    }

    /// Writes a `u32` emulation attribute.
    pub fn set_attribute_u32(&mut self, attr: EmulationDescAttr, value: u32) -> Result<()> {
        self.set_attr(attr, &value.to_ne_bytes())
    }

    /// Reads a `u32` emulation attribute.
    pub fn attribute_u32(&self, attr: EmulationDescAttr) -> Result<u32> {
        let mut buf = [0u8; size_of::<u32>()];
        let n = self.get_attribute_bytes(attr, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected emulation u32 attribute size"));
        }
        Ok(u32::from_ne_bytes(buf))
    }
}

impl Drop for EmulationDesc {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: `self.raw` was created by `cublasLtEmulationDescCreate` and is destroyed once.
            let code = unsafe { lt_sys::cublasLtEmulationDescDestroy(self.raw) };
            teardown_lt(&self.cuda, code);
            self.raw = std::ptr::null_mut();
        }
    }
}

/// Capability attribute selector for [`MatmulAlgo`] (`cublasLtMatmulAlgoCapAttributes_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AlgoCapAttr {
    /// Split-K support (`CUBLASLT_ALGO_CAP_SPLITK_SUPPORT`).
    SplitkSupport,
    /// Reduction scheme mask (`CUBLASLT_ALGO_CAP_REDUCTION_SCHEME_MASK`).
    ReductionSchemeMask,
    /// CTA swizzling support (`CUBLASLT_ALGO_CAP_CTA_SWIZZLING_SUPPORT`).
    CtaSwizzlingSupport,
    /// Strided batch support (`CUBLASLT_ALGO_CAP_STRIDED_BATCH_SUPPORT`).
    StridedBatchSupport,
    /// Out-of-place result support (`CUBLASLT_ALGO_CAP_OUT_OF_PLACE_RESULT_SUPPORT`).
    OutOfPlaceResultSupport,
    /// Uplo support (`CUBLASLT_ALGO_CAP_UPLO_SUPPORT`).
    UploSupport,
    /// Supported tile IDs (`CUBLASLT_ALGO_CAP_TILE_IDS`).
    TileIds,
    /// Max custom option value (`CUBLASLT_ALGO_CAP_CUSTOM_OPTION_MAX`).
    CustomOptionMax,
    /// Custom memory order (`CUBLASLT_ALGO_CAP_CUSTOM_MEMORY_ORDER`).
    CustomMemoryOrder,
    /// Pointer mode mask (`CUBLASLT_ALGO_CAP_POINTER_MODE_MASK`).
    PointerModeMask,
    /// Epilogue mask (`CUBLASLT_ALGO_CAP_EPILOGUE_MASK`).
    EpilogueMask,
    /// Supported stages IDs (`CUBLASLT_ALGO_CAP_STAGES_IDS`).
    StagesIds,
    /// Negative leading dimension support (`CUBLASLT_ALGO_CAP_LD_NEGATIVE`).
    LdNegative,
    /// Numerical implementation flags (`CUBLASLT_ALGO_CAP_NUMERICAL_IMPL_FLAGS`).
    NumericalImplFlags,
    /// Min alignment for A (`CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_A_BYTES`).
    MinAlignmentABytes,
    /// Min alignment for B (`CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_B_BYTES`).
    MinAlignmentBBytes,
    /// Min alignment for C (`CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_C_BYTES`).
    MinAlignmentCBytes,
    /// Min alignment for D (`CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_D_BYTES`).
    MinAlignmentDBytes,
    /// Pointer-array batch support (`CUBLASLT_ALGO_CAP_POINTER_ARRAY_BATCH_SUPPORT`).
    PointerArrayBatchSupport,
    /// Floating-point emulation support (`CUBLASLT_ALGO_CAP_FLOATING_POINT_EMULATION_SUPPORT`).
    FloatingPointEmulationSupport,
    /// Pointer-array grouped support (`CUBLASLT_ALGO_CAP_POINTER_ARRAY_GROUPED_SUPPORT`).
    PointerArrayGroupedSupport,
    /// Raw `cublasLtMatmulAlgoCapAttributes_t` value.
    Raw(lt_sys::cublasLtMatmulAlgoCapAttributes_t),
}

impl AlgoCapAttr {
    /// Converts to `cublaslt_sys::cublasLtMatmulAlgoCapAttributes_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtMatmulAlgoCapAttributes_t {
        match self {
            Self::SplitkSupport => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_SPLITK_SUPPORT
            }
            Self::ReductionSchemeMask => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_REDUCTION_SCHEME_MASK
            }
            Self::CtaSwizzlingSupport => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_CTA_SWIZZLING_SUPPORT
            }
            Self::StridedBatchSupport => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_STRIDED_BATCH_SUPPORT
            }
            Self::OutOfPlaceResultSupport => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_OUT_OF_PLACE_RESULT_SUPPORT
            }
            Self::UploSupport => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_UPLO_SUPPORT
            }
            Self::TileIds => lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_TILE_IDS,
            Self::CustomOptionMax => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_CUSTOM_OPTION_MAX
            }
            Self::CustomMemoryOrder => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_CUSTOM_MEMORY_ORDER
            }
            Self::PointerModeMask => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_POINTER_MODE_MASK
            }
            Self::EpilogueMask => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_EPILOGUE_MASK
            }
            Self::StagesIds => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_STAGES_IDS
            }
            Self::LdNegative => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_LD_NEGATIVE
            }
            Self::NumericalImplFlags => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_NUMERICAL_IMPL_FLAGS
            }
            Self::MinAlignmentABytes => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_A_BYTES
            }
            Self::MinAlignmentBBytes => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_B_BYTES
            }
            Self::MinAlignmentCBytes => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_C_BYTES
            }
            Self::MinAlignmentDBytes => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_MIN_ALIGNMENT_D_BYTES
            }
            Self::PointerArrayBatchSupport => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_POINTER_ARRAY_BATCH_SUPPORT
            }
            Self::FloatingPointEmulationSupport => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_FLOATING_POINT_EMULATION_SUPPORT
            }
            Self::PointerArrayGroupedSupport => {
                lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_POINTER_ARRAY_GROUPED_SUPPORT
            }
            Self::Raw(v) => v,
        }
    }
}

/// Configuration attribute selector for [`MatmulAlgo`] (`cublasLtMatmulAlgoConfigAttributes_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AlgoConfigAttr {
    /// Algorithm ID (`CUBLASLT_ALGO_CONFIG_ID`).
    Id,
    /// Tile ID (`CUBLASLT_ALGO_CONFIG_TILE_ID`).
    TileId,
    /// Split-K factor (`CUBLASLT_ALGO_CONFIG_SPLITK_NUM`).
    SplitkNum,
    /// Reduction scheme (`CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME`).
    ReductionScheme,
    /// CTA swizzling (`CUBLASLT_ALGO_CONFIG_CTA_SWIZZLING`).
    CtaSwizzling,
    /// Custom option (`CUBLASLT_ALGO_CONFIG_CUSTOM_OPTION`).
    CustomOption,
    /// Stages ID (`CUBLASLT_ALGO_CONFIG_STAGES_ID`).
    StagesId,
    /// Inner shape ID (`CUBLASLT_ALGO_CONFIG_INNER_SHAPE_ID`).
    InnerShapeId,
    /// Cluster shape ID (`CUBLASLT_ALGO_CONFIG_CLUSTER_SHAPE_ID`).
    ClusterShapeId,
    /// Raw `cublasLtMatmulAlgoConfigAttributes_t` value.
    Raw(lt_sys::cublasLtMatmulAlgoConfigAttributes_t),
}

impl AlgoConfigAttr {
    /// Converts to `cublaslt_sys::cublasLtMatmulAlgoConfigAttributes_t`.
    #[inline]
    pub const fn raw(self) -> lt_sys::cublasLtMatmulAlgoConfigAttributes_t {
        match self {
            Self::Id => lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_ID,
            Self::TileId => {
                lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_TILE_ID
            }
            Self::SplitkNum => {
                lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_SPLITK_NUM
            }
            Self::ReductionScheme => {
                lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME
            }
            Self::CtaSwizzling => {
                lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_CTA_SWIZZLING
            }
            Self::CustomOption => {
                lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_CUSTOM_OPTION
            }
            Self::StagesId => {
                lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_STAGES_ID
            }
            Self::InnerShapeId => {
                lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_INNER_SHAPE_ID
            }
            Self::ClusterShapeId => {
                lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_CLUSTER_SHAPE_ID
            }
            Self::Raw(v) => v,
        }
    }
}

/// Opaque matmul algorithm descriptor (`cublasLtMatmulAlgo_t`).
#[must_use]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct MatmulAlgo {
    raw: lt_sys::cublasLtMatmulAlgo_t,
}

impl MatmulAlgo {
    /// Returns a zeroed `MatmulAlgo`.
    #[inline]
    pub fn zeroed() -> Self {
        Self {
            raw: lt_sys::cublasLtMatmulAlgo_t::default(),
        }
    }

    /// Returns a reference to the underlying `cublasLtMatmulAlgo_t`.
    #[inline]
    pub const fn raw(&self) -> &lt_sys::cublasLtMatmulAlgo_t {
        &self.raw
    }

    /// Reads raw capability bytes (`cublasLtMatmulAlgoCapGetAttribute`), returning bytes written.
    pub fn cap_attribute_bytes(&self, attr: AlgoCapAttr, buf: &mut [u8]) -> Result<usize> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut written = 0usize;
        // SAFETY: `self.raw` is a valid POD struct, `buf` is writable for `buf.len()` bytes.
        check_lt("cublasLtMatmulAlgoCapGetAttribute", unsafe {
            lt_sys::cublasLtMatmulAlgoCapGetAttribute(
                &self.raw,
                attr.raw(),
                buf.as_mut_ptr().cast(),
                buf.len(),
                &mut written,
            )
        })?;
        Ok(written)
    }

    /// Queries an `i32` capability attribute.
    pub fn cap_i32(&self, attr: AlgoCapAttr) -> Result<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.cap_attribute_bytes(attr, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected algo capability i32 size"));
        }
        Ok(i32::from_ne_bytes(buf))
    }

    /// Queries a `u32` capability attribute.
    pub fn cap_u32(&self, attr: AlgoCapAttr) -> Result<u32> {
        let mut buf = [0u8; size_of::<u32>()];
        let n = self.cap_attribute_bytes(attr, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected algo capability u32 size"));
        }
        Ok(u32::from_ne_bytes(buf))
    }

    /// Queries a `u64` capability attribute.
    pub fn cap_u64(&self, attr: AlgoCapAttr) -> Result<u64> {
        let mut buf = [0u8; size_of::<u64>()];
        let n = self.cap_attribute_bytes(attr, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected algo capability u64 size"));
        }
        Ok(u64::from_ne_bytes(buf))
    }

    /// Queries an `i32` array capability attribute (such as `TileIds` or `StagesIds`), returning
    /// the number of elements written into `out`.
    pub fn cap_i32_slice(&self, attr: AlgoCapAttr, out: &mut [i32]) -> Result<usize> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut written = 0usize;
        let byte_len = std::mem::size_of_val(out);
        // SAFETY: `out` is a contiguous writable slice of `i32` of `byte_len` bytes.
        check_lt("cublasLtMatmulAlgoCapGetAttribute", unsafe {
            lt_sys::cublasLtMatmulAlgoCapGetAttribute(
                &self.raw,
                attr.raw(),
                out.as_mut_ptr().cast(),
                byte_len,
                &mut written,
            )
        })?;
        Ok((written / size_of::<i32>()).min(out.len()))
    }

    /// Writes raw configuration bytes (`cublasLtMatmulAlgoConfigSetAttribute`).
    pub fn set_config_bytes(&mut self, attr: AlgoConfigAttr, buf: &[u8]) -> Result<()> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        // SAFETY: `self.raw` is a valid POD struct and `buf` is readable for `buf.len()` bytes.
        check_lt("cublasLtMatmulAlgoConfigSetAttribute", unsafe {
            lt_sys::cublasLtMatmulAlgoConfigSetAttribute(
                &mut self.raw,
                attr.raw(),
                buf.as_ptr().cast(),
                buf.len(),
            )
        })
    }

    /// Reads raw configuration bytes (`cublasLtMatmulAlgoConfigGetAttribute`), returning bytes written.
    pub fn config_bytes(&self, attr: AlgoConfigAttr, buf: &mut [u8]) -> Result<usize> {
        lt_sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut written = 0usize;
        // SAFETY: `self.raw` is a valid POD struct and `buf` is writable for `buf.len()` bytes.
        check_lt("cublasLtMatmulAlgoConfigGetAttribute", unsafe {
            lt_sys::cublasLtMatmulAlgoConfigGetAttribute(
                &self.raw,
                attr.raw(),
                buf.as_mut_ptr().cast(),
                buf.len(),
                &mut written,
            )
        })?;
        Ok(written)
    }

    /// Queries the algorithm ID (`CUBLASLT_ALGO_CONFIG_ID`).
    pub fn id(&self) -> Result<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.config_bytes(AlgoConfigAttr::Id, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected algo config ID size"));
        }
        Ok(i32::from_ne_bytes(buf))
    }

    /// Sets the tile ID (`CUBLASLT_ALGO_CONFIG_TILE_ID`).
    pub fn set_tile_id(&mut self, tile_id: u32) -> Result<()> {
        self.set_config_bytes(AlgoConfigAttr::TileId, &tile_id.to_ne_bytes())
    }

    /// Queries the tile ID (`CUBLASLT_ALGO_CONFIG_TILE_ID`).
    pub fn tile_id(&self) -> Result<u32> {
        let mut buf = [0u8; size_of::<u32>()];
        let n = self.config_bytes(AlgoConfigAttr::TileId, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected algo config TileId size"));
        }
        Ok(u32::from_ne_bytes(buf))
    }

    /// Sets the split-K factor (`CUBLASLT_ALGO_CONFIG_SPLITK_NUM`).
    pub fn set_splitk_num(&mut self, splitk: i32) -> Result<()> {
        self.set_config_bytes(AlgoConfigAttr::SplitkNum, &splitk.to_ne_bytes())
    }

    /// Queries the split-K factor (`CUBLASLT_ALGO_CONFIG_SPLITK_NUM`).
    pub fn splitk_num(&self) -> Result<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        let n = self.config_bytes(AlgoConfigAttr::SplitkNum, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid("unexpected algo config SplitkNum size"));
        }
        Ok(i32::from_ne_bytes(buf))
    }

    /// Sets the reduction scheme (`CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME`).
    pub fn set_reduction_scheme(&mut self, scheme: ReductionScheme) -> Result<()> {
        let v = scheme.raw() as u32;
        self.set_config_bytes(AlgoConfigAttr::ReductionScheme, &v.to_ne_bytes())
    }

    /// Queries the reduction scheme (`CUBLASLT_ALGO_CONFIG_REDUCTION_SCHEME`).
    pub fn reduction_scheme(&self) -> Result<ReductionScheme> {
        let mut buf = [0u8; size_of::<u32>()];
        let n = self.config_bytes(AlgoConfigAttr::ReductionScheme, &mut buf)?;
        if n != buf.len() {
            return Err(Error::Invalid(
                "unexpected algo config ReductionScheme size",
            ));
        }
        Ok(ReductionScheme::from_raw(
            u32::from_ne_bytes(buf) as lt_sys::cublasLtReductionScheme_t
        ))
    }

    /// Sets CTA swizzling (`CUBLASLT_ALGO_CONFIG_CTA_SWIZZLING`).
    pub fn set_cta_swizzling(&mut self, swizzle: u32) -> Result<()> {
        self.set_config_bytes(AlgoConfigAttr::CtaSwizzling, &swizzle.to_ne_bytes())
    }

    /// Sets the custom option (`CUBLASLT_ALGO_CONFIG_CUSTOM_OPTION`).
    pub fn set_custom_option(&mut self, option: u32) -> Result<()> {
        self.set_config_bytes(AlgoConfigAttr::CustomOption, &option.to_ne_bytes())
    }

    /// Sets the stages ID (`CUBLASLT_ALGO_CONFIG_STAGES_ID`).
    pub fn set_stages_id(&mut self, stages_id: u32) -> Result<()> {
        self.set_config_bytes(AlgoConfigAttr::StagesId, &stages_id.to_ne_bytes())
    }

    /// Sets the inner shape ID (`CUBLASLT_ALGO_CONFIG_INNER_SHAPE_ID`).
    pub fn set_inner_shape_id(&mut self, shape_id: u16) -> Result<()> {
        self.set_config_bytes(AlgoConfigAttr::InnerShapeId, &shape_id.to_ne_bytes())
    }

    /// Sets the cluster shape ID (`CUBLASLT_ALGO_CONFIG_CLUSTER_SHAPE_ID`).
    pub fn set_cluster_shape_id(&mut self, cluster_id: u16) -> Result<()> {
        self.set_config_bytes(AlgoConfigAttr::ClusterShapeId, &cluster_id.to_ne_bytes())
    }
}

/// Algorithm heuristic result returned by [`LtHandle::matmul_heuristic`].
#[must_use]
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct HeuristicResult {
    raw: lt_sys::cublasLtMatmulHeuristicResult_t,
}

impl HeuristicResult {
    fn zeroed() -> Self {
        // SAFETY: `cublasLtMatmulHeuristicResult_t` is plain C POD data; zeroing is valid.
        let raw = unsafe { std::mem::zeroed() };
        Self { raw }
    }

    /// Constructs a `HeuristicResult` from an explicitly initialized [`MatmulAlgo`] and
    /// `workspace_size` in bytes.
    ///
    /// Prefer [`LtHandle::matmul_algo_check`], which returns the checked algo with its required
    /// workspace size. An undersized `workspace_size` here is not memory-unsafe: `cublasLtMatmul`
    /// rejects it with `CUBLAS_STATUS_INVALID_VALUE` (`cublasLt.h`, `cublasLtMatmul` retvals).
    pub fn from_algo(algo: MatmulAlgo, workspace_size: usize) -> Self {
        let mut res = Self::zeroed();
        res.raw.algo = algo.raw;
        res.raw.workspaceSize = workspace_size;
        res.raw.state = lt_sys::cublasStatus_t_CUBLAS_STATUS_SUCCESS;
        res
    }

    /// Returns the underlying [`MatmulAlgo`].
    #[inline]
    pub const fn algo(&self) -> MatmulAlgo {
        MatmulAlgo { raw: self.raw.algo }
    }

    /// Returns the raw [`Status`] reported for this heuristic entry.
    #[inline]
    pub const fn status(&self) -> Status {
        Status(self.raw.state as crate::sys::cublasStatus_t)
    }

    /// Returns `true` when this heuristic entry succeeded (`CUBLAS_STATUS_SUCCESS`).
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.raw.state == lt_sys::cublasStatus_t_CUBLAS_STATUS_SUCCESS
    }

    /// Required workspace size in bytes for this algorithm.
    #[inline]
    pub fn workspace_size(&self) -> usize {
        self.raw.workspaceSize
    }

    /// Estimated GPU wave occupancy reported by cuBLASLt.
    #[inline]
    pub fn waves_count(&self) -> f32 {
        self.raw.wavesCount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_and_epilogue_roundtrip_raw_values() {
        for order in [
            Order::Col,
            Order::Row,
            Order::Col32,
            Order::Col44r28c,
            Order::Col322r4r4,
        ] {
            assert_eq!(Order::from_raw(order.raw()).unwrap(), order);
        }
        assert!(matches!(Order::from_raw(999), Err(Error::Invalid(_))));

        for ep in [
            Epilogue::Default,
            Epilogue::Relu,
            Epilogue::Bias,
            Epilogue::ReluBias,
            Epilogue::Gelu,
            Epilogue::GeluBias,
            Epilogue::Raw(12345),
        ] {
            assert_eq!(Epilogue::from_raw(ep.raw()), ep);
        }
    }

    #[test]
    fn reduction_scheme_and_search_mode_mappings() {
        for rs in [
            ReductionScheme::None,
            ReductionScheme::Inplace,
            ReductionScheme::ComputeType,
            ReductionScheme::OutputType,
            ReductionScheme::Mask,
            ReductionScheme::Raw(42),
        ] {
            assert_eq!(ReductionScheme::from_raw(rs.raw()), rs);
        }

        assert_eq!(
            SearchMode::BestFit.raw(),
            lt_sys::cublasLtMatmulSearch_t_CUBLASLT_SEARCH_BEST_FIT
        );
        assert_eq!(
            SearchMode::LimitedByAlgoId.raw(),
            lt_sys::cublasLtMatmulSearch_t_CUBLASLT_SEARCH_LIMITED_BY_ALGO_ID
        );
        assert_eq!(SearchMode::Raw(7).raw(), 7);
    }

    #[test]
    fn attribute_and_property_enum_raw_mappings() {
        assert_eq!(
            LibraryProperty::MajorVersion.raw(),
            lt_sys::libraryPropertyType_t_MAJOR_VERSION
        );
        assert_eq!(
            LibraryProperty::MinorVersion.raw(),
            lt_sys::libraryPropertyType_t_MINOR_VERSION
        );
        assert_eq!(
            LibraryProperty::PatchLevel.raw(),
            lt_sys::libraryPropertyType_t_PATCH_LEVEL
        );
        assert_eq!(
            BatchMode::Strided.raw(),
            lt_sys::cublasLtBatchMode_t_CUBLASLT_BATCH_MODE_STRIDED
        );
        assert_eq!(
            BatchMode::PointerArray.raw(),
            lt_sys::cublasLtBatchMode_t_CUBLASLT_BATCH_MODE_POINTER_ARRAY
        );
        assert_eq!(
            BatchMode::Grouped.raw(),
            lt_sys::cublasLtBatchMode_t_CUBLASLT_BATCH_MODE_GROUPED
        );
        assert_eq!(
            MatrixTransformDescAttr::TransA.raw(),
            lt_sys::cublasLtMatrixTransformDescAttributes_t_CUBLASLT_MATRIX_TRANSFORM_DESC_TRANSA
        );
        assert_eq!(
            EmulationDescAttr::Strategy.raw(),
            lt_sys::cublasLtEmulationDescAttributes_t_CUBLASLT_EMULATION_DESC_STRATEGY
        );
        assert_eq!(
            AlgoCapAttr::TileIds.raw(),
            lt_sys::cublasLtMatmulAlgoCapAttributes_t_CUBLASLT_ALGO_CAP_TILE_IDS
        );
        assert_eq!(
            AlgoConfigAttr::SplitkNum.raw(),
            lt_sys::cublasLtMatmulAlgoConfigAttributes_t_CUBLASLT_ALGO_CONFIG_SPLITK_NUM
        );
        assert_eq!(
            PreferenceAttr::MaxWavesCount.raw(),
            lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_MAX_WAVES_COUNT
        );
    }

    #[test]
    fn heuristic_from_algo_sets_valid_status_and_workspace() {
        let algo = MatmulAlgo::zeroed();
        let h = HeuristicResult::from_algo(algo, 4096);
        assert!(h.is_valid());
        assert!(h.status().is_success());
        assert_eq!(h.workspace_size(), 4096);
        assert_eq!(h.algo().raw().data, [0u64; 8]);
    }

    #[test]
    fn logger_open_file_rejects_interior_nul_and_set_file_rejects_null() {
        let err = logger_open_file("bad\0path.log").unwrap_err();
        assert!(matches!(err, Error::Invalid(_)));

        // SAFETY: testing null pointer rejection before any FFI call.
        let err = unsafe { logger_set_file(std::ptr::null_mut()) }.unwrap_err();
        assert!(matches!(err, Error::Invalid(_)));
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cublasLt64_13.dll; run via xtask gpu-test"]
    fn gpu_cublaslt_version_cache_and_transform() {
        assert!(version().expect("cublasLt version") > 0);
        assert!(cudart_version().expect("cudart version") > 0);
        assert!(get_property(LibraryProperty::MajorVersion).expect("major") >= 12);

        let cap = heuristics_cache_capacity().expect("cache cap");
        set_heuristics_cache_capacity(cap).expect("restore cache cap");

        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        let lt = LtHandle::new(&ctx).expect("lt handle");

        let mut tdesc = MatrixTransformDesc::new(&ctx, DataType::F32).expect("transform desc");
        tdesc.set_trans_a(Operation::T).expect("set trans_a");
        assert_eq!(tdesc.trans_a().expect("get trans_a"), Operation::T);
        assert_eq!(
            tdesc.pointer_mode().expect("get pointer_mode"),
            PointerMode::Host
        );

        let a_host = vec![1.0f32, 2.0, 3.0, 4.0];
        let a_dev = cuda_core::DeviceBuffer::from_host(&stream, &a_host).unwrap();
        let mut c_dev = cuda_core::DeviceBuffer::zeroed(&stream, 4).unwrap();
        let a_layout = MatrixLayout::new(&ctx, DataType::F32, 2, 2, 2).unwrap();
        let c_layout = MatrixLayout::new(&ctx, DataType::F32, 2, 2, 2).unwrap();

        lt.matrix_transform_f32(
            &stream, &tdesc, 2.0, &a_dev, &a_layout, 0.0, None, &mut c_dev, &c_layout,
        )
        .unwrap();
        let c_host = c_dev.to_host_vec(&stream).unwrap();
        assert!((c_host[0] - 2.0).abs() < 1e-5);
        assert!((c_host[1] - 6.0).abs() < 1e-5);
        assert!((c_host[2] - 4.0).abs() < 1e-5);
        assert!((c_host[3] - 8.0).abs() < 1e-5);
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cublasLt64_13.dll; run via xtask gpu-test"]
    fn gpu_cublaslt_algo_ids_init_check_and_stream_heuristic() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        let lt = LtHandle::new(&ctx).expect("lt handle");

        let mut ids = Vec::new();
        let n = lt
            .matmul_algo_ids(
                ComputeType::F32,
                DataType::F32,
                DataType::F32,
                DataType::F32,
                DataType::F32,
                DataType::F32,
                16,
                &mut ids,
            )
            .unwrap();
        assert!(n > 0);

        let algo = lt
            .matmul_algo_init(
                ComputeType::F32,
                DataType::F32,
                DataType::F32,
                DataType::F32,
                DataType::F32,
                DataType::F32,
                ids[0],
            )
            .unwrap();
        assert_eq!(algo.id().unwrap(), ids[0]);
        let _ = algo.cap_i32(AlgoCapAttr::SplitkSupport).unwrap();

        let desc = MatmulDesc::new(&ctx, ComputeType::F32, DataType::F32).unwrap();
        let a_layout = MatrixLayout::new(&ctx, DataType::F32, 4, 4, 4).unwrap();
        let b_layout = MatrixLayout::new(&ctx, DataType::F32, 4, 4, 4).unwrap();
        let c_layout = MatrixLayout::new(&ctx, DataType::F32, 4, 4, 4).unwrap();
        let d_layout = MatrixLayout::new(&ctx, DataType::F32, 4, 4, 4).unwrap();
        let pref = Preference::new(&ctx).unwrap();

        let heuristic = lt
            .matmul_heuristic_for_stream(
                &stream, &desc, &a_layout, &b_layout, &c_layout, &d_layout, &pref,
            )
            .unwrap();
        // Check the heuristic's algo: a default-configured `ids[0]` may legitimately be
        // `CUBLAS_STATUS_NOT_SUPPORTED` for this 4x4 problem (observed on sm_86, cuBLASLt 13).
        let supported = heuristic.algo();
        let checked = lt
            .matmul_algo_check(&desc, &a_layout, &b_layout, &c_layout, &d_layout, &supported)
            .unwrap();
        // The check result must carry the checked algo (cuBLASLt leaves `result.algo` untouched).
        assert_eq!(checked.algo().raw().data, supported.raw().data);
        assert_eq!(checked.algo().id().unwrap(), supported.id().unwrap());
    }

    #[test]
    fn ld_validation_follows_documented_order_minimums() {
        // COL / ROW: ld >= rows / cols (CUBLASLT_MATRIX_LAYOUT_LD).
        assert!(validate_ld(Order::Col, 4, 4, 4).is_ok());
        assert!(matches!(
            validate_ld(Order::Col, 4, 4, 1),
            Err(Error::Invalid(_))
        ));
        assert!(validate_ld(Order::Row, 4, 6, 6).is_ok());
        assert!(matches!(
            validate_ld(Order::Row, 4, 6, 4),
            Err(Error::Invalid(_))
        ));
        // Header examples: 2 rows COL32 -> 64; 1 row COL4_4R2_8C -> 256; 1 row COL32_2R_4R4 -> 1024.
        assert!(validate_ld(Order::Col32, 2, 33, 64).is_ok());
        assert!(validate_ld(Order::Col32, 2, 33, 63).is_err());
        assert!(validate_ld(Order::Col44r28c, 1, 33, 256).is_ok());
        assert!(validate_ld(Order::Col44r28c, 1, 33, 255).is_err());
        assert!(validate_ld(Order::Col322r4r4, 1, 33, 1024).is_ok());
        assert!(validate_ld(Order::Col322r4r4, 1, 33, 1023).is_err());
        assert!(validate_ld(Order::Col, 4, 4, -1).is_err());
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cublasLt64_13.dll; run via xtask gpu-test"]
    fn gpu_layout_raw_setters_cannot_desync_typed_checks() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        let lt = LtHandle::new(&ctx).expect("lt handle");

        // LT-06: ld < rows is rejected before reaching cuBLASLt.
        assert!(matches!(
            MatrixLayout::new(&ctx, DataType::F32, 4, 4, 1),
            Err(Error::Invalid(_))
        ));

        // LT-02: a pointer-array layout has no dense extent.
        let mut ptr_layout = MatrixLayout::new(&ctx, DataType::F32, 4, 4, 4).unwrap();
        assert_eq!(ptr_layout.min_elems().unwrap(), 16);
        ptr_layout.set_batch_mode(BatchMode::PointerArray).unwrap();
        assert!(matches!(ptr_layout.min_elems(), Err(Error::Invalid(_))));
        ptr_layout.set_batch_mode(BatchMode::Strided).unwrap();
        assert_eq!(ptr_layout.min_elems().unwrap(), 16);

        // LT-05: a grouped layout has no dense extent either.
        let dims = cuda_core::DeviceBuffer::from_host(&stream, &[4i64]).unwrap();
        let dim_ptr = (dims.cu_deviceptr() as usize) as *const c_void;
        // SAFETY: `dims` holds `group_count = 1` i64 entries and outlives the layout's use here.
        let grouped = unsafe {
            MatrixLayout::new_grouped(&ctx, DataType::F32, 1, dim_ptr, dim_ptr, dim_ptr)
        }
        .expect("grouped layout");
        assert!(matches!(grouped.min_elems(), Err(Error::Invalid(_))));

        // BL-04 (Lt): overlapping output batches are rejected by matmul_f32.
        let a = cuda_core::DeviceBuffer::from_host(&stream, &[1.0f32; 32]).unwrap();
        let b = cuda_core::DeviceBuffer::from_host(&stream, &[1.0f32; 32]).unwrap();
        let c = cuda_core::DeviceBuffer::from_host(&stream, &[0.0f32; 32]).unwrap();
        let mut d = cuda_core::DeviceBuffer::<f32>::zeroed(&stream, 32).unwrap();
        let desc = MatmulDesc::new(&ctx, ComputeType::F32, DataType::F32).unwrap();
        let mut layouts = Vec::new();
        for _ in 0..4 {
            let mut l = MatrixLayout::new(&ctx, DataType::F32, 4, 4, 4).unwrap();
            l.set_batch(2, 16).unwrap();
            layouts.push(l);
        }
        let mut d_overlap = MatrixLayout::new(&ctx, DataType::F32, 4, 4, 4).unwrap();
        d_overlap.set_batch(2, 0).unwrap();
        let mut scratch = Scratch::new(&stream);
        let heuristic = HeuristicResult::from_algo(MatmulAlgo::zeroed(), 0);
        let err = lt.matmul_f32(
            &stream,
            &desc,
            1.0,
            &a,
            &layouts[0],
            &b,
            &layouts[1],
            0.0,
            &c,
            &layouts[2],
            &mut d,
            &d_overlap,
            &heuristic,
            &mut scratch,
            0,
        );
        assert!(matches!(err, Err(Error::Invalid(_))));
    }
}
