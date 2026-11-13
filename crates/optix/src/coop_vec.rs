//! Cooperative vectors, host side: sizing and converting the weight matrices that device code
//! multiplies with `optixCoopVecMatMul`.
//!
//! **What.** [`matrix_size`] asks OptiX how many bytes an `N x K` matrix takes in a layout and
//! element type (`optixCoopVecMatrixComputeSize`; results are multiples of [`MATRIX_ALIGN`]). A
//! [`Network`] describes the matrices of one network inside a byte buffer
//! (`OptixNetworkDescription`): [`Network::packed`] lays them out back to back at aligned offsets
//! with sizes from OptiX, [`Network::new`] takes an explicit layout. [`convert`] runs
//! `optixCoopVecMatrixConvert` on a stream and turns `count` networks of one layout / element
//! type (typically row-major `f32` from training) into another (typically inferencing-optimal
//! `f16` or `f8`).
//! **Checks** before OptiX sees anything: shapes match matrix by matrix, the element-type pair is
//! one OptiX converts, 8-bit float outputs use an optimal layout, offsets, base addresses and
//! network strides are [`MATRIX_ALIGN`]-aligned, every matrix is at least the size OptiX
//! computes, and the networks lie inside their buffers without the source and destination
//! overlapping. **Used by** neural shading in render-rs and the nv-rs OptiX GPU tests.

use crate::{DeviceContext, Error, Result, check, entry, sys};
use cuda_core::{DeviceBuffer, DeviceCopy, Stream};

/// Alignment of matrix offsets, network base addresses and network strides: the 64 bytes
/// `optix_host.h` states for `optixCoopVecMatrixConvert` and `optixCoopVecMatrixComputeSize`
/// (prose only; the SDK has no macro for it).
pub const MATRIX_ALIGN: usize = 64;

/// Matrix element type (`OptixCoopVecElemType`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElemType {
    /// 16-bit float.
    F16,
    /// 32-bit float.
    F32,
    /// 8-bit unsigned integer.
    U8,
    /// 8-bit signed integer.
    I8,
    /// 32-bit unsigned integer.
    U32,
    /// 32-bit signed integer.
    I32,
    /// 8-bit float, 4 exponent and 3 mantissa bits.
    F8E4M3,
    /// 8-bit float, 5 exponent and 2 mantissa bits.
    F8E5M2,
}

impl ElemType {
    fn to_sys(self) -> sys::OptixCoopVecElemType {
        match self {
            Self::F16 => sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_FLOAT16,
            Self::F32 => sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_FLOAT32,
            Self::U8 => sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_UINT8,
            Self::I8 => sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_INT8,
            Self::U32 => sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_UINT32,
            Self::I32 => sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_INT32,
            Self::F8E4M3 => sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_FLOAT8_E4M3,
            Self::F8E5M2 => sys::OptixCoopVecElemType_OPTIX_COOP_VEC_ELEM_TYPE_FLOAT8_E5M2,
        }
    }

    /// Bytes per element.
    pub fn bytes(self) -> u32 {
        match self {
            Self::F16 => size_of::<u16>() as u32,
            Self::F32 => size_of::<f32>() as u32,
            Self::U8 | Self::I8 | Self::F8E4M3 | Self::F8E5M2 => size_of::<u8>() as u32,
            Self::U32 | Self::I32 => size_of::<u32>() as u32,
        }
    }

    /// Float precision rank (8-bit < 16-bit < 32-bit); `None` for integers.
    fn float_rank(self) -> Option<u8> {
        match self {
            Self::F8E4M3 | Self::F8E5M2 => Some(0),
            Self::F16 => Some(1),
            Self::F32 => Some(2),
            Self::U8 | Self::I8 | Self::U32 | Self::I32 => None,
        }
    }
}

/// Matrix memory layout (`OptixCoopVecMatrixLayout`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MatrixLayout {
    /// Rows of `K` elements, `stride` bytes apart.
    RowMajor,
    /// Columns of `N` elements, `stride` bytes apart.
    ColumnMajor,
    /// Opaque layout fastest for `optixCoopVecMatMul`.
    InferencingOptimal,
    /// Opaque layout fastest for `optixCoopVecOuterProductAccumulate`.
    TrainingOptimal,
}

