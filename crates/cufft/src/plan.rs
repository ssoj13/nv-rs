//! [`Plan`]: 1D, 2D, 3D, and batched (`Many`) cuFFT plans with caller-owned [`Scratch`].
//!
//! **What.** Wraps `cufftHandle` creation (`cufftCreate` + `cufftSetAutoAllocation(0)` +
//! `cufftMakePlan*`), workspace size estimation (`cufftEstimate*` / `cufftGetSize*`), byte-budget
//! enforcement via [`Scratch`], stream binding (`cufftSetStream`), and typed execution for
//! `C2C`, `R2C`, `C2R`, `Z2Z`, `D2Z`, and `Z2D`.
//! **Why.** Prevents hidden driver/cuFFT workspace allocations by enforcing two-phase sizing and
//! explicit byte budgets, retains shared workspace ownership and holds the scratch lock from
//! work-area rebinding until the exec is queued to prevent dangling native pointers, and validates
//! transform types, stream compatibility, buffer extents, and batch layouts before launching.
//! `Plan` is `Send` but `!Sync` (cuFFT plans must not be used from several threads at once).
//! **Used by** `cufft` callers and T0/T3 tests.

use crate::error::{check, teardown};
use crate::scratch::Shared;
use crate::{Error, Result, Scratch, sys};
use cuda_core::{Context, DeviceBuffer, DeviceCopy, Stream};
use cuda_core_ext::{same_context, same_stream};
use std::cell::Cell;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, MutexGuard};

/// Single-precision complex number matching `cufftComplex` (`float2`, 8-byte aligned).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C, align(8))]
pub struct Complex32(pub [f32; 2]);

impl Complex32 {
    /// Creates a complex number `re + i * im`.
    #[inline]
    pub const fn new(re: f32, im: f32) -> Self {
        Self([re, im])
    }

    /// Real part.
    #[inline]
    pub const fn re(self) -> f32 {
        self.0[0]
    }

    /// Imaginary part.
    #[inline]
    pub const fn im(self) -> f32 {
        self.0[1]
    }
}

// SAFETY: `Complex32` is a `#[repr(C, align(8))]` pair of `f32` with valid all-zero bit pattern.
unsafe impl DeviceCopy for Complex32 {}

/// Double-precision complex number matching `cufftDoubleComplex` (`double2`, 16-byte aligned).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C, align(16))]
pub struct Complex64(pub [f64; 2]);

impl Complex64 {
    /// Creates a complex number `re + i * im`.
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self([re, im])
    }

    /// Real part.
    #[inline]
    pub const fn re(self) -> f64 {
        self.0[0]
    }

    /// Imaginary part.
    #[inline]
    pub const fn im(self) -> f64 {
        self.0[1]
    }
}

// SAFETY: `Complex64` is a `#[repr(C, align(16))]` pair of `f64` with valid all-zero bit pattern.
unsafe impl DeviceCopy for Complex64 {}

// Compile-time layout proofs: `cufftComplex` == `Complex32` and `cufftDoubleComplex` == `Complex64`.
const _: () = assert!(size_of::<sys::cufftComplex>() == size_of::<Complex32>());
const _: () = assert!(align_of::<sys::cufftComplex>() == align_of::<Complex32>());
const _: () = assert!(size_of::<sys::cufftDoubleComplex>() == size_of::<Complex64>());
const _: () = assert!(align_of::<sys::cufftDoubleComplex>() == align_of::<Complex64>());

/// cuFFT transform kind (`cufftType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FftType {
    /// Single-precision real-to-complex (`CUFFT_R2C`).
    R2C,
    /// Single-precision complex-to-real (`CUFFT_C2R`).
    C2R,
    /// Single-precision complex-to-complex (`CUFFT_C2C`).
    C2C,
    /// Double-precision real-to-complex (`CUFFT_D2Z`).
    D2Z,
    /// Double-precision complex-to-real (`CUFFT_Z2D`).
    Z2D,
    /// Double-precision complex-to-complex (`CUFFT_Z2Z`).
    Z2Z,
}

impl FftType {
    /// Converts to `cufft_sys::cufftType_t`.
    #[inline]
    pub const fn raw(self) -> sys::cufftType_t {
        match self {
            Self::R2C => sys::cufftType_t_CUFFT_R2C,
            Self::C2R => sys::cufftType_t_CUFFT_C2R,
            Self::C2C => sys::cufftType_t_CUFFT_C2C,
            Self::D2Z => sys::cufftType_t_CUFFT_D2Z,
            Self::Z2D => sys::cufftType_t_CUFFT_Z2D,
            Self::Z2Z => sys::cufftType_t_CUFFT_Z2Z,
        }
    }
}

/// Complex-to-complex transform direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Forward transform (`CUFFT_FORWARD = -1`).
    Forward,
    /// Inverse transform (`CUFFT_INVERSE = 1`).
    ///
    /// cuFFT does not normalize: a forward transform followed by an inverse one returns the input
    /// multiplied by the transform size (the product of the plan's dimensions). Divide by it to
    /// recover the input.
    Inverse,
}

impl Direction {
    /// Converts to `cufftExecC2C` / `cufftExecZ2Z` `direction` argument.
    #[inline]
    pub const fn raw(self) -> i32 {
        match self {
            Self::Forward => sys::CUFFT_FORWARD,
            Self::Inverse => sys::CUFFT_INVERSE as i32,
        }
    }
}

/// Descriptor for a general batched 1D/2D/3D cuFFT plan (`cufftMakePlanMany`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManyDesc {
    /// Dimensionality (`1`, `2`, or `3`).
    pub rank: usize,
    /// Transform dimensions `[n0, n1, n2]` (only first `rank` entries are used).
    pub n: [i32; 3],
    /// Optional input storage dimensions; `None` means contiguous `n`. Must be `Some` exactly when
    /// `onembed` is `Some`: cuFFT itself rejects a mixed NULL/non-NULL pair with
    /// `CUFFT_INVALID_VALUE`, while the same sizes and strides succeed with both embeds set or
    /// both NULL (observed on cuFFT 12 / CUDA 13.3, regression test
    /// `gpu_plan_many_rejects_mixed_null_embeds`). With both NULL, cuFFT ignores the strides and
    /// distances and uses the dense layout.
    pub inembed: Option<[i32; 3]>,
    /// Stride between consecutive input elements.
    pub istride: i32,
    /// Distance between consecutive input batches.
    pub idist: i32,
    /// Optional output storage dimensions; `None` means contiguous. Must be `Some` exactly when
    /// `inembed` is `Some`.
    pub onembed: Option<[i32; 3]>,
    /// Stride between consecutive output elements.
    pub ostride: i32,
    /// Distance between consecutive output batches.
    pub odist: i32,
    /// Transform type.
    pub fft_type: FftType,
    /// Number of transforms in the batch.
    pub batch: i32,
}

