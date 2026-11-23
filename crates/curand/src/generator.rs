//! [`Generator`] and [`PoissonDistribution`]: host and device RNGs, seeds, offsets, ordering,
//! Sobol dimensions, stream binding, and distribution generation.
//!
//! **What.** Wraps `curandGenerator_t` for both device (`curandCreateGenerator`) and host
//! (`curandCreateGeneratorHost`) execution across pseudo-random (`Xorwow`, `Mrg32k3a`, `Mtgp32`,
//! `Mt19937`, `Philox4_32_10`) and quasi-random (`Sobol32`, `ScrambledSobol32`, `Sobol64`,
//! `ScrambledSobol64`) algorithms.
//! **Why.** Enforces host-vs-device, pseudo-vs-quasi, and distribution parameter invariants
//! before calling cuRAND, binds streams on device generators, and performs zero heap allocations
//! except for the completion lease every device `generate_*` takes on its output
//! (`cuda_core_ext::lease`).
//! **Used by** `curand` callers and T0/T3 tests.

use crate::error::{check, ensure_loaded, teardown};
use crate::{Error, Result};
use cuda_core::{Context, DeviceBuffer, Stream};
use cuda_core_ext::{Access, enqueue_leased};
use std::sync::Arc;

/// Random number generator algorithm (`curandRngType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RngType {
    /// Default pseudo-random generator (`CURAND_RNG_PSEUDO_DEFAULT`).
    PseudoDefault,
    /// XORWOW (`CURAND_RNG_PSEUDO_XORWOW`).
    Xorwow,
    /// MRG32k3a (`CURAND_RNG_PSEUDO_MRG32K3A`).
    Mrg32k3a,
    /// Mersenne Twister MTGP32 (`CURAND_RNG_PSEUDO_MTGP32`).
    Mtgp32,
    /// Mersenne Twister MT19937 (`CURAND_RNG_PSEUDO_MT19937`).
    Mt19937,
    /// Philox-4x32-10 (`CURAND_RNG_PSEUDO_PHILOX4_32_10`).
    Philox4_32_10,
    /// Default quasi-random generator (`CURAND_RNG_QUASI_DEFAULT`).
    QuasiDefault,
    /// 32-bit Sobol (`CURAND_RNG_QUASI_SOBOL32`).
    Sobol32,
    /// 32-bit scrambled Sobol (`CURAND_RNG_QUASI_SCRAMBLED_SOBOL32`).
    ScrambledSobol32,
    /// 64-bit Sobol (`CURAND_RNG_QUASI_SOBOL64`).
    Sobol64,
    /// 64-bit scrambled Sobol (`CURAND_RNG_QUASI_SCRAMBLED_SOBOL64`).
    ScrambledSobol64,
}

impl RngType {
    /// Converts to `cuda_bindings::curandRngType_t`.
    #[inline]
    pub const fn raw(self) -> cuda_bindings::curandRngType_t {
        match self {
            Self::PseudoDefault => cuda_bindings::curandRngType_CURAND_RNG_PSEUDO_DEFAULT,
            Self::Xorwow => cuda_bindings::curandRngType_CURAND_RNG_PSEUDO_XORWOW,
            Self::Mrg32k3a => cuda_bindings::curandRngType_CURAND_RNG_PSEUDO_MRG32K3A,
            Self::Mtgp32 => cuda_bindings::curandRngType_CURAND_RNG_PSEUDO_MTGP32,
            Self::Mt19937 => cuda_bindings::curandRngType_CURAND_RNG_PSEUDO_MT19937,
            Self::Philox4_32_10 => cuda_bindings::curandRngType_CURAND_RNG_PSEUDO_PHILOX4_32_10,
            Self::QuasiDefault => cuda_bindings::curandRngType_CURAND_RNG_QUASI_DEFAULT,
            Self::Sobol32 => cuda_bindings::curandRngType_CURAND_RNG_QUASI_SOBOL32,
            Self::ScrambledSobol32 => {
                cuda_bindings::curandRngType_CURAND_RNG_QUASI_SCRAMBLED_SOBOL32
            }
            Self::Sobol64 => cuda_bindings::curandRngType_CURAND_RNG_QUASI_SOBOL64,
            Self::ScrambledSobol64 => {
                cuda_bindings::curandRngType_CURAND_RNG_QUASI_SCRAMBLED_SOBOL64
            }
        }
    }