impl MatrixLayout {
    fn to_sys(self) -> sys::OptixCoopVecMatrixLayout {
        match self {
            Self::RowMajor => sys::OptixCoopVecMatrixLayout_OPTIX_COOP_VEC_MATRIX_LAYOUT_ROW_MAJOR,
            Self::ColumnMajor => sys::OptixCoopVecMatrixLayout_OPTIX_COOP_VEC_MATRIX_LAYOUT_COLUMN_MAJOR,
            Self::InferencingOptimal => sys::OptixCoopVecMatrixLayout_OPTIX_COOP_VEC_MATRIX_LAYOUT_INFERENCING_OPTIMAL,
            Self::TrainingOptimal => sys::OptixCoopVecMatrixLayout_OPTIX_COOP_VEC_MATRIX_LAYOUT_TRAINING_OPTIMAL,
        }
    }

    /// An opaque layout: OptiX ignores the row/column stride.
    pub fn is_optimal(self) -> bool {
        matches!(self, Self::InferencingOptimal | Self::TrainingOptimal)
    }
}

/// Bytes of a packed row (row-major) or column (column-major); 0 for optimal layouts.
fn packed_stride(n: u32, k: u32, elem: ElemType, layout: MatrixLayout) -> Result<u32> {
    let stride = match layout {
        MatrixLayout::RowMajor => k.checked_mul(elem.bytes()),
        MatrixLayout::ColumnMajor => n.checked_mul(elem.bytes()),
        MatrixLayout::InferencingOptimal | MatrixLayout::TrainingOptimal => Some(0),
    };
    stride.ok_or(Error::Invalid("matrix row or column exceeds 4 GiB"))
}

/// Rejects a stride OptiX would misread: optimal layouts take 0, the others at least a packed
/// row or column.
fn check_stride(n: u32, k: u32, elem: ElemType, layout: MatrixLayout, stride: usize) -> Result<()> {
    let packed = packed_stride(n, k, elem, layout)? as usize;
    if layout.is_optimal() && stride != 0 {
        return Err(Error::Invalid("optimal matrix layouts take stride 0"));
    }
    if !layout.is_optimal() && stride < packed {
        return Err(Error::Invalid("matrix stride is smaller than a packed row or column"));
    }
    Ok(())
}

/// One `N x K` matrix of a network: `optixCoopVecMatMul` maps a `K`-vector to an `N`-vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix {
    /// Rows: the output vector length.
    pub n: u32,
    /// Columns: the input vector length.
    pub k: u32,
    /// Element type.
    pub elem: ElemType,
    /// Memory layout.
    pub layout: MatrixLayout,
    /// Bytes between rows (row-major) or columns (column-major), at least a packed one; 0 for
    /// optimal layouts.
    pub stride: u32,
    /// Byte offset from the network's base, a multiple of [`MATRIX_ALIGN`].
    pub offset: u32,
    /// Bytes the matrix occupies (at least what [`matrix_size`] reports).
    pub size: u32,
}

impl Matrix {
    fn to_sys(self) -> sys::OptixCoopVecMatrixDescription {
        sys::OptixCoopVecMatrixDescription {
            N: self.n,
            K: self.k,
            offsetInBytes: self.offset,
            elementType: self.elem.to_sys(),
            layout: self.layout.to_sys(),
            rowColumnStrideInBytes: self.stride,
            sizeInBytes: self.size,
        }
    }
}

/// Bytes an `n x k` matrix of `elem` takes in `layout` (`optixCoopVecMatrixComputeSize`), a
/// multiple of [`MATRIX_ALIGN`]. `stride` is the row/column stride for row- and column-major
/// layouts (0 = packed) and must be 0 for optimal layouts.
pub fn matrix_size(ctx: &DeviceContext, n: u32, k: u32, elem: ElemType, layout: MatrixLayout, stride: usize) -> Result<usize> {
    if n == 0 || k == 0 {
        return Err(Error::Invalid("matrix dimensions must be non-zero"));
    }
    if stride != 0 {
        check_stride(n, k, elem, layout, stride)?;
    }
    ctx.cuda().bind_to_thread()?;
    let mut size = 0usize;
    // SAFETY: plain values in, one writable size_t out.
    check("optixCoopVecMatrixComputeSize", unsafe {
        entry!(ctx.table(), optixCoopVecMatrixComputeSize)(ctx.raw(), n, k, elem.to_sys(), layout.to_sys(), stride, &mut size)
    })?;
    Ok(size)
}

