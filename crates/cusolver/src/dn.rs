//! Dense cuSOLVER (`cusolverDn`) handle and factorizations/solvers (`GETRF`/`GETRS`,
//! `POTRF`/`POTRS`, `GEQRF`/`ORGQR`, `GESVD`, `SYEVD`).
//!
//! **What.**
//! - [`DnHandle`] — owns `cusolverDnHandle_t`, binds a `cuda_core::Stream`, and destroys on
//!   `Drop` (recording teardown errors on `cuda_core::Context`).
//! - [`DnFloat`] — sealed trait implemented for `f32` and `f64`, dispatching to `S*` and `D*`
//!   entry points.
//! - Two-phase workspace sizing: every workspace-using routine queries `*_bufferSize`, checks
//!   the caller's byte budget, and reserves into a caller-owned reusable [`Scratch`].
//!
//! **Column-major convention.** All `cusolverDn` routines operate on column-major matrices with
//! leading dimension `lda >= rows`.

use crate::{EigMode, Error, FillMode, Op, Result, Scratch, check, ensure_loaded, sys, teardown};
use cuda_core::{Context, DeviceBuffer, Stream};
use std::sync::Arc;

/// Singular vector job mode for [`DnHandle::gesvd`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SvdJob {
    /// Compute all `m` (or `n`) singular vectors (`'A'`).
    All,
    /// Compute the first `min(m, n)` singular vectors (`'S'`).
    Some,
    /// Overwrite `A` with the first `min(m, n)` singular vectors (`'O'`).
    Overwrite,
    /// Do not compute singular vectors (`'N'`).
    None,
}

impl SvdJob {
    pub(crate) const fn to_schar(self) -> i8 {
        match self {
            Self::All => b'A' as i8,
            Self::Some => b'S' as i8,
            Self::Overwrite => b'O' as i8,
            Self::None => b'N' as i8,
        }
    }
}

/// Validates a column-major matrix buffer of `rows x cols` with leading dimension `lda`.
pub fn validate_col_major_mat(rows: i32, cols: i32, lda: i32, buf_len: usize) -> Result<()> {
    if rows < 0 || cols < 0 || lda < rows.max(1) {
        return Err(Error::InvalidInput(
            "invalid matrix dimensions or leading dimension lda < max(1, rows)",
        ));
    }
    if cols > 0 {
        let req = (lda as usize)
            .checked_mul(cols as usize)
            .ok_or(Error::InvalidInput("matrix element count overflow"))?;
        if buf_len < req {
            return Err(Error::InvalidInput(
                "matrix device buffer smaller than lda * cols",
            ));
        }
    }
    Ok(())
}

/// Validates `getrf` arguments.
pub fn validate_getrf(
    m: i32,
    n: i32,
    lda: i32,
    a_len: usize,
    ipiv_len: usize,
    info_len: usize,
) -> Result<()> {
    validate_col_major_mat(m, n, lda, a_len)?;
    let min_mn = m.min(n) as usize;
    if ipiv_len < min_mn {
        return Err(Error::InvalidInput(
            "getrf ipiv buffer smaller than min(m, n)",
        ));
    }
    if info_len < 1 {
        return Err(Error::InvalidInput(
            "dev_info buffer must hold at least 1 i32",
        ));
    }
    Ok(())
}

/// Validates `getrs` arguments.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn validate_getrs(
    n: i32,
    nrhs: i32,
    lda: i32,
    ldb: i32,
    a_len: usize,
    ipiv_len: usize,
    b_len: usize,
    info_len: usize,
) -> Result<()> {
    validate_col_major_mat(n, n, lda, a_len)?;
    validate_col_major_mat(n, nrhs, ldb, b_len)?;
    if ipiv_len < n as usize {
        return Err(Error::InvalidInput("getrs ipiv buffer smaller than n"));
    }
    if info_len < 1 {
        return Err(Error::InvalidInput(
            "dev_info buffer must hold at least 1 i32",
        ));
    }
    Ok(())
}

/// Validates `potrf` arguments.
pub fn validate_potrf(n: i32, lda: i32, a_len: usize, info_len: usize) -> Result<()> {
    validate_col_major_mat(n, n, lda, a_len)?;
    if info_len < 1 {
        return Err(Error::InvalidInput(
            "dev_info buffer must hold at least 1 i32",
        ));
    }
    Ok(())
}

/// Validates `potrs` arguments.
pub fn validate_potrs(
    n: i32,
    nrhs: i32,
    lda: i32,
    ldb: i32,
    a_len: usize,
    b_len: usize,
    info_len: usize,
) -> Result<()> {
    validate_col_major_mat(n, n, lda, a_len)?;
    validate_col_major_mat(n, nrhs, ldb, b_len)?;
    if info_len < 1 {
        return Err(Error::InvalidInput(
            "dev_info buffer must hold at least 1 i32",
        ));
    }
    Ok(())
}