impl ManyDesc {
    fn req_dims(&self, is_output: bool) -> [i32; 3] {
        let mut req = self.n;
        let last = self.rank - 1;
        let half_complex = self.n[last] / 2 + 1;
        match (self.fft_type, is_output) {
            (FftType::R2C | FftType::D2Z, true) | (FftType::C2R | FftType::Z2D, false) => {
                req[last] = half_complex;
            }
            _ => {}
        }
        req
    }

    /// Validates `ManyDesc` parameters before reaching cuFFT.
    pub fn validate(&self) -> Result<()> {
        if !(1..=3).contains(&self.rank) {
            return Err(Error::Invalid("ManyDesc rank must be 1, 2, or 3"));
        }
        // cuFFT rejects a mixed NULL/non-NULL embed pair with CUFFT_INVALID_VALUE (the same
        // sizes and strides succeed with both set or both NULL; regression test
        // `gpu_plan_many_rejects_mixed_null_embeds`), so the mix is rejected here with a typed
        // error before reaching cuFFT.
        if self.inembed.is_some() != self.onembed.is_some() {
            return Err(Error::Invalid(
                "ManyDesc inembed and onembed must both be Some or both be None",
            ));
        }
        for i in 0..self.rank {
            if self.n[i] <= 0 {
                return Err(Error::Invalid(
                    "ManyDesc transform dimensions must be positive",
                ));
            }
            if let Some(in_e) = self.inembed
                && in_e[i] <= 0
            {
                return Err(Error::Invalid(
                    "ManyDesc inembed dimensions must be positive",
                ));
            }
            if let Some(out_e) = self.onembed
                && out_e[i] <= 0
            {
                return Err(Error::Invalid(
                    "ManyDesc onembed dimensions must be positive",
                ));
            }
        }
        let req_in = self.req_dims(false);
        let req_out = self.req_dims(true);
        for i in 1..self.rank {
            if let Some(in_e) = self.inembed
                && in_e[i] < req_in[i]
            {
                return Err(Error::Invalid(
                    "ManyDesc inembed[i] must be >= required input dim for i >= 1",
                ));
            }
            if let Some(out_e) = self.onembed
                && out_e[i] < req_out[i]
            {
                return Err(Error::Invalid(
                    "ManyDesc onembed[i] must be >= required output dim for i >= 1",
                ));
            }
        }
        if self.istride <= 0 || self.ostride <= 0 {
            return Err(Error::Invalid(
                "ManyDesc istride and ostride must be positive",
            ));
        }
        if self.batch <= 0 {
            return Err(Error::Invalid("ManyDesc batch must be positive"));
        }
        if self.batch > 1 && (self.idist <= 0 || self.odist <= 0) {
            return Err(Error::Invalid(
                "ManyDesc idist and odist must be positive when batch > 1",
            ));
        }
        Ok(())
    }

    /// Computes minimum `(input_elems, output_elems)` required by this `ManyDesc` using checked
    /// arithmetic.
    ///
    /// Per `cufftMakePlanMany` semantics:
    /// - When both embeds are `None`, cuFFT ignores strides and distances and assumes the
    ///   contiguous dense layout on both sides (mixed `Some`/`None` is rejected by [`Self::validate`]).
    /// - When embed arrays are provided (`Some`), `embed[0]` is ignored (dim 0 uses `req_dim[0]`), and
    ///   `embed[1..rank]` give the storage pitch of inner dimensions.
    ///
    /// With explicit embeds and `batch > 1`, output batches must not overlap, otherwise the
    /// transforms of one batch write the same elements concurrently. Accepted layouts: disjoint
    /// blocks (`odist` at least one transform's output span) or interleaved batches
    /// (`(batch - 1) * odist < ostride`). Overlapping input batches stay allowed because cuFFT only
    /// reads them for `C2C`/`R2C`/`Z2Z`/`D2Z`.
    pub fn min_io_elems(&self) -> Result<(usize, usize)> {
        self.validate()?;
        let (dense_in, dense_out) =
            dense_io_elems(&self.n[..self.rank], self.batch, self.fft_type)?;
        let req_in = self.req_dims(false);
        let req_out = self.req_dims(true);
        let max_idx_for = |req: [i32; 3], embed: [i32; 3]| -> Result<usize> {
            let mut idx = (req[0] - 1) as usize;
            for i in 1..self.rank {
                idx = idx
                    .checked_mul(embed[i] as usize)
                    .and_then(|v| v.checked_add((req[i] - 1) as usize))
                    .ok_or(Error::Invalid("ManyDesc embed index overflow"))?;
            }
            Ok(idx)
        };
        let span = |max_idx: usize, stride: i32, dist: i32| -> Result<usize> {
            let intra = max_idx
                .checked_mul(stride as usize)
                .and_then(|s| s.checked_add(1))
                .ok_or(Error::Invalid("ManyDesc element span overflow"))?;
            if self.batch == 1 {
                Ok(intra)
            } else {
                let batch_off = ((self.batch - 1) as usize)
                    .checked_mul(dist as usize)
                    .ok_or(Error::Invalid("ManyDesc batch distance overflow"))?;
                intra
                    .checked_add(batch_off)
                    .ok_or(Error::Invalid("ManyDesc total span overflow"))
            }
        };
        let in_span = match self.inembed {
            None => dense_in,
            Some(embed) => span(max_idx_for(req_in, embed)?, self.istride, self.idist)?,
        };
        let out_span = match self.onembed {
            None => dense_out,
            Some(embed) => span(max_idx_for(req_out, embed)?, self.ostride, self.odist)?,
        };
        if let Some(embed) = self.onembed
            && self.batch > 1
        {
            // Offsets are `b * odist + ostride * m` with distinct linear embed indices `m` (inner
            // pitches are >= the required dims). Disjoint blocks keep batches apart; with
            // `(batch - 1) * odist < ostride` every `b * odist` is a distinct residue mod `ostride`.
            let out_intra = max_idx_for(req_out, embed)?
                .checked_mul(self.ostride as usize)
                .and_then(|s| s.checked_add(1))
                .ok_or(Error::Invalid("ManyDesc element span overflow"))?;
            let disjoint_blocks = self.odist as usize >= out_intra;
            let interleaved = ((self.batch - 1) as usize)
                .checked_mul(self.odist as usize)
                .is_some_and(|s| s < self.ostride as usize);
            if !disjoint_blocks && !interleaved {
                return Err(Error::Invalid(
                    "ManyDesc output batches overlap: odist must cover one transform's output span, or (batch - 1) * odist must be < ostride",
                ));
            }
        }
        Ok((in_span, out_span))
    }
}

