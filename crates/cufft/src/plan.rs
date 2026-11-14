//! [`Plan`]: 1D, 2D, 3D, and batched (`Many`) cuFFT plans with caller-owned [`Scratch`].
//!
//! **What.** Wraps `cufftHandle` creation (`cufftCreate` + `cufftSetAutoAllocation(0)` +
//! `cufftMakePlan*`), workspace size estimation (`cufftEstimate*` / `cufftGetSize*`), byte-budget
//! enforcement via [`Scratch`], stream binding (`cufftSetStream`), and typed execution for
//! `C2C`, `R2C`, `C2R`, `Z2Z`, `D2Z`, and `Z2D`.
//! **Why.** Prevents hidden driver/cuFFT workspace allocations by enforcing two-phase sizing and
//! explicit byte budgets, and validates transform types and buffer extents before launching.
//! **Used by** `cufft` callers and T0/T3 tests.

use crate::error::{check, teardown};
use crate::{Error, Result, Scratch, sys};
use cuda_core::{Context, DeviceBuffer, DeviceCopy, Stream};
use std::sync::Arc;

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
    /// Optional input storage dimensions; `None` means contiguous `n`.
    pub inembed: Option<[i32; 3]>,
    /// Stride between consecutive input elements.
    pub istride: i32,
    /// Distance between consecutive input batches.
    pub idist: i32,
    /// Optional output storage dimensions; `None` means contiguous.
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
        for i in 0..self.rank {
            if self.n[i] <= 0 {
                return Err(Error::Invalid("ManyDesc transform dimensions must be positive"));
            }
            if let Some(in_e) = self.inembed
                && in_e[i] <= 0
            {
                return Err(Error::Invalid("ManyDesc inembed dimensions must be positive"));
            }
            if let Some(out_e) = self.onembed
                && out_e[i] <= 0
            {
                return Err(Error::Invalid("ManyDesc onembed dimensions must be positive"));
            }
        }
        let req_in = self.req_dims(false);
        let req_out = self.req_dims(true);
        for i in 1..self.rank {
            if let Some(in_e) = self.inembed
                && in_e[i] < req_in[i]
            {
                return Err(Error::Invalid("ManyDesc inembed[i] must be >= required input dim for i >= 1"));
            }
            if let Some(out_e) = self.onembed
                && out_e[i] < req_out[i]
            {
                return Err(Error::Invalid("ManyDesc onembed[i] must be >= required output dim for i >= 1"));
            }
        }
        if self.istride <= 0 || self.ostride <= 0 {
            return Err(Error::Invalid("ManyDesc istride and ostride must be positive"));
        }
        if self.batch <= 0 {
            return Err(Error::Invalid("ManyDesc batch must be positive"));
        }
        if self.batch > 1 && (self.idist <= 0 || self.odist <= 0) {
            return Err(Error::Invalid("ManyDesc idist and odist must be positive when batch > 1"));
        }
        Ok(())
    }

    /// Computes minimum `(input_elems, output_elems)` required by this `ManyDesc` using checked
    /// arithmetic.
    ///
    /// Per `cufftMakePlanMany` semantics, `embed[0]` is ignored (dim 0 uses `req_dim[0]`), and
    /// `embed[1..rank]` give the storage pitch of inner dimensions.
    pub fn min_io_elems(&self) -> Result<(usize, usize)> {
        self.validate()?;
        let req_in = self.req_dims(false);
        let req_out = self.req_dims(true);
        let max_idx_for = |req: [i32; 3], embed: Option<[i32; 3]>| -> Result<usize> {
            let eff_embed = embed.unwrap_or(req);
            let mut idx = (req[0] - 1) as usize;
            for i in 1..self.rank {
                idx = idx
                    .checked_mul(eff_embed[i] as usize)
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
                intra.checked_add(batch_off).ok_or(Error::Invalid("ManyDesc total span overflow"))
            }
        };
        let in_span = span(max_idx_for(req_in, self.inembed)?, self.istride, self.idist)?;
        let out_span = span(max_idx_for(req_out, self.onembed)?, self.ostride, self.odist)?;
        Ok((in_span, out_span))
    }
}