/// Validates `geqrf` arguments.
pub fn validate_geqrf(
    m: i32,
    n: i32,
    lda: i32,
    a_len: usize,
    tau_len: usize,
    info_len: usize,
) -> Result<()> {
    validate_col_major_mat(m, n, lda, a_len)?;
    if tau_len < m.min(n) as usize {
        return Err(Error::InvalidInput(
            "geqrf tau buffer smaller than min(m, n)",
        ));
    }
    if info_len < 1 {
        return Err(Error::InvalidInput(
            "dev_info buffer must hold at least 1 i32",
        ));
    }
    Ok(())
}

/// Validates `orgqr` arguments.
pub fn validate_orgqr(
    m: i32,
    n: i32,
    k: i32,
    lda: i32,
    a_len: usize,
    tau_len: usize,
    info_len: usize,
) -> Result<()> {
    if k < 0 || k > n || n > m {
        return Err(Error::InvalidInput("orgqr requires 0 <= k <= n <= m"));
    }
    validate_col_major_mat(m, n, lda, a_len)?;
    if tau_len < k as usize {
        return Err(Error::InvalidInput("orgqr tau buffer smaller than k"));
    }
    if info_len < 1 {
        return Err(Error::InvalidInput(
            "dev_info buffer must hold at least 1 i32",
        ));
    }
    Ok(())
}

/// Validates `gesvd` arguments (cuSOLVER requires `m >= n`).
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn validate_gesvd(
    jobu: SvdJob,
    jobvt: SvdJob,
    m: i32,
    n: i32,
    lda: i32,
    ldu: i32,
    ldvt: i32,
    a_len: usize,
    s_len: usize,
    u_len: usize,
    vt_len: usize,
    info_len: usize,
) -> Result<()> {
    if m < n {
        return Err(Error::InvalidInput("cusolverDn gesvd requires m >= n"));
    }
    if jobu == SvdJob::Overwrite && jobvt == SvdJob::Overwrite {
        return Err(Error::InvalidInput(
            "gesvd cannot set both jobu and jobvt to Overwrite",
        ));
    }
    validate_col_major_mat(m, n, lda, a_len)?;
    if s_len < n as usize {
        return Err(Error::InvalidInput(
            "gesvd singular values buffer smaller than n",
        ));
    }
    let u_cols = match jobu {
        SvdJob::All => m,
        SvdJob::Some => n,
        SvdJob::Overwrite | SvdJob::None => 0,
    };
    if u_cols > 0 {
        validate_col_major_mat(m, u_cols, ldu, u_len)?;
    }
    let vt_rows = match jobvt {
        SvdJob::All | SvdJob::Some => n,
        SvdJob::Overwrite | SvdJob::None => 0,
    };
    if vt_rows > 0 {
        validate_col_major_mat(vt_rows, n, ldvt, vt_len)?;
    }
    if info_len < 1 {
        return Err(Error::InvalidInput(
            "dev_info buffer must hold at least 1 i32",
        ));
    }
    Ok(())
}