/// Computes minimum `(input_elems, output_elems)` for a contiguous 1D/2D/3D transform.
pub(crate) fn dense_io_elems(
    dims: &[i32],
    batch: i32,
    fft_type: FftType,
) -> Result<(usize, usize)> {
    if batch <= 0 || dims.is_empty() || dims.iter().any(|&d| d <= 0) {
        return Err(Error::Invalid("FFT dimensions and batch must be positive"));
    }
    let last = *dims.last().ok_or(Error::Invalid("empty FFT dimensions"))? as usize;
    let mut prefix = batch as usize;
    for &d in &dims[..dims.len() - 1] {
        prefix = prefix
            .checked_mul(d as usize)
            .ok_or(Error::Invalid("FFT element count overflow"))?;
    }
    let real_elems = prefix
        .checked_mul(last)
        .ok_or(Error::Invalid("FFT real element count overflow"))?;
    let half_complex_elems = prefix
        .checked_mul(last / 2 + 1)
        .ok_or(Error::Invalid("FFT complex element count overflow"))?;
    match fft_type {
        FftType::C2C | FftType::Z2Z => Ok((real_elems, real_elems)),
        FftType::R2C | FftType::D2Z => Ok((real_elems, half_complex_elems)),
        FftType::C2R | FftType::Z2D => Ok((half_complex_elems, real_elems)),
    }
}

/// Internal RAII guard for `cufftHandle` during two-phase construction.
struct RawHandle {
    handle: sys::cufftHandle,
    cuda: Arc<Context>,
}

impl RawHandle {
    fn create(cuda: &Arc<Context>) -> Result<Self> {
        sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        cuda.bind_to_thread()?;
        let mut handle: sys::cufftHandle = 0;
        // SAFETY: `cuda` is bound; `handle` is a valid out-pointer.
        check("cufftCreate", unsafe { sys::cufftCreate(&mut handle) })?;
        let guard = Self {
            handle,
            cuda: cuda.clone(),
        };
        // Disable cuFFT's hidden workspace allocation so all workspace comes from `Scratch`.
        // SAFETY: `guard.handle` is a freshly created valid plan handle.
        check("cufftSetAutoAllocation", unsafe {
            sys::cufftSetAutoAllocation(guard.handle, 0)
        })?;
        Ok(guard)
    }

    fn into_handle(mut self) -> sys::cufftHandle {
        let h = self.handle;
        self.handle = 0;
        h
    }
}

impl Drop for RawHandle {
    fn drop(&mut self) {
        if self.handle != 0 {
            // A failed bind leaves its driver error on `self.cuda` and skips the native call.
            if cuda_core_ext::teardown::bind(&self.cuda) {
                // SAFETY: `self.handle` was created by `cufftCreate` and has not been transferred.
                let code = unsafe { sys::cufftDestroy(self.handle) };
                teardown(code, "cufftDestroy");
            }
            self.handle = 0;
        }
    }
}

/// Owning cuFFT plan handle (`cufftHandle`).
pub struct Plan {
    handle: sys::cufftHandle,
    cuda: Arc<Context>,
    fft_type: FftType,
    work_size: usize,
    min_input_elems: usize,
    min_output_elems: usize,
    workspace: Option<Arc<Shared>>,
    last_generation: AtomicU64,
    last_ws_ptr: AtomicUsize,
    /// Makes `Plan` `!Sync`: cuFFT allows concurrent use only of different plans, and each exec
    /// is a non-atomic `cufftSetWorkArea` + `cufftSetStream` + `cufftExec*` sequence on `&self`.
    _not_sync: PhantomData<Cell<()>>,
}

impl std::fmt::Debug for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plan")
            .field("handle", &self.handle)
            .field("fft_type", &self.fft_type)
            .field("work_size", &self.work_size)
            .field("min_input_elems", &self.min_input_elems)
            .field("min_output_elems", &self.min_output_elems)
            .finish()
    }
}

// SAFETY: `cufftHandle` has no thread-local state and every call rebinds `cuda` on the calling
// thread, so moving ownership is safe; concurrent use is excluded by `_not_sync` (`!Sync`).
unsafe impl Send for Plan {}

impl Plan {
    /// Estimates workspace bytes for a 1D transform without creating a plan.
    pub fn estimate_1d(nx: i32, fft_type: FftType, batch: i32) -> Result<usize> {
        let _ = dense_io_elems(&[nx], batch, fft_type)?;
        sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut work_size = 0usize;
        // SAFETY: `work_size` is a valid out-pointer.
        check("cufftEstimate1d", unsafe {
            sys::cufftEstimate1d(nx, fft_type.raw(), batch, &mut work_size)
        })?;
        Ok(work_size)
    }

    /// Estimates workspace bytes for a 2D transform without creating a plan.
    pub fn estimate_2d(nx: i32, ny: i32, fft_type: FftType) -> Result<usize> {
        let _ = dense_io_elems(&[nx, ny], 1, fft_type)?;
        sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut work_size = 0usize;
        // SAFETY: `work_size` is a valid out-pointer.
        check("cufftEstimate2d", unsafe {
            sys::cufftEstimate2d(nx, ny, fft_type.raw(), &mut work_size)
        })?;
        Ok(work_size)
    }

