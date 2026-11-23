//! Generic cuSPARSE operations: `SpMV`, `SpMM`, `SpGEMM`, `SpSV`, and `SpSM`.
//!
//! **What.**
//! - [`spmv_buffer_size`] / [`spmv`] — sparse matrix-vector product `y = alpha * op(A) * x + beta * y`.
//! - [`spmm_buffer_size`] / [`spmm`] — sparse-dense matrix product `C = alpha * op(A) * op(B) + beta * C`.
//! - [`SpGemm`] — sparse-sparse matrix product `C = alpha * op(A) * op(B) + beta * C` with
//!   two-phase `work_estimation` and `compute` sizing, byte-budget enforcement, and `copy`.
//! - [`SpSv`] / [`SpSm`] — sparse triangular vector/matrix solve with analysis + solve phases.
//!
//! **Workspace discipline.** Every operation queries required workspace bytes first, checks the
//! caller's byte budget, and allocates on the target [`Stream`]. `spmv` / `spmm` reuse the
//! caller-owned [`Scratch`] (no steady-state allocations once it reaches the high-water mark).
//! `cusparseSpSV_solve`, `cusparseSpSM_solve`, and `cusparseSpGEMM_copy` take no buffer argument
//! and keep reading the buffers passed to the earlier phases, so [`SpSv`], [`SpSm`], and
//! [`SpGemm`] own those buffers (the caller's `Scratch` only names the stream) and require every
//! later phase to run on that same stream, which also orders the stream-ordered free on drop.
//!
//! **Validation.** Host scalars `alpha`/`beta` must have `S::DATA_TYPE == compute_type`
//! (cuSPARSE reads them as `compute_type`), every descriptor must belong to the handle's
//! `Context`, and input sparse descriptors must have bound buffers.

use crate::descr::{DnMatDescr, DnVecDescr, SpMatDescr, SparseScalar};
use crate::{
    DataType, Error, Handle, Op, Result, Scratch, check, ensure_loaded, same_stream, sys, teardown,
};
use cuda_core::{Context, Stream};
use std::ffi::c_void;
use std::sync::Arc;

/// Algorithm selection for `SpMV` (`cusparseSpMVAlg_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SpMvAlg {
    /// `CUSPARSE_SPMV_ALG_DEFAULT`.
    #[default]
    Default,
    /// `CUSPARSE_SPMV_CSR_ALG1`.
    CsrAlg1,
    /// `CUSPARSE_SPMV_CSR_ALG2`.
    CsrAlg2,
    /// `CUSPARSE_SPMV_COO_ALG1`.
    CooAlg1,
    /// `CUSPARSE_SPMV_COO_ALG2`.
    CooAlg2,
}

impl SpMvAlg {
    pub(crate) fn to_sys(self) -> sys::cusparseSpMVAlg_t {
        match self {
            Self::Default => sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_ALG_DEFAULT,
            Self::CsrAlg1 => sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_CSR_ALG1,
            Self::CsrAlg2 => sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_CSR_ALG2,
            Self::CooAlg1 => sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_COO_ALG1,
            Self::CooAlg2 => sys::cusparseSpMVAlg_t_CUSPARSE_SPMV_COO_ALG2,
        }
    }
}

/// Algorithm selection for `SpMM` (`cusparseSpMMAlg_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SpMmAlg {
    /// `CUSPARSE_SPMM_ALG_DEFAULT`.
    #[default]
    Default,
    /// `CUSPARSE_SPMM_CSR_ALG1`.
    CsrAlg1,
    /// `CUSPARSE_SPMM_CSR_ALG2`.
    CsrAlg2,
    /// `CUSPARSE_SPMM_CSR_ALG3`.
    CsrAlg3,
    /// `CUSPARSE_SPMM_COO_ALG1`.
    CooAlg1,
    /// `CUSPARSE_SPMM_COO_ALG2`.
    CooAlg2,
    /// `CUSPARSE_SPMM_COO_ALG3`.
    CooAlg3,
    /// `CUSPARSE_SPMM_COO_ALG4`.
    CooAlg4,
    /// `CUSPARSE_SPMM_BLOCKED_ELL_ALG1`.
    BlockedEllAlg1,
}

impl SpMmAlg {
    pub(crate) fn to_sys(self) -> sys::cusparseSpMMAlg_t {
        match self {
            Self::Default => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_ALG_DEFAULT,
            Self::CsrAlg1 => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_CSR_ALG1,
            Self::CsrAlg2 => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_CSR_ALG2,
            Self::CsrAlg3 => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_CSR_ALG3,
            Self::CooAlg1 => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_COO_ALG1,
            Self::CooAlg2 => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_COO_ALG2,
            Self::CooAlg3 => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_COO_ALG3,
            Self::CooAlg4 => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_COO_ALG4,
            Self::BlockedEllAlg1 => sys::cusparseSpMMAlg_t_CUSPARSE_SPMM_BLOCKED_ELL_ALG1,
        }
    }
}