/// Validates `syevd` arguments.
pub fn validate_syevd(n: i32, lda: i32, a_len: usize, w_len: usize, info_len: usize) -> Result<()> {
    validate_col_major_mat(n, n, lda, a_len)?;
    if w_len < n as usize {
        return Err(Error::InvalidInput(
            "syevd eigenvalues buffer smaller than n",
        ));
    }
    if info_len < 1 {
        return Err(Error::InvalidInput(
            "dev_info buffer must hold at least 1 i32",
        ));
    }
    Ok(())
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Real floating-point scalar supported by dense cuSOLVER (`f32` and `f64`).
pub trait DnFloat: sealed::Sealed + Copy + 'static {
    /// Queries `getrf` workspace size in elements.
    ///
    /// # Safety
    /// `handle` and `a` must be valid for `m x n` with `lda`.
    unsafe fn getrf_buffer_size(
        handle: sys::cusolverDnHandle_t,
        m: i32,
        n: i32,
        a: *mut Self,
        lda: i32,
    ) -> Result<i32>;

    /// Executes `getrf` in-place.
    ///
    /// # Safety
    /// All pointers and workspace must be valid on `handle`'s current stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn getrf(
        handle: sys::cusolverDnHandle_t,
        m: i32,
        n: i32,
        a: *mut Self,
        lda: i32,
        work: *mut Self,
        ipiv: *mut i32,
        info: *mut i32,
    ) -> Result<()>;

    /// Executes `getrs` solve.
    ///
    /// # Safety
    /// All pointers must be valid on `handle`'s current stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn getrs(
        handle: sys::cusolverDnHandle_t,
        trans: sys::cublasOperation_t,
        n: i32,
        nrhs: i32,
        a: *const Self,
        lda: i32,
        ipiv: *const i32,
        b: *mut Self,
        ldb: i32,
        info: *mut i32,
    ) -> Result<()>;

    /// Queries `potrf` workspace size in elements.
    ///
    /// # Safety
    /// `handle` and `a` must be valid.
    unsafe fn potrf_buffer_size(
        handle: sys::cusolverDnHandle_t,
        uplo: sys::cublasFillMode_t,
        n: i32,
        a: *mut Self,
        lda: i32,
    ) -> Result<i32>;

    /// Executes `potrf` in-place.
    ///
    /// # Safety
    /// All pointers and workspace must be valid on `handle`'s current stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn potrf(
        handle: sys::cusolverDnHandle_t,
        uplo: sys::cublasFillMode_t,
        n: i32,
        a: *mut Self,
        lda: i32,
        work: *mut Self,
        lwork: i32,
        info: *mut i32,
    ) -> Result<()>;

    /// Executes `potrs` solve.
    ///
    /// # Safety
    /// All pointers must be valid on `handle`'s current stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn potrs(
        handle: sys::cusolverDnHandle_t,
        uplo: sys::cublasFillMode_t,
        n: i32,
        nrhs: i32,
        a: *const Self,
        lda: i32,
        b: *mut Self,
        ldb: i32,
        info: *mut i32,
    ) -> Result<()>;

    /// Queries `geqrf` workspace size in elements.
    ///
    /// # Safety
    /// `handle` and `a` must be valid.
    unsafe fn geqrf_buffer_size(
        handle: sys::cusolverDnHandle_t,
        m: i32,
        n: i32,
        a: *mut Self,
        lda: i32,
    ) -> Result<i32>;

    /// Executes `geqrf` in-place.
    ///
    /// # Safety
    /// All pointers and workspace must be valid on `handle`'s current stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn geqrf(
        handle: sys::cusolverDnHandle_t,
        m: i32,
        n: i32,
        a: *mut Self,
        lda: i32,
        tau: *mut Self,
        work: *mut Self,
        lwork: i32,
        info: *mut i32,
    ) -> Result<()>;

    /// Queries `orgqr` workspace size in elements.
    ///
    /// # Safety
    /// `handle`, `a`, and `tau` must be valid.
    unsafe fn orgqr_buffer_size(
        handle: sys::cusolverDnHandle_t,
        m: i32,
        n: i32,
        k: i32,
        a: *const Self,
        lda: i32,
        tau: *const Self,
    ) -> Result<i32>;

    /// Executes `orgqr` in-place.
    ///
    /// # Safety
    /// All pointers and workspace must be valid on `handle`'s current stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn orgqr(
        handle: sys::cusolverDnHandle_t,
        m: i32,
        n: i32,
        k: i32,
        a: *mut Self,
        lda: i32,
        tau: *const Self,
        work: *mut Self,
        lwork: i32,
        info: *mut i32,
    ) -> Result<()>;

    /// Queries `gesvd` workspace size in elements.
    ///
    /// # Safety
    /// `handle` must be valid.
    unsafe fn gesvd_buffer_size(handle: sys::cusolverDnHandle_t, m: i32, n: i32) -> Result<i32>;

    /// Executes `gesvd`.
    ///
    /// # Safety
    /// All pointers and workspace must be valid on `handle`'s current stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn gesvd(
        handle: sys::cusolverDnHandle_t,
        jobu: i8,
        jobvt: i8,
        m: i32,
        n: i32,
        a: *mut Self,
        lda: i32,
        s: *mut Self,
        u: *mut Self,
        ldu: i32,
        vt: *mut Self,
        ldvt: i32,
        work: *mut Self,
        lwork: i32,
        rwork: *mut Self,
        info: *mut i32,
    ) -> Result<()>;

    /// Queries `syevd` workspace size in elements.
    ///
    /// # Safety
    /// `handle`, `a`, and `w` must be valid.
    unsafe fn syevd_buffer_size(
        handle: sys::cusolverDnHandle_t,
        jobz: sys::cusolverEigMode_t,
        uplo: sys::cublasFillMode_t,
        n: i32,
        a: *const Self,
        lda: i32,
        w: *const Self,
    ) -> Result<i32>;

    /// Executes `syevd`.
    ///
    /// # Safety
    /// All pointers and workspace must be valid on `handle`'s current stream.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    unsafe fn syevd(
        handle: sys::cusolverDnHandle_t,
        jobz: sys::cusolverEigMode_t,
        uplo: sys::cublasFillMode_t,
        n: i32,
        a: *mut Self,
        lda: i32,
        w: *mut Self,
        work: *mut Self,
        lwork: i32,
        info: *mut i32,
    ) -> Result<()>;
}

