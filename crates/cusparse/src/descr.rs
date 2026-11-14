//! Generic sparse and dense descriptors for cuSPARSE (`SpMatDescr`, `DnVecDescr`, `DnMatDescr`).
//!
//! **What.**
//! - [`SpMatDescr`] — CSR, CSC, COO, and Blocked-ELL sparse matrix descriptors with upfront
//!   dimension, block-size, and buffer-byte-length validation.
//! - [`DnVecDescr`] — dense vector descriptor.
//! - [`DnMatDescr`] — dense matrix descriptor (row-major or column-major) with leading-dimension
//!   validation.
//!
//! **Safety.** Descriptors hold an `Arc<Context>` and destroy their native handle on `Drop`,
//! recording any teardown error on `Context`. Callers must ensure the backing device buffers
//! remain alive for as long as the descriptor is used in cuSPARSE operations.
//!
//! **Used by** [`crate::ops`].

use crate::{DataType, DiagType, Error, FillMode, IndexBase, IndexType, Order, Result, check, ensure_loaded, sys, teardown};
use cuda_core::{Context, DeviceBuffer};
use std::ffi::c_void;
use std::sync::Arc;

/// Sealed trait for Rust integer types usable as cuSPARSE sparse indices.
pub trait SparseIndex: Copy + 'static {
    /// Corresponding [`IndexType`].
    const INDEX_TYPE: IndexType;
}

impl SparseIndex for u16 {
    const INDEX_TYPE: IndexType = IndexType::U16;
}

impl SparseIndex for i32 {
    const INDEX_TYPE: IndexType = IndexType::I32;
}

impl SparseIndex for i64 {
    const INDEX_TYPE: IndexType = IndexType::I64;
}

/// Sealed trait for Rust scalar types usable as cuSPARSE matrix/vector values.
pub trait SparseScalar: Copy + 'static {
    /// Corresponding [`DataType`].
    const DATA_TYPE: DataType;
}

impl SparseScalar for f32 {
    const DATA_TYPE: DataType = DataType::F32;
}

impl SparseScalar for f64 {
    const DATA_TYPE: DataType = DataType::F64;
}

impl SparseScalar for [f32; 2] {
    const DATA_TYPE: DataType = DataType::C32;
}

impl SparseScalar for [f64; 2] {
    const DATA_TYPE: DataType = DataType::C64;
}

impl SparseScalar for i8 {
    const DATA_TYPE: DataType = DataType::I8;
}

impl SparseScalar for u8 {
    const DATA_TYPE: DataType = DataType::U8;
}

impl SparseScalar for i32 {
    const DATA_TYPE: DataType = DataType::I32;
}

impl SparseScalar for u32 {
    const DATA_TYPE: DataType = DataType::U32;
}

/// Sparse storage format tracked by [`SpMatDescr`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SparseFormat {
    /// Compressed Sparse Row.
    Csr,
    /// Compressed Sparse Column.
    Csc,
    /// Coordinate (triplet) format.
    Coo,
    /// Blocked-ELLPACK format.
    BlockedEll {
        /// Square block size (`ellBlockSize`).
        block_size: i64,
        /// Number of non-zero columns in the padded ELL matrix (`ellCols`).
        ell_cols: i64,
    },
}

/// Validates CSR buffer byte lengths and dimensions before calling `cusparseCreateCsr`.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn validate_csr(
    rows: i64,
    cols: i64,
    nnz: i64,
    row_offsets_bytes: usize,
    col_ind_bytes: usize,
    values_bytes: usize,
    row_off_type: IndexType,
    col_ind_type: IndexType,
    value_type: DataType,
) -> Result<()> {
    if rows < 0 || cols < 0 || nnz < 0 {
        return Err(Error::InvalidInput("CSR rows, cols, and nnz must be non-negative"));
    }
    let req_off = (rows as usize)
        .checked_add(1)
        .and_then(|n| n.checked_mul(row_off_type.byte_size()))
        .ok_or(Error::InvalidInput("CSR row_offsets byte size overflow"))?;
    let req_col = (nnz as usize)
        .checked_mul(col_ind_type.byte_size())
        .ok_or(Error::InvalidInput("CSR col_ind byte size overflow"))?;
    let req_val = (nnz as usize)
        .checked_mul(value_type.byte_size())
        .ok_or(Error::InvalidInput("CSR values byte size overflow"))?;
    if row_offsets_bytes < req_off {
        return Err(Error::InvalidInput("CSR row_offsets buffer smaller than (rows + 1) elements"));
    }
    if col_ind_bytes < req_col {
        return Err(Error::InvalidInput("CSR col_ind buffer smaller than nnz elements"));
    }
    if values_bytes < req_val {
        return Err(Error::InvalidInput("CSR values buffer smaller than nnz elements"));
    }
    Ok(())
}