/// Algorithm selection for `SpGEMM` (`cusparseSpGEMMAlg_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SpGemmAlg {
    /// `CUSPARSE_SPGEMM_DEFAULT`.
    #[default]
    Default,
    /// `CUSPARSE_SPGEMM_CSR_ALG_DETERMINITIC`.
    Deterministic,
    /// `CUSPARSE_SPGEMM_CSR_ALG_NONDETERMINITIC`.
    NonDeterministic,
    /// `CUSPARSE_SPGEMM_ALG1`.
    Alg1,
    /// `CUSPARSE_SPGEMM_ALG2`.
    Alg2,
    /// `CUSPARSE_SPGEMM_ALG3`.
    Alg3,
}

impl SpGemmAlg {
    pub(crate) fn to_sys(self) -> sys::cusparseSpGEMMAlg_t {
        match self {
            Self::Default => sys::cusparseSpGEMMAlg_t_CUSPARSE_SPGEMM_DEFAULT,
            Self::Deterministic => sys::cusparseSpGEMMAlg_t_CUSPARSE_SPGEMM_CSR_ALG_DETERMINITIC,
            Self::NonDeterministic => {
                sys::cusparseSpGEMMAlg_t_CUSPARSE_SPGEMM_CSR_ALG_NONDETERMINITIC
            }
            Self::Alg1 => sys::cusparseSpGEMMAlg_t_CUSPARSE_SPGEMM_ALG1,
            Self::Alg2 => sys::cusparseSpGEMMAlg_t_CUSPARSE_SPGEMM_ALG2,
            Self::Alg3 => sys::cusparseSpGEMMAlg_t_CUSPARSE_SPGEMM_ALG3,
        }
    }
}

/// Algorithm selection for `SpSV` (`cusparseSpSVAlg_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SpSvAlg {
    /// `CUSPARSE_SPSV_ALG_DEFAULT`.
    #[default]
    Default,
}

impl SpSvAlg {
    pub(crate) fn to_sys(self) -> sys::cusparseSpSVAlg_t {
        match self {
            Self::Default => sys::cusparseSpSVAlg_t_CUSPARSE_SPSV_ALG_DEFAULT,
        }
    }
}

/// Algorithm selection for `SpSM` (`cusparseSpSMAlg_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SpSmAlg {
    /// `CUSPARSE_SPSM_ALG_DEFAULT`.
    #[default]
    Default,
}

impl SpSmAlg {
    pub(crate) fn to_sys(self) -> sys::cusparseSpSMAlg_t {
        match self {
            Self::Default => sys::cusparseSpSMAlg_t_CUSPARSE_SPSM_ALG_DEFAULT,
        }
    }
}

/// Validates `SpMV` operand dimensions: `y = alpha * op_a(A) * x + beta * y`.
pub fn validate_spmv_dims(
    op_a: Op,
    a_rows: i64,
    a_cols: i64,
    x_len: i64,
    y_len: i64,
) -> Result<()> {
    let (eff_rows, eff_cols) = match op_a {
        Op::NonTranspose => (a_rows, a_cols),
        Op::Transpose | Op::ConjTranspose => (a_cols, a_rows),
    };
    if eff_cols != x_len {
        return Err(Error::InvalidInput(
            "SpMV op(A) columns must match x length",
        ));
    }
    if eff_rows != y_len {
        return Err(Error::InvalidInput("SpMV op(A) rows must match y length"));
    }
    Ok(())
}

/// Validates `SpMM` operand dimensions: `C = alpha * op_a(A) * op_b(B) + beta * C`.
#[expect(
    clippy::too_many_arguments,
    reason = "validates op(A) * op(B) -> C dimensions"
)]
pub fn validate_spmm_dims(
    op_a: Op,
    op_b: Op,
    a_rows: i64,
    a_cols: i64,
    b_rows: i64,
    b_cols: i64,
    c_rows: i64,
    c_cols: i64,
) -> Result<()> {
    let (m_a, k_a) = match op_a {
        Op::NonTranspose => (a_rows, a_cols),
        Op::Transpose | Op::ConjTranspose => (a_cols, a_rows),
    };
    let (k_b, n_b) = match op_b {
        Op::NonTranspose => (b_rows, b_cols),
        Op::Transpose | Op::ConjTranspose => (b_cols, b_rows),
    };
    if k_a != k_b {
        return Err(Error::InvalidInput(
            "SpMM inner dimensions of op(A) and op(B) must match",
        ));
    }
    if m_a != c_rows || n_b != c_cols {
        return Err(Error::InvalidInput(
            "SpMM output matrix C dimensions do not match op(A) * op(B)",
        ));
    }
    Ok(())
}