macro_rules! impl_dn_float {
    ($ty:ty, $getrf_bs:ident, $getrf:ident, $getrs:ident, $potrf_bs:ident, $potrf:ident, $potrs:ident, $geqrf_bs:ident, $geqrf:ident, $orgqr_bs:ident, $orgqr:ident, $gesvd_bs:ident, $gesvd:ident, $syevd_bs:ident, $syevd:ident) => {
        impl DnFloat for $ty {
            unsafe fn getrf_buffer_size(
                handle: sys::cusolverDnHandle_t,
                m: i32,
                n: i32,
                a: *mut Self,
                lda: i32,
            ) -> Result<i32> {
                let mut lwork = 0i32;
                // SAFETY: caller guarantees valid handle and device pointer.
                check(stringify!($getrf_bs), unsafe {
                    sys::$getrf_bs(handle, m, n, a, lda, &mut lwork)
                })?;
                Ok(lwork)
            }
            unsafe fn getrf(
                handle: sys::cusolverDnHandle_t,
                m: i32,
                n: i32,
                a: *mut Self,
                lda: i32,
                work: *mut Self,
                ipiv: *mut i32,
                info: *mut i32,
            ) -> Result<()> {
                // SAFETY: caller guarantees valid pointers and workspace.
                check(stringify!($getrf), unsafe {
                    sys::$getrf(handle, m, n, a, lda, work, ipiv, info)
                })
            }
            unsafe fn getrs(
                handle: sys::cusolverDnHandle_t,
                trans: sys::cublasOperation_t,
                n: i32,
                nrhs: i32,
                a: *const Self,
                lda: i32,
                ipiv: *const i32,
                b: *mut Self,
                ldb: i32,
                info: *mut i32,
            ) -> Result<()> {
                // SAFETY: caller guarantees valid pointers.
                check(stringify!($getrs), unsafe {
                    sys::$getrs(handle, trans, n, nrhs, a, lda, ipiv, b, ldb, info)
                })
            }
            unsafe fn potrf_buffer_size(
                handle: sys::cusolverDnHandle_t,
                uplo: sys::cublasFillMode_t,
                n: i32,
                a: *mut Self,
                lda: i32,
            ) -> Result<i32> {
                let mut lwork = 0i32;
                // SAFETY: caller guarantees valid handle and device pointer.
                check(stringify!($potrf_bs), unsafe {
                    sys::$potrf_bs(handle, uplo, n, a, lda, &mut lwork)
                })?;
                Ok(lwork)
            }
            unsafe fn potrf(
                handle: sys::cusolverDnHandle_t,
                uplo: sys::cublasFillMode_t,
                n: i32,
                a: *mut Self,
                lda: i32,
                work: *mut Self,
                lwork: i32,
                info: *mut i32,
            ) -> Result<()> {
                // SAFETY: caller guarantees valid pointers and workspace.
                check(stringify!($potrf), unsafe {
                    sys::$potrf(handle, uplo, n, a, lda, work, lwork, info)
                })
            }
            unsafe fn potrs(
                handle: sys::cusolverDnHandle_t,
                uplo: sys::cublasFillMode_t,
                n: i32,
                nrhs: i32,
                a: *const Self,
                lda: i32,
                b: *mut Self,
                ldb: i32,
                info: *mut i32,
            ) -> Result<()> {
                // SAFETY: caller guarantees valid pointers.
                check(stringify!($potrs), unsafe {
                    sys::$potrs(handle, uplo, n, nrhs, a, lda, b, ldb, info)
                })
            }
            unsafe fn geqrf_buffer_size(
                handle: sys::cusolverDnHandle_t,
                m: i32,
                n: i32,
                a: *mut Self,
                lda: i32,
            ) -> Result<i32> {
                let mut lwork = 0i32;
                // SAFETY: caller guarantees valid handle and device pointer.
                check(stringify!($geqrf_bs), unsafe {
                    sys::$geqrf_bs(handle, m, n, a, lda, &mut lwork)
                })?;
                Ok(lwork)
            }
            unsafe fn geqrf(
                handle: sys::cusolverDnHandle_t,
                m: i32,
                n: i32,
                a: *mut Self,
                lda: i32,
                tau: *mut Self,
                work: *mut Self,
                lwork: i32,
                info: *mut i32,
            ) -> Result<()> {
                // SAFETY: caller guarantees valid pointers and workspace.
                check(stringify!($geqrf), unsafe {
                    sys::$geqrf(handle, m, n, a, lda, tau, work, lwork, info)
                })
            }
            unsafe fn orgqr_buffer_size(
                handle: sys::cusolverDnHandle_t,
                m: i32,
                n: i32,
                k: i32,
                a: *const Self,
                lda: i32,
                tau: *const Self,
            ) -> Result<i32> {
                let mut lwork = 0i32;
                // SAFETY: caller guarantees valid handle and device pointers.
                check(stringify!($orgqr_bs), unsafe {
                    sys::$orgqr_bs(handle, m, n, k, a, lda, tau, &mut lwork)
                })?;
                Ok(lwork)
            }
            unsafe fn orgqr(
                handle: sys::cusolverDnHandle_t,
                m: i32,
                n: i32,
                k: i32,
                a: *mut Self,
                lda: i32,
                tau: *const Self,
                work: *mut Self,
                lwork: i32,
                info: *mut i32,
            ) -> Result<()> {
                // SAFETY: caller guarantees valid pointers and workspace.
                check(stringify!($orgqr), unsafe {
                    sys::$orgqr(handle, m, n, k, a, lda, tau, work, lwork, info)
                })
            }
            unsafe fn gesvd_buffer_size(
                handle: sys::cusolverDnHandle_t,
                m: i32,
                n: i32,
            ) -> Result<i32> {
                let mut lwork = 0i32;
                // SAFETY: caller guarantees valid handle.
                check(stringify!($gesvd_bs), unsafe {
                    sys::$gesvd_bs(handle, m, n, &mut lwork)
                })?;
                Ok(lwork)
            }
            unsafe fn gesvd(
                handle: sys::cusolverDnHandle_t,
                jobu: i8,
                jobvt: i8,
                m: i32,
                n: i32,
                a: *mut Self,
                lda: i32,
                s: *mut Self,
                u: *mut Self,
                ldu: i32,
                vt: *mut Self,
                ldvt: i32,
                work: *mut Self,
                lwork: i32,
                rwork: *mut Self,
                info: *mut i32,
            ) -> Result<()> {
                // SAFETY: caller guarantees valid pointers and workspace.
                check(stringify!($gesvd), unsafe {
                    sys::$gesvd(
                        handle, jobu, jobvt, m, n, a, lda, s, u, ldu, vt, ldvt, work, lwork, rwork,
                        info,
                    )
                })
            }
            unsafe fn syevd_buffer_size(
                handle: sys::cusolverDnHandle_t,
                jobz: sys::cusolverEigMode_t,
                uplo: sys::cublasFillMode_t,
                n: i32,
                a: *const Self,
                lda: i32,
                w: *const Self,
            ) -> Result<i32> {
                let mut lwork = 0i32;
                // SAFETY: caller guarantees valid handle and device pointers.
                check(stringify!($syevd_bs), unsafe {
                    sys::$syevd_bs(handle, jobz, uplo, n, a, lda, w, &mut lwork)
                })?;
                Ok(lwork)
            }
            unsafe fn syevd(
                handle: sys::cusolverDnHandle_t,
                jobz: sys::cusolverEigMode_t,
                uplo: sys::cublasFillMode_t,
                n: i32,
                a: *mut Self,
                lda: i32,
                w: *mut Self,
                work: *mut Self,
                lwork: i32,
                info: *mut i32,
            ) -> Result<()> {
                // SAFETY: caller guarantees valid pointers and workspace.
                check(stringify!($syevd), unsafe {
                    sys::$syevd(handle, jobz, uplo, n, a, lda, w, work, lwork, info)
                })
            }
        }
    };
}