/// Validates CSC buffer byte lengths and dimensions before calling `cusparseCreateCsc`.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn validate_csc(
    rows: i64,
    cols: i64,
    nnz: i64,
    col_offsets_bytes: usize,
    row_ind_bytes: usize,
    values_bytes: usize,
    col_off_type: IndexType,
    row_ind_type: IndexType,
    value_type: DataType,
) -> Result<()> {
    if rows < 0 || cols < 0 || nnz < 0 {
        return Err(Error::InvalidInput("CSC rows, cols, and nnz must be non-negative"));
    }
    let req_off = (cols as usize)
        .checked_add(1)
        .and_then(|n| n.checked_mul(col_off_type.byte_size()))
        .ok_or(Error::InvalidInput("CSC col_offsets byte size overflow"))?;
    let req_row = (nnz as usize)
        .checked_mul(row_ind_type.byte_size())
        .ok_or(Error::InvalidInput("CSC row_ind byte size overflow"))?;
    let req_val = (nnz as usize)
        .checked_mul(value_type.byte_size())
        .ok_or(Error::InvalidInput("CSC values byte size overflow"))?;
    if col_offsets_bytes < req_off {
        return Err(Error::InvalidInput("CSC col_offsets buffer smaller than (cols + 1) elements"));
    }
    if row_ind_bytes < req_row {
        return Err(Error::InvalidInput("CSC row_ind buffer smaller than nnz elements"));
    }
    if values_bytes < req_val {
        return Err(Error::InvalidInput("CSC values buffer smaller than nnz elements"));
    }
    Ok(())
}

/// Validates COO buffer byte lengths and dimensions before calling `cusparseCreateCoo`.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn validate_coo(
    rows: i64,
    cols: i64,
    nnz: i64,
    row_ind_bytes: usize,
    col_ind_bytes: usize,
    values_bytes: usize,
    idx_type: IndexType,
    value_type: DataType,
) -> Result<()> {
    if rows < 0 || cols < 0 || nnz < 0 {
        return Err(Error::InvalidInput("COO rows, cols, and nnz must be non-negative"));
    }
    let req_idx = (nnz as usize)
        .checked_mul(idx_type.byte_size())
        .ok_or(Error::InvalidInput("COO index byte size overflow"))?;
    let req_val = (nnz as usize)
        .checked_mul(value_type.byte_size())
        .ok_or(Error::InvalidInput("COO values byte size overflow"))?;
    if row_ind_bytes < req_idx {
        return Err(Error::InvalidInput("COO row_ind buffer smaller than nnz elements"));
    }
    if col_ind_bytes < req_idx {
        return Err(Error::InvalidInput("COO col_ind buffer smaller than nnz elements"));
    }
    if values_bytes < req_val {
        return Err(Error::InvalidInput("COO values buffer smaller than nnz elements"));
    }
    Ok(())
}

/// Validates Blocked-ELL dimensions and buffer byte lengths before calling `cusparseCreateBlockedEll`.
#[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
pub fn validate_blocked_ell(
    rows: i64,
    cols: i64,
    block_size: i64,
    ell_cols: i64,
    col_ind_bytes: usize,
    values_bytes: usize,
    idx_type: IndexType,
    value_type: DataType,
) -> Result<()> {
    if rows < 0 || cols < 0 || block_size <= 0 || ell_cols < 0 {
        return Err(Error::InvalidInput("Blocked-ELL requires non-negative dimensions and block_size > 0"));
    }
    if rows % block_size != 0 || cols % block_size != 0 || ell_cols % block_size != 0 {
        return Err(Error::InvalidInput("Blocked-ELL rows, cols, and ell_cols must be multiples of block_size"));
    }
    let block_rows = (rows / block_size) as usize;
    let block_ell_cols = (ell_cols / block_size) as usize;
    let req_col = block_rows
        .checked_mul(block_ell_cols)
        .and_then(|n| n.checked_mul(idx_type.byte_size()))
        .ok_or(Error::InvalidInput("Blocked-ELL col_ind byte size overflow"))?;
    let req_val = (rows as usize)
        .checked_mul(ell_cols as usize)
        .and_then(|n| n.checked_mul(value_type.byte_size()))
        .ok_or(Error::InvalidInput("Blocked-ELL values byte size overflow"))?;
    if col_ind_bytes < req_col {
        return Err(Error::InvalidInput("Blocked-ELL col_ind buffer too small"));
    }
    if values_bytes < req_val {
        return Err(Error::InvalidInput("Blocked-ELL values buffer too small"));
    }
    Ok(())
}