    /// Returns `true` for quasi-random Sobol generators.
    #[inline]
    pub const fn is_quasi(self) -> bool {
        matches!(
            self,
            Self::QuasiDefault
                | Self::Sobol32
                | Self::ScrambledSobol32
                | Self::Sobol64
                | Self::ScrambledSobol64
        )
    }

    /// Returns `true` for pseudo-random generators.
    #[inline]
    pub const fn is_pseudo(self) -> bool {
        !self.is_quasi()
    }
}

/// Output ordering policy (`curandOrdering_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ordering {
    /// Best performance (`CURAND_ORDERING_PSEUDO_BEST`).
    PseudoBest,
    /// Default pseudo-random ordering (`CURAND_ORDERING_PSEUDO_DEFAULT`).
    PseudoDefault,
    /// Seeded ordering (`CURAND_ORDERING_PSEUDO_SEEDED`).
    PseudoSeeded,
    /// Legacy ordering (`CURAND_ORDERING_PSEUDO_LEGACY`).
    PseudoLegacy,
    /// Dynamic ordering (`CURAND_ORDERING_PSEUDO_DYNAMIC`).
    PseudoDynamic,
    /// Default quasi-random ordering (`CURAND_ORDERING_QUASI_DEFAULT`).
    QuasiDefault,
}

impl Ordering {
    /// Converts to `cuda_bindings::curandOrdering_t`.
    #[inline]
    pub const fn raw(self) -> cuda_bindings::curandOrdering_t {
        match self {
            Self::PseudoBest => cuda_bindings::curandOrdering_CURAND_ORDERING_PSEUDO_BEST,
            Self::PseudoDefault => cuda_bindings::curandOrdering_CURAND_ORDERING_PSEUDO_DEFAULT,
            Self::PseudoSeeded => cuda_bindings::curandOrdering_CURAND_ORDERING_PSEUDO_SEEDED,
            Self::PseudoLegacy => cuda_bindings::curandOrdering_CURAND_ORDERING_PSEUDO_LEGACY,
            Self::PseudoDynamic => cuda_bindings::curandOrdering_CURAND_ORDERING_PSEUDO_DYNAMIC,
            Self::QuasiDefault => cuda_bindings::curandOrdering_CURAND_ORDERING_QUASI_DEFAULT,
        }
    }
}

/// Validates normal/log-normal distribution parameters before calling cuRAND.
pub(crate) fn validate_normal_params(
    rng_type: RngType,
    len: usize,
    stddev_positive: bool,
    finite: bool,
) -> Result<()> {
    if !finite || !stddev_positive {
        return Err(Error::Invalid(
            "normal/log-normal distribution requires finite mean and positive finite stddev",
        ));
    }
    if rng_type.is_pseudo() && !len.is_multiple_of(2) {
        return Err(Error::Invalid(
            "pseudo-random normal/log-normal generation requires an even output length",
        ));
    }
    Ok(())
}

/// Validates Poisson `lambda` before calling cuRAND.
pub(crate) fn validate_poisson_lambda(lambda: f64) -> Result<()> {
    if !lambda.is_finite() || lambda <= 0.0 {
        return Err(Error::Invalid("Poisson lambda must be finite and positive"));
    }
    Ok(())
}

/// Owning Poisson discrete distribution histogram (`curandDiscreteDistribution_t`).
///
/// The histogram lives in device memory (`curand.h`: "pointer to the histogram in device
/// memory"). Nothing in this crate reads it: it is passed through [`Self::raw`] to user CUDA
/// kernels that sample it with the device API `curand_discrete(state, distribution)`
/// (`curand_discrete2.h`). Bulk Poisson generation through the host API
/// ([`Generator::generate_poisson`] / [`Generator::generate_host_poisson`]) does not need this type.
pub struct PoissonDistribution {
    raw: cuda_bindings::curandDiscreteDistribution_t,
    cuda: Option<Arc<Context>>,
}

