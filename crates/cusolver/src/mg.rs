//! Multi-GPU cuSOLVER (`cusolverMg`) handle, device grid, distributed matrix descriptor,
//! and multi-GPU `POTRF`/`POTRS`/`POTRI`, `GETRF`/`GETRS`, and `SYEVD` with two-phase workspace
//! sizing.
//!
//! **What.**
//! - [`MgHandle`] — owns `cusolverMgHandle_t` and selects participating GPU ordinals.
//! - [`MgGrid`] — owns `cudaLibMgGrid_t` (2D process/device grid in row-major or column-major
//!   mapping).
//! - [`MgMatrixDesc`] — owns `cudaLibMgMatrixDesc_t` describing a 2D block-cyclic distributed
//!   matrix over an [`MgGrid`].
//! - Two-phase workspace sizing (`*_bufferSize` -> byte budget check -> caller-owned per-GPU
//!   work pointers) for `potrf`, `potrs`, `potri`, `getrf`, `getrs`, and `syevd`.
//! - Every per-GPU pointer slice is checked to hold an entry for each device cusolverMg indexes
//!   (the larger of the `device_select` count and the descriptor's grid size) before the call;
//!   pointer validity and `lwork` sizing remain the caller's `unsafe` contract.

use crate::{
    EigMode, Error, FillMode, Op, Result, check_mg, ensure_mg_loaded, mg_sys, teardown_check_mg,
};
use cuda_core::Context;
use cuda_core_ext::teardown;
use std::ffi::c_void;
use std::sync::Arc;

/// Device grid mapping order (`cusolverMgGridMapping_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MgGridMapping {
    /// Column-major (`CUDALIBMG_GRID_MAPPING_COL_MAJOR`).
    ColMajor,
    /// Row-major (`CUDALIBMG_GRID_MAPPING_ROW_MAJOR`).
    RowMajor,
}

impl MgGridMapping {
    pub(crate) fn to_sys(self) -> mg_sys::cusolverMgGridMapping_t {
        match self {
            Self::ColMajor => mg_sys::cusolverMgGridMapping_t_CUDALIBMG_GRID_MAPPING_COL_MAJOR,
            Self::RowMajor => mg_sys::cusolverMgGridMapping_t_CUDALIBMG_GRID_MAPPING_ROW_MAJOR,
        }
    }
}

/// Element/compute data type for `cusolverMg` descriptors (`cudaDataType`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MgDataType {
    /// 32-bit float (`CUDA_R_32F`).
    F32,
    /// 64-bit float (`CUDA_R_64F`).
    F64,
    /// Complex 32-bit float (`CUDA_C_32F`).
    C32,
    /// Complex 64-bit float (`CUDA_C_64F`).
    C64,
}

impl MgDataType {
    /// Byte size of one element.
    pub const fn byte_size(self) -> usize {
        match self {
            Self::F32 => 4,
            Self::F64 | Self::C32 => 8,
            Self::C64 => 16,
        }
    }

    /// Returns `true` if the data type is real (`F32` or `F64`).
    pub const fn is_real(self) -> bool {
        matches!(self, Self::F32 | Self::F64)
    }

    pub(crate) fn to_sys(self) -> mg_sys::cudaDataType {
        match self {
            Self::F32 => mg_sys::cudaDataType_t_CUDA_R_32F,
            Self::F64 => mg_sys::cudaDataType_t_CUDA_R_64F,
            Self::C32 => mg_sys::cudaDataType_t_CUDA_C_32F,
            Self::C64 => mg_sys::cudaDataType_t_CUDA_C_64F,
        }
    }
}

/// Validates multi-GPU grid dimensions against the supplied device ordinal list.
pub fn validate_mg_grid(num_row_devices: i32, num_col_devices: i32, devices: &[i32]) -> Result<()> {
    if num_row_devices <= 0 || num_col_devices <= 0 {
        return Err(Error::InvalidInput(
            "MgGrid requires positive row and column device counts",
        ));
    }
    let expected = (num_row_devices as usize)
        .checked_mul(num_col_devices as usize)
        .ok_or(Error::InvalidInput("MgGrid device count overflow"))?;
    if devices.len() != expected {
        return Err(Error::InvalidInput(
            "MgGrid device slice length must equal num_row_devices * num_col_devices",
        ));
    }
    Ok(())
}