/// Validates dense matrix dimensions and leading dimension before calling `cusparseCreateDnMat`.
pub fn validate_dn_mat(rows: i64, cols: i64, ld: i64, order: Order, values_bytes: usize, value_type: DataType) -> Result<()> {
    if rows < 0 || cols < 0 || ld <= 0 {
        return Err(Error::InvalidInput("dense matrix requires non-negative rows/cols and ld > 0"));
    }
    let (major_dim, minor_dim) = match order {
        Order::Col => (cols, rows),
        Order::Row => (rows, cols),
    };
    if ld < minor_dim {
        return Err(Error::InvalidInput("dense matrix leading dimension ld is smaller than minor extent"));
    }
    let req = if major_dim == 0 || minor_dim == 0 {
        0
    } else {
        (major_dim as usize)
            .checked_mul(ld as usize)
            .and_then(|n| n.checked_mul(value_type.byte_size()))
            .ok_or(Error::InvalidInput("dense matrix byte size overflow"))?
    };
    if values_bytes < req {
        return Err(Error::InvalidInput("dense matrix device buffer smaller than required ld * major_dim"));
    }
    Ok(())
}

/// Owning wrapper around `cusparseSpMatDescr_t`.
pub struct SpMatDescr {
    raw: sys::cusparseSpMatDescr_t,
    cuda: Arc<Context>,
    rows: i64,
    cols: i64,
    nnz: i64,
    format: SparseFormat,
    value_type: DataType,
}

// SAFETY: cuSPARSE descriptors are metadata handles; thread safety is governed by caller sync.
unsafe impl Send for SpMatDescr {}