/// Validates that the host scalar type `S` of `alpha`/`beta` equals `compute_type`.
///
/// cuSPARSE reads `alpha` and `beta` as `compute_type` values; a narrower `S` would be read past
/// its end on the host.
pub fn validate_scalar_type<S: SparseScalar>(compute_type: DataType) -> Result<()> {
    if S::DATA_TYPE != compute_type {
        return Err(Error::InvalidInput(
            "host scalar type of alpha/beta must equal compute_type",
        ));
    }
    Ok(())
}

/// Rejects any descriptor `Context` that is not the handle's `Context`.
fn ensure_handle_context(handle: &Handle, contexts: &[&Arc<Context>]) -> Result<()> {
    if contexts.iter().all(|c| Arc::ptr_eq(handle.cuda(), *c)) {
        Ok(())
    } else {
        Err(Error::InvalidInput(
            "descriptor belongs to a different CUDA context than the cuSPARSE handle",
        ))
    }
}

/// Returns the workspace owned by `slot` for `stream`, replacing it with an empty one bound to
/// `scratch`'s stream when it is missing or bound to another stream.
fn owned_workspace<'a>(
    slot: &'a mut Option<Scratch>,
    scratch: &Scratch,
    stream: &Stream,
) -> Result<&'a mut Scratch> {
    if !same_stream(scratch.stream(), stream) {
        return Err(Error::InvalidInput(
            "scratch used on a different stream than it was created for",
        ));
    }
    if !slot
        .as_ref()
        .is_some_and(|ws| same_stream(ws.stream(), stream))
    {
        *slot = Some(Scratch::new(scratch.stream()));
    }
    slot.as_mut()
        .ok_or(Error::InvalidInput("owned workspace missing after creation"))
}

/// Requires that an earlier phase left its workspace in `ws` on `stream`.
fn ensure_phase_on(ws: Option<&Scratch>, stream: &Stream, msg: &'static str) -> Result<()> {
    match ws {
        Some(ws) if same_stream(ws.stream(), stream) => Ok(()),
        _ => Err(Error::InvalidInput(msg)),
    }
}

/// Queries the workspace byte size required for `spmv`.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn spmv_buffer_size<S: SparseScalar>(
    handle: &Handle,
    op_a: Op,
    alpha: &S,
    mat_a: &SpMatDescr,
    vec_x: &DnVecDescr,
    beta: &S,
    vec_y: &mut DnVecDescr,
    compute_type: DataType,
    alg: SpMvAlg,
) -> Result<usize> {
    validate_scalar_type::<S>(compute_type)?;
    ensure_handle_context(handle, &[mat_a.cuda(), vec_x.cuda(), vec_y.cuda()])?;
    mat_a.ensure_bound()?;
    handle.ensure_host_pointer_mode()?;
    validate_spmv_dims(op_a, mat_a.rows(), mat_a.cols(), vec_x.len(), vec_y.len())?;
    let mut size = 0usize;
    // SAFETY: descriptors and host scalars are valid for the duration of the call.
    check("cusparseSpMV_bufferSize", unsafe {
        sys::cusparseSpMV_bufferSize(
            handle.raw(),
            op_a.to_sys(),
            std::ptr::from_ref(alpha).cast::<c_void>(),
            mat_a.raw_const(),
            vec_x.raw_const(),
            std::ptr::from_ref(beta).cast::<c_void>(),
            vec_y.raw(),
            compute_type.to_sys(),
            alg.to_sys(),
            &mut size,
        )
    })?;
    Ok(size)
}

/// Executes `y = alpha * op_a(A) * x + beta * y` on `stream`, using `scratch` within `budget`
/// bytes.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn spmv<S: SparseScalar>(
    handle: &Handle,
    stream: &Stream,
    op_a: Op,
    alpha: &S,
    mat_a: &SpMatDescr,
    vec_x: &DnVecDescr,
    beta: &S,
    vec_y: &mut DnVecDescr,
    compute_type: DataType,
    alg: SpMvAlg,
    scratch: &mut Scratch,
    budget: usize,
) -> Result<()> {
    handle.set_stream(stream)?;
    let req = spmv_buffer_size(
        handle,
        op_a,
        alpha,
        mat_a,
        vec_x,
        beta,
        vec_y,
        compute_type,
        alg,
    )?;
    let (ws_ptr, _) = scratch.reserve_with_budget(stream, req, budget)?;
    // SAFETY: `ws_ptr` holds at least `req` bytes on `stream`; descriptors are valid.
    check("cusparseSpMV_preprocess", unsafe {
        sys::cusparseSpMV_preprocess(
            handle.raw(),
            op_a.to_sys(),
            std::ptr::from_ref(alpha).cast::<c_void>(),
            mat_a.raw_const(),
            vec_x.raw_const(),
            std::ptr::from_ref(beta).cast::<c_void>(),
            vec_y.raw(),
            compute_type.to_sys(),
            alg.to_sys(),
            ws_ptr,
        )
    })?;
    // SAFETY: `ws_ptr` holds at least `req` bytes on `stream`; descriptors are valid.
    check("cusparseSpMV", unsafe {
        sys::cusparseSpMV(
            handle.raw(),
            op_a.to_sys(),
            std::ptr::from_ref(alpha).cast::<c_void>(),
            mat_a.raw_const(),
            vec_x.raw_const(),
            std::ptr::from_ref(beta).cast::<c_void>(),
            vec_y.raw(),
            compute_type.to_sys(),
            alg.to_sys(),
            ws_ptr,
        )
    })
}