    /// Estimates workspace bytes for a 3D transform without creating a plan.
    pub fn estimate_3d(nx: i32, ny: i32, nz: i32, fft_type: FftType) -> Result<usize> {
        let _ = dense_io_elems(&[nx, ny, nz], 1, fft_type)?;
        sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut work_size = 0usize;
        // SAFETY: `work_size` is a valid out-pointer.
        check("cufftEstimate3d", unsafe {
            sys::cufftEstimate3d(nx, ny, nz, fft_type.raw(), &mut work_size)
        })?;
        Ok(work_size)
    }

    fn finish_build(
        guard: RawHandle,
        fft_type: FftType,
        work_size: usize,
        min_input_elems: usize,
        min_output_elems: usize,
        budget: usize,
        scratch: &mut Scratch,
    ) -> Result<Self> {
        let stream = scratch.stream().clone();
        if !same_context(stream.context(), &guard.cuda) {
            return Err(Error::Invalid(
                "scratch stream belongs to a different CUDA context than the plan",
            ));
        }
        let (ws_ptr, ws_gen) = scratch.reserve(&stream, work_size, budget)?;
        let handle = guard.handle;
        if work_size > 0 {
            guard.cuda.bind_to_thread()?;
            // SAFETY: `handle` is live with auto-allocation disabled; `ws_ptr` has `work_size` bytes.
            check("cufftSetWorkArea", unsafe {
                sys::cufftSetWorkArea(handle, ws_ptr as usize as *mut c_void)
            })?;
        }
        let cuda = guard.cuda.clone();
        let handle = guard.into_handle();
        let (workspace, ws_gen, ptr) = if work_size > 0 {
            (Some(scratch.inner.clone()), ws_gen, ws_ptr as usize)
        } else {
            (None, 0, 0)
        };
        Ok(Self {
            handle,
            cuda,
            fft_type,
            work_size,
            min_input_elems,
            min_output_elems,
            workspace,
            last_generation: AtomicU64::new(ws_gen),
            last_ws_ptr: AtomicUsize::new(ptr),
            _not_sync: PhantomData,
        })
    }

    /// Creates a 1D cuFFT plan (`cufftMakePlan1d`) with caller-owned [`Scratch`] admitted against `budget`.
    pub fn plan_1d(
        cuda: &Arc<Context>,
        nx: i32,
        fft_type: FftType,
        batch: i32,
        budget: usize,
        scratch: &mut Scratch,
    ) -> Result<Self> {
        let (min_in, min_out) = dense_io_elems(&[nx], batch, fft_type)?;
        let guard = RawHandle::create(cuda)?;
        let mut work_size = 0usize;
        // SAFETY: `guard.handle` is live and `work_size` is a valid out-pointer.
        check("cufftMakePlan1d", unsafe {
            sys::cufftMakePlan1d(guard.handle, nx, fft_type.raw(), batch, &mut work_size)
        })?;
        Self::finish_build(guard, fft_type, work_size, min_in, min_out, budget, scratch)
    }

    /// Creates a 2D cuFFT plan (`cufftMakePlan2d`) with caller-owned [`Scratch`] admitted against `budget`.
    pub fn plan_2d(
        cuda: &Arc<Context>,
        nx: i32,
        ny: i32,
        fft_type: FftType,
        budget: usize,
        scratch: &mut Scratch,
    ) -> Result<Self> {
        let (min_in, min_out) = dense_io_elems(&[nx, ny], 1, fft_type)?;
        let guard = RawHandle::create(cuda)?;
        let mut work_size = 0usize;
        // SAFETY: `guard.handle` is live and `work_size` is a valid out-pointer.
        check("cufftMakePlan2d", unsafe {
            sys::cufftMakePlan2d(guard.handle, nx, ny, fft_type.raw(), &mut work_size)
        })?;
        Self::finish_build(guard, fft_type, work_size, min_in, min_out, budget, scratch)
    }

    /// Creates a 3D cuFFT plan (`cufftMakePlan3d`) with caller-owned [`Scratch`] admitted against `budget`.
    pub fn plan_3d(
        cuda: &Arc<Context>,
        nx: i32,
        ny: i32,
        nz: i32,
        fft_type: FftType,
        budget: usize,
        scratch: &mut Scratch,
    ) -> Result<Self> {
        let (min_in, min_out) = dense_io_elems(&[nx, ny, nz], 1, fft_type)?;
        let guard = RawHandle::create(cuda)?;
        let mut work_size = 0usize;
        // SAFETY: `guard.handle` is live and `work_size` is a valid out-pointer.
        check("cufftMakePlan3d", unsafe {
            sys::cufftMakePlan3d(guard.handle, nx, ny, nz, fft_type.raw(), &mut work_size)
        })?;
        Self::finish_build(guard, fft_type, work_size, min_in, min_out, budget, scratch)
    }

    /// Creates a general batched 1D/2D/3D cuFFT plan (`cufftMakePlanMany`) with zero heap
    /// allocations.
    pub fn plan_many(
        cuda: &Arc<Context>,
        desc: &ManyDesc,
        budget: usize,
        scratch: &mut Scratch,
    ) -> Result<Self> {
        let (min_in, min_out) = desc.min_io_elems()?;
        let guard = RawHandle::create(cuda)?;
        let mut n = desc.n;
        let mut inembed = desc.inembed.unwrap_or([0; 3]);
        let mut onembed = desc.onembed.unwrap_or([0; 3]);
        let inembed_ptr = if desc.inembed.is_some() {
            inembed.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };
        let onembed_ptr = if desc.onembed.is_some() {
            onembed.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };
        let mut work_size = 0usize;
        // SAFETY: `n`, `inembed`, `onembed` are stack arrays valid for `desc.rank <= 3`.
        check("cufftMakePlanMany", unsafe {
            sys::cufftMakePlanMany(
                guard.handle,
                desc.rank as i32,
                n.as_mut_ptr(),
                inembed_ptr,
                desc.istride,
                desc.idist,
                onembed_ptr,
                desc.ostride,
                desc.odist,
                desc.fft_type.raw(),
                desc.batch,
                &mut work_size,
            )
        })?;
        Self::finish_build(
            guard,
            desc.fft_type,
            work_size,
            min_in,
            min_out,
            budget,
            scratch,
        )
    }

    /// Returns the raw `cufftHandle`.
    #[inline]
    pub fn raw(&self) -> sys::cufftHandle {
        self.handle
    }

    /// Returns the owning CUDA context.
    #[inline]
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// Returns the transform type this plan was built for.
    #[inline]
    pub fn fft_type(&self) -> FftType {
        self.fft_type
    }