impl SpMatDescr {
    /// Creates a CSR sparse matrix descriptor from typed device buffers.
    ///
    /// # Safety
    ///
    /// `row_offsets`, `col_ind`, and `values` must remain valid and allocated on `cuda` for as
    /// long as this descriptor (or any plan referencing it) is used.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn csr<R: SparseIndex, C: SparseIndex, V: SparseScalar>(
        cuda: &Arc<Context>,
        rows: i64,
        cols: i64,
        nnz: i64,
        row_offsets: &DeviceBuffer<R>,
        col_ind: &DeviceBuffer<C>,
        values: &DeviceBuffer<V>,
        idx_base: IndexBase,
    ) -> Result<Self> {
        let row_bytes = row_offsets
            .len()
            .checked_mul(std::mem::size_of::<R>())
            .ok_or(Error::InvalidInput("CSR row_offsets byte size overflow"))?;
        let col_bytes = col_ind
            .len()
            .checked_mul(std::mem::size_of::<C>())
            .ok_or(Error::InvalidInput("CSR col_ind byte size overflow"))?;
        let val_bytes = values
            .len()
            .checked_mul(std::mem::size_of::<V>())
            .ok_or(Error::InvalidInput("CSR values byte size overflow"))?;
        validate_csr(rows, cols, nnz, row_bytes, col_bytes, val_bytes, R::INDEX_TYPE, C::INDEX_TYPE, V::DATA_TYPE)?;
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseSpMatDescr_t = std::ptr::null_mut();
        // SAFETY: validated buffer sizes; pointers come from live `DeviceBuffer`s on `cuda`.
        check("cusparseCreateCsr", unsafe {
            sys::cusparseCreateCsr(
                &mut raw,
                rows,
                cols,
                nnz,
                row_offsets.cu_deviceptr() as *mut c_void,
                col_ind.cu_deviceptr() as *mut c_void,
                values.cu_deviceptr() as *mut c_void,
                R::INDEX_TYPE.to_sys(),
                C::INDEX_TYPE.to_sys(),
                idx_base.to_sys(),
                V::DATA_TYPE.to_sys(),
            )
        })?;
        Ok(Self { raw, cuda: cuda.clone(), rows, cols, nnz, format: SparseFormat::Csr, value_type: V::DATA_TYPE })
    }

    /// Creates an empty CSR descriptor (null pointers, `nnz = 0`) to be populated by `SpGemm` copy.
    pub fn csr_empty(
        cuda: &Arc<Context>,
        rows: i64,
        cols: i64,
        row_off_type: IndexType,
        col_ind_type: IndexType,
        idx_base: IndexBase,
        value_type: DataType,
    ) -> Result<Self> {
        if rows < 0 || cols < 0 {
            return Err(Error::InvalidInput("CSR rows and cols must be non-negative"));
        }
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseSpMatDescr_t = std::ptr::null_mut();
        // SAFETY: cuSPARSE accepts null pointers with `nnz == 0` for SpGEMM output descriptors.
        check("cusparseCreateCsr", unsafe {
            sys::cusparseCreateCsr(
                &mut raw,
                rows,
                cols,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                row_off_type.to_sys(),
                col_ind_type.to_sys(),
                idx_base.to_sys(),
                value_type.to_sys(),
            )
        })?;
        Ok(Self { raw, cuda: cuda.clone(), rows, cols, nnz: 0, format: SparseFormat::Csr, value_type })
    }

    /// Creates a CSC sparse matrix descriptor from typed device buffers.
    ///
    /// # Safety
    ///
    /// `col_offsets`, `row_ind`, and `values` must remain valid for as long as this descriptor is used.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn csc<C: SparseIndex, R: SparseIndex, V: SparseScalar>(
        cuda: &Arc<Context>,
        rows: i64,
        cols: i64,
        nnz: i64,
        col_offsets: &DeviceBuffer<C>,
        row_ind: &DeviceBuffer<R>,
        values: &DeviceBuffer<V>,
        idx_base: IndexBase,
    ) -> Result<Self> {
        let col_bytes = col_offsets
            .len()
            .checked_mul(std::mem::size_of::<C>())
            .ok_or(Error::InvalidInput("CSC col_offsets byte size overflow"))?;
        let row_bytes = row_ind
            .len()
            .checked_mul(std::mem::size_of::<R>())
            .ok_or(Error::InvalidInput("CSC row_ind byte size overflow"))?;
        let val_bytes = values
            .len()
            .checked_mul(std::mem::size_of::<V>())
            .ok_or(Error::InvalidInput("CSC values byte size overflow"))?;
        validate_csc(rows, cols, nnz, col_bytes, row_bytes, val_bytes, C::INDEX_TYPE, R::INDEX_TYPE, V::DATA_TYPE)?;
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseSpMatDescr_t = std::ptr::null_mut();
        // SAFETY: validated buffer sizes; pointers come from live `DeviceBuffer`s on `cuda`.
        check("cusparseCreateCsc", unsafe {
            sys::cusparseCreateCsc(
                &mut raw,
                rows,
                cols,
                nnz,
                col_offsets.cu_deviceptr() as *mut c_void,
                row_ind.cu_deviceptr() as *mut c_void,
                values.cu_deviceptr() as *mut c_void,
                C::INDEX_TYPE.to_sys(),
                R::INDEX_TYPE.to_sys(),
                idx_base.to_sys(),
                V::DATA_TYPE.to_sys(),
            )
        })?;
        Ok(Self { raw, cuda: cuda.clone(), rows, cols, nnz, format: SparseFormat::Csc, value_type: V::DATA_TYPE })
    }

    /// Creates a COO sparse matrix descriptor from typed device buffers.
    ///
    /// # Safety
    ///
    /// `row_ind`, `col_ind`, and `values` must remain valid for as long as this descriptor is used.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn coo<I: SparseIndex, V: SparseScalar>(
        cuda: &Arc<Context>,
        rows: i64,
        cols: i64,
        nnz: i64,
        row_ind: &DeviceBuffer<I>,
        col_ind: &DeviceBuffer<I>,
        values: &DeviceBuffer<V>,
        idx_base: IndexBase,
    ) -> Result<Self> {
        let row_bytes = row_ind
            .len()
            .checked_mul(std::mem::size_of::<I>())
            .ok_or(Error::InvalidInput("COO row_ind byte size overflow"))?;
        let col_bytes = col_ind
            .len()
            .checked_mul(std::mem::size_of::<I>())
            .ok_or(Error::InvalidInput("COO col_ind byte size overflow"))?;
        let val_bytes = values
            .len()
            .checked_mul(std::mem::size_of::<V>())
            .ok_or(Error::InvalidInput("COO values byte size overflow"))?;
        validate_coo(rows, cols, nnz, row_bytes, col_bytes, val_bytes, I::INDEX_TYPE, V::DATA_TYPE)?;
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseSpMatDescr_t = std::ptr::null_mut();
        // SAFETY: validated buffer sizes; pointers come from live `DeviceBuffer`s on `cuda`.
        check("cusparseCreateCoo", unsafe {
            sys::cusparseCreateCoo(
                &mut raw,
                rows,
                cols,
                nnz,
                row_ind.cu_deviceptr() as *mut c_void,
                col_ind.cu_deviceptr() as *mut c_void,
                values.cu_deviceptr() as *mut c_void,
                I::INDEX_TYPE.to_sys(),
                idx_base.to_sys(),
                V::DATA_TYPE.to_sys(),
            )
        })?;
        Ok(Self { raw, cuda: cuda.clone(), rows, cols, nnz, format: SparseFormat::Coo, value_type: V::DATA_TYPE })
    }

    /// Creates a Blocked-ELLPACK sparse matrix descriptor from typed device buffers.
    ///
    /// # Safety
    ///
    /// `col_ind` and `values` must remain valid for as long as this descriptor is used.
    #[expect(clippy::too_many_arguments, reason = "matches NVIDIA C API signature")]
    pub unsafe fn blocked_ell<I: SparseIndex, V: SparseScalar>(
        cuda: &Arc<Context>,
        rows: i64,
        cols: i64,
        block_size: i64,
        ell_cols: i64,
        col_ind: &DeviceBuffer<I>,
        values: &DeviceBuffer<V>,
        idx_base: IndexBase,
    ) -> Result<Self> {
        let col_bytes = col_ind
            .len()
            .checked_mul(std::mem::size_of::<I>())
            .ok_or(Error::InvalidInput("Blocked-ELL col_ind byte size overflow"))?;
        let val_bytes = values
            .len()
            .checked_mul(std::mem::size_of::<V>())
            .ok_or(Error::InvalidInput("Blocked-ELL values byte size overflow"))?;
        let nnz = rows
            .checked_mul(ell_cols)
            .ok_or(Error::InvalidInput("Blocked-ELL nnz overflow"))?;
        validate_blocked_ell(rows, cols, block_size, ell_cols, col_bytes, val_bytes, I::INDEX_TYPE, V::DATA_TYPE)?;
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseSpMatDescr_t = std::ptr::null_mut();
        // SAFETY: validated buffer sizes; pointers come from live `DeviceBuffer`s on `cuda`.
        check("cusparseCreateBlockedEll", unsafe {
            sys::cusparseCreateBlockedEll(
                &mut raw,
                rows,
                cols,
                block_size,
                ell_cols,
                col_ind.cu_deviceptr() as *mut c_void,
                values.cu_deviceptr() as *mut c_void,
                I::INDEX_TYPE.to_sys(),
                idx_base.to_sys(),
                V::DATA_TYPE.to_sys(),
            )
        })?;
        Ok(Self {
            raw,
            cuda: cuda.clone(),
            rows,
            cols,
            nnz,
            format: SparseFormat::BlockedEll { block_size, ell_cols },
            value_type: V::DATA_TYPE,
        })
    }

    /// Updates CSR device pointers (e.g. after `SpGemm::compute` determines `nnz` and allocates `col_ind`/`values`).
    ///
    /// # Safety
    ///
    /// The buffers must match the index/value types of this CSR descriptor and outlive its use.
    pub unsafe fn set_csr_pointers<R: SparseIndex, C: SparseIndex, V: SparseScalar>(
        &mut self,
        nnz: i64,
        row_offsets: &DeviceBuffer<R>,
        col_ind: &DeviceBuffer<C>,
        values: &DeviceBuffer<V>,
    ) -> Result<()> {
        if self.format != SparseFormat::Csr {
            return Err(Error::InvalidInput("set_csr_pointers requires a CSR descriptor"));
        }
        let row_bytes = row_offsets
            .len()
            .checked_mul(std::mem::size_of::<R>())
            .ok_or(Error::InvalidInput("CSR row_offsets byte size overflow"))?;
        let col_bytes = col_ind
            .len()
            .checked_mul(std::mem::size_of::<C>())
            .ok_or(Error::InvalidInput("CSR col_ind byte size overflow"))?;
        let val_bytes = values
            .len()
            .checked_mul(std::mem::size_of::<V>())
            .ok_or(Error::InvalidInput("CSR values byte size overflow"))?;
        validate_csr(self.rows, self.cols, nnz, row_bytes, col_bytes, val_bytes, R::INDEX_TYPE, C::INDEX_TYPE, V::DATA_TYPE)?;
        // SAFETY: `self.raw` is a valid CSR descriptor and buffers are validated and live on `self.cuda`.
        check("cusparseCsrSetPointers", unsafe {
            sys::cusparseCsrSetPointers(
                self.raw,
                row_offsets.cu_deviceptr() as *mut c_void,
                col_ind.cu_deviceptr() as *mut c_void,
                values.cu_deviceptr() as *mut c_void,
            )
        })?;
        self.nnz = nnz;
        Ok(())
    }

    /// Queries `(rows, cols, nnz)` directly from cuSPARSE (`cusparseSpMatGetSize`).
    pub fn query_size(&self) -> Result<(i64, i64, i64)> {
        let (mut r, mut c, mut n) = (0i64, 0i64, 0i64);
        // SAFETY: `self.raw` is valid and out-pointers are non-null.
        check("cusparseSpMatGetSize", unsafe { sys::cusparseSpMatGetSize(self.raw.cast_const(), &mut r, &mut c, &mut n) })?;
        Ok((r, c, n))
    }

    /// Sets the triangular fill mode attribute (`CUSPARSE_SPMAT_FILL_MODE`) for `SpSV`/`SpSM`.
    pub fn set_fill_mode(&mut self, fill: FillMode) -> Result<()> {
        let mut raw = fill.to_sys();
        // SAFETY: `raw` is a `cusparseFillMode_t` value of size `size_of_val(&raw)`.
        check("cusparseSpMatSetAttribute", unsafe {
            sys::cusparseSpMatSetAttribute(
                self.raw,
                sys::cusparseSpMatAttribute_t_CUSPARSE_SPMAT_FILL_MODE,
                (&raw mut raw).cast::<c_void>(),
                std::mem::size_of_val(&raw),
            )
        })
    }

    /// Sets the diagonal unitness attribute (`CUSPARSE_SPMAT_DIAG_TYPE`) for `SpSV`/`SpSM`.
    pub fn set_diag_type(&mut self, diag: DiagType) -> Result<()> {
        let mut raw = diag.to_sys();
        // SAFETY: `raw` is a `cusparseDiagType_t` value of size `size_of_val(&raw)`.
        check("cusparseSpMatSetAttribute", unsafe {
            sys::cusparseSpMatSetAttribute(
                self.raw,
                sys::cusparseSpMatAttribute_t_CUSPARSE_SPMAT_DIAG_TYPE,
                (&raw mut raw).cast::<c_void>(),
                std::mem::size_of_val(&raw),
            )
        })
    }

    /// Raw `cusparseSpMatDescr_t`.
    pub fn raw(&self) -> sys::cusparseSpMatDescr_t {
        self.raw
    }

    /// Raw `cusparseConstSpMatDescr_t`.
    pub fn raw_const(&self) -> sys::cusparseConstSpMatDescr_t {
        self.raw.cast_const()
    }

    /// Matrix row count.
    pub fn rows(&self) -> i64 {
        self.rows
    }

    /// Matrix column count.
    pub fn cols(&self) -> i64 {
        self.cols
    }

    /// Matrix non-zero count (or `rows * ell_cols` for Blocked-ELL).
    pub fn nnz(&self) -> i64 {
        self.nnz
    }

    /// Sparse storage format.
    pub fn format(&self) -> SparseFormat {
        self.format
    }

    /// Element value type.
    pub fn value_type(&self) -> DataType {
        self.value_type
    }

    /// Owning CUDA context.
    pub fn cuda(&self) -> &Arc<Context> {
        &self.cuda
    }
}