impl_dn_float!(
    f32,
    cusolverDnSgetrf_bufferSize,
    cusolverDnSgetrf,
    cusolverDnSgetrs,
    cusolverDnSpotrf_bufferSize,
    cusolverDnSpotrf,
    cusolverDnSpotrs,
    cusolverDnSgeqrf_bufferSize,
    cusolverDnSgeqrf,
    cusolverDnSorgqr_bufferSize,
    cusolverDnSorgqr,
    cusolverDnSgesvd_bufferSize,
    cusolverDnSgesvd,
    cusolverDnSsyevd_bufferSize,
    cusolverDnSsyevd
);

impl_dn_float!(
    f64,
    cusolverDnDgetrf_bufferSize,
    cusolverDnDgetrf,
    cusolverDnDgetrs,
    cusolverDnDpotrf_bufferSize,
    cusolverDnDpotrf,
    cusolverDnDpotrs,
    cusolverDnDgeqrf_bufferSize,
    cusolverDnDgeqrf,
    cusolverDnDorgqr_bufferSize,
    cusolverDnDorgqr,
    cusolverDnDgesvd_bufferSize,
    cusolverDnDgesvd,
    cusolverDnDsyevd_bufferSize,
    cusolverDnDsyevd
);

/// Owning handle to a dense cuSOLVER context (`cusolverDnHandle_t`).
pub struct DnHandle {
    raw: sys::cusolverDnHandle_t,
    cuda: Arc<Context>,
}