/// Validates distributed matrix descriptor dimensions and block sizes.
pub fn validate_mg_matrix_desc(
    num_rows: i64,
    num_cols: i64,
    row_block_size: i64,
    col_block_size: i64,
) -> Result<()> {
    if num_rows <= 0 || num_cols <= 0 || row_block_size <= 0 || col_block_size <= 0 {
        return Err(Error::InvalidInput(
            "MgMatrixDesc requires positive dimensions and block sizes",
        ));
    }
    Ok(())
}

/// Validates that eigenvalue output type `data_type_w` for `syevd` is real (`F32` or `F64`).
pub fn validate_mg_syevd_w_type(data_type_w: MgDataType) -> Result<()> {
    if !data_type_w.is_real() {
        return Err(Error::InvalidInput(
            "syevd eigenvalue type data_type_w must be real (F32 or F64)",
        ));
    }
    Ok(())
}

/// Validates that every per-GPU pointer array (`array_d_*`) has an entry for each device that
/// cusolverMg indexes: the devices selected on the handle and the devices of the matrix grid.
pub fn validate_mg_ptr_arrays(selected: usize, grid: usize, lens: &[usize]) -> Result<()> {
    let need = selected.max(grid);
    if lens.iter().any(|&len| len < need) {
        return Err(Error::InvalidInput(
            "cusolverMg per-GPU pointer array has fewer entries than participating devices",
        ));
    }
    Ok(())
}

/// Checks `lwork` elements of `compute_type` against `budget_bytes` per GPU.
pub fn check_mg_work_budget(
    lwork: i64,
    compute_type: MgDataType,
    budget_bytes: usize,
) -> Result<usize> {
    if lwork < 0 {
        return Err(Error::InvalidInput("cusolverMg reported negative lwork"));
    }
    let bytes = usize::try_from(lwork)
        .ok()
        .and_then(|n| n.checked_mul(compute_type.byte_size()))
        .ok_or(Error::InvalidInput(
            "cusolverMg workspace byte size overflow",
        ))?;
    if bytes > budget_bytes {
        return Err(Error::Budget {
            requested: bytes,
            available: budget_bytes,
        });
    }
    Ok(bytes)
}

/// Owning multi-GPU cuSOLVER handle (`cusolverMgHandle_t`).
#[must_use]
pub struct MgHandle {
    raw: mg_sys::cusolverMgHandle_t,
    cuda: Arc<Context>,
    /// Device count passed to the last successful `device_select` (0 before it).
    devices: usize,
}

// SAFETY: `cusolverMgHandle_t` is externally synchronized; `!Sync` prevents concurrent calls.
unsafe impl Send for MgHandle {}

impl MgHandle {
    /// Creates a new `cusolverMgHandle_t` associated with primary `cuda` context.
    pub fn new(cuda: &Arc<Context>) -> Result<Self> {
        ensure_mg_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: mg_sys::cusolverMgHandle_t = std::ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check_mg("cusolverMgCreate", unsafe {
            mg_sys::cusolverMgCreate(&mut raw)
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            devices: 0,
        })
    }

    /// Rejects per-GPU pointer arrays shorter than the devices cusolverMg indexes for `descr`.
    fn check_ptr_arrays(&self, descr: &MgMatrixDesc, lens: &[usize]) -> Result<()> {
        validate_mg_ptr_arrays(self.devices, descr.grid.devices, lens)
    }

    /// Selects the CUDA device ordinals participating in multi-GPU operations.
    pub fn device_select(&mut self, devices: &[i32]) -> Result<()> {
        if devices.is_empty() {
            return Err(Error::InvalidInput(
                "device_select requires at least one device ordinal",
            ));
        }
        self.cuda.bind_to_thread()?;
        let nb =
            i32::try_from(devices.len()).map_err(|_| Error::InvalidInput("too many devices"))?;
        // SAFETY: `self.raw` is valid and `devices` has `nb` elements.
        check_mg("cusolverMgDeviceSelect", unsafe {
            mg_sys::cusolverMgDeviceSelect(self.raw, nb, devices.as_ptr().cast_mut())
        })?;
        self.devices = devices.len();
        Ok(())
    }