impl Drop for SpMatDescr {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusparseDestroySpMat", || {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                unsafe { sys::cusparseDestroySpMat(raw.cast_const()) }
            });
        }
    }
}

/// Owning wrapper around `cusparseDnVecDescr_t`.
pub struct DnVecDescr {
    raw: sys::cusparseDnVecDescr_t,
    cuda: Arc<Context>,
    len: i64,
    value_type: DataType,
}

// SAFETY: cuSPARSE descriptors are metadata handles; thread safety is governed by caller sync.
unsafe impl Send for DnVecDescr {}

impl DnVecDescr {
    /// Creates a dense vector descriptor wrapping `values`.
    ///
    /// # Safety
    ///
    /// `values` must remain allocated and valid on `cuda` for as long as this descriptor is used.
    pub unsafe fn new<V: SparseScalar>(cuda: &Arc<Context>, values: &DeviceBuffer<V>) -> Result<Self> {
        let len = i64::try_from(values.len()).map_err(|_| Error::InvalidInput("dense vector length exceeds i64::MAX"))?;
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseDnVecDescr_t = std::ptr::null_mut();
        // SAFETY: `values` is a live device buffer of `len` elements of `V::DATA_TYPE`.
        check("cusparseCreateDnVec", unsafe {
            sys::cusparseCreateDnVec(&mut raw, len, values.cu_deviceptr() as *mut c_void, V::DATA_TYPE.to_sys())
        })?;
        Ok(Self { raw, cuda: cuda.clone(), len, value_type: V::DATA_TYPE })
    }