// SAFETY: `curandDiscreteDistribution_t` is immutable after creation (`!Sync` to be conservative).
unsafe impl Send for PoissonDistribution {}

impl PoissonDistribution {
    /// Precomputes a device-memory Poisson distribution histogram for `lambda > 0.0`.
    ///
    /// No context is bound: cuRAND allocates the histogram in whatever CUDA context is current on
    /// the calling thread (or the CUDA runtime's primary context of the current device when none
    /// is), and `Drop` frees it in whatever context is current on the dropping thread, writing a
    /// failure to stderr. Prefer [`Self::new_on`], which binds `cuda` for creation and destruction
    /// and keeps a teardown failure in the `cuda_core_ext::teardown` sink.
    pub fn new(lambda: f64) -> Result<Self> {
        validate_poisson_lambda(lambda)?;
        ensure_loaded()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("curandCreatePoissonDistribution", unsafe {
            cuda_bindings::curandCreatePoissonDistribution(lambda, &mut raw)
        })?;
        Ok(Self { raw, cuda: None })
    }

    /// Precomputes a Poisson distribution histogram on `cuda`; `Drop` binds `cuda` and keeps a
    /// failed `curandDestroyDistribution` in the `cuda_core_ext::teardown` sink.
    pub fn new_on(cuda: &Arc<Context>, lambda: f64) -> Result<Self> {
        validate_poisson_lambda(lambda)?;
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `cuda` is bound; `raw` is a valid out-pointer.
        check("curandCreatePoissonDistribution", unsafe {
            cuda_bindings::curandCreatePoissonDistribution(lambda, &mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: Some(cuda.clone()),
        })
    }

    /// Explicitly destroys the distribution histogram and returns any cuRAND error.
    pub fn destroy(mut self) -> Result<()> {
        let raw = self.raw;
        self.raw = std::ptr::null_mut();
        if raw.is_null() {
            return Ok(());
        }
        if let Some(ctx) = &self.cuda {
            ctx.bind_to_thread()?;
        }
        // SAFETY: `raw` was created by `curandCreatePoissonDistribution` and is destroyed once.
        check("curandDestroyDistribution", unsafe {
            cuda_bindings::curandDestroyDistribution(raw)
        })
    }

    /// Returns the raw `curandDiscreteDistribution_t`.
    #[inline]
    pub fn raw(&self) -> cuda_bindings::curandDiscreteDistribution_t {
        self.raw
    }
}

impl Drop for PoissonDistribution {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            if let Some(ctx) = &self.cuda {
                // A failed bind leaves its driver error on `ctx` and skips the native call.
                if cuda_core_ext::teardown::bind(ctx) {
                    // SAFETY: `self.raw` was created by `curandCreatePoissonDistribution` and is destroyed once.
                    let code = unsafe { cuda_bindings::curandDestroyDistribution(self.raw) };
                    teardown(code, "curandDestroyDistribution");
                }
            } else {
                // SAFETY: `self.raw` was created by `curandCreatePoissonDistribution` and is destroyed once.
                let code = unsafe { cuda_bindings::curandDestroyDistribution(self.raw) };
                teardown(code, "curandDestroyDistribution");
            }
            self.raw = std::ptr::null_mut();
        }
    }
}

/// Owning cuRAND generator (`curandGenerator_t`), either device-side or host-side.
pub struct Generator {
    raw: cuda_bindings::curandGenerator_t,
    rng_type: RngType,
    cuda: Option<Arc<Context>>,
}

// SAFETY: `curandGenerator_t` has no thread-local state; moving ownership is safe (`!Sync`).
unsafe impl Send for Generator {}