    /// Queries workspace element count for `cusolverMgPotrf` and checks it against `budget_bytes`.
    ///
    /// # Safety
    ///
    /// `array_d_a` must hold valid per-GPU device pointers matching `descr_a` on the selected devices.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn potrf_buffer_size(
        &self,
        uplo: FillMode,
        n: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        compute_type: MgDataType,
        budget_bytes: usize,
    ) -> Result<(i64, usize)> {
        self.check_ptr_arrays(descr_a, &[array_d_a.len()])?;
        self.cuda.bind_to_thread()?;
        let mut lwork = 0i64;
        // SAFETY: caller guarantees `array_d_a` holds valid per-device pointers for `descr_a`.
        check_mg("cusolverMgPotrf_bufferSize", unsafe {
            mg_sys::cusolverMgPotrf_bufferSize(
                self.raw,
                uplo.to_mg_sys(),
                n,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                compute_type.to_sys(),
                &mut lwork,
            )
        })?;
        let bytes = check_mg_work_budget(lwork, compute_type, budget_bytes)?;
        Ok((lwork, bytes))
    }

    /// Executes multi-GPU Cholesky factorization `cusolverMgPotrf`, returning `h_info`.
    ///
    /// # Safety
    ///
    /// `array_d_a` and `array_d_work` must hold valid per-GPU device pointers sized for `descr_a`
    /// and `lwork` elements of `compute_type`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn potrf(
        &self,
        uplo: FillMode,
        n: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        compute_type: MgDataType,
        array_d_work: &mut [*mut c_void],
        lwork: i64,
    ) -> Result<i32> {
        self.check_ptr_arrays(descr_a, &[array_d_a.len(), array_d_work.len()])?;
        self.cuda.bind_to_thread()?;
        let mut h_info = 0i32;
        // SAFETY: caller guarantees per-GPU matrix and workspace pointers.
        check_mg("cusolverMgPotrf", unsafe {
            mg_sys::cusolverMgPotrf(
                self.raw,
                uplo.to_mg_sys(),
                n,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                compute_type.to_sys(),
                array_d_work.as_mut_ptr(),
                lwork,
                &mut h_info,
            )
        })?;
        Ok(h_info)
    }

    /// Queries workspace element count for `cusolverMgPotrs` and checks it against `budget_bytes`.
    ///
    /// # Safety
    ///
    /// `array_d_a` and `array_d_b` must hold valid per-GPU device pointers.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn potrs_buffer_size(
        &self,
        uplo: FillMode,
        n: i32,
        nrhs: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        array_d_b: &mut [*mut c_void],
        ib: i32,
        jb: i32,
        descr_b: &MgMatrixDesc,
        compute_type: MgDataType,
        budget_bytes: usize,
    ) -> Result<(i64, usize)> {
        self.check_ptr_arrays(descr_a, &[array_d_a.len()])?;
        self.check_ptr_arrays(descr_b, &[array_d_b.len()])?;
        self.cuda.bind_to_thread()?;
        let mut lwork = 0i64;
        // SAFETY: caller guarantees per-GPU matrix pointers.
        check_mg("cusolverMgPotrs_bufferSize", unsafe {
            mg_sys::cusolverMgPotrs_bufferSize(
                self.raw,
                uplo.to_mg_sys(),
                n,
                nrhs,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                array_d_b.as_mut_ptr(),
                ib,
                jb,
                descr_b.raw(),
                compute_type.to_sys(),
                &mut lwork,
            )
        })?;
        let bytes = check_mg_work_budget(lwork, compute_type, budget_bytes)?;
        Ok((lwork, bytes))
    }

    /// Executes multi-GPU Cholesky solve `cusolverMgPotrs`, returning `h_info`.
    ///
    /// # Safety
    ///
    /// `array_d_a`, `array_d_b`, and `array_d_work` must hold valid per-GPU device pointers.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn potrs(
        &self,
        uplo: FillMode,
        n: i32,
        nrhs: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        array_d_b: &mut [*mut c_void],
        ib: i32,
        jb: i32,
        descr_b: &MgMatrixDesc,
        compute_type: MgDataType,
        array_d_work: &mut [*mut c_void],
        lwork: i64,
    ) -> Result<i32> {
        self.check_ptr_arrays(descr_a, &[array_d_a.len(), array_d_work.len()])?;
        self.check_ptr_arrays(descr_b, &[array_d_b.len()])?;
        self.cuda.bind_to_thread()?;
        let mut h_info = 0i32;
        // SAFETY: caller guarantees per-GPU matrix and workspace pointers.
        check_mg("cusolverMgPotrs", unsafe {
            mg_sys::cusolverMgPotrs(
                self.raw,
                uplo.to_mg_sys(),
                n,
                nrhs,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                array_d_b.as_mut_ptr(),
                ib,
                jb,
                descr_b.raw(),
                compute_type.to_sys(),
                array_d_work.as_mut_ptr(),
                lwork,
                &mut h_info,
            )
        })?;
        Ok(h_info)
    }

    /// Queries workspace element count for `cusolverMgGetrf` and checks it against `budget_bytes`.
    ///
    /// # Safety
    ///
    /// `array_d_a` and `array_d_ipiv` must hold valid per-GPU device pointers.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn getrf_buffer_size(
        &self,
        m: i32,
        n: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        array_d_ipiv: &mut [*mut i32],
        compute_type: MgDataType,
        budget_bytes: usize,
    ) -> Result<(i64, usize)> {
        self.check_ptr_arrays(descr_a, &[array_d_a.len(), array_d_ipiv.len()])?;
        self.cuda.bind_to_thread()?;
        let mut lwork = 0i64;
        // SAFETY: caller guarantees per-GPU matrix and pivot pointers.
        check_mg("cusolverMgGetrf_bufferSize", unsafe {
            mg_sys::cusolverMgGetrf_bufferSize(
                self.raw,
                m,
                n,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                array_d_ipiv.as_mut_ptr(),
                compute_type.to_sys(),
                &mut lwork,
            )
        })?;
        let bytes = check_mg_work_budget(lwork, compute_type, budget_bytes)?;
        Ok((lwork, bytes))
    }

    /// Executes multi-GPU LU factorization `cusolverMgGetrf`, returning `info`.
    ///
    /// # Safety
    ///
    /// `array_d_a`, `array_d_ipiv`, and `array_d_work` must hold valid per-GPU device pointers.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn getrf(
        &self,
        m: i32,
        n: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        array_d_ipiv: &mut [*mut i32],
        compute_type: MgDataType,
        array_d_work: &mut [*mut c_void],
        lwork: i64,
    ) -> Result<i32> {
        self.check_ptr_arrays(
            descr_a,
            &[array_d_a.len(), array_d_ipiv.len(), array_d_work.len()],
        )?;
        self.cuda.bind_to_thread()?;
        let mut info = 0i32;
        // SAFETY: caller guarantees per-GPU matrix, pivot, and workspace pointers.
        check_mg("cusolverMgGetrf", unsafe {
            mg_sys::cusolverMgGetrf(
                self.raw,
                m,
                n,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                array_d_ipiv.as_mut_ptr(),
                compute_type.to_sys(),
                array_d_work.as_mut_ptr(),
                lwork,
                &mut info,
            )
        })?;
        Ok(info)
    }

    /// Queries workspace element count for `cusolverMgGetrs` and checks it against `budget_bytes`.
    ///
    /// # Safety
    ///
    /// `array_d_a`, `array_d_ipiv`, and `array_d_b` must hold valid per-GPU device pointers.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn getrs_buffer_size(
        &self,
        trans: Op,
        n: i32,
        nrhs: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        array_d_ipiv: &mut [*mut i32],
        array_d_b: &mut [*mut c_void],
        ib: i32,
        jb: i32,
        descr_b: &MgMatrixDesc,
        compute_type: MgDataType,
        budget_bytes: usize,
    ) -> Result<(i64, usize)> {
        self.check_ptr_arrays(descr_a, &[array_d_a.len(), array_d_ipiv.len()])?;
        self.check_ptr_arrays(descr_b, &[array_d_b.len()])?;
        self.cuda.bind_to_thread()?;
        let mut lwork = 0i64;
        // SAFETY: caller guarantees per-GPU matrix and pivot pointers.
        check_mg("cusolverMgGetrs_bufferSize", unsafe {
            mg_sys::cusolverMgGetrs_bufferSize(
                self.raw,
                trans.to_mg_sys(),
                n,
                nrhs,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                array_d_ipiv.as_mut_ptr(),
                array_d_b.as_mut_ptr(),
                ib,
                jb,
                descr_b.raw(),
                compute_type.to_sys(),
                &mut lwork,
            )
        })?;
        let bytes = check_mg_work_budget(lwork, compute_type, budget_bytes)?;
        Ok((lwork, bytes))
    }

    /// Executes multi-GPU LU solve `cusolverMgGetrs`, returning `info`.
    ///
    /// # Safety
    ///
    /// `array_d_a`, `array_d_ipiv`, `array_d_b`, and `array_d_work` must hold valid per-GPU pointers.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn getrs(
        &self,
        trans: Op,
        n: i32,
        nrhs: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        array_d_ipiv: &mut [*mut i32],
        array_d_b: &mut [*mut c_void],
        ib: i32,
        jb: i32,
        descr_b: &MgMatrixDesc,
        compute_type: MgDataType,
        array_d_work: &mut [*mut c_void],
        lwork: i64,
    ) -> Result<i32> {
        self.check_ptr_arrays(
            descr_a,
            &[array_d_a.len(), array_d_ipiv.len(), array_d_work.len()],
        )?;
        self.check_ptr_arrays(descr_b, &[array_d_b.len()])?;
        self.cuda.bind_to_thread()?;
        let mut info = 0i32;
        // SAFETY: caller guarantees per-GPU matrix, pivot, and workspace pointers.
        check_mg("cusolverMgGetrs", unsafe {
            mg_sys::cusolverMgGetrs(
                self.raw,
                trans.to_mg_sys(),
                n,
                nrhs,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                array_d_ipiv.as_mut_ptr(),
                array_d_b.as_mut_ptr(),
                ib,
                jb,
                descr_b.raw(),
                compute_type.to_sys(),
                array_d_work.as_mut_ptr(),
                lwork,
                &mut info,
            )
        })?;
        Ok(info)
    }

    /// Queries workspace element count for `cusolverMgPotri` and checks it against `budget_bytes`.
    ///
    /// # Safety
    ///
    /// `array_d_a` must hold valid per-GPU device pointers matching `descr_a` on the selected devices.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn potri_buffer_size(
        &self,
        uplo: FillMode,
        n: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        compute_type: MgDataType,
        budget_bytes: usize,
    ) -> Result<(i64, usize)> {
        self.check_ptr_arrays(descr_a, &[array_d_a.len()])?;
        self.cuda.bind_to_thread()?;
        let mut lwork = 0i64;
        // SAFETY: caller guarantees `array_d_a` holds valid per-device pointers for `descr_a`.
        check_mg("cusolverMgPotri_bufferSize", unsafe {
            mg_sys::cusolverMgPotri_bufferSize(
                self.raw,
                uplo.to_mg_sys(),
                n,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                compute_type.to_sys(),
                &mut lwork,
            )
        })?;
        let bytes = check_mg_work_budget(lwork, compute_type, budget_bytes)?;
        Ok((lwork, bytes))
    }

    /// Executes multi-GPU Cholesky inverse `cusolverMgPotri`, returning `h_info`.
    ///
    /// # Safety
    ///
    /// `array_d_a` and `array_d_work` must hold valid per-GPU device pointers sized for `descr_a`
    /// and `lwork` elements of `compute_type`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn potri(
        &self,
        uplo: FillMode,
        n: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        compute_type: MgDataType,
        array_d_work: &mut [*mut c_void],
        lwork: i64,
    ) -> Result<i32> {
        self.check_ptr_arrays(descr_a, &[array_d_a.len(), array_d_work.len()])?;
        self.cuda.bind_to_thread()?;
        let mut h_info = 0i32;
        // SAFETY: caller guarantees per-GPU matrix and workspace pointers.
        check_mg("cusolverMgPotri", unsafe {
            mg_sys::cusolverMgPotri(
                self.raw,
                uplo.to_mg_sys(),
                n,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                compute_type.to_sys(),
                array_d_work.as_mut_ptr(),
                lwork,
                &mut h_info,
            )
        })?;
        Ok(h_info)
    }

    /// Queries workspace element count for `cusolverMgSyevd` and checks it against `budget_bytes`.
    ///
    /// # Safety
    ///
    /// `array_d_a` must hold valid per-GPU device pointers matching `descr_a`, and `w` must be a
    /// valid host eigenvalue buffer pointer for `n` elements of `data_type_w`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn syevd_buffer_size(
        &self,
        jobz: EigMode,
        uplo: FillMode,
        n: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        w: *mut c_void,
        data_type_w: MgDataType,
        compute_type: MgDataType,
        budget_bytes: usize,
    ) -> Result<(i64, usize)> {
        validate_mg_syevd_w_type(data_type_w)?;
        self.check_ptr_arrays(descr_a, &[array_d_a.len()])?;
        self.cuda.bind_to_thread()?;
        let mut lwork = 0i64;
        // SAFETY: caller guarantees `array_d_a` and `w` are valid pointers for `descr_a` and `n`.
        check_mg("cusolverMgSyevd_bufferSize", unsafe {
            mg_sys::cusolverMgSyevd_bufferSize(
                self.raw,
                jobz.to_mg_sys(),
                uplo.to_mg_sys(),
                n,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                w,
                data_type_w.to_sys(),
                compute_type.to_sys(),
                &mut lwork,
            )
        })?;
        let bytes = check_mg_work_budget(lwork, compute_type, budget_bytes)?;
        Ok((lwork, bytes))
    }

    /// Executes multi-GPU symmetric/Hermitian eigensolver `cusolverMgSyevd`, returning `info`.
    ///
    /// # Safety
    ///
    /// `array_d_a`, `w`, and `array_d_work` must hold valid pointers sized for `descr_a`, `n`
    /// eigenvalues of `data_type_w`, and `lwork` elements of `compute_type`.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn syevd(
        &self,
        jobz: EigMode,
        uplo: FillMode,
        n: i32,
        array_d_a: &mut [*mut c_void],
        ia: i32,
        ja: i32,
        descr_a: &MgMatrixDesc,
        w: *mut c_void,
        data_type_w: MgDataType,
        compute_type: MgDataType,
        array_d_work: &mut [*mut c_void],
        lwork: i64,
    ) -> Result<i32> {
        validate_mg_syevd_w_type(data_type_w)?;
        self.check_ptr_arrays(descr_a, &[array_d_a.len(), array_d_work.len()])?;
        self.cuda.bind_to_thread()?;
        let mut info = 0i32;
        // SAFETY: caller guarantees per-GPU matrix, eigenvalue, and workspace pointers.
        check_mg("cusolverMgSyevd", unsafe {
            mg_sys::cusolverMgSyevd(
                self.raw,
                jobz.to_mg_sys(),
                uplo.to_mg_sys(),
                n,
                array_d_a.as_mut_ptr(),
                ia,
                ja,
                descr_a.raw(),
                w,
                data_type_w.to_sys(),
                compute_type.to_sys(),
                array_d_work.as_mut_ptr(),
                lwork,
                &mut info,
            )
        })?;
        Ok(info)
    }
}