/// The matrices of one network inside a byte buffer (`OptixNetworkDescription`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Network {
    matrices: Vec<Matrix>,
    bytes: usize,
}

impl Network {
    /// A network with an explicit layout: non-empty, non-zero shapes and sizes, aligned offsets,
    /// strides valid for each layout, no two matrices overlapping.
    pub fn new(matrices: Vec<Matrix>) -> Result<Self> {
        if matrices.is_empty() {
            return Err(Error::Invalid("a network needs at least one matrix"));
        }
        for m in &matrices {
            if m.n == 0 || m.k == 0 || m.size == 0 {
                return Err(Error::Invalid("matrix dimensions and size must be non-zero"));
            }
            if !(m.offset as usize).is_multiple_of(MATRIX_ALIGN) {
                return Err(Error::Invalid("matrix offset is not a multiple of MATRIX_ALIGN"));
            }
            check_stride(m.n, m.k, m.elem, m.layout, m.stride as usize)?;
        }
        let mut spans: Vec<(u64, u64)> = matrices.iter().map(|m| (u64::from(m.offset), u64::from(m.offset) + u64::from(m.size))).collect();
        spans.sort_unstable();
        if spans.windows(2).any(|w| matches!(w, [a, b] if a.1 > b.0)) {
            return Err(Error::Invalid("network matrices overlap"));
        }
        let end = spans.iter().map(|s| s.1).fold(0, u64::max);
        let bytes = usize::try_from(end).map_err(|_| Error::Invalid("network exceeds the address space"))?;
        Ok(Self { matrices, bytes })
    }

    /// Matrices of `shapes` (`(n, k)` each, in order) in one element type and layout, packed
    /// back to back at [`MATRIX_ALIGN`]-aligned offsets with sizes from [`matrix_size`].
    pub fn packed(ctx: &DeviceContext, shapes: &[(u32, u32)], elem: ElemType, layout: MatrixLayout) -> Result<Self> {
        let too_big = || Error::Invalid("network exceeds 4 GiB");
        let mut matrices = Vec::with_capacity(shapes.len());
        let mut offset = 0usize;
        for &(n, k) in shapes {
            let stride = packed_stride(n, k, elem, layout)?;
            let size = matrix_size(ctx, n, k, elem, layout, stride as usize)?;
            matrices.push(Matrix {
                n,
                k,
                elem,
                layout,
                stride,
                offset: u32::try_from(offset).map_err(|_| too_big())?,
                size: u32::try_from(size).map_err(|_| too_big())?,
            });
            offset = offset.checked_add(size).and_then(|o| o.checked_next_multiple_of(MATRIX_ALIGN)).ok_or_else(too_big)?;
        }
        Self::new(matrices)
    }

    /// The matrices, in network order.
    pub fn matrices(&self) -> &[Matrix] {
        &self.matrices
    }

    /// Bytes from the network's base to the end of its last matrix.
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    fn to_sys(&self) -> Vec<sys::OptixCoopVecMatrixDescription> {
        self.matrices.iter().map(|m| m.to_sys()).collect()
    }
}

/// Networks sharing one [`Network`] layout, `stride` bytes apart in `buffer`.
pub struct Batch<'a, B> {
    /// Layout of every network.
    pub network: &'a Network,
    /// The device buffer holding them, starting at its base.
    pub buffer: B,
    /// Bytes between consecutive networks: a multiple of [`MATRIX_ALIGN`], at least
    /// [`Network::bytes`]; ignored for a single network.
    pub stride: usize,
}

