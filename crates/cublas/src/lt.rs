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
use crate::{ComputeType, DataType, Error, Operation, PointerMode, Result, Scratch, lt_sys};
use cuda_core::{Context, DeviceBuffer, Stream};
use std::ffi::c_void;
use std::sync::Arc;

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
}

/// Owning cuBLASLt handle (`cublasLtHandle_t`).
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

/// Matmul operation descriptor (`cublasLtMatmulDesc_t`).
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

    /// Sets the transpose operation for matrix A (`CUBLASLT_MATMUL_DESC_TRANSA`).
    pub fn set_trans_a(&mut self, op: Operation) -> Result<()> {
        let v = op.raw_lt();
        // SAFETY: `v` has the documented `int32_t` (`cublasOperation_t`) size.
        check_lt("cublasLtMatmulDescSetAttribute", unsafe {
            lt_sys::cublasLtMatmulDescSetAttribute(
                self.raw,
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_TRANSA,
                (&raw const v).cast(),
                size_of_val(&v),
            )
        })
    }

    /// Sets the transpose operation for matrix B (`CUBLASLT_MATMUL_DESC_TRANSB`).
    pub fn set_trans_b(&mut self, op: Operation) -> Result<()> {
        let v = op.raw_lt();
        // SAFETY: `v` has the documented `int32_t` (`cublasOperation_t`) size.
        check_lt("cublasLtMatmulDescSetAttribute", unsafe {
            lt_sys::cublasLtMatmulDescSetAttribute(
                self.raw,
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_TRANSB,
                (&raw const v).cast(),
                size_of_val(&v),
            )
        })
    }

    /// Sets the scalar pointer mode (`CUBLASLT_MATMUL_DESC_POINTER_MODE`).
    pub fn set_pointer_mode(&mut self, mode: PointerMode) -> Result<()> {
        let v = mode.raw_lt();
        // SAFETY: `v` has the documented `int32_t` (`cublasLtPointerMode_t`) size.
        check_lt("cublasLtMatmulDescSetAttribute", unsafe {
            lt_sys::cublasLtMatmulDescSetAttribute(
                self.raw,
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_POINTER_MODE,
                (&raw const v).cast(),
                size_of_val(&v),
            )
        })?;
        self.pointer_mode = mode;
        Ok(())
    }

    /// Sets the fused epilogue (`CUBLASLT_MATMUL_DESC_EPILOGUE`).
    pub fn set_epilogue(&mut self, epilogue: Epilogue) -> Result<()> {
        let v = epilogue.raw();
        // SAFETY: `v` has the documented `uint32_t` (`cublasLtEpilogue_t`) size.
        check_lt("cublasLtMatmulDescSetAttribute", unsafe {
            lt_sys::cublasLtMatmulDescSetAttribute(
                self.raw,
                lt_sys::cublasLtMatmulDescAttributes_t_CUBLASLT_MATMUL_DESC_EPILOGUE,
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

/// Matrix layout descriptor (`cublasLtMatrixLayout_t`).
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
        })
    }

    /// Sets the matrix memory order (`CUBLASLT_MATRIX_LAYOUT_ORDER`).
    pub fn set_order(&mut self, order: Order) -> Result<()> {
        let v = order.raw();
        // SAFETY: `v` has the documented `int32_t` (`cublasLtOrder_t`) size.
        check_lt("cublasLtMatrixLayoutSetAttribute", unsafe {
            lt_sys::cublasLtMatrixLayoutSetAttribute(
                self.raw,
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_ORDER,
                (&raw const v).cast(),
                size_of_val(&v),
            )
        })?;
        self.order = order;
        Ok(())
    }

    /// Sets batched count and strided batch offset in elements.
    pub fn set_batch(&mut self, batch_count: i32, strided_offset: i64) -> Result<()> {
        if batch_count <= 0 || strided_offset < 0 {
            return Err(Error::Invalid(
                "batch_count must be positive and strided_offset non-negative",
            ));
        }
        // SAFETY: `batch_count` is `int32_t` as documented for `CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT`.
        check_lt("cublasLtMatrixLayoutSetAttribute", unsafe {
            lt_sys::cublasLtMatrixLayoutSetAttribute(
                self.raw,
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT,
                (&raw const batch_count).cast(),
                size_of_val(&batch_count),
            )
        })?;
        // SAFETY: `strided_offset` is `int64_t` as documented for `CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET`.
        check_lt("cublasLtMatrixLayoutSetAttribute", unsafe {
            lt_sys::cublasLtMatrixLayoutSetAttribute(
                self.raw,
                lt_sys::cublasLtMatrixLayoutAttribute_t_CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET,
                (&raw const strided_offset).cast(),
                size_of_val(&strided_offset),
            )
        })?;
        self.batch_count = batch_count;
        self.strided_offset = strided_offset;
        Ok(())
    }

    /// Minimum required element count for buffers bound to this layout.
    pub fn min_elems(&self) -> Result<usize> {
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

/// Heuristic search preference descriptor (`cublasLtMatmulPreference_t`).
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

    /// Sets the maximum workspace size in bytes (`CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES`).
    pub fn set_max_workspace_bytes(&mut self, bytes: u64) -> Result<()> {
        // SAFETY: `bytes` is `uint64_t` as documented for `CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES`.
        check_lt("cublasLtMatmulPreferenceSetAttribute", unsafe {
            lt_sys::cublasLtMatmulPreferenceSetAttribute(
                self.raw,
                lt_sys::cublasLtMatmulPreferenceAttributes_t_CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES,
                (&raw const bytes).cast(),
                size_of_val(&bytes),
            )
        })
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

/// Algorithm heuristic result returned by [`LtHandle::matmul_heuristic`].
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