impl Drop for MgHandle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            if teardown::bind(&self.cuda) {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                let status = unsafe { mg_sys::cusolverMgDestroy(raw) };
                teardown_check_mg("cusolverMgDestroy", status);
            }
        }
    }
}

/// Owning multi-GPU device grid (`cudaLibMgGrid_t`).
#[must_use]
pub struct MgGrid {
    raw: mg_sys::cudaLibMgGrid_t,
    cuda: Arc<Context>,
    /// `num_row_devices * num_col_devices` (validated against the device list at creation).
    devices: usize,
}

// SAFETY: `cudaLibMgGrid_t` is metadata; thread safety is governed by caller sync.
unsafe impl Send for MgGrid {}
// SAFETY: shared read-only reference held by child `MgMatrixDesc` descriptors.
unsafe impl Sync for MgGrid {}

impl MgGrid {
    /// Creates a 2D device grid over `devices`.
    pub fn new(
        cuda: &Arc<Context>,
        num_row_devices: i32,
        num_col_devices: i32,
        devices: &[i32],
        mapping: MgGridMapping,
    ) -> Result<Arc<Self>> {
        validate_mg_grid(num_row_devices, num_col_devices, devices)?;
        ensure_mg_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: mg_sys::cudaLibMgGrid_t = std::ptr::null_mut();
        // SAFETY: `devices` has `num_row_devices * num_col_devices` entries.
        check_mg("cusolverMgCreateDeviceGrid", unsafe {
            mg_sys::cusolverMgCreateDeviceGrid(
                &mut raw,
                num_row_devices,
                num_col_devices,
                devices.as_ptr(),
                mapping.to_sys(),
            )
        })?;
        Ok(Arc::new(Self {
            raw,
            cuda: cuda.clone(),
            devices: devices.len(),
        }))
    }