/// Converts `count` networks from `src`'s layout and element types into `dst`'s
/// (`optixCoopVecMatrixConvert`), on `stream`. Element types may differ only between floats, one
/// of them `F32` or `F16` and the other of lower precision; 8-bit float outputs need an optimal
/// layout.
///
/// # Safety
///
/// Both buffers must stay alive, `src` unmodified and `dst` unread, until the work completes on
/// `stream`: synchronize it, or order their reuse and release after this call on it.
pub unsafe fn convert<T: DeviceCopy, U: DeviceCopy>(ctx: &DeviceContext, stream: &Stream, count: u32, src: Batch<'_, &DeviceBuffer<T>>, dst: Batch<'_, &mut DeviceBuffer<U>>) -> Result<()> {
    if count == 0 {
        return Err(Error::Invalid("convert needs at least one network"));
    }
    let (from, to) = (src.network.matrices(), dst.network.matrices());
    if from.len() != to.len() {
        return Err(Error::Invalid("source and destination networks differ in matrix count"));
    }
    for (a, b) in from.iter().zip(to) {
        if (a.n, a.k) != (b.n, b.k) {
            return Err(Error::Invalid("source and destination matrix shapes differ"));
        }
        if !convertible(a.elem, b.elem) {
            return Err(Error::Invalid("element types differ and are not a float pair with F32 or F16 on the higher-precision side"));
        }
        if matches!(b.elem, ElemType::F8E4M3 | ElemType::F8E5M2) && !b.layout.is_optimal() {
            return Err(Error::Invalid("8-bit float matrices must be converted into an optimal layout"));
        }
        for m in [a, b] {
            if (m.size as usize) < matrix_size(ctx, m.n, m.k, m.elem, m.layout, m.stride as usize)? {
                return Err(Error::Invalid("a matrix is smaller than optixCoopVecMatrixComputeSize reports"));
            }
        }
    }
    let (src_lo, src_hi) = extent(src.network, src.buffer.cu_deviceptr(), src.buffer.num_bytes(), count, src.stride)?;
    let (dst_lo, dst_hi) = extent(dst.network, dst.buffer.cu_deviceptr(), dst.buffer.num_bytes(), count, dst.stride)?;
    if src_lo < dst_hi && dst_lo < src_hi {
        return Err(Error::Invalid("source and destination networks overlap"));
    }
    let (mut src_desc, mut dst_desc) = (src.network.to_sys(), dst.network.to_sys());
    // Bounded by `from.len() == to.len()`, which `Network::new` built from a Vec; checked anyway.
    let layers = u32::try_from(src_desc.len()).map_err(|_| Error::Invalid("too many network matrices"))?;
    let src_net = sys::OptixNetworkDescription { layers: src_desc.as_mut_ptr(), numLayers: layers };
    let dst_net = sys::OptixNetworkDescription { layers: dst_desc.as_mut_ptr(), numLayers: layers };
    ctx.cuda().bind_to_thread()?;
    // SAFETY: descriptions and the arrays they point at outlive the call; both device ranges were
    // checked to lie inside their buffers, aligned and disjoint; the buffers' lifetime on the
    // stream is the caller's contract.
    check("optixCoopVecMatrixConvert", unsafe {
        entry!(ctx.table(), optixCoopVecMatrixConvert)(
            ctx.raw(),
            stream.cu_stream(),
            count,
            &src_net,
            src.buffer.cu_deviceptr(),
            src.stride,
            &dst_net,
            dst.buffer.cu_deviceptr(),
            dst.stride,
        )
    })
}

/// The element-type pairs `optixCoopVecMatrixConvert` accepts: equal, or two floats of
/// different precision (so the higher one is `F16` or `F32`).
fn convertible(from: ElemType, to: ElemType) -> bool {
    if from == to {
        return true;
    }
    matches!((from.float_rank(), to.float_rank()), (Some(a), Some(b)) if a != b)
}

/// Device byte range `count` networks take in a buffer at `base` holding `len` bytes.
fn extent(network: &Network, base: sys::CUdeviceptr, len: usize, count: u32, stride: usize) -> Result<(u64, u64)> {
    if !base.is_multiple_of(MATRIX_ALIGN as u64) {
        return Err(Error::Invalid("network buffer base is not MATRIX_ALIGN-aligned"));
    }
    let span = if count > 1 {
        if stride < network.bytes() || !stride.is_multiple_of(MATRIX_ALIGN) {
            return Err(Error::Invalid("network stride must be a multiple of MATRIX_ALIGN and at least Network::bytes"));
        }
        ((count - 1) as usize).checked_mul(stride).and_then(|s| s.checked_add(network.bytes())).ok_or(Error::Invalid("networks exceed the address space"))?
    } else {
        network.bytes()
    };
    if span > len {
        return Err(Error::Invalid("networks extend past the end of their buffer"));
    }
    Ok((base, base.saturating_add(span as u64)))
}
