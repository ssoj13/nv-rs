//! Multi-GPU cuSOLVER (`cusolverMg`) handle, device grid, distributed matrix descriptor,
//! and multi-GPU `POTRF`/`POTRS`/`GETRF`/`GETRS` with two-phase workspace sizing.
//!
//! **What.**
//! - [`MgHandle`] — owns `cusolverMgHandle_t` and selects participating GPU ordinals.
//! - [`MgGrid`] — owns `cudaLibMgGrid_t` (2D process/device grid in row-major or column-major
//!   mapping).
//! - [`MgMatrixDesc`] — owns `cudaLibMgMatrixDesc_t` describing a 2D block-cyclic distributed
//!   matrix over an [`MgGrid`].
//! - Two-phase workspace sizing (`*_bufferSize` -> byte budget check -> caller-owned per-GPU
//!   work pointers) for `potrf`, `potrs`, `getrf`, and `getrs`.

use crate::{Error, FillMode, Op, Result, check_mg, ensure_mg_loaded, mg_sys};
use cuda_core::{Context, DriverError};
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
        return Err(Error::InvalidInput("MgGrid requires positive row and column device counts"));
    }
    let expected = (num_row_devices as usize)
        .checked_mul(num_col_devices as usize)
        .ok_or(Error::InvalidInput("MgGrid device count overflow"))?;
    if devices.len() != expected {
        return Err(Error::InvalidInput("MgGrid device slice length must equal num_row_devices * num_col_devices"));
    }
    Ok(())
}

/// Validates distributed matrix descriptor dimensions and block sizes.
pub fn validate_mg_matrix_desc(num_rows: i64, num_cols: i64, row_block_size: i64, col_block_size: i64) -> Result<()> {
    if num_rows <= 0 || num_cols <= 0 || row_block_size <= 0 || col_block_size <= 0 {
        return Err(Error::InvalidInput("MgMatrixDesc requires positive dimensions and block sizes"));
    }
    Ok(())
}

/// Checks `lwork` elements of `compute_type` against `budget_bytes` per GPU.
pub fn check_mg_work_budget(lwork: i64, compute_type: MgDataType, budget_bytes: usize) -> Result<usize> {
    if lwork < 0 {
        return Err(Error::InvalidInput("cusolverMg reported negative lwork"));
    }
    let bytes = usize::try_from(lwork)
        .ok()
        .and_then(|n| n.checked_mul(compute_type.byte_size()))
        .ok_or(Error::InvalidInput("cusolverMg workspace byte size overflow"))?;
    if bytes > budget_bytes {
        return Err(Error::Budget { requested: bytes, available: budget_bytes });
    }
    Ok(bytes)
}

/// Owning multi-GPU cuSOLVER handle (`cusolverMgHandle_t`).
pub struct MgHandle {
    raw: mg_sys::cusolverMgHandle_t,
    cuda: Arc<Context>,
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
        check_mg("cusolverMgCreate", unsafe { mg_sys::cusolverMgCreate(&mut raw) })?;
        Ok(Self { raw, cuda: cuda.clone() })
    }

    /// Selects the CUDA device ordinals participating in multi-GPU operations.
    pub fn device_select(&mut self, devices: &[i32]) -> Result<()> {
        if devices.is_empty() {
            return Err(Error::InvalidInput("device_select requires at least one device ordinal"));
        }
        self.cuda.bind_to_thread()?;
        let nb = i32::try_from(devices.len()).map_err(|_| Error::InvalidInput("too many devices"))?;
        // SAFETY: `self.raw` is valid and `devices` has `nb` elements.
        check_mg("cusolverMgDeviceSelect", unsafe {
            mg_sys::cusolverMgDeviceSelect(self.raw, nb, devices.as_ptr().cast_mut())
        })
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
}

impl Drop for MgHandle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            if let Err(e) = self.cuda.bind_to_thread() {
                self.cuda.record_err(Err::<(), _>(e));
                return;
            }
            // SAFETY: `raw` is owned by `self` and destroyed once.
            if let Err(e) = check_mg("cusolverMgDestroy", unsafe { mg_sys::cusolverMgDestroy(raw) }) {
                let _ = e;
                self.cuda
                    .record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE)));
            }
        }
    }
}

/// Owning multi-GPU device grid (`cudaLibMgGrid_t`).
pub struct MgGrid {
    raw: mg_sys::cudaLibMgGrid_t,
    cuda: Arc<Context>,
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
            mg_sys::cusolverMgCreateDeviceGrid(&mut raw, num_row_devices, num_col_devices, devices.as_ptr(), mapping.to_sys())
        })?;
        Ok(Arc::new(Self { raw, cuda: cuda.clone() }))
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
            if let Err(e) = self.cuda.bind_to_thread() {
                self.cuda.record_err(Err::<(), _>(e));
                return;
            }
            // SAFETY: `raw` is owned by `self` and destroyed once.
            if let Err(e) = check_mg("cusolverMgDestroyGrid", unsafe { mg_sys::cusolverMgDestroyGrid(raw) }) {
                let _ = e;
                self.cuda
                    .record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE)));
            }
        }
    }
}

/// Owning multi-GPU distributed matrix descriptor (`cudaLibMgMatrixDesc_t`).
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
        Ok(Self { raw, grid: grid.clone() })
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
            if let Err(e) = self.grid.cuda.bind_to_thread() {
                self.grid.cuda.record_err(Err::<(), _>(e));
                return;
            }
            // SAFETY: `raw` is owned by `self` and destroyed once.
            if let Err(e) = check_mg("cusolverMgDestroyMatrixDesc", unsafe { mg_sys::cusolverMgDestroyMatrixDesc(raw) }) {
                let _ = e;
                self.grid
                    .cuda
                    .record_err(Err::<(), _>(DriverError(cuda_core::sys::cudaError_enum_CUDA_ERROR_INVALID_VALUE)));
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
        assert!(matches!(check_mg_work_budget(-1, MgDataType::F32, 1024), Err(Error::InvalidInput(_))));
        assert!(matches!(
            check_mg_work_budget(512, MgDataType::F64, 1024),
            Err(Error::Budget { requested: 4096, available: 1024 })
        ));
        assert_eq!(check_mg_work_budget(128, MgDataType::F64, 1024).unwrap(), 1024);
    }
}