    /// Raw `cudaLibMgGrid_t`.
    pub fn raw(&self) -> mg_sys::cudaLibMgGrid_t {
        self.raw
    }
}

impl Drop for MgGrid {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            if teardown::bind(&self.cuda) {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                let status = unsafe { mg_sys::cusolverMgDestroyGrid(raw) };
                teardown_check_mg("cusolverMgDestroyGrid", status);
            }
        }
    }
}

/// Owning multi-GPU distributed matrix descriptor (`cudaLibMgMatrixDesc_t`).
#[must_use]
pub struct MgMatrixDesc {
    raw: mg_sys::cudaLibMgMatrixDesc_t,
    grid: Arc<MgGrid>,
}

// SAFETY: `cudaLibMgMatrixDesc_t` is metadata; thread safety is governed by caller sync.
unsafe impl Send for MgMatrixDesc {}

impl MgMatrixDesc {
    /// Creates a 2D block-cyclic distributed matrix descriptor over `grid`.
    pub fn new(
        grid: &Arc<MgGrid>,
        num_rows: i64,
        num_cols: i64,
        row_block_size: i64,
        col_block_size: i64,
        data_type: MgDataType,
    ) -> Result<Self> {
        validate_mg_matrix_desc(num_rows, num_cols, row_block_size, col_block_size)?;
        ensure_mg_loaded()?;
        grid.cuda.bind_to_thread()?;
        let mut raw: mg_sys::cudaLibMgMatrixDesc_t = std::ptr::null_mut();
        // SAFETY: `grid.raw()` is alive and dimensions were validated above.
        check_mg("cusolverMgCreateMatrixDesc", unsafe {
            mg_sys::cusolverMgCreateMatrixDesc(
                &mut raw,
                num_rows,
                num_cols,
                row_block_size,
                col_block_size,
                data_type.to_sys(),
                grid.raw(),
            )
        })?;
        Ok(Self {
            raw,
            grid: grid.clone(),
        })
    }