/// Computes minimum `(input_elems, output_elems)` for a contiguous 1D/2D/3D transform.
pub(crate) fn dense_io_elems(dims: &[i32], batch: i32, fft_type: FftType) -> Result<(usize, usize)> {
    if batch <= 0 || dims.is_empty() || dims.iter().any(|&d| d <= 0) {
        return Err(Error::Invalid("FFT dimensions and batch must be positive"));
    }
    let last = *dims.last().ok_or(Error::Invalid("empty FFT dimensions"))? as usize;
    let mut prefix = batch as usize;
    for &d in &dims[..dims.len() - 1] {
        prefix = prefix.checked_mul(d as usize).ok_or(Error::Invalid("FFT element count overflow"))?;
    }
    let real_elems = prefix.checked_mul(last).ok_or(Error::Invalid("FFT real element count overflow"))?;
    let half_complex_elems = prefix.checked_mul(last / 2 + 1).ok_or(Error::Invalid("FFT complex element count overflow"))?;
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
        let guard = Self { handle, cuda: cuda.clone() };
        // Disable cuFFT's hidden workspace allocation so all workspace comes from `Scratch`.
        // SAFETY: `guard.handle` is a freshly created valid plan handle.
        check("cufftSetAutoAllocation", unsafe { sys::cufftSetAutoAllocation(guard.handle, 0) })?;
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
            if self.cuda.bind_to_thread().is_ok() {
                // SAFETY: `self.handle` was created by `cufftCreate` and has not been transferred.
                let code = unsafe { sys::cufftDestroy(self.handle) };
                teardown(&self.cuda, code);
            } else {
                teardown(&self.cuda, sys::cufftResult_t_CUFFT_SETUP_FAILED);
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
}

// SAFETY: `cufftHandle` has no thread-local state; moving ownership is safe, concurrent use is `!Sync`.
unsafe impl Send for Plan {}

impl Plan {
    /// Estimates workspace bytes for a 1D transform without creating a plan.
    pub fn estimate_1d(nx: i32, fft_type: FftType, batch: i32) -> Result<usize> {
        let _ = dense_io_elems(&[nx], batch, fft_type)?;
        sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut work_size = 0usize;
        // SAFETY: `work_size` is a valid out-pointer.
        check("cufftEstimate1d", unsafe { sys::cufftEstimate1d(nx, fft_type.raw(), batch, &mut work_size) })?;
        Ok(work_size)
    }

    /// Estimates workspace bytes for a 2D transform without creating a plan.
    pub fn estimate_2d(nx: i32, ny: i32, fft_type: FftType) -> Result<usize> {
        let _ = dense_io_elems(&[nx, ny], 1, fft_type)?;
        sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut work_size = 0usize;
        // SAFETY: `work_size` is a valid out-pointer.
        check("cufftEstimate2d", unsafe { sys::cufftEstimate2d(nx, ny, fft_type.raw(), &mut work_size) })?;
        Ok(work_size)
    }

    /// Estimates workspace bytes for a 3D transform without creating a plan.
    pub fn estimate_3d(nx: i32, ny: i32, nz: i32, fft_type: FftType) -> Result<usize> {
        let _ = dense_io_elems(&[nx, ny, nz], 1, fft_type)?;
        sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
        let mut work_size = 0usize;
        // SAFETY: `work_size` is a valid out-pointer.
        check("cufftEstimate3d", unsafe { sys::cufftEstimate3d(nx, ny, nz, fft_type.raw(), &mut work_size) })?;
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
        if !Arc::ptr_eq(stream.context(), &guard.cuda) {
            return Err(Error::Invalid("scratch stream belongs to a different CUDA context than the plan"));
        }
        let ws_ptr = scratch.reserve(&stream, work_size, budget)?;
        let handle = guard.handle;
        if work_size > 0 {
            // SAFETY: `handle` is live with auto-allocation disabled; `ws_ptr` has `work_size` bytes.
            check("cufftSetWorkArea", unsafe { sys::cufftSetWorkArea(handle, ws_ptr) })?;
        }
        let cuda = guard.cuda.clone();
        let handle = guard.into_handle();
        Ok(Self { handle, cuda, fft_type, work_size, min_input_elems, min_output_elems })
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
        let inembed_ptr = if desc.inembed.is_some() { inembed.as_mut_ptr() } else { std::ptr::null_mut() };
        let onembed_ptr = if desc.onembed.is_some() { onembed.as_mut_ptr() } else { std::ptr::null_mut() };
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
        Self::finish_build(guard, desc.fft_type, work_size, min_in, min_out, budget, scratch)
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

    /// Rebinds this plan's workspace to `scratch` (admitting `self.work_size()` against `budget`).
    pub fn set_work_area(&mut self, scratch: &mut Scratch, budget: usize) -> Result<()> {
        let stream = scratch.stream().clone();
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid("scratch stream belongs to a different CUDA context than the plan"));
        }
        let ws_ptr = scratch.reserve(&stream, self.work_size, budget)?;
        if self.work_size > 0 {
            self.cuda.bind_to_thread()?;
            // SAFETY: `self.handle` is live and `ws_ptr` has `self.work_size` device bytes.
            check("cufftSetWorkArea", unsafe { sys::cufftSetWorkArea(self.handle, ws_ptr) })?;
        }
        Ok(())
    }

    /// Binds `stream` to this plan.
    pub fn set_stream(&self, stream: &Stream) -> Result<()> {
        if !Arc::ptr_eq(stream.context(), &self.cuda) {
            return Err(Error::Invalid("stream belongs to a different CUDA context than the cuFFT plan"));
        }
        self.cuda.bind_to_thread()?;
        // SAFETY: `self.handle` and `stream.cu_stream()` belong to the same live context.
        check("cufftSetStream", unsafe { sys::cufftSetStream(self.handle, stream.cu_stream()) })
    }

    fn check_exec(&self, expected: FftType, in_len: usize, out_len: usize) -> Result<()> {
        if self.fft_type != expected {
            return Err(Error::Invalid("cuFFT plan transform type does not match execution method"));
        }
        if in_len < self.min_input_elems || out_len < self.min_output_elems {
            return Err(Error::Invalid("device buffer smaller than required cuFFT transform extent"));
        }
        Ok(())
    }

    /// Executes a single-precision complex-to-complex FFT (`cufftExecC2C`).
    pub fn exec_c2c(
        &self,
        stream: &Stream,
        input: &DeviceBuffer<Complex32>,
        output: &mut DeviceBuffer<Complex32>,
        direction: Direction,
    ) -> Result<()> {
        self.check_exec(FftType::C2C, input.len(), output.len())?;
        self.set_stream(stream)?;
        // SAFETY: `cufftComplex` matches `Complex32` layout; cuFFT reads `input` without mutating it.
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
        self.set_stream(stream)?;
        // SAFETY: buffer extents and transform type are validated above; stream is bound.
        check("cufftExecR2C", unsafe {
            sys::cufftExecR2C(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftReal,
                output.cu_deviceptr() as usize as *mut sys::cufftComplex,
            )
        })
    }

    /// Executes a single-precision complex-to-real inverse FFT (`cufftExecC2R`).
    pub fn exec_c2r(
        &self,
        stream: &Stream,
        input: &DeviceBuffer<Complex32>,
        output: &mut DeviceBuffer<f32>,
    ) -> Result<()> {
        self.check_exec(FftType::C2R, input.len(), output.len())?;
        self.set_stream(stream)?;
        // SAFETY: buffer extents and transform type are validated above; stream is bound.
        check("cufftExecC2R", unsafe {
            sys::cufftExecC2R(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftComplex,
                output.cu_deviceptr() as usize as *mut sys::cufftReal,
            )
        })
    }

    /// Executes a double-precision complex-to-complex FFT (`cufftExecZ2Z`).
    pub fn exec_z2z(
        &self,
        stream: &Stream,
        input: &DeviceBuffer<Complex64>,
        output: &mut DeviceBuffer<Complex64>,
        direction: Direction,
    ) -> Result<()> {
        self.check_exec(FftType::Z2Z, input.len(), output.len())?;
        self.set_stream(stream)?;
        // SAFETY: `cufftDoubleComplex` matches `Complex64` layout; stream is bound.
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
        self.set_stream(stream)?;
        // SAFETY: buffer extents and transform type are validated above; stream is bound.
        check("cufftExecD2Z", unsafe {
            sys::cufftExecD2Z(
                self.handle,
                input.cu_deviceptr() as usize as *mut sys::cufftDoubleReal,
                output.cu_deviceptr() as usize as *mut sys::cufftDoubleComplex,
            )
        })
    }

    /// Executes a double-precision complex-to-real inverse FFT (`cufftExecZ2D`).
    pub fn exec_z2d(
        &self,
        stream: &Stream,
        input: &DeviceBuffer<Complex64>,
        output: &mut DeviceBuffer<f64>,
    ) -> Result<()> {
        self.check_exec(FftType::Z2D, input.len(), output.len())?;
        self.set_stream(stream)?;
        // SAFETY: buffer extents and transform type are validated above; stream is bound.
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
            if self.cuda.bind_to_thread().is_ok() {
                // SAFETY: `self.handle` was created by `cufftCreate` and is destroyed once.
                let code = unsafe { sys::cufftDestroy(self.handle) };
                teardown(&self.cuda, code);
            } else {
                teardown(&self.cuda, sys::cufftResult_t_CUFFT_SETUP_FAILED);
            }
            self.handle = 0;
        }
    }
}