// SAFETY: `cusolverDnHandle_t` can be moved across host threads when externally synchronized;
// `!Sync` prevents concurrent un-synchronized calls on one handle.
unsafe impl Send for DnHandle {}

impl DnHandle {
    /// Creates a new dense cuSOLVER handle on `cuda`.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusolverDnHandle_t = std::ptr::null_mut();
        // SAFETY: `cuda` is bound and `raw` is a valid out-pointer.
        check("cusolverDnCreate", unsafe {
            sys::cusolverDnCreate(&mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
        })
    }

    /// Raw `cusolverDnHandle_t`.
    pub fn raw(&self) -> sys::cusolverDnHandle_t {
        self.raw
    }

    /// Owning CUDA context.
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }

    /// Binds `stream` to this handle.
    pub fn set_stream(&self, stream: &Stream) -> Result<()> {
        if !Arc::ptr_eq(&self.cuda, stream.context()) {
            return Err(Error::InvalidInput(
                "stream belongs to a different CUDA context than DnHandle",
            ));
        }
        // SAFETY: `self.raw` and `stream.cu_stream()` are valid on `self.cuda`.
        check("cusolverDnSetStream", unsafe {
            sys::cusolverDnSetStream(self.raw, stream.cu_stream())
        })
    }

    /// Computes LU factorization `P * A = L * U` in-place on `stream`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn getrf<T: DnFloat>(
        &self,
        stream: &Stream,
        m: i32,
        n: i32,
        a: &mut DeviceBuffer<T>,
        lda: i32,
        ipiv: &mut DeviceBuffer<i32>,
        dev_info: &mut DeviceBuffer<i32>,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<()> {
        validate_getrf(m, n, lda, a.len(), ipiv.len(), dev_info.len())?;
        self.set_stream(stream)?;
        let a_ptr = a.cu_deviceptr() as *mut T;
        // SAFETY: dimensions and buffers were validated above.
        let lwork = unsafe { T::getrf_buffer_size(self.raw, m, n, a_ptr, lda) }?;
        let (ws, _) = scratch.reserve_elements::<T>(stream, lwork, budget)?;
        // SAFETY: workspace `ws` has `lwork` elements of `T` on `stream`.
        unsafe {
            T::getrf(
                self.raw,
                m,
                n,
                a_ptr,
                lda,
                ws,
                ipiv.cu_deviceptr() as *mut i32,
                dev_info.cu_deviceptr() as *mut i32,
            )
        }
    }

    /// Solves `op(A) * X = B` using the LU factors from [`DnHandle::getrf`].
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn getrs<T: DnFloat>(
        &self,
        stream: &Stream,
        trans: Op,
        n: i32,
        nrhs: i32,
        a: &DeviceBuffer<T>,
        lda: i32,
        ipiv: &DeviceBuffer<i32>,
        b: &mut DeviceBuffer<T>,
        ldb: i32,
        dev_info: &mut DeviceBuffer<i32>,
    ) -> Result<()> {
        validate_getrs(
            n,
            nrhs,
            lda,
            ldb,
            a.len(),
            ipiv.len(),
            b.len(),
            dev_info.len(),
        )?;
        self.set_stream(stream)?;
        // SAFETY: dimensions and buffers were validated above.
        unsafe {
            T::getrs(
                self.raw,
                trans.to_sys(),
                n,
                nrhs,
                a.cu_deviceptr() as *const T,
                lda,
                ipiv.cu_deviceptr() as *const i32,
                b.cu_deviceptr() as *mut T,
                ldb,
                dev_info.cu_deviceptr() as *mut i32,
            )
        }
    }

    /// Computes Cholesky factorization `A = L * L^T` or `U^T * U` in-place on `stream`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn potrf<T: DnFloat>(
        &self,
        stream: &Stream,
        uplo: FillMode,
        n: i32,
        a: &mut DeviceBuffer<T>,
        lda: i32,
        dev_info: &mut DeviceBuffer<i32>,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<()> {
        validate_potrf(n, lda, a.len(), dev_info.len())?;
        self.set_stream(stream)?;
        let a_ptr = a.cu_deviceptr() as *mut T;
        // SAFETY: dimensions and buffers were validated above.
        let lwork = unsafe { T::potrf_buffer_size(self.raw, uplo.to_sys(), n, a_ptr, lda) }?;
        let (ws, lwork) = scratch.reserve_elements::<T>(stream, lwork, budget)?;
        // SAFETY: workspace `ws` has `lwork` elements of `T` on `stream`.
        unsafe {
            T::potrf(
                self.raw,
                uplo.to_sys(),
                n,
                a_ptr,
                lda,
                ws,
                lwork,
                dev_info.cu_deviceptr() as *mut i32,
            )
        }
    }

    /// Solves `A * X = B` using the Cholesky factor from [`DnHandle::potrf`].
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn potrs<T: DnFloat>(
        &self,
        stream: &Stream,
        uplo: FillMode,
        n: i32,
        nrhs: i32,
        a: &DeviceBuffer<T>,
        lda: i32,
        b: &mut DeviceBuffer<T>,
        ldb: i32,
        dev_info: &mut DeviceBuffer<i32>,
    ) -> Result<()> {
        validate_potrs(n, nrhs, lda, ldb, a.len(), b.len(), dev_info.len())?;
        self.set_stream(stream)?;
        // SAFETY: dimensions and buffers were validated above.
        unsafe {
            T::potrs(
                self.raw,
                uplo.to_sys(),
                n,
                nrhs,
                a.cu_deviceptr() as *const T,
                lda,
                b.cu_deviceptr() as *mut T,
                ldb,
                dev_info.cu_deviceptr() as *mut i32,
            )
        }
    }

    /// Computes QR factorization `A = Q * R` in-place on `stream`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn geqrf<T: DnFloat>(
        &self,
        stream: &Stream,
        m: i32,
        n: i32,
        a: &mut DeviceBuffer<T>,
        lda: i32,
        tau: &mut DeviceBuffer<T>,
        dev_info: &mut DeviceBuffer<i32>,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<()> {
        validate_geqrf(m, n, lda, a.len(), tau.len(), dev_info.len())?;
        self.set_stream(stream)?;
        let a_ptr = a.cu_deviceptr() as *mut T;
        // SAFETY: dimensions and buffers were validated above.
        let lwork = unsafe { T::geqrf_buffer_size(self.raw, m, n, a_ptr, lda) }?;
        let (ws, lwork) = scratch.reserve_elements::<T>(stream, lwork, budget)?;
        // SAFETY: workspace `ws` has `lwork` elements of `T` on `stream`.
        unsafe {
            T::geqrf(
                self.raw,
                m,
                n,
                a_ptr,
                lda,
                tau.cu_deviceptr() as *mut T,
                ws,
                lwork,
                dev_info.cu_deviceptr() as *mut i32,
            )
        }
    }

    /// Generates the orthogonal matrix `Q` from Householder reflectors produced by [`DnHandle::geqrf`].
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn orgqr<T: DnFloat>(
        &self,
        stream: &Stream,
        m: i32,
        n: i32,
        k: i32,
        a: &mut DeviceBuffer<T>,
        lda: i32,
        tau: &DeviceBuffer<T>,
        dev_info: &mut DeviceBuffer<i32>,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<()> {
        validate_orgqr(m, n, k, lda, a.len(), tau.len(), dev_info.len())?;
        self.set_stream(stream)?;
        let a_ptr = a.cu_deviceptr() as *mut T;
        let tau_ptr = tau.cu_deviceptr() as *const T;
        // SAFETY: dimensions and buffers were validated above.
        let lwork = unsafe { T::orgqr_buffer_size(self.raw, m, n, k, a_ptr, lda, tau_ptr) }?;
        let (ws, lwork) = scratch.reserve_elements::<T>(stream, lwork, budget)?;
        // SAFETY: workspace `ws` has `lwork` elements of `T` on `stream`.
        unsafe {
            T::orgqr(
                self.raw,
                m,
                n,
                k,
                a_ptr,
                lda,
                tau_ptr,
                ws,
                lwork,
                dev_info.cu_deviceptr() as *mut i32,
            )
        }
    }

    /// Computes singular value decomposition `A = U * diag(S) * V^T` on `stream` (`m >= n`).
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn gesvd<T: DnFloat>(
        &self,
        stream: &Stream,
        jobu: SvdJob,
        jobvt: SvdJob,
        m: i32,
        n: i32,
        a: &mut DeviceBuffer<T>,
        lda: i32,
        s: &mut DeviceBuffer<T>,
        u: Option<(&mut DeviceBuffer<T>, i32)>,
        vt: Option<(&mut DeviceBuffer<T>, i32)>,
        dev_info: &mut DeviceBuffer<i32>,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<()> {
        let (u_ptr, ldu, u_len) = match u {
            Some((buf, ld)) => (buf.cu_deviceptr() as *mut T, ld, buf.len()),
            None => (std::ptr::null_mut(), 1, 0),
        };
        let (vt_ptr, ldvt, vt_len) = match vt {
            Some((buf, ld)) => (buf.cu_deviceptr() as *mut T, ld, buf.len()),
            None => (std::ptr::null_mut(), 1, 0),
        };
        validate_gesvd(
            jobu,
            jobvt,
            m,
            n,
            lda,
            ldu,
            ldvt,
            a.len(),
            s.len(),
            u_len,
            vt_len,
            dev_info.len(),
        )?;
        self.set_stream(stream)?;
        // SAFETY: `self.raw` is valid.
        let lwork = unsafe { T::gesvd_buffer_size(self.raw, m, n) }?;
        let (ws, lwork) = scratch.reserve_elements::<T>(stream, lwork, budget)?;
        // SAFETY: all pointers and workspace were validated above.
        unsafe {
            T::gesvd(
                self.raw,
                jobu.to_schar(),
                jobvt.to_schar(),
                m,
                n,
                a.cu_deviceptr() as *mut T,
                lda,
                s.cu_deviceptr() as *mut T,
                u_ptr,
                ldu,
                vt_ptr,
                ldvt,
                ws,
                lwork,
                std::ptr::null_mut(),
                dev_info.cu_deviceptr() as *mut i32,
            )
        }
    }

    /// Computes eigenvalues (and optionally eigenvectors) of a real symmetric matrix `A` on `stream`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub fn syevd<T: DnFloat>(
        &self,
        stream: &Stream,
        jobz: EigMode,
        uplo: FillMode,
        n: i32,
        a: &mut DeviceBuffer<T>,
        lda: i32,
        w: &mut DeviceBuffer<T>,
        dev_info: &mut DeviceBuffer<i32>,
        scratch: &mut Scratch,
        budget: usize,
    ) -> Result<()> {
        validate_syevd(n, lda, a.len(), w.len(), dev_info.len())?;
        self.set_stream(stream)?;
        let a_ptr = a.cu_deviceptr() as *mut T;
        let w_ptr = w.cu_deviceptr() as *mut T;
        // SAFETY: dimensions and buffers were validated above.
        let lwork = unsafe {
            T::syevd_buffer_size(self.raw, jobz.to_sys(), uplo.to_sys(), n, a_ptr, lda, w_ptr)
        }?;
        let (ws, lwork) = scratch.reserve_elements::<T>(stream, lwork, budget)?;
        // SAFETY: workspace `ws` has `lwork` elements of `T` on `stream`.
        unsafe {
            T::syevd(
                self.raw,
                jobz.to_sys(),
                uplo.to_sys(),
                n,
                a_ptr,
                lda,
                w_ptr,
                ws,
                lwork,
                dev_info.cu_deviceptr() as *mut i32,
            )
        }
    }
}