    /// Raw `cudaLibMgMatrixDesc_t`.
    pub fn raw(&self) -> mg_sys::cudaLibMgMatrixDesc_t {
        self.raw
    }
}

impl Drop for MgMatrixDesc {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            if teardown::bind(&self.grid.cuda) {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                let status = unsafe { mg_sys::cusolverMgDestroyMatrixDesc(raw) };
                teardown_check_mg("cusolverMgDestroyMatrixDesc", status);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mg_grid_and_desc_validation() {
        assert!(validate_mg_grid(0, 2, &[0, 1]).is_err());
        assert!(validate_mg_grid(1, 2, &[0]).is_err());
        assert!(validate_mg_grid(1, 2, &[0, 1]).is_ok());

        assert!(validate_mg_matrix_desc(0, 4, 2, 2).is_err());
        assert!(validate_mg_matrix_desc(4, 4, 0, 2).is_err());
        assert!(validate_mg_matrix_desc(4, 4, 2, 2).is_ok());
    }

    #[test]
    fn mg_work_budget_enforces_limit() {
        assert!(matches!(
            check_mg_work_budget(-1, MgDataType::F32, 1024),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            check_mg_work_budget(512, MgDataType::F64, 1024),
            Err(Error::Budget {
                requested: 4096,
                available: 1024
            })
        ));
        assert_eq!(
            check_mg_work_budget(128, MgDataType::F64, 1024).unwrap(),
            1024
        );
    }