/// Queries the workspace byte size required for `spmm`.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn spmm_buffer_size<S: SparseScalar>(
    handle: &Handle,
    op_a: Op,
    op_b: Op,
    alpha: &S,
    mat_a: &SpMatDescr,
    mat_b: &DnMatDescr,
    beta: &S,
    mat_c: &mut DnMatDescr,
    compute_type: DataType,
    alg: SpMmAlg,
) -> Result<usize> {
    validate_scalar_type::<S>(compute_type)?;
    ensure_handle_context(handle, &[mat_a.cuda(), mat_b.cuda(), mat_c.cuda()])?;
    mat_a.ensure_bound()?;
    handle.ensure_host_pointer_mode()?;
    validate_spmm_dims(
        op_a,
        op_b,
        mat_a.rows(),
        mat_a.cols(),
        mat_b.rows(),
        mat_b.cols(),
        mat_c.rows(),
        mat_c.cols(),
    )?;
    let mut size = 0usize;
    // SAFETY: descriptors and host scalars are valid for the duration of the call.
    check("cusparseSpMM_bufferSize", unsafe {
        sys::cusparseSpMM_bufferSize(
            handle.raw(),
            op_a.to_sys(),
            op_b.to_sys(),
            std::ptr::from_ref(alpha).cast::<c_void>(),
            mat_a.raw_const(),
            mat_b.raw_const(),
            std::ptr::from_ref(beta).cast::<c_void>(),
            mat_c.raw(),
            compute_type.to_sys(),
            alg.to_sys(),
            &mut size,
        )
    })?;
    Ok(size)
}

/// Executes `C = alpha * op_a(A) * op_b(B) + beta * C` on `stream`, using `scratch` within
/// `budget` bytes.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn spmm<S: SparseScalar>(
    handle: &Handle,
    stream: &Stream,
    op_a: Op,
    op_b: Op,
    alpha: &S,
    mat_a: &SpMatDescr,
    mat_b: &DnMatDescr,
    beta: &S,
    mat_c: &mut DnMatDescr,
    compute_type: DataType,
    alg: SpMmAlg,
    scratch: &mut Scratch,
    budget: usize,
) -> Result<()> {
    handle.set_stream(stream)?;
    let req = spmm_buffer_size(
        handle,
        op_a,
        op_b,
        alpha,
        mat_a,
        mat_b,
        beta,
        mat_c,
        compute_type,
        alg,
    )?;
    let (ws_ptr, _) = scratch.reserve_with_budget(stream, req, budget)?;
    // SAFETY: `ws_ptr` holds at least `req` bytes on `stream`; descriptors are valid.
    check("cusparseSpMM_preprocess", unsafe {
        sys::cusparseSpMM_preprocess(
            handle.raw(),
            op_a.to_sys(),
            op_b.to_sys(),
            std::ptr::from_ref(alpha).cast::<c_void>(),
            mat_a.raw_const(),
            mat_b.raw_const(),
            std::ptr::from_ref(beta).cast::<c_void>(),
            mat_c.raw(),
            compute_type.to_sys(),
            alg.to_sys(),
            ws_ptr,
        )
    })?;
    // SAFETY: `ws_ptr` holds at least `req` bytes on `stream`; descriptors are valid.
    check("cusparseSpMM", unsafe {
        sys::cusparseSpMM(
            handle.raw(),
            op_a.to_sys(),
            op_b.to_sys(),
            std::ptr::from_ref(alpha).cast::<c_void>(),
            mat_a.raw_const(),
            mat_b.raw_const(),
            std::ptr::from_ref(beta).cast::<c_void>(),
            mat_c.raw(),
            compute_type.to_sys(),
            alg.to_sys(),
            ws_ptr,
        )
    })
}

/// Owning `cusparseSpGEMMDescr_t` for two-phase sparse-sparse matrix multiplication.
///
/// Owns `externalBuffer1` (from [`SpGemm::work_estimation`]) and `externalBuffer2` (from
/// [`SpGemm::compute`]): the native descriptor keeps referencing them until
/// `cusparseSpGEMM_copy`, which takes no buffer argument. All three phases must run on the
/// same stream.
pub struct SpGemm {
    raw: sys::cusparseSpGEMMDescr_t,
    cuda: Arc<Context>,
    ws1: Option<Scratch>,
    ws2: Option<Scratch>,
}