    /// Returns the workspace size in bytes required by this plan.
    #[inline]
    pub fn work_size(&self) -> usize {
        self.work_size
    }

    /// Returns the CUDA [`Stream`] on which this plan's workspace was allocated, or `None` if
    /// the plan requires no workspace (`work_size == 0`).
    #[inline]
    pub fn workspace_stream(&self) -> Option<&Arc<Stream>> {
        self.workspace.as_ref().map(|ws| &ws.stream)
    }

    /// Rebinds this plan's workspace to `scratch` (admitting `self.work_size()` against `budget`).
    pub fn set_work_area(&mut self, scratch: &mut Scratch, budget: usize) -> Result<()> {
        let stream = scratch.stream().clone();
        if !same_context(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "scratch stream belongs to a different CUDA context than the plan",
            ));
        }
        let (ws_ptr, ws_gen) = scratch.reserve(&stream, self.work_size, budget)?;
        if self.work_size > 0 {
            self.cuda.bind_to_thread()?;
            // SAFETY: `self.handle` is live and `ws_ptr` has `self.work_size` device bytes.
            check("cufftSetWorkArea", unsafe {
                sys::cufftSetWorkArea(self.handle, ws_ptr as usize as *mut c_void)
            })?;
            self.workspace = Some(scratch.inner.clone());
            self.last_generation.store(ws_gen, Ordering::Release);
            self.last_ws_ptr.store(ws_ptr as usize, Ordering::Release);
        } else {
            self.workspace = None;
            self.last_generation.store(0, Ordering::Release);
            self.last_ws_ptr.store(0, Ordering::Release);
        }
        Ok(())
    }

    /// Binds `stream` to this plan.
    ///
    /// If the plan requires workspace (`work_size > 0`), `stream` must belong to the same CUDA
    /// context and share the same native stream as the workspace allocation.
    pub fn set_stream(&self, stream: &Stream) -> Result<()> {
        if !same_context(stream.context(), &self.cuda) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than the cuFFT plan",
            ));
        }
        if let Some(ws) = &self.workspace
            && !same_stream(stream, &ws.stream)
        {
            return Err(Error::Invalid(
                "stream does not match cuFFT plan workspace stream",
            ));
        }
        self.cuda.bind_to_thread()?;
        // SAFETY: `self.handle` and `stream.cu_stream()` belong to the same live context.
        check("cufftSetStream", unsafe {
            sys::cufftSetStream(self.handle, stream.cu_stream())
        })
    }

    /// Rebinds the work area if the shared scratch buffer changed and returns the scratch lock.
    ///
    /// The caller must hold the returned guard until `cufftExec*` has been enqueued: while it is
    /// held no clone of the [`Scratch`] can `reserve` (and so drop) the buffer bound to this plan.
    /// Once the exec is queued on the scratch stream, a later drop frees the buffer with
    /// `cuMemFreeAsync` on that same stream, ordered after the exec.
    fn ensure_workspace(
        &self,
        stream: &Stream,
    ) -> Result<Option<MutexGuard<'_, cuda_core_ext::Scratch>>> {
        if self.work_size == 0 {
            return Ok(None);
        }
        let ws = self.workspace.as_ref().ok_or(Error::Invalid(
            "cuFFT plan requires workspace but none is retained",
        ))?;
        if !same_stream(stream, &ws.stream) {
            return Err(Error::Invalid(
                "execution stream does not match cuFFT plan workspace stream",
            ));
        }
        let state = ws.lock();
        let current_gen = state.generation();
        let last_gen = self.last_generation.load(Ordering::Acquire);
        let last_ptr = self.last_ws_ptr.load(Ordering::Acquire);
        let (dptr, cap) = state.as_raw_parts();
        if cap == 0 {
            return Err(Error::Invalid("scratch buffer missing after allocation"));
        }
        if cap < self.work_size {
            return Err(Error::Invalid(
                "scratch buffer capacity smaller than required cuFFT work area",
            ));
        }
        let current_ptr = (dptr as usize) as *mut c_void;
        if current_gen != last_gen || current_ptr as usize != last_ptr {
            self.cuda.bind_to_thread()?;
            // SAFETY: `self.handle` is live and `current_ptr` has at least `self.work_size` device bytes.
            check("cufftSetWorkArea", unsafe {
                sys::cufftSetWorkArea(self.handle, current_ptr)
            })?;
            self.last_generation.store(current_gen, Ordering::Release);
            self.last_ws_ptr
                .store(current_ptr as usize, Ordering::Release);
        }
        Ok(Some(state))
    }

    fn check_exec(&self, expected: FftType, in_len: usize, out_len: usize) -> Result<()> {
        if self.fft_type != expected {
            return Err(Error::Invalid(
                "cuFFT plan transform type does not match execution method",
            ));
        }
        if in_len < self.min_input_elems || out_len < self.min_output_elems {
            return Err(Error::Invalid(
                "device buffer smaller than required cuFFT transform extent",
            ));
        }
        Ok(())
    }

    /// Executes a single-precision complex-to-complex FFT (`cufftExecC2C`).
    ///
    /// [`Direction::Inverse`] is unnormalized: the output is scaled by the transform size.
    pub fn exec_c2c(
        &self,
        stream: &Stream,
        input: &DeviceBuffer<Complex32>,
        output: &mut DeviceBuffer<Complex32>,
        direction: Direction,
    ) -> Result<()> {
        self.check_exec(FftType::C2C, input.len(), output.len())?;
        let _workspace = self.ensure_workspace(stream)?;
        self.set_stream(stream)?;
        // SAFETY: `cufftComplex` matches `Complex32` layout; cuFFT reads `input` without mutating it; stream and workspace are bound.
        check("cufftExecC2C", unsafe {
            sys::cufftExecC2C(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftComplex,
                output.cu_deviceptr() as usize as *mut sys::cufftComplex,
                direction.raw(),
            )
        })
    }

    /// Executes a single-precision real-to-complex FFT (`cufftExecR2C`).
    pub fn exec_r2c(
        &self,
        stream: &Stream,
        input: &DeviceBuffer<f32>,
        output: &mut DeviceBuffer<Complex32>,
    ) -> Result<()> {
        self.check_exec(FftType::R2C, input.len(), output.len())?;
        let _workspace = self.ensure_workspace(stream)?;
        self.set_stream(stream)?;
        // SAFETY: buffer extents and transform type are validated above; stream and workspace are bound.
        check("cufftExecR2C", unsafe {
            sys::cufftExecR2C(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftReal,
                output.cu_deviceptr() as usize as *mut sys::cufftComplex,
            )
        })
    }

    /// Executes a single-precision complex-to-real inverse FFT (`cufftExecC2R`).
    ///
    /// Unnormalized: the output is scaled by the transform size (the real-domain element count of
    /// one transform).
    ///
    /// # Note on Input Mutation
    /// Per NVIDIA cuFFT documentation, out-of-place C2R transforms overwrite the input buffer
    /// during execution. `input` is taken as `&mut DeviceBuffer<Complex32>` to enforce exclusive
    /// access and document destructive execution in the type system.
    pub fn exec_c2r(
        &self,
        stream: &Stream,
        input: &mut DeviceBuffer<Complex32>,
        output: &mut DeviceBuffer<f32>,
    ) -> Result<()> {
        self.check_exec(FftType::C2R, input.len(), output.len())?;
        let _workspace = self.ensure_workspace(stream)?;
        self.set_stream(stream)?;
        // SAFETY: buffer extents and transform type are validated above; stream and workspace are bound;
        // `input` is exclusively borrowed and mutated per cuFFT C2R specification.
        check("cufftExecC2R", unsafe {
            sys::cufftExecC2R(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftComplex,
                output.cu_deviceptr() as usize as *mut sys::cufftReal,
            )
        })
    }

    /// Executes a double-precision complex-to-complex FFT (`cufftExecZ2Z`).
    ///
    /// [`Direction::Inverse`] is unnormalized: the output is scaled by the transform size.
    pub fn exec_z2z(
        &self,
        stream: &Stream,
        input: &DeviceBuffer<Complex64>,
        output: &mut DeviceBuffer<Complex64>,
        direction: Direction,
    ) -> Result<()> {
        self.check_exec(FftType::Z2Z, input.len(), output.len())?;
        let _workspace = self.ensure_workspace(stream)?;
        self.set_stream(stream)?;
        // SAFETY: `cufftDoubleComplex` matches `Complex64` layout; stream and workspace are bound.
        check("cufftExecZ2Z", unsafe {
            sys::cufftExecZ2Z(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftDoubleComplex,
                output.cu_deviceptr() as usize as *mut sys::cufftDoubleComplex,
                direction.raw(),
            )
        })
    }

    /// Executes a double-precision real-to-complex FFT (`cufftExecD2Z`).
    pub fn exec_d2z(
        &self,
        stream: &Stream,
        input: &DeviceBuffer<f64>,
        output: &mut DeviceBuffer<Complex64>,
    ) -> Result<()> {
        self.check_exec(FftType::D2Z, input.len(), output.len())?;
        let _workspace = self.ensure_workspace(stream)?;
        self.set_stream(stream)?;
        // SAFETY: buffer extents and transform type are validated above; stream and workspace are bound.
        check("cufftExecD2Z", unsafe {
            sys::cufftExecD2Z(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftDoubleReal,
                output.cu_deviceptr() as usize as *mut sys::cufftDoubleComplex,
            )
        })
    }

    /// Executes a double-precision complex-to-real inverse FFT (`cufftExecZ2D`).
    ///
    /// Unnormalized: the output is scaled by the transform size (the real-domain element count of
    /// one transform).
    ///
    /// # Note on Input Mutation
    /// Per NVIDIA cuFFT documentation, out-of-place Z2D transforms overwrite the input buffer
    /// during execution. `input` is taken as `&mut DeviceBuffer<Complex64>` to enforce exclusive
    /// access and document destructive execution in the type system.
    pub fn exec_z2d(
        &self,
        stream: &Stream,
        input: &mut DeviceBuffer<Complex64>,
        output: &mut DeviceBuffer<f64>,
    ) -> Result<()> {
        self.check_exec(FftType::Z2D, input.len(), output.len())?;
        let _workspace = self.ensure_workspace(stream)?;
        self.set_stream(stream)?;
        // SAFETY: buffer extents and transform type are validated above; stream and workspace are bound;
        // `input` is exclusively borrowed and mutated per cuFFT Z2D specification.
        check("cufftExecZ2D", unsafe {
            sys::cufftExecZ2D(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftDoubleComplex,
                output.cu_deviceptr() as usize as *mut sys::cufftDoubleReal,
            )
        })
    }
}