    #[test]
    fn mg_data_type_mapping_and_byte_sizes() {
        assert_eq!(MgDataType::F32.byte_size(), 4);
        assert_eq!(MgDataType::F64.byte_size(), 8);
        assert_eq!(MgDataType::C32.byte_size(), 8);
        assert_eq!(MgDataType::C64.byte_size(), 16);

        assert!(MgDataType::F32.is_real());
        assert!(MgDataType::F64.is_real());
        assert!(!MgDataType::C32.is_real());
        assert!(!MgDataType::C64.is_real());

        assert_eq!(MgDataType::F32.to_sys(), mg_sys::cudaDataType_t_CUDA_R_32F);
        assert_eq!(MgDataType::C64.to_sys(), mg_sys::cudaDataType_t_CUDA_C_64F);
        assert_eq!(
            MgGridMapping::ColMajor.to_sys(),
            mg_sys::cusolverMgGridMapping_t_CUDALIBMG_GRID_MAPPING_COL_MAJOR
        );
        assert_eq!(
            MgGridMapping::RowMajor.to_sys(),
            mg_sys::cusolverMgGridMapping_t_CUDALIBMG_GRID_MAPPING_ROW_MAJOR
        );
        assert_eq!(
            EigMode::NoVector.to_mg_sys(),
            mg_sys::cusolverEigMode_t_CUSOLVER_EIG_MODE_NOVECTOR
        );
        assert_eq!(
            EigMode::Vector.to_mg_sys(),
            mg_sys::cusolverEigMode_t_CUSOLVER_EIG_MODE_VECTOR
        );
    }