// SAFETY: `cusparseSpGEMMDescr_t` is externally synchronized by callers.
unsafe impl Send for SpGemm {}

impl SpGemm {
    /// Creates a new `SpGEMM` descriptor on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseSpGEMMDescr_t = std::ptr::null_mut();
        // SAFETY: `cuda` is current and `raw` is a valid out-pointer.
        check("cusparseSpGEMM_createDescr", unsafe {
            sys::cusparseSpGEMM_createDescr(&mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            ws1: None,
            ws2: None,
        })
    }

    /// Phase 1: two-phase `cusparseSpGEMM_workEstimation` (queries `bufferSize1`, allocates it
    /// within `budget` into a buffer owned by `self`, then runs work estimation).
    ///
    /// `scratch` must be bound to `stream`; only its stream is used. Starting phase 1 on another
    /// stream than a previous run drops the previous buffers.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn work_estimation<S: SparseScalar>(
        &mut self,
        handle: &Handle,
        stream: &Stream,
        op_a: Op,
        op_b: Op,
        alpha: &S,
        mat_a: &SpMatDescr,
        mat_b: &SpMatDescr,
        beta: &S,
        mat_c: &mut SpMatDescr,
        compute_type: DataType,
        alg: SpGemmAlg,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<usize> {
        validate_scalar_type::<S>(compute_type)?;
        ensure_handle_context(
            handle,
            &[&self.cuda, mat_a.cuda(), mat_b.cuda(), mat_c.cuda()],
        )?;
        mat_a.ensure_bound()?;
        mat_b.ensure_bound()?;
        handle.ensure_host_pointer_mode()?;
        handle.set_stream(stream)?;
        if !self
            .ws1
            .as_ref()
            .is_some_and(|ws| same_stream(ws.stream(), stream))
        {
            // Buffer2 of a run on another stream belongs to that stream; start this run clean.
            self.ws2 = None;
        }
        let mut size1 = 0usize;
        // SAFETY: size query with null externalBuffer1.
        check("cusparseSpGEMM_workEstimation", unsafe {
            sys::cusparseSpGEMM_workEstimation(
                handle.raw(),
                op_a.to_sys(),
                op_b.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                mat_b.raw_const(),
                std::ptr::from_ref(beta).cast::<c_void>(),
                mat_c.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
                &mut size1,
                std::ptr::null_mut(),
            )
        })?;
        let (ws1, _) = owned_workspace(&mut self.ws1, scratch, stream)?
            .reserve_with_budget(stream, size1, budget)?;
        // SAFETY: `ws1` has at least `size1` bytes on `stream` and is owned by `self` until the
        // descriptor is destroyed or phase 1 is rerun.
        check("cusparseSpGEMM_workEstimation", unsafe {
            sys::cusparseSpGEMM_workEstimation(
                handle.raw(),
                op_a.to_sys(),
                op_b.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                mat_b.raw_const(),
                std::ptr::from_ref(beta).cast::<c_void>(),
                mat_c.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
                &mut size1,
                ws1,
            )
        })?;
        Ok(size1)
    }

    /// Phase 2: two-phase `cusparseSpGEMM_compute` (queries `bufferSize2`, allocates it within
    /// `budget` into a buffer owned by `self`, then computes the product structure and values).
    ///
    /// Requires [`SpGemm::work_estimation`] on the same `stream`; `scratch` must be bound to
    /// `stream` and only its stream is used.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn compute<S: SparseScalar>(
        &mut self,
        handle: &Handle,
        stream: &Stream,
        op_a: Op,
        op_b: Op,
        alpha: &S,
        mat_a: &SpMatDescr,
        mat_b: &SpMatDescr,
        beta: &S,
        mat_c: &mut SpMatDescr,
        compute_type: DataType,
        alg: SpGemmAlg,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<usize> {
        validate_scalar_type::<S>(compute_type)?;
        ensure_handle_context(
            handle,
            &[&self.cuda, mat_a.cuda(), mat_b.cuda(), mat_c.cuda()],
        )?;
        mat_a.ensure_bound()?;
        mat_b.ensure_bound()?;
        ensure_phase_on(
            self.ws1.as_ref(),
            stream,
            "SpGEMM compute requires work_estimation on the same stream",
        )?;
        handle.ensure_host_pointer_mode()?;
        handle.set_stream(stream)?;
        let mut size2 = 0usize;
        // SAFETY: size query with null externalBuffer2.
        check("cusparseSpGEMM_compute", unsafe {
            sys::cusparseSpGEMM_compute(
                handle.raw(),
                op_a.to_sys(),
                op_b.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                mat_b.raw_const(),
                std::ptr::from_ref(beta).cast::<c_void>(),
                mat_c.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
                &mut size2,
                std::ptr::null_mut(),
            )
        })?;
        let (ws2, _) = owned_workspace(&mut self.ws2, scratch, stream)?
            .reserve_with_budget(stream, size2, budget)?;
        // SAFETY: `ws2` has at least `size2` bytes on `stream`; it and buffer1 are owned by
        // `self` until the descriptor is destroyed or phase 1 is rerun.
        check("cusparseSpGEMM_compute", unsafe {
            sys::cusparseSpGEMM_compute(
                handle.raw(),
                op_a.to_sys(),
                op_b.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                mat_b.raw_const(),
                std::ptr::from_ref(beta).cast::<c_void>(),
                mat_c.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
                &mut size2,
                ws2,
            )
        })?;
        Ok(size2)
    }