impl Drop for DnHandle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusolverDnDestroy", || {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                unsafe { sys::cusolverDnDestroy(raw) }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dn_validation_rejects_bad_leading_dimensions_and_buffers() {
        assert!(validate_col_major_mat(4, 3, 3, 12).is_err());
        assert!(validate_col_major_mat(4, 3, 4, 11).is_err());
        assert!(validate_col_major_mat(4, 3, 4, 12).is_ok());

        assert!(validate_getrf(4, 3, 4, 12, 2, 1).is_err());
        assert!(validate_getrf(4, 3, 4, 12, 3, 0).is_err());
        assert!(validate_getrf(4, 3, 4, 12, 3, 1).is_ok());

        assert!(validate_orgqr(4, 5, 3, 4, 20, 3, 1).is_err());
        assert!(validate_orgqr(4, 3, 4, 4, 12, 4, 1).is_err());
        assert!(validate_orgqr(4, 3, 3, 4, 12, 3, 1).is_ok());

        assert!(validate_gesvd(SvdJob::All, SvdJob::All, 2, 3, 2, 2, 3, 6, 2, 4, 9, 1).is_err());
        assert!(
            validate_gesvd(
                SvdJob::Overwrite,
                SvdJob::Overwrite,
                3,
                2,
                3,
                1,
                1,
                6,
                2,
                0,
                0,
                1
            )
            .is_err()
        );
        assert!(validate_gesvd(SvdJob::All, SvdJob::All, 3, 2, 3, 3, 2, 6, 2, 9, 4, 1).is_ok());
    }
}