    #[test]
    fn mg_syevd_w_type_validation() {
        assert!(validate_mg_syevd_w_type(MgDataType::F32).is_ok());
        assert!(validate_mg_syevd_w_type(MgDataType::F64).is_ok());
        assert!(matches!(
            validate_mg_syevd_w_type(MgDataType::C32),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_mg_syevd_w_type(MgDataType::C64),
            Err(Error::InvalidInput(_))
        ));
    }

    #[test]
    fn mg_work_budget_overflow_is_rejected() {
        assert!(matches!(
            check_mg_work_budget(i64::MAX, MgDataType::C64, usize::MAX),
            Err(Error::InvalidInput(_))
        ));
    }

    #[test]
    fn mg_ptr_arrays_must_cover_participating_devices() {
        // Two selected devices: a one-entry slice would be read past its end by cusolverMg.
        assert!(matches!(
            validate_mg_ptr_arrays(2, 2, &[1]),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_mg_ptr_arrays(2, 1, &[2, 1]),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_mg_ptr_arrays(0, 2, &[1]),
            Err(Error::InvalidInput(_))
        ));
        assert!(validate_mg_ptr_arrays(2, 2, &[2, 2, 3]).is_ok());
        assert!(validate_mg_ptr_arrays(0, 1, &[1]).is_ok());
    }

    /// S3 (decides it): whether cusolverMg itself rejects a descriptor data type that differs from
    /// `compute_type`. If this fails, mixed types reach the native routine unchecked and the
    /// wrapper must validate them.
    #[test]
    #[ignore = "GPU: requires CUDA device and cusolverMg library; run with --ignored"]
    fn mg_potrf_rejects_desc_compute_type_mismatch() {
        let cuda = Context::new(0).expect("CUDA context on device 0");
        let stream = cuda.default_stream();
        let mut handle = MgHandle::new(&cuda).expect("cusolverMgCreate");
        handle.device_select(&[0]).expect("cusolverMgDeviceSelect");
        let grid = MgGrid::new(&cuda, 1, 1, &[0], MgGridMapping::ColMajor)
            .expect("cusolverMgCreateDeviceGrid");
        let desc = MgMatrixDesc::new(&grid, 4, 4, 4, 4, MgDataType::F32)
            .expect("cusolverMgCreateMatrixDesc");
        let a = cuda_core::DeviceBuffer::<f32>::zeroed(&stream, 16).expect("a");
        let mut array_d_a = [a.cu_deviceptr() as *mut c_void];
        // SAFETY: `array_d_a` holds one live device pointer of 16 f32 matching the 4x4 descriptor.
        let res = unsafe {
            handle.potrf_buffer_size(
                FillMode::Lower,
                4,
                &mut array_d_a,
                1,
                1,
                &desc,
                MgDataType::F64,
                usize::MAX,
            )
        };
        assert!(
            res.is_err(),
            "cusolverMg accepted an F32 descriptor with F64 compute: {res:?}"
        );
    }

    #[test]
    #[ignore = "GPU: requires CUDA device and cusolverMg library; run with --ignored"]
    fn mg_handle_grid_and_desc_lifecycle() {
        let cuda = Context::new(0).expect("CUDA context on device 0");
        let mut handle = MgHandle::new(&cuda).expect("cusolverMgCreate");
        handle.device_select(&[0]).expect("cusolverMgDeviceSelect");
        let grid = MgGrid::new(&cuda, 1, 1, &[0], MgGridMapping::ColMajor)
            .expect("cusolverMgCreateDeviceGrid");
        // cusolverMg supports only 1D column block-cyclic layouts: the row block must span all
        // rows (`mb == m`); `mb = 2` is rejected with CUSOLVER_STATUS_INVALID_VALUE.
        let _desc = MgMatrixDesc::new(&grid, 4, 4, 4, 2, MgDataType::F32)
            .expect("cusolverMgCreateMatrixDesc");
    }
}