    /// Phase 3: `cusparseSpGEMM_copy`, writing the computed `C` into `mat_c`'s buffers after
    /// `mat_c.set_csr_pointers(...)`.
    ///
    /// Requires [`SpGemm::compute`] on the same `stream`, and `mat_c` bound to buffers validated
    /// for its current native nnz.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn copy<S: SparseScalar>(
        &mut self,
        handle: &Handle,
        stream: &Stream,
        op_a: Op,
        op_b: Op,
        alpha: &S,
        mat_a: &SpMatDescr,
        mat_b: &SpMatDescr,
        beta: &S,
        mat_c: &mut SpMatDescr,
        compute_type: DataType,
        alg: SpGemmAlg,
    ) -> Result<()> {
        validate_scalar_type::<S>(compute_type)?;
        ensure_handle_context(
            handle,
            &[&self.cuda, mat_a.cuda(), mat_b.cuda(), mat_c.cuda()],
        )?;
        mat_a.ensure_bound()?;
        mat_b.ensure_bound()?;
        mat_c.ensure_bound()?;
        let (_, _, native_nnz) = mat_c.query_size()?;
        if native_nnz != mat_c.nnz() {
            return Err(Error::InvalidInput(
                "SpGEMM copy: mat_c buffers were bound for a different nnz than compute produced",
            ));
        }
        ensure_phase_on(
            self.ws2.as_ref(),
            stream,
            "SpGEMM copy requires compute on the same stream",
        )?;
        handle.ensure_host_pointer_mode()?;
        handle.set_stream(stream)?;
        // SAFETY: `mat_c` is bound to buffers validated for its native nnz; buffer1/buffer2
        // referenced by `self.raw` are owned by `self` and were used on `stream`.
        check("cusparseSpGEMM_copy", unsafe {
            sys::cusparseSpGEMM_copy(
                handle.raw(),
                op_a.to_sys(),
                op_b.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                mat_b.raw_const(),
                std::ptr::from_ref(beta).cast::<c_void>(),
                mat_c.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
            )
        })
    }
}

impl Drop for SpGemm {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusparseSpGEMM_destroyDescr", || {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                unsafe { sys::cusparseSpGEMM_destroyDescr(raw) }
            });
        }
    }
}

/// Owning `cusparseSpSVDescr_t` for sparse triangular vector solve (`op(A) * y = alpha * x`).
///
/// Owns the analysis buffer: `cusparseSpSV_solve` takes no buffer argument and reads the one
/// passed to `cusparseSpSV_analysis`, so `solve` must run on the stream used by `prepare`.
pub struct SpSv {
    raw: sys::cusparseSpSVDescr_t,
    cuda: Arc<Context>,
    ws: Option<Scratch>,
}

// SAFETY: `cusparseSpSVDescr_t` is externally synchronized by callers.
unsafe impl Send for SpSv {}

impl SpSv {
    /// Creates a new `SpSV` descriptor on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseSpSVDescr_t = std::ptr::null_mut();
        // SAFETY: `cuda` is current and `raw` is a valid out-pointer.
        check("cusparseSpSV_createDescr", unsafe {
            sys::cusparseSpSV_createDescr(&mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            ws: None,
        })
    }