    /// Raw `cusparseDnVecDescr_t`.
    pub fn raw(&self) -> sys::cusparseDnVecDescr_t {
        self.raw
    }

    /// Raw `cusparseConstDnVecDescr_t`.
    pub fn raw_const(&self) -> sys::cusparseConstDnVecDescr_t {
        self.raw.cast_const()
    }

    /// Number of elements in the vector.
    pub fn len(&self) -> i64 {
        self.len
    }

    /// Returns `true` if the vector has zero elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Element scalar type.
    pub fn value_type(&self) -> DataType {
        self.value_type
    }
}

impl Drop for DnVecDescr {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusparseDestroyDnVec", || {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                unsafe { sys::cusparseDestroyDnVec(raw.cast_const()) }
            });
        }
    }
}

/// Owning wrapper around `cusparseDnMatDescr_t`.
pub struct DnMatDescr {
    raw: sys::cusparseDnMatDescr_t,
    cuda: Arc<Context>,
    rows: i64,
    cols: i64,
    ld: i64,
    order: Order,
    value_type: DataType,
}

// SAFETY: cuSPARSE descriptors are metadata handles; thread safety is governed by caller sync.
unsafe impl Send for DnMatDescr {}

impl DnMatDescr {
    /// Creates a dense matrix descriptor wrapping `values` with `rows`, `cols`, leading dimension
    /// `ld`, and storage `order`.
    ///
    /// # Safety
    ///
    /// `values` must remain allocated and valid on `cuda` for as long as this descriptor is used.
    pub unsafe fn new<V: SparseScalar>(
        cuda: &Arc<Context>,
        rows: i64,
        cols: i64,
        ld: i64,
        values: &DeviceBuffer<V>,
        order: Order,
    ) -> Result<Self> {
        let val_bytes = values
            .len()
            .checked_mul(std::mem::size_of::<V>())
            .ok_or(Error::InvalidInput("dense matrix byte size overflow"))?;
        validate_dn_mat(rows, cols, ld, order, val_bytes, V::DATA_TYPE)?;
        ensure_loaded()?;
        cuda.bind_to_thread()?;
        let mut raw: sys::cusparseDnMatDescr_t = std::ptr::null_mut();
        // SAFETY: dimensions and buffer length were validated above.
        check("cusparseCreateDnMat", unsafe {
            sys::cusparseCreateDnMat(
                &mut raw,
                rows,
                cols,
                ld,
                values.cu_deviceptr() as *mut c_void,
                V::DATA_TYPE.to_sys(),
                order.to_sys(),
            )
        })?;
        Ok(Self { raw, cuda: cuda.clone(), rows, cols, ld, order, value_type: V::DATA_TYPE })
    }