impl Drop for Plan {
    fn drop(&mut self) {
        if self.handle != 0 {
            // A failed bind leaves its driver error on `self.cuda` and skips the native call.
            if cuda_core_ext::teardown::bind(&self.cuda) {
                // SAFETY: `self.handle` was created by `cufftCreate` and is destroyed once.
                let code = unsafe { sys::cufftDestroy(self.handle) };
                teardown(code, "cufftDestroy");
            }
            self.handle = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::FftResult;

    #[test]
    fn many_desc_null_embed_batch_layout_normalization() {
        // FFI-2 reproduction case: rank 1, n=[8, 1, 1], inembed=None, onembed=None, batch=2
        // Default contiguous layout requires 16 elements (2 * 8), not 9 ((8-1)*1 + 1 + (2-1)*1).
        let desc_c2c = ManyDesc {
            rank: 1,
            n: [8, 1, 1],
            inembed: None,
            istride: 1,
            idist: 1,
            onembed: None,
            ostride: 1,
            odist: 1,
            fft_type: FftType::C2C,
            batch: 2,
        };
        assert_eq!(desc_c2c.min_io_elems().unwrap(), (16, 16));

        // When explicit embeds are provided, custom strides/distances are respected; overlapping
        // input batches (idist = 1) are allowed, output batches must be disjoint (odist = 8).
        let desc_c2c_explicit = ManyDesc {
            inembed: Some([8, 1, 1]),
            onembed: Some([8, 1, 1]),
            odist: 8,
            ..desc_c2c
        };
        assert_eq!(desc_c2c_explicit.min_io_elems().unwrap(), (9, 16));
    }

    #[test]
    fn many_desc_rejects_mixed_embeds() {
        // FFT-03: cuFFT ignores every advanced layout parameter when either embed is NULL, so the
        // explicit side would be laid out densely (16 output elements) while the wrapper sized it
        // from the explicit embed (9 elements).
        let desc = ManyDesc {
            rank: 1,
            n: [8, 1, 1],
            inembed: None,
            istride: 1,
            idist: 1,
            onembed: Some([8, 1, 1]),
            ostride: 1,
            odist: 1,
            fft_type: FftType::C2C,
            batch: 2,
        };
        assert!(matches!(desc.validate(), Err(Error::Invalid(_))));
        assert!(matches!(desc.min_io_elems(), Err(Error::Invalid(_))));

        let desc_out_null = ManyDesc {
            inembed: Some([8, 1, 1]),
            onembed: None,
            ..desc
        };
        assert!(matches!(desc_out_null.validate(), Err(Error::Invalid(_))));
        assert!(matches!(
            desc_out_null.min_io_elems(),
            Err(Error::Invalid(_))
        ));
    }

    #[test]
    fn many_desc_rejects_overlapping_output_batches() {
        // FFT-04: odist = 1 with ostride = 1 makes 4 transforms of 1024 points write overlapping
        // output ranges concurrently.
        let overlapping = ManyDesc {
            rank: 1,
            n: [1024, 1, 1],
            inembed: Some([1024, 1, 1]),
            istride: 1,
            idist: 1024,
            onembed: Some([1024, 1, 1]),
            ostride: 1,
            odist: 1,
            fft_type: FftType::C2C,
            batch: 4,
        };
        assert!(matches!(
            overlapping.min_io_elems(),
            Err(Error::Invalid(_))
        ));

        // Disjoint blocks: odist equals one transform's output span.
        let blocks = ManyDesc {
            odist: 1024,
            ..overlapping
        };
        assert_eq!(blocks.min_io_elems().unwrap(), (4096, 4096));

        // Interleaved: batch b writes offsets b + 4 * k, (batch - 1) * odist = 3 < ostride = 4.
        let interleaved = ManyDesc {
            ostride: 4,
            odist: 1,
            ..overlapping
        };
        assert_eq!(interleaved.min_io_elems().unwrap(), (4096, 4096));

        // One batch too many for the interleave: batch 4 would land on batch 0's element 1.
        let interleaved_overflow = ManyDesc {
            batch: 5,
            ..interleaved
        };
        assert!(matches!(
            interleaved_overflow.min_io_elems(),
            Err(Error::Invalid(_))
        ));

        // R2C 2D: one output plane spans (16 - 1) * 17 + (17 - 1) + 1 = 272 complex elements.
        let r2c = ManyDesc {
            rank: 2,
            n: [16, 32, 1],
            inembed: Some([16, 32, 1]),
            istride: 1,
            idist: 512,
            onembed: Some([16, 17, 1]),
            ostride: 1,
            odist: 271,
            fft_type: FftType::R2C,
            batch: 2,
        };
        assert!(matches!(r2c.min_io_elems(), Err(Error::Invalid(_))));
        let r2c_ok = ManyDesc { odist: 272, ..r2c };
        assert_eq!(r2c_ok.min_io_elems().unwrap(), (1024, 544));
    }

    #[test]
    fn plan_is_send_but_not_sync() {
        fn assert_send<T: Send>() {}
        assert_send::<Plan>();

        // FFT-01 compile-time proof that `Plan` is `!Sync`: if `Plan: Sync`, both impls apply,
        // the `_` below is ambiguous and this test module fails to compile.
        trait AmbiguousIfSync<A> {
            fn check() {}
        }
        impl<T: ?Sized> AmbiguousIfSync<()> for T {}
        struct IsSync;
        impl<T: ?Sized + Sync> AmbiguousIfSync<IsSync> for T {}
        <Plan as AmbiguousIfSync<_>>::check();
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cufft64_12.dll; run via xtask gpu-test"]
    fn gpu_exec_keeps_scratch_bound_while_another_thread_grows_it() {
        // FFT-02: another thread holding a `Scratch` clone grows it (dropping the buffer bound to
        // `plan`) while `plan` executes. Probabilistic: run it under
        // `compute-sanitizer --tool memcheck`, which reports the use-after-free directly.
        let ctx = Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        let mut scratch = Scratch::new(&stream);
        // The smallest prime length that needs a work area: cuFFT 12 on sm_86 runs length 1009
        // without one, so the length is probed instead of assumed.
        let (n, plan) = [1_009, 2_003, 4_001, 8_009]
            .into_iter()
            .map(|n| {
                let plan = Plan::plan_1d(&ctx, n, FftType::C2C, 1, 1 << 30, &mut scratch).unwrap();
                (n, plan)
            })
            .find(|(_, plan)| plan.work_size() > 0)
            .expect("no probed length needs a work area; test is vacuous");

        let host = vec![Complex32::new(1.0, 0.0); n as usize];
        let input = DeviceBuffer::from_host(&stream, &host).unwrap();
        let mut output = DeviceBuffer::<Complex32>::zeroed(&stream, n as usize).unwrap();

        let grower = {
            let ctx = ctx.clone();
            let mut scratch = scratch.clone();
            std::thread::spawn(move || {
                // Growing lengths with large prime factors need ever larger work areas, so each
                // plan larger than the current reservation makes `reserve` drop and replace the
                // shared buffer.
                for n in [
                    2_003, 4_001, 8_009, 16_001, 32_003, 64_007, 128_021, 256_019, 524_287,
                    1_048_573,
                ] {
                    drop(Plan::plan_1d(&ctx, n, FftType::C2C, 1, 1 << 30, &mut scratch).unwrap());
                }
            })
        };
        for _ in 0..500 {
            plan.exec_c2c(&stream, &input, &mut output, Direction::Forward)
                .unwrap();
        }
        grower.join().expect("grower thread");

        // Forward transform of a constant 1: bin 0 = n, every other bin = 0 (f32 rounding of an
        // n-point Bluestein transform stays far below 1e-3 * n).
        let tol = 1e-3 * n as f32;
        let out = output.to_host_vec(&stream).unwrap();
        assert!((out[0].re() - n as f32).abs() < tol, "bin 0 = {:?}", out[0]);
        for (k, c) in out.iter().enumerate().skip(1) {
            assert!(c.re().abs() < tol && c.im().abs() < tol, "bin {k} = {c:?}");
        }
    }

    /// Raw `cufftMakePlanMany` status for rank 1, `n = 8`, batch 2, C2C, `istride = 1`,
    /// `idist = 8`, `ostride = 2`, `odist = 1` (interleaved output) with the given embeds.
    fn plan_many_status(
        ctx: &Arc<Context>,
        inembed: Option<[i32; 3]>,
        onembed: Option<[i32; 3]>,
    ) -> sys::cufftResult {
        let guard = RawHandle::create(ctx).unwrap();
        let mut n = [8i32, 1, 1];
        let mut in_e = inembed.unwrap_or([0; 3]);
        let mut out_e = onembed.unwrap_or([0; 3]);
        let in_ptr = if inembed.is_some() {
            in_e.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };
        let out_ptr = if onembed.is_some() {
            out_e.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };
        let mut work_size = 0usize;
        // SAFETY: `guard.handle` is live; `n` and the embeds are stack arrays valid for rank 1;
        // `cufftMakePlanMany` accepts NULL embeds.
        unsafe {
            sys::cufftMakePlanMany(
                guard.handle,
                1,
                n.as_mut_ptr(),
                in_ptr,
                1,
                8,
                out_ptr,
                2,
                1,
                FftType::C2C.raw(),
                2,
                &mut work_size,
            )
        }
    }

    /// FFT-03 regression of observed native behaviour (cuFFT 12, CUDA 13.3, RTX 3080 Ti): with
    /// the same sizes and strides, cuFFT rejects either mixed NULL/non-NULL embed pair with
    /// `CUFFT_INVALID_VALUE` and accepts both "both set" and "both NULL". The rejection is caused
    /// by the NULL mix itself, so `ManyDesc::validate` rejecting the mix matches cuFFT.
    #[test]
    #[ignore = "GPU: needs CUDA driver + cufft64_12.dll; run via xtask gpu-test"]
    fn gpu_plan_many_rejects_mixed_null_embeds() {
        let ctx = Context::new(0).expect("cuda context");
        let e = Some([8i32, 1, 1]);
        for (name, inembed, onembed, expected) in [
            ("in NULL, out set", None, e, FftResult::INVALID_VALUE),
            ("in set, out NULL", e, None, FftResult::INVALID_VALUE),
            ("both set", e, e, FftResult::SUCCESS),
            ("both NULL", None, None, FftResult::SUCCESS),
        ] {
            let got = FftResult(plan_many_status(&ctx, inembed, onembed));
            assert_eq!(got, expected, "{name}");
        }
    }

    #[test]
    fn many_desc_null_embed_all_transform_types() {
        // R2C: 2D [16, 32], batch=3.
        // Dense input: 3 * 16 * 32 = 1536 real elements.
        // Dense output: 3 * 16 * (32/2 + 1) = 3 * 16 * 17 = 816 complex elements.
        let desc_r2c = ManyDesc {
            rank: 2,
            n: [16, 32, 1],
            inembed: None,
            istride: 1,
            idist: 100, // ignored by cuFFT when inembed is None
            onembed: None,
            ostride: 1,
            odist: 50, // ignored by cuFFT when onembed is None
            fft_type: FftType::R2C,
            batch: 3,
        };
        assert_eq!(desc_r2c.min_io_elems().unwrap(), (1536, 816));

        // C2R: 2D [16, 32], batch=3.
        // Dense input: 816 complex elements.
        // Dense output: 1536 real elements.
        let desc_c2r = ManyDesc {
            fft_type: FftType::C2R,
            ..desc_r2c
        };
        assert_eq!(desc_c2r.min_io_elems().unwrap(), (816, 1536));

        // D2Z: 2D [8, 16], batch=2.
        // Dense input: 2 * 8 * 16 = 256 real elements.
        // Dense output: 2 * 8 * (16/2 + 1) = 2 * 8 * 9 = 144 complex elements.
        let desc_d2z = ManyDesc {
            rank: 2,
            n: [8, 16, 1],
            inembed: None,
            istride: 1,
            idist: 10,
            onembed: None,
            ostride: 1,
            odist: 10,
            fft_type: FftType::D2Z,
            batch: 2,
        };
        assert_eq!(desc_d2z.min_io_elems().unwrap(), (256, 144));

        // Z2D: 2D [8, 16], batch=2.
        let desc_z2d = ManyDesc {
            fft_type: FftType::Z2D,
            ..desc_d2z
        };
        assert_eq!(desc_z2d.min_io_elems().unwrap(), (144, 256));

        // Z2Z: 3D [4, 4, 4], batch=2.
        // Dense: 2 * 4 * 4 * 4 = 128 elements.
        let desc_z2z = ManyDesc {
            rank: 3,
            n: [4, 4, 4],
            inembed: None,
            istride: 1,
            idist: 1,
            onembed: None,
            ostride: 1,
            odist: 1,
            fft_type: FftType::Z2Z,
            batch: 2,
        };
        assert_eq!(desc_z2z.min_io_elems().unwrap(), (128, 128));
    }

    #[test]
    fn complex_types_layout_and_methods() {
        let c32 = Complex32::new(1.5, -2.5);
        assert_eq!(c32.re(), 1.5);
        assert_eq!(c32.im(), -2.5);
        assert_eq!(c32.0, [1.5, -2.5]);

        let c64 = Complex64::new(3.0, -4.0);
        assert_eq!(c64.re(), 3.0);
        assert_eq!(c64.im(), -4.0);
        assert_eq!(c64.0, [3.0, -4.0]);
    }

    #[test]
    fn direction_and_fft_type_raw_codes() {
        assert_eq!(Direction::Forward.raw(), sys::CUFFT_FORWARD);
        assert_eq!(Direction::Inverse.raw(), sys::CUFFT_INVERSE as i32);

        assert_eq!(FftType::R2C.raw(), sys::cufftType_t_CUFFT_R2C);
        assert_eq!(FftType::C2R.raw(), sys::cufftType_t_CUFFT_C2R);
        assert_eq!(FftType::C2C.raw(), sys::cufftType_t_CUFFT_C2C);
        assert_eq!(FftType::D2Z.raw(), sys::cufftType_t_CUFFT_D2Z);
        assert_eq!(FftType::Z2D.raw(), sys::cufftType_t_CUFFT_Z2D);
        assert_eq!(FftType::Z2Z.raw(), sys::cufftType_t_CUFFT_Z2Z);
    }
}