    /// Queries workspace bytes, allocates them within `budget` into an analysis buffer owned by
    /// `self`, and runs `cusparseSpSV_analysis`.
    ///
    /// `scratch` must be bound to `stream`; only its stream is used.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn prepare<S: SparseScalar>(
        &mut self,
        handle: &Handle,
        stream: &Stream,
        op_a: Op,
        alpha: &S,
        mat_a: &SpMatDescr,
        vec_x: &DnVecDescr,
        vec_y: &mut DnVecDescr,
        compute_type: DataType,
        alg: SpSvAlg,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<usize> {
        if mat_a.rows() != mat_a.cols()
            || mat_a.rows() != vec_x.len()
            || mat_a.rows() != vec_y.len()
        {
            return Err(Error::InvalidInput(
                "SpSV requires square A matching x and y lengths",
            ));
        }
        validate_scalar_type::<S>(compute_type)?;
        ensure_handle_context(
            handle,
            &[&self.cuda, mat_a.cuda(), vec_x.cuda(), vec_y.cuda()],
        )?;
        mat_a.ensure_bound()?;
        handle.ensure_host_pointer_mode()?;
        handle.set_stream(stream)?;
        let mut size = 0usize;
        // SAFETY: descriptors and host scalar are valid.
        check("cusparseSpSV_bufferSize", unsafe {
            sys::cusparseSpSV_bufferSize(
                handle.raw(),
                op_a.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                vec_x.raw_const(),
                vec_y.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
                &mut size,
            )
        })?;
        let (ws, _) = owned_workspace(&mut self.ws, scratch, stream)?
            .reserve_with_budget(stream, size, budget)?;
        // SAFETY: `ws` holds at least `size` bytes on `stream` and is owned by `self` for as long
        // as `self.raw` can read it; descriptors are valid.
        check("cusparseSpSV_analysis", unsafe {
            sys::cusparseSpSV_analysis(
                handle.raw(),
                op_a.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                vec_x.raw_const(),
                vec_y.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
                ws,
            )
        })?;
        Ok(size)
    }

    /// Executes `cusparseSpSV_solve` using the analysis stored in `self`.
    ///
    /// Requires [`SpSv::prepare`] on the same `stream`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn solve<S: SparseScalar>(
        &self,
        handle: &Handle,
        stream: &Stream,
        op_a: Op,
        alpha: &S,
        mat_a: &SpMatDescr,
        vec_x: &DnVecDescr,
        vec_y: &mut DnVecDescr,
        compute_type: DataType,
        alg: SpSvAlg,
    ) -> Result<()> {
        validate_scalar_type::<S>(compute_type)?;
        ensure_handle_context(
            handle,
            &[&self.cuda, mat_a.cuda(), vec_x.cuda(), vec_y.cuda()],
        )?;
        mat_a.ensure_bound()?;
        ensure_phase_on(
            self.ws.as_ref(),
            stream,
            "SpSV solve requires prepare on the same stream",
        )?;
        handle.ensure_host_pointer_mode()?;
        handle.set_stream(stream)?;
        // SAFETY: `prepare` has analyzed this descriptor on `stream` into a buffer owned by `self`.
        check("cusparseSpSV_solve", unsafe {
            sys::cusparseSpSV_solve(
                handle.raw(),
                op_a.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                vec_x.raw_const(),
                vec_y.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
            )
        })
    }
}

impl Drop for SpSv {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusparseSpSV_destroyDescr", || {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                unsafe { sys::cusparseSpSV_destroyDescr(raw) }
            });
        }
    }
}

/// Owning `cusparseSpSMDescr_t` for sparse triangular matrix solve (`op(A) * C = alpha * op(B)`).
///
/// Owns the analysis buffer: `cusparseSpSM_solve` takes no buffer argument and reads the one
/// passed to `cusparseSpSM_analysis`, so `solve` must run on the stream used by `prepare`.
pub struct SpSm {
    raw: sys::cusparseSpSMDescr_t,
    cuda: Arc<Context>,
    ws: Option<Scratch>,
}

// SAFETY: `cusparseSpSMDescr_t` is externally synchronized by callers.
unsafe impl Send for SpSm {}

impl SpSm {
    /// Creates a new `SpSM` descriptor on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseSpSMDescr_t = std::ptr::null_mut();
        // SAFETY: `cuda` is current and `raw` is a valid out-pointer.
        check("cusparseSpSM_createDescr", unsafe {
            sys::cusparseSpSM_createDescr(&mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            ws: None,
        })
    }