impl Generator {
    /// Creates a device-side cuRAND generator on `cuda`.
    pub fn new_device(cuda: &Arc<Context>, rng_type: RngType) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `cuda` is bound; `raw` is a valid out-pointer.
        check("curandCreateGenerator", unsafe {
            cuda_bindings::curandCreateGenerator(&mut raw, rng_type.raw())
        })?;
        Ok(Self {
            raw,
            rng_type,
            cuda: Some(cuda.clone()),
        })
    }

    /// Creates a host-side cuRAND generator.
    pub fn new_host(rng_type: RngType) -> Result<Self> {
        ensure_loaded()?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("curandCreateGeneratorHost", unsafe {
            cuda_bindings::curandCreateGeneratorHost(&mut raw, rng_type.raw())
        })?;
        Ok(Self {
            raw,
            rng_type,
            cuda: None,
        })
    }

    /// Returns the raw `curandGenerator_t`.
    #[inline]
    pub fn raw(&self) -> cuda_bindings::curandGenerator_t {
        self.raw
    }

    /// Returns the generator algorithm kind.
    #[inline]
    pub fn rng_type(&self) -> RngType {
        self.rng_type
    }

    /// Returns `true` if this is a host-side generator.
    #[inline]
    pub fn is_host(&self) -> bool {
        self.cuda.is_none()
    }

    /// Binds `stream` to a device-side generator. Fails on host generators or mismatched contexts.
    pub fn set_stream(&self, stream: &Stream) -> Result<()> {
        let Some(ctx) = &self.cuda else {
            return Err(Error::Invalid(
                "cannot bind a CUDA stream to a host cuRAND generator",
            ));
        };
        if !cuda_core_ext::same_context(stream.context(), ctx) {
            return Err(Error::Invalid(
                "stream belongs to a different CUDA context than the cuRAND generator",
            ));
        }
        ctx.bind_to_thread()?;
        // SAFETY: `self.raw` and `stream.cu_stream()` belong to the same live context.
        check("curandSetStream", unsafe {
            cuda_bindings::curandSetStream(self.raw, stream.cu_stream())
        })
    }

    /// Sets the seed for a pseudo-random generator.
    pub fn set_seed(&self, seed: u64) -> Result<()> {
        if self.rng_type.is_quasi() {
            return Err(Error::Invalid(
                "quasi-random Sobol generators do not take a seed",
            ));
        }
        if let Some(ctx) = &self.cuda {
            ctx.bind_to_thread()?;
        }
        // SAFETY: `self.raw` is a live pseudo-random generator.
        check("curandSetPseudoRandomGeneratorSeed", unsafe {
            cuda_bindings::curandSetPseudoRandomGeneratorSeed(self.raw, seed)
        })
    }

    /// Sets the sequence offset.
    pub fn set_offset(&self, offset: u64) -> Result<()> {
        if let Some(ctx) = &self.cuda {
            ctx.bind_to_thread()?;
        }
        // SAFETY: `self.raw` is a live generator.
        check("curandSetGeneratorOffset", unsafe {
            cuda_bindings::curandSetGeneratorOffset(self.raw, offset)
        })
    }

    /// Sets the output ordering policy.
    pub fn set_ordering(&self, ordering: Ordering) -> Result<()> {
        if let Some(ctx) = &self.cuda {
            ctx.bind_to_thread()?;
        }
        // SAFETY: `self.raw` is a live generator.
        check("curandSetGeneratorOrdering", unsafe {
            cuda_bindings::curandSetGeneratorOrdering(self.raw, ordering.raw())
        })
    }

    /// Sets the number of dimensions (`>= 1`) for a quasi-random Sobol generator.
    pub fn set_dimensions(&self, num_dimensions: u32) -> Result<()> {
        if !self.rng_type.is_quasi() {
            return Err(Error::Invalid(
                "dimensions only apply to quasi-random Sobol generators",
            ));
        }
        if num_dimensions == 0 {
            return Err(Error::Invalid("quasi-random dimensions must be at least 1"));
        }
        if let Some(ctx) = &self.cuda {
            ctx.bind_to_thread()?;
        }
        // SAFETY: `self.raw` is a live quasi-random generator.
        check("curandSetQuasiRandomGeneratorDimensions", unsafe {
            cuda_bindings::curandSetQuasiRandomGeneratorDimensions(self.raw, num_dimensions)
        })
    }

    /// Forces generation of starting seeds on the generator (`curandGenerateSeeds`).
    pub fn generate_seeds(&self) -> Result<()> {
        if let Some(ctx) = &self.cuda {
            ctx.bind_to_thread()?;
        }
        // SAFETY: `self.raw` is a live generator.
        check("curandGenerateSeeds", unsafe {
            cuda_bindings::curandGenerateSeeds(self.raw)
        })
    }

    fn prepare_device(&self, stream: &Stream) -> Result<()> {
        if self.cuda.is_none() {
            return Err(Error::Invalid(
                "device generation called on a host cuRAND generator",
            ));
        }
        self.set_stream(stream)
    }

    fn ensure_host(&self) -> Result<()> {
        if self.cuda.is_some() {
            return Err(Error::Invalid(
                "host generation called on a device cuRAND generator",
            ));
        }
        Ok(())
    }

    // ---- Device generation ----

    /// Generates `out.len()` uniformly distributed 32-bit unsigned integers on device.
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_u32(&self, stream: &Stream, out: &mut DeviceBuffer<u32>) -> Result<()> {
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: `self.raw` is bound to `stream`; `out` is a live device buffer of
            // `out.len()` u32s in the stream's context (checked by the lease), and the write
            // lease keeps it allocated until the generation queued on `stream` completes.
            check("curandGenerate", unsafe {
                cuda_bindings::curandGenerate(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut u32,
                    out.len(),
                )
            })
        })
    }

    /// Generates `out.len()` uniformly distributed 64-bit unsigned integers on device (Sobol64).
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_u64(&self, stream: &Stream, out: &mut DeviceBuffer<u64>) -> Result<()> {
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: `self.raw` is bound to `stream`; `out` is a live device buffer of
            // `out.len()` u64s in the stream's context (checked by the lease), and the write
            // lease keeps it allocated until the generation queued on `stream` completes.
            check("curandGenerateLongLong", unsafe {
                cuda_bindings::curandGenerateLongLong(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut u64,
                    out.len(),
                )
            })
        })
    }

    /// Generates `out.len()` uniform `f32` values in `(0.0, 1.0]` on device.
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_uniform_f32(&self, stream: &Stream, out: &mut DeviceBuffer<f32>) -> Result<()> {
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: `self.raw` is bound to `stream`; `out` is a live device buffer of
            // `out.len()` f32s in the stream's context (checked by the lease), and the write
            // lease keeps it allocated until the generation queued on `stream` completes.
            check("curandGenerateUniform", unsafe {
                cuda_bindings::curandGenerateUniform(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut f32,
                    out.len(),
                )
            })
        })
    }

    /// Generates `out.len()` uniform `f64` values in `(0.0, 1.0]` on device.
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_uniform_f64(&self, stream: &Stream, out: &mut DeviceBuffer<f64>) -> Result<()> {
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: `self.raw` is bound to `stream`; `out` is a live device buffer of
            // `out.len()` f64s in the stream's context (checked by the lease), and the write
            // lease keeps it allocated until the generation queued on `stream` completes.
            check("curandGenerateUniformDouble", unsafe {
                cuda_bindings::curandGenerateUniformDouble(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut f64,
                    out.len(),
                )
            })
        })
    }

    /// Generates `out.len()` normally distributed `f32` values on device.
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_normal_f32(
        &self,
        stream: &Stream,
        out: &mut DeviceBuffer<f32>,
        mean: f32,
        stddev: f32,
    ) -> Result<()> {
        validate_normal_params(
            self.rng_type,
            out.len(),
            stddev > 0.0,
            mean.is_finite() && stddev.is_finite(),
        )?;
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: parameters and even length (for pseudo-random) are validated above; `out`
            // is in the stream's context (checked by the lease), and the write lease keeps it
            // allocated until the generation queued on `stream` completes.
            check("curandGenerateNormal", unsafe {
                cuda_bindings::curandGenerateNormal(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut f32,
                    out.len(),
                    mean,
                    stddev,
                )
            })
        })
    }

    /// Generates `out.len()` normally distributed `f64` values on device.
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_normal_f64(
        &self,
        stream: &Stream,
        out: &mut DeviceBuffer<f64>,
        mean: f64,
        stddev: f64,
    ) -> Result<()> {
        validate_normal_params(
            self.rng_type,
            out.len(),
            stddev > 0.0,
            mean.is_finite() && stddev.is_finite(),
        )?;
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: parameters and even length (for pseudo-random) are validated above; `out`
            // is in the stream's context (checked by the lease), and the write lease keeps it
            // allocated until the generation queued on `stream` completes.
            check("curandGenerateNormalDouble", unsafe {
                cuda_bindings::curandGenerateNormalDouble(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut f64,
                    out.len(),
                    mean,
                    stddev,
                )
            })
        })
    }

    /// Generates `out.len()` log-normally distributed `f32` values on device.
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_log_normal_f32(
        &self,
        stream: &Stream,
        out: &mut DeviceBuffer<f32>,
        mean: f32,
        stddev: f32,
    ) -> Result<()> {
        validate_normal_params(
            self.rng_type,
            out.len(),
            stddev > 0.0,
            mean.is_finite() && stddev.is_finite(),
        )?;
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: parameters and even length (for pseudo-random) are validated above; `out`
            // is in the stream's context (checked by the lease), and the write lease keeps it
            // allocated until the generation queued on `stream` completes.
            check("curandGenerateLogNormal", unsafe {
                cuda_bindings::curandGenerateLogNormal(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut f32,
                    out.len(),
                    mean,
                    stddev,
                )
            })
        })
    }

    /// Generates `out.len()` log-normally distributed `f64` values on device.
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_log_normal_f64(
        &self,
        stream: &Stream,
        out: &mut DeviceBuffer<f64>,
        mean: f64,
        stddev: f64,
    ) -> Result<()> {
        validate_normal_params(
            self.rng_type,
            out.len(),
            stddev > 0.0,
            mean.is_finite() && stddev.is_finite(),
        )?;
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: parameters and even length (for pseudo-random) are validated above; `out`
            // is in the stream's context (checked by the lease), and the write lease keeps it
            // allocated until the generation queued on `stream` completes.
            check("curandGenerateLogNormalDouble", unsafe {
                cuda_bindings::curandGenerateLogNormalDouble(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut f64,
                    out.len(),
                    mean,
                    stddev,
                )
            })
        })
    }

    /// Generates `out.len()` Poisson-distributed `u32` values on device.
    ///
    /// `out` stays allocated until the queued generation completes, even if it is dropped first.
    /// Fails with `CUDA_ERROR_NOT_READY` while another stream still has unfinished work on `out`.
    pub fn generate_poisson(
        &self,
        stream: &Stream,
        out: &mut DeviceBuffer<u32>,
        lambda: f64,
    ) -> Result<()> {
        validate_poisson_lambda(lambda)?;
        self.prepare_device(stream)?;
        enqueue_leased(stream, [(out.storage(), Access::Write)], || {
            // SAFETY: `lambda > 0.0` is validated; `out` is a live device buffer in the stream's
            // context (checked by the lease), and the write lease keeps it allocated until the
            // generation queued on `stream` completes.
            check("curandGeneratePoisson", unsafe {
                cuda_bindings::curandGeneratePoisson(
                    self.raw,
                    out.cu_deviceptr() as usize as *mut u32,
                    out.len(),
                    lambda,
                )
            })
        })
    }

    // ---- Host generation ----

    /// Generates `out.len()` uniformly distributed 32-bit unsigned integers on host.
    pub fn generate_host_u32(&self, out: &mut [u32]) -> Result<()> {
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice of `out.len()` elements.
        check("curandGenerate", unsafe {
            cuda_bindings::curandGenerate(self.raw, out.as_mut_ptr(), out.len())
        })
    }

    /// Generates `out.len()` uniformly distributed 64-bit unsigned integers on host (Sobol64).
    pub fn generate_host_u64(&self, out: &mut [u64]) -> Result<()> {
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice of `out.len()` elements.
        check("curandGenerateLongLong", unsafe {
            cuda_bindings::curandGenerateLongLong(self.raw, out.as_mut_ptr(), out.len())
        })
    }

    /// Generates `out.len()` uniform `f32` values in `(0.0, 1.0]` on host.
    pub fn generate_host_uniform_f32(&self, out: &mut [f32]) -> Result<()> {
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice of `out.len()` elements.
        check("curandGenerateUniform", unsafe {
            cuda_bindings::curandGenerateUniform(self.raw, out.as_mut_ptr(), out.len())
        })
    }

    /// Generates `out.len()` uniform `f64` values in `(0.0, 1.0]` on host.
    pub fn generate_host_uniform_f64(&self, out: &mut [f64]) -> Result<()> {
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice of `out.len()` elements.
        check("curandGenerateUniformDouble", unsafe {
            cuda_bindings::curandGenerateUniformDouble(self.raw, out.as_mut_ptr(), out.len())
        })
    }

    /// Generates `out.len()` normally distributed `f32` values on host.
    pub fn generate_host_normal_f32(&self, out: &mut [f32], mean: f32, stddev: f32) -> Result<()> {
        validate_normal_params(
            self.rng_type,
            out.len(),
            stddev > 0.0,
            mean.is_finite() && stddev.is_finite(),
        )?;
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice and parameters are validated.
        check("curandGenerateNormal", unsafe {
            cuda_bindings::curandGenerateNormal(self.raw, out.as_mut_ptr(), out.len(), mean, stddev)
        })
    }

    /// Generates `out.len()` normally distributed `f64` values on host.
    pub fn generate_host_normal_f64(&self, out: &mut [f64], mean: f64, stddev: f64) -> Result<()> {
        validate_normal_params(
            self.rng_type,
            out.len(),
            stddev > 0.0,
            mean.is_finite() && stddev.is_finite(),
        )?;
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice and parameters are validated.
        check("curandGenerateNormalDouble", unsafe {
            cuda_bindings::curandGenerateNormalDouble(
                self.raw,
                out.as_mut_ptr(),
                out.len(),
                mean,
                stddev,
            )
        })
    }

    /// Generates `out.len()` log-normally distributed `f32` values on host.
    pub fn generate_host_log_normal_f32(
        &self,
        out: &mut [f32],
        mean: f32,
        stddev: f32,
    ) -> Result<()> {
        validate_normal_params(
            self.rng_type,
            out.len(),
            stddev > 0.0,
            mean.is_finite() && stddev.is_finite(),
        )?;
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice and parameters are validated.
        check("curandGenerateLogNormal", unsafe {
            cuda_bindings::curandGenerateLogNormal(
                self.raw,
                out.as_mut_ptr(),
                out.len(),
                mean,
                stddev,
            )
        })
    }

    /// Generates `out.len()` log-normally distributed `f64` values on host.
    pub fn generate_host_log_normal_f64(
        &self,
        out: &mut [f64],
        mean: f64,
        stddev: f64,
    ) -> Result<()> {
        validate_normal_params(
            self.rng_type,
            out.len(),
            stddev > 0.0,
            mean.is_finite() && stddev.is_finite(),
        )?;
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice and parameters are validated.
        check("curandGenerateLogNormalDouble", unsafe {
            cuda_bindings::curandGenerateLogNormalDouble(
                self.raw,
                out.as_mut_ptr(),
                out.len(),
                mean,
                stddev,
            )
        })
    }

    /// Generates `out.len()` Poisson-distributed `u32` values on host.
    pub fn generate_host_poisson(&self, out: &mut [u32], lambda: f64) -> Result<()> {
        validate_poisson_lambda(lambda)?;
        self.ensure_host()?;
        // SAFETY: `out` is a valid mutable host slice and `lambda > 0.0` is validated.
        check("curandGeneratePoisson", unsafe {
            cuda_bindings::curandGeneratePoisson(self.raw, out.as_mut_ptr(), out.len(), lambda)
        })
    }
}

impl Drop for Generator {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            if let Some(ctx) = &self.cuda {
                // A failed bind leaves its driver error on `ctx` and skips the native call.
                if cuda_core_ext::teardown::bind(ctx) {
                    // SAFETY: `self.raw` was created by `curandCreateGenerator` and is destroyed once.
                    let code = unsafe { cuda_bindings::curandDestroyGenerator(self.raw) };
                    teardown(code, "curandDestroyGenerator");
                }
            } else {
                // SAFETY: `self.raw` was created by `curandCreateGeneratorHost` and is destroyed once.
                let code = unsafe { cuda_bindings::curandDestroyGenerator(self.raw) };
                teardown(code, "curandDestroyGenerator");
            }
            self.raw = std::ptr::null_mut();
        }
    }
}