    /// Raw `cusparseDnMatDescr_t`.
    pub fn raw(&self) -> sys::cusparseDnMatDescr_t {
        self.raw
    }

    /// Raw `cusparseConstDnMatDescr_t`.
    pub fn raw_const(&self) -> sys::cusparseConstDnMatDescr_t {
        self.raw.cast_const()
    }

    /// Number of rows.
    pub fn rows(&self) -> i64 {
        self.rows
    }

    /// Number of columns.
    pub fn cols(&self) -> i64 {
        self.cols
    }

    /// Leading dimension.
    pub fn ld(&self) -> i64 {
        self.ld
    }

    /// Storage order.
    pub fn order(&self) -> Order {
        self.order
    }

    /// Element scalar type.
    pub fn value_type(&self) -> DataType {
        self.value_type
    }
}

impl Drop for DnMatDescr {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            let raw = self.raw;
            self.raw = std::ptr::null_mut();
            teardown(&self.cuda, "cusparseDestroyDnMat", || {
                // SAFETY: `raw` is owned by `self` and destroyed once.
                unsafe { sys::cusparseDestroyDnMat(raw.cast_const()) }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csr_validation_rejects_negative_and_undersized_buffers() {
        assert!(matches!(
            validate_csr(-1, 3, 2, 16, 8, 8, IndexType::I32, IndexType::I32, DataType::F32),
            Err(Error::InvalidInput(_))
        ));
        // 3 rows needs (3+1)*4 = 16 bytes for row_offsets
        assert!(matches!(
            validate_csr(3, 3, 2, 12, 8, 8, IndexType::I32, IndexType::I32, DataType::F32),
            Err(Error::InvalidInput(_))
        ));
        // nnz=2 needs 2*4 = 8 bytes for col_ind and values
        assert!(matches!(
            validate_csr(3, 3, 2, 16, 4, 8, IndexType::I32, IndexType::I32, DataType::F32),
            Err(Error::InvalidInput(_))
        ));
        assert!(matches!(
            validate_csr(3, 3, 2, 16, 8, 4, IndexType::I32, IndexType::I32, DataType::F32),
            Err(Error::InvalidInput(_))
        ));
        assert!(validate_csr(3, 3, 2, 16, 8, 8, IndexType::I32, IndexType::I32, DataType::F32).is_ok());
    }

    #[test]
    fn csc_and_coo_validation_checks() {
        // CSC: 4 cols needs (4+1)*4 = 20 bytes
        assert!(validate_csc(3, 4, 2, 16, 8, 8, IndexType::I32, IndexType::I32, DataType::F32).is_err());
        assert!(validate_csc(3, 4, 2, 20, 8, 8, IndexType::I32, IndexType::I32, DataType::F32).is_ok());
        // COO: nnz=5 needs 5*8=40 bytes for I64 indices and 5*8=40 bytes for F64 values
        assert!(validate_coo(4, 4, 5, 32, 40, 40, IndexType::I64, DataType::F64).is_err());
        assert!(validate_coo(4, 4, 5, 40, 40, 40, IndexType::I64, DataType::F64).is_ok());
    }

    #[test]
    fn blocked_ell_validation_enforces_block_multiples() {
        // rows=5 not divisible by block_size=2
        assert!(validate_blocked_ell(5, 4, 2, 2, 64, 64, IndexType::I32, DataType::F32).is_err());
        // block_size=0 invalid
        assert!(validate_blocked_ell(4, 4, 0, 2, 64, 64, IndexType::I32, DataType::F32).is_err());
        // valid: 4x4, block_size=2, ell_cols=2 -> col_ind = (2*1)*4 = 8 bytes, values = 4*2*4 = 32 bytes
        assert!(validate_blocked_ell(4, 4, 2, 2, 8, 32, IndexType::I32, DataType::F32).is_ok());
    }

    #[test]
    fn dn_mat_validation_checks_leading_dimension() {
        assert!(validate_dn_mat(4, 3, 3, Order::Col, 48, DataType::F32).is_err());
        assert!(validate_dn_mat(4, 3, 4, Order::Col, 48, DataType::F32).is_ok());
        assert!(validate_dn_mat(4, 3, 2, Order::Row, 48, DataType::F32).is_err());
        assert!(validate_dn_mat(4, 3, 3, Order::Row, 48, DataType::F32).is_ok());
    }
}