    /// Queries workspace bytes, allocates them within `budget` into an analysis buffer owned by
    /// `self`, and runs `cusparseSpSM_analysis`.
    ///
    /// `scratch` must be bound to `stream`; only its stream is used.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn prepare<S: SparseScalar>(
        &mut self,
        handle: &Handle,
        stream: &Stream,
        op_a: Op,
        op_b: Op,
        alpha: &S,
        mat_a: &SpMatDescr,
        mat_b: &DnMatDescr,
        mat_c: &mut DnMatDescr,
        compute_type: DataType,
        alg: SpSmAlg,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<usize> {
        if mat_a.rows() != mat_a.cols() {
            return Err(Error::InvalidInput("SpSM requires square sparse matrix A"));
        }
        validate_scalar_type::<S>(compute_type)?;
        ensure_handle_context(
            handle,
            &[&self.cuda, mat_a.cuda(), mat_b.cuda(), mat_c.cuda()],
        )?;
        mat_a.ensure_bound()?;
        validate_spmm_dims(
            op_a,
            op_b,
            mat_a.rows(),
            mat_a.cols(),
            mat_b.rows(),
            mat_b.cols(),
            mat_c.rows(),
            mat_c.cols(),
        )?;
        handle.ensure_host_pointer_mode()?;
        handle.set_stream(stream)?;
        let mut size = 0usize;
        // SAFETY: descriptors and host scalar are valid.
        check("cusparseSpSM_bufferSize", unsafe {
            sys::cusparseSpSM_bufferSize(
                handle.raw(),
                op_a.to_sys(),
                op_b.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                mat_b.raw_const(),
                mat_c.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
                &mut size,
            )
        })?;
        let (ws, _) = owned_workspace(&mut self.ws, scratch, stream)?
            .reserve_with_budget(stream, size, budget)?;
        // SAFETY: `ws` holds at least `size` bytes on `stream` and is owned by `self` for as long
        // as `self.raw` can read it; descriptors are valid.
        check("cusparseSpSM_analysis", unsafe {
            sys::cusparseSpSM_analysis(
                handle.raw(),
                op_a.to_sys(),
                op_b.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                mat_b.raw_const(),
                mat_c.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
                ws,
            )
        })?;
        Ok(size)
    }

    /// Executes `cusparseSpSM_solve` using the analysis stored in `self`.
    ///
    /// Requires [`SpSm::prepare`] on the same `stream`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn solve<S: SparseScalar>(
        &self,
        handle: &Handle,
        stream: &Stream,
        op_a: Op,
        op_b: Op,
        alpha: &S,
        mat_a: &SpMatDescr,
        mat_b: &DnMatDescr,
        mat_c: &mut DnMatDescr,
        compute_type: DataType,
        alg: SpSmAlg,
    ) -> Result<()> {
        validate_scalar_type::<S>(compute_type)?;
        ensure_handle_context(
            handle,
            &[&self.cuda, mat_a.cuda(), mat_b.cuda(), mat_c.cuda()],
        )?;
        mat_a.ensure_bound()?;
        ensure_phase_on(
            self.ws.as_ref(),
            stream,
            "SpSM solve requires prepare on the same stream",
        )?;
        handle.ensure_host_pointer_mode()?;
        handle.set_stream(stream)?;
        // SAFETY: `prepare` has analyzed this descriptor on `stream` into a buffer owned by `self`.
        check("cusparseSpSM_solve", unsafe {
            sys::cusparseSpSM_solve(
                handle.raw(),
                op_a.to_sys(),
                op_b.to_sys(),
                std::ptr::from_ref(alpha).cast::<c_void>(),
                mat_a.raw_const(),
                mat_b.raw_const(),
                mat_c.raw(),
                compute_type.to_sys(),
                alg.to_sys(),
                self.raw,
            )
        })
    }
}

impl Drop for SpSm {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusparseSpSM_destroyDescr", || {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                unsafe { sys::cusparseSpSM_destroyDescr(raw) }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spmv_dimension_validation() {
        assert!(validate_spmv_dims(Op::NonTranspose, 3, 4, 4, 3).is_ok());
        assert!(validate_spmv_dims(Op::NonTranspose, 3, 4, 3, 3).is_err());
        assert!(validate_spmv_dims(Op::NonTranspose, 3, 4, 4, 4).is_err());
        assert!(validate_spmv_dims(Op::Transpose, 3, 4, 3, 4).is_ok());
        assert!(validate_spmv_dims(Op::Transpose, 3, 4, 4, 3).is_err());
    }

    #[test]
    fn spmm_dimension_validation() {
        assert!(validate_spmm_dims(Op::NonTranspose, Op::NonTranspose, 3, 4, 4, 2, 3, 2).is_ok());
        assert!(validate_spmm_dims(Op::NonTranspose, Op::Transpose, 3, 4, 2, 4, 3, 2).is_ok());
        assert!(validate_spmm_dims(Op::NonTranspose, Op::NonTranspose, 3, 4, 5, 2, 3, 2).is_err());
        assert!(validate_spmm_dims(Op::NonTranspose, Op::NonTranspose, 3, 4, 4, 2, 4, 2).is_err());
    }

    #[test]
    fn scalar_type_must_equal_compute_type() {
        // F64 descriptors with F64 compute are valid in cuSPARSE, so an `&f32` alpha would be
        // read as 8 bytes from a 4-byte host value.
        assert!(matches!(
            validate_scalar_type::<f32>(DataType::F64),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_scalar_type::<f64>(DataType::C64),
            Err(Error::InvalidInput(_))
        ));
        assert!(validate_scalar_type::<f32>(DataType::F32).is_ok());
        assert!(validate_scalar_type::<[f64; 2]>(DataType::C64).is_ok());
        assert!(validate_scalar_type::<i32>(DataType::I32).is_ok());
    }
}
