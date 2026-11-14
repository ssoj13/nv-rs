//! RAII descriptors for tensors, filters, convolutions, activations, and pooling windows.
//!
//! N-dimensional descriptors copy dimensions and strides into fixed [`MAX_DIMS`]-element stack
//! arrays so descriptor creation and queries perform zero heap allocations.

use std::ptr;
use std::sync::Arc;

use cuda_core::Context;
use cudnn_sys as sys;

use crate::{
    check, checked_i32, record_teardown, DataType, Error, Handle, NanPropagation, Result,
    TensorFormat,
};

/// Maximum tensor rank supported by cuDNN (`CUDNN_DIM_MAX`).
pub const MAX_DIMS: usize = 8;

/// Convolution algorithm mode (`cudnnConvolutionMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConvMode {
    /// Mathematical convolution with kernel flip (`CUDNN_CONVOLUTION`).
    Convolution,
    /// Correlation without kernel flip (`CUDNN_CROSS_CORRELATION`), standard in CNNs.
    CrossCorrelation,
}

impl ConvMode {
    /// Converts to the underlying `cudnnConvolutionMode_t`.
    pub const fn to_sys(self) -> sys::cudnnConvolutionMode_t {
        match self {
            Self::Convolution => sys::cudnnConvolutionMode_t_CUDNN_CONVOLUTION,
            Self::CrossCorrelation => sys::cudnnConvolutionMode_t_CUDNN_CROSS_CORRELATION,
        }
    }
}

/// Math precision policy (`cudnnMathType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MathType {
    /// Default IEEE/library precision (`CUDNN_DEFAULT_MATH`).
    Default,
    /// Allow Tensor Core execution without implicit downcasting (`CUDNN_TENSOR_OP_MATH`).
    TensorOp,
    /// Allow Tensor Core execution with downcasting (`CUDNN_TENSOR_OP_MATH_ALLOW_CONVERSION`).
    TensorOpAllowConversion,
    /// Restrict to FMA instructions (`CUDNN_FMA_MATH`).
    Fma,
}

impl MathType {
    /// Converts to the underlying `cudnnMathType_t`.
    pub const fn to_sys(self) -> sys::cudnnMathType_t {
        match self {
            Self::Default => sys::cudnnMathType_t_CUDNN_DEFAULT_MATH,
            Self::TensorOp => sys::cudnnMathType_t_CUDNN_TENSOR_OP_MATH,
            Self::TensorOpAllowConversion => {
                sys::cudnnMathType_t_CUDNN_TENSOR_OP_MATH_ALLOW_CONVERSION
            }
            Self::Fma => sys::cudnnMathType_t_CUDNN_FMA_MATH,
        }
    }
}

/// Forward convolution algorithm (`cudnnConvolutionFwdAlgo_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConvFwdAlgo {
    /// Implicit GEMM without forming the input matrix (`CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM`).
    ImplicitGemm,
    /// Implicit GEMM with precomputed indices (`CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM`).
    ImplicitPrecompGemm,
    /// Explicit GEMM (`CUDNN_CONVOLUTION_FWD_ALGO_GEMM`).
    Gemm,
    /// Direct convolution (`CUDNN_CONVOLUTION_FWD_ALGO_DIRECT`).
    Direct,
    /// FFT-based convolution (`CUDNN_CONVOLUTION_FWD_ALGO_FFT`).
    Fft,
    /// Tiled FFT convolution (`CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING`).
    FftTiling,
    /// Winograd transform (`CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD`).
    Winograd,
    /// Non-fused Winograd transform (`CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED`).
    WinogradNonfused,
}

impl ConvFwdAlgo {
    /// Converts to the underlying `cudnnConvolutionFwdAlgo_t`.
    pub const fn to_sys(self) -> sys::cudnnConvolutionFwdAlgo_t {
        match self {
            Self::ImplicitGemm => sys::cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM,
            Self::ImplicitPrecompGemm => {
                sys::cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM
            }
            Self::Gemm => sys::cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_GEMM,
            Self::Direct => sys::cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_DIRECT,
            Self::Fft => sys::cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_FFT,
            Self::FftTiling => sys::cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING,
            Self::Winograd => sys::cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD,
            Self::WinogradNonfused => {
                sys::cudnnConvolutionFwdAlgo_t_CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED
            }
        }
    }
}

/// Activation function kind (`cudnnActivationMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActivationMode {
    /// Logistic sigmoid (`CUDNN_ACTIVATION_SIGMOID`).
    Sigmoid,
    /// Rectified linear unit (`CUDNN_ACTIVATION_RELU`).
    Relu,
    /// Hyperbolic tangent (`CUDNN_ACTIVATION_TANH`).
    Tanh,
    /// Clipped ReLU (`CUDNN_ACTIVATION_CLIPPED_RELU`).
    ClippedRelu,
    /// Exponential linear unit (`CUDNN_ACTIVATION_ELU`).
    Elu,
    /// Identity pass-through (`CUDNN_ACTIVATION_IDENTITY`).
    Identity,
    /// Swish (`CUDNN_ACTIVATION_SWISH`).
    Swish,
}

impl ActivationMode {
    /// Converts to the underlying `cudnnActivationMode_t`.
    pub const fn to_sys(self) -> sys::cudnnActivationMode_t {
        match self {
            Self::Sigmoid => sys::cudnnActivationMode_t_CUDNN_ACTIVATION_SIGMOID,
            Self::Relu => sys::cudnnActivationMode_t_CUDNN_ACTIVATION_RELU,
            Self::Tanh => sys::cudnnActivationMode_t_CUDNN_ACTIVATION_TANH,
            Self::ClippedRelu => sys::cudnnActivationMode_t_CUDNN_ACTIVATION_CLIPPED_RELU,
            Self::Elu => sys::cudnnActivationMode_t_CUDNN_ACTIVATION_ELU,
            Self::Identity => sys::cudnnActivationMode_t_CUDNN_ACTIVATION_IDENTITY,
            Self::Swish => sys::cudnnActivationMode_t_CUDNN_ACTIVATION_SWISH,
        }
    }
}

/// Pooling reduction mode (`cudnnPoolingMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PoolingMode {
    /// Maximum over the window (`CUDNN_POOLING_MAX`).
    Max,
    /// Average including padded positions (`CUDNN_POOLING_AVERAGE_COUNT_INCLUDE_PADDING`).
    AverageCountIncludePadding,
    /// Average excluding padded positions (`CUDNN_POOLING_AVERAGE_COUNT_EXCLUDE_PADDING`).
    AverageCountExcludePadding,
    /// Deterministic max pooling (`CUDNN_POOLING_MAX_DETERMINISTIC`).
    MaxDeterministic,
}

impl PoolingMode {
    /// Converts to the underlying `cudnnPoolingMode_t`.
    pub const fn to_sys(self) -> sys::cudnnPoolingMode_t {
        match self {
            Self::Max => sys::cudnnPoolingMode_t_CUDNN_POOLING_MAX,
            Self::AverageCountIncludePadding => {
                sys::cudnnPoolingMode_t_CUDNN_POOLING_AVERAGE_COUNT_INCLUDE_PADDING
            }
            Self::AverageCountExcludePadding => {
                sys::cudnnPoolingMode_t_CUDNN_POOLING_AVERAGE_COUNT_EXCLUDE_PADDING
            }
            Self::MaxDeterministic => sys::cudnnPoolingMode_t_CUDNN_POOLING_MAX_DETERMINISTIC,
        }
    }
}

/// Copies a slice into a stack array up to `MAX_DIMS` entries, checking every value fits `i32`.
fn copy_i32_dims(src: &[usize], out: &mut [i32; MAX_DIMS], what: &'static str) -> Result<i32> {
    if src.is_empty() || src.len() > MAX_DIMS {
        return Err(Error::Invalid("tensor/descriptor rank must be in 1..=8"));
    }
    for (dst, &val) in out.iter_mut().zip(src.iter()) {
        *dst = checked_i32(val, what)?;
    }
    Ok(src.len() as i32)
}

/// Owned `cudnnTensorDescriptor_t` describing tensor layout and element type.
pub struct TensorDescriptor {
    raw: sys::cudnnTensorDescriptor_t,
    ctx: Arc<Context>,
    dtype: DataType,
    size_bytes: usize,
}

// SAFETY: Descriptors are immutable after construction and destroyed once on Drop; sharing
// read-only references across threads is safe when synchronized by caller.
unsafe impl Send for TensorDescriptor {}
// SAFETY: `TensorDescriptor` is immutable after construction (`&self` methods only read descriptor metadata).
unsafe impl Sync for TensorDescriptor {}

impl TensorDescriptor {
    /// Allocates an unconfigured tensor descriptor (for derived descriptors such as BatchNorm).
    pub(crate) fn create_empty() -> Result<sys::cudnnTensorDescriptor_t> {
        sys::load()?;
        let mut raw: sys::cudnnTensorDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("cudnnCreateTensorDescriptor", unsafe { sys::cudnnCreateTensorDescriptor(&mut raw) })?;
        Ok(raw)
    }

    /// Creates a 4D tensor descriptor (`[N, C, H, W]`).
    pub fn new_4d(
        ctx: &Arc<Context>,
        format: TensorFormat,
        dtype: DataType,
        dims: [usize; 4],
    ) -> Result<Self> {
        if dims.contains(&0) {
            return Err(Error::Invalid("tensor dimensions must be non-zero"));
        }
        let n = checked_i32(dims[0], "tensor n exceeds i32")?;
        let c = checked_i32(dims[1], "tensor c exceeds i32")?;
        let h = checked_i32(dims[2], "tensor h exceeds i32")?;
        let w = checked_i32(dims[3], "tensor w exceeds i32")?;
        let raw = Self::create_empty()?;
        // SAFETY: `raw` was created above and cleaned up if configuration fails.
        let status = unsafe {
            sys::cudnnSetTensor4dDescriptor(raw, format.to_sys(), dtype.to_sys(), n, c, h, w)
        };
        if let Err(err) = check("cudnnSetTensor4dDescriptor", status) {
            // SAFETY: `raw` is valid and unowned on error.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyTensorDescriptor(raw));
            }
            return Err(err);
        }
        Self::finish(ctx, raw, dtype)
    }

    /// Creates an N-dimensional tensor descriptor with explicit strides (`1..=8` dimensions).
    pub fn new_nd(
        ctx: &Arc<Context>,
        dtype: DataType,
        dims: &[usize],
        strides: &[usize],
    ) -> Result<Self> {
        if dims.len() != strides.len() {
            return Err(Error::Invalid("tensor dims and strides must have matching rank"));
        }
        if dims.contains(&0) {
            return Err(Error::Invalid("tensor dimensions must be non-zero"));
        }
        let mut dim_a = [0i32; MAX_DIMS];
        let mut stride_a = [0i32; MAX_DIMS];
        let nb_dims = copy_i32_dims(dims, &mut dim_a, "tensor dim exceeds i32")?;
        let _ = copy_i32_dims(strides, &mut stride_a, "tensor stride exceeds i32")?;
        let raw = Self::create_empty()?;
        // SAFETY: `raw` is valid and both arrays hold `nb_dims` initialized elements.
        let status = unsafe {
            sys::cudnnSetTensorNdDescriptor(
                raw,
                dtype.to_sys(),
                nb_dims,
                dim_a.as_ptr(),
                stride_a.as_ptr(),
            )
        };
        if let Err(err) = check("cudnnSetTensorNdDescriptor", status) {
            // SAFETY: `raw` is valid and unowned on error.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyTensorDescriptor(raw));
            }
            return Err(err);
        }
        Self::finish(ctx, raw, dtype)
    }

    pub(crate) fn finish(
        ctx: &Arc<Context>,
        raw: sys::cudnnTensorDescriptor_t,
        dtype: DataType,
    ) -> Result<Self> {
        let mut size_bytes = 0usize;
        // SAFETY: `raw` is initialized and `size_bytes` is a valid out-pointer.
        let status = unsafe { sys::cudnnGetTensorSizeInBytes(raw, &mut size_bytes) };
        if let Err(err) = check("cudnnGetTensorSizeInBytes", status) {
            // SAFETY: `raw` is valid and unowned on error.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyTensorDescriptor(raw));
            }
            return Err(err);
        }
        Ok(Self { raw, ctx: ctx.clone(), dtype, size_bytes })
    }

    /// Raw `cudnnTensorDescriptor_t`.
    pub fn as_raw(&self) -> sys::cudnnTensorDescriptor_t {
        self.raw
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    /// Tensor element data type.
    pub fn data_type(&self) -> DataType {
        self.dtype
    }

    /// Minimum device buffer span in bytes required by cuDNN for this descriptor.
    pub fn size_in_bytes(&self) -> usize {
        self.size_bytes
    }
}

impl Drop for TensorDescriptor {
    fn drop(&mut self) {
        // SAFETY: `self.raw` was created by `cudnnCreateTensorDescriptor` and is dropped once.
        let code = unsafe { sys::cudnnDestroyTensorDescriptor(self.raw) };
        record_teardown(&self.ctx, code);
    }
}

/// Owned `cudnnFilterDescriptor_t` describing convolution kernel weights.
pub struct FilterDescriptor {
    raw: sys::cudnnFilterDescriptor_t,
    ctx: Arc<Context>,
    dtype: DataType,
    size_bytes: usize,
}

// SAFETY: Descriptors are immutable after construction and destroyed once on Drop.
unsafe impl Send for FilterDescriptor {}
// SAFETY: `FilterDescriptor` exposes read-only descriptor metadata after construction.
unsafe impl Sync for FilterDescriptor {}

impl FilterDescriptor {
    /// Creates a 4D filter descriptor (`[K, C, H, W]`).
    pub fn new_4d(
        ctx: &Arc<Context>,
        format: TensorFormat,
        dtype: DataType,
        dims: [usize; 4],
    ) -> Result<Self> {
        if dims.contains(&0) {
            return Err(Error::Invalid("filter dimensions must be non-zero"));
        }
        sys::load()?;
        let k = checked_i32(dims[0], "filter k exceeds i32")?;
        let c = checked_i32(dims[1], "filter c exceeds i32")?;
        let h = checked_i32(dims[2], "filter h exceeds i32")?;
        let w = checked_i32(dims[3], "filter w exceeds i32")?;

        let mut raw: sys::cudnnFilterDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("cudnnCreateFilterDescriptor", unsafe { sys::cudnnCreateFilterDescriptor(&mut raw) })?;
        // SAFETY: `raw` was initialized above.
        let status =
            unsafe { sys::cudnnSetFilter4dDescriptor(raw, dtype.to_sys(), format.to_sys(), k, c, h, w) };
        if let Err(err) = check("cudnnSetFilter4dDescriptor", status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyFilterDescriptor(raw));
            }
            return Err(err);
        }
        let mut size_bytes = 0usize;
        // SAFETY: `raw` is configured and `size_bytes` is a valid out-pointer.
        let sz_status = unsafe { sys::cudnnGetFilterSizeInBytes(raw, &mut size_bytes) };
        if let Err(err) = check("cudnnGetFilterSizeInBytes", sz_status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyFilterDescriptor(raw));
            }
            return Err(err);
        }
        Ok(Self { raw, ctx: ctx.clone(), dtype, size_bytes })
    }

    /// Creates an N-dimensional filter descriptor (`1..=8` dimensions).
    pub fn new_nd(
        ctx: &Arc<Context>,
        format: TensorFormat,
        dtype: DataType,
        dims: &[usize],
    ) -> Result<Self> {
        if dims.contains(&0) {
            return Err(Error::Invalid("filter dimensions must be non-zero"));
        }
        sys::load()?;
        let mut dim_a = [0i32; MAX_DIMS];
        let nb_dims = copy_i32_dims(dims, &mut dim_a, "filter dim exceeds i32")?;
        let mut raw: sys::cudnnFilterDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("cudnnCreateFilterDescriptor", unsafe { sys::cudnnCreateFilterDescriptor(&mut raw) })?;
        // SAFETY: `raw` is live and `dim_a` contains `nb_dims` valid elements.
        let status = unsafe {
            sys::cudnnSetFilterNdDescriptor(raw, dtype.to_sys(), format.to_sys(), nb_dims, dim_a.as_ptr())
        };
        if let Err(err) = check("cudnnSetFilterNdDescriptor", status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyFilterDescriptor(raw));
            }
            return Err(err);
        }
        let mut size_bytes = 0usize;
        // SAFETY: `raw` is configured and `size_bytes` is a valid out-pointer.
        let sz_status = unsafe { sys::cudnnGetFilterSizeInBytes(raw, &mut size_bytes) };
        if let Err(err) = check("cudnnGetFilterSizeInBytes", sz_status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyFilterDescriptor(raw));
            }
            return Err(err);
        }
        Ok(Self { raw, ctx: ctx.clone(), dtype, size_bytes })
    }

    /// Raw `cudnnFilterDescriptor_t`.
    pub fn as_raw(&self) -> sys::cudnnFilterDescriptor_t {
        self.raw
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    /// Element data type.
    pub fn data_type(&self) -> DataType {
        self.dtype
    }

    /// Minimum device buffer span in bytes required by cuDNN for this filter.
    pub fn size_in_bytes(&self) -> usize {
        self.size_bytes
    }
}

impl Drop for FilterDescriptor {
    fn drop(&mut self) {
        // SAFETY: `self.raw` was created by `cudnnCreateFilterDescriptor` and is dropped once.
        let code = unsafe { sys::cudnnDestroyFilterDescriptor(self.raw) };
        record_teardown(&self.ctx, code);
    }
}

/// Owned `cudnnConvolutionDescriptor_t` specifying padding, strides, dilation, and compute type.
pub struct ConvolutionDescriptor {
    raw: sys::cudnnConvolutionDescriptor_t,
    ctx: Arc<Context>,
}

// SAFETY: Mutated only via `&mut self` (`set_group_count`, `set_math_type`), never shared mutably.
unsafe impl Send for ConvolutionDescriptor {}
// SAFETY: `&self` methods on `ConvolutionDescriptor` are read-only queries in cuDNN.
unsafe impl Sync for ConvolutionDescriptor {}

impl ConvolutionDescriptor {
    /// Creates a 2D convolution descriptor (`pad = [h, w]`, `stride = [h, w]`, `dilation = [h, w]`).
    pub fn new_2d(
        ctx: &Arc<Context>,
        pad: [usize; 2],
        stride: [usize; 2],
        dilation: [usize; 2],
        mode: ConvMode,
        compute_type: DataType,
    ) -> Result<Self> {
        if stride[0] == 0 || stride[1] == 0 || dilation[0] == 0 || dilation[1] == 0 {
            return Err(Error::Invalid("convolution stride and dilation must be non-zero"));
        }
        sys::load()?;
        let pad_h = checked_i32(pad[0], "conv pad_h exceeds i32")?;
        let pad_w = checked_i32(pad[1], "conv pad_w exceeds i32")?;
        let stride_h = checked_i32(stride[0], "conv stride_h exceeds i32")?;
        let stride_w = checked_i32(stride[1], "conv stride_w exceeds i32")?;
        let dil_h = checked_i32(dilation[0], "conv dilation_h exceeds i32")?;
        let dil_w = checked_i32(dilation[1], "conv dilation_w exceeds i32")?;

        let mut raw: sys::cudnnConvolutionDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("cudnnCreateConvolutionDescriptor", unsafe {
            sys::cudnnCreateConvolutionDescriptor(&mut raw)
        })?;
        // SAFETY: `raw` was initialized above.
        let status = unsafe {
            sys::cudnnSetConvolution2dDescriptor(
                raw,
                pad_h,
                pad_w,
                stride_h,
                stride_w,
                dil_h,
                dil_w,
                mode.to_sys(),
                compute_type.to_sys(),
            )
        };
        if let Err(err) = check("cudnnSetConvolution2dDescriptor", status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyConvolutionDescriptor(raw));
            }
            return Err(err);
        }
        Ok(Self { raw, ctx: ctx.clone() })
    }

    /// Creates an N-dimensional convolution descriptor (`1..=6` spatial dimensions; tensor rank `<= 8`).
    pub fn new_nd(
        ctx: &Arc<Context>,
        pad: &[usize],
        stride: &[usize],
        dilation: &[usize],
        mode: ConvMode,
        compute_type: DataType,
    ) -> Result<Self> {
        if pad.len() != stride.len() || pad.len() != dilation.len() {
            return Err(Error::Invalid("convolution pad, stride, and dilation ranks must match"));
        }
        if stride.contains(&0) || dilation.contains(&0) {
            return Err(Error::Invalid("convolution stride and dilation must be non-zero"));
        }
        sys::load()?;
        let mut pad_a = [0i32; MAX_DIMS];
        let mut stride_a = [0i32; MAX_DIMS];
        let mut dil_a = [0i32; MAX_DIMS];
        let rank = copy_i32_dims(pad, &mut pad_a, "conv pad exceeds i32")?;
        let _ = copy_i32_dims(stride, &mut stride_a, "conv stride exceeds i32")?;
        let _ = copy_i32_dims(dilation, &mut dil_a, "conv dilation exceeds i32")?;

        let mut raw: sys::cudnnConvolutionDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("cudnnCreateConvolutionDescriptor", unsafe {
            sys::cudnnCreateConvolutionDescriptor(&mut raw)
        })?;
        // SAFETY: `raw` is initialized and all three arrays hold `rank` initialized `i32`s.
        let status = unsafe {
            sys::cudnnSetConvolutionNdDescriptor(
                raw,
                rank,
                pad_a.as_ptr(),
                stride_a.as_ptr(),
                dil_a.as_ptr(),
                mode.to_sys(),
                compute_type.to_sys(),
            )
        };
        if let Err(err) = check("cudnnSetConvolutionNdDescriptor", status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyConvolutionDescriptor(raw));
            }
            return Err(err);
        }
        Ok(Self { raw, ctx: ctx.clone() })
    }

    /// Configures grouped convolution count (`group_count >= 1`).
    pub fn set_group_count(&mut self, group_count: usize) -> Result<()> {
        if group_count == 0 {
            return Err(Error::Invalid("group_count must be at least 1"));
        }
        let groups = checked_i32(group_count, "group_count exceeds i32")?;
        // SAFETY: `self.raw` is a live convolution descriptor.
        check("cudnnSetConvolutionGroupCount", unsafe {
            sys::cudnnSetConvolutionGroupCount(self.raw, groups)
        })
    }

    /// Configures the Tensor Core / precision math mode.
    pub fn set_math_type(&mut self, math: MathType) -> Result<()> {
        // SAFETY: `self.raw` is a live convolution descriptor.
        check("cudnnSetConvolutionMathType", unsafe {
            sys::cudnnSetConvolutionMathType(self.raw, math.to_sys())
        })
    }

    /// Computes output `[N, C, H, W]` dimensions for a 2D forward convolution.
    pub fn output_dim_4d(
        &self,
        input: &TensorDescriptor,
        filter: &FilterDescriptor,
    ) -> Result<[usize; 4]> {
        if !Arc::ptr_eq(&self.ctx, input.context()) || !Arc::ptr_eq(&self.ctx, filter.context()) {
            return Err(Error::Invalid("descriptors belong to different CUDA contexts"));
        }
        let (mut n, mut c, mut h, mut w) = (0i32, 0i32, 0i32, 0i32);
        // SAFETY: all descriptors are live and out-pointers are valid stack integers.
        let status = unsafe {
            sys::cudnnGetConvolution2dForwardOutputDim(
                self.raw,
                input.as_raw(),
                filter.as_raw(),
                &mut n,
                &mut c,
                &mut h,
                &mut w,
            )
        };
        check("cudnnGetConvolution2dForwardOutputDim", status)?;
        Ok([
            usize::try_from(n).map_err(|_| Error::Invalid("negative output n"))?,
            usize::try_from(c).map_err(|_| Error::Invalid("negative output c"))?,
            usize::try_from(h).map_err(|_| Error::Invalid("negative output h"))?,
            usize::try_from(w).map_err(|_| Error::Invalid("negative output w"))?,
        ])
    }

    /// Queries the forward workspace size in bytes for `algo`.
    pub fn forward_workspace_size(
        &self,
        handle: &Handle,
        x_desc: &TensorDescriptor,
        w_desc: &FilterDescriptor,
        y_desc: &TensorDescriptor,
        algo: ConvFwdAlgo,
    ) -> Result<usize> {
        crate::ops::conv_forward_workspace_size(handle, x_desc, w_desc, self, y_desc, algo)
    }

    /// Raw `cudnnConvolutionDescriptor_t`.
    pub fn as_raw(&self) -> sys::cudnnConvolutionDescriptor_t {
        self.raw
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }
}

impl Drop for ConvolutionDescriptor {
    fn drop(&mut self) {
        // SAFETY: `self.raw` was created by `cudnnCreateConvolutionDescriptor` and dropped once.
        let code = unsafe { sys::cudnnDestroyConvolutionDescriptor(self.raw) };
        record_teardown(&self.ctx, code);
    }
}

/// Owned `cudnnActivationDescriptor_t`.
pub struct ActivationDescriptor {
    raw: sys::cudnnActivationDescriptor_t,
    ctx: Arc<Context>,
}

// SAFETY: Descriptors are immutable after construction and destroyed once on Drop.
unsafe impl Send for ActivationDescriptor {}
// SAFETY: `ActivationDescriptor` exposes read-only handle access after construction.
unsafe impl Sync for ActivationDescriptor {}

impl ActivationDescriptor {
    /// Creates an activation descriptor. `coef` sets the upper bound for [`ActivationMode::ClippedRelu`],
    /// alpha slope for [`ActivationMode::Elu`], or scale for [`ActivationMode::Swish`].
    pub fn new(
        ctx: &Arc<Context>,
        mode: ActivationMode,
        nan_prop: NanPropagation,
        coef: f64,
    ) -> Result<Self> {
        sys::load()?;
        let mut raw: sys::cudnnActivationDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("cudnnCreateActivationDescriptor", unsafe {
            sys::cudnnCreateActivationDescriptor(&mut raw)
        })?;
        // SAFETY: `raw` was initialized above.
        let status =
            unsafe { sys::cudnnSetActivationDescriptor(raw, mode.to_sys(), nan_prop.to_sys(), coef) };
        if let Err(err) = check("cudnnSetActivationDescriptor", status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyActivationDescriptor(raw));
            }
            return Err(err);
        }
        Ok(Self { raw, ctx: ctx.clone() })
    }

    /// Raw `cudnnActivationDescriptor_t`.
    pub fn as_raw(&self) -> sys::cudnnActivationDescriptor_t {
        self.raw
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }
}

impl Drop for ActivationDescriptor {
    fn drop(&mut self) {
        // SAFETY: `self.raw` was created by `cudnnCreateActivationDescriptor` and dropped once.
        let code = unsafe { sys::cudnnDestroyActivationDescriptor(self.raw) };
        record_teardown(&self.ctx, code);
    }
}

/// Owned `cudnnPoolingDescriptor_t`.
pub struct PoolingDescriptor {
    raw: sys::cudnnPoolingDescriptor_t,
    ctx: Arc<Context>,
}

// SAFETY: Descriptors are immutable after construction and destroyed once on Drop.
unsafe impl Send for PoolingDescriptor {}
// SAFETY: `PoolingDescriptor` exposes read-only handle access after construction.
unsafe impl Sync for PoolingDescriptor {}

impl PoolingDescriptor {
    /// Creates a 2D pooling descriptor (`window = [h, w]`, `pad = [h, w]`, `stride = [h, w]`).
    pub fn new_2d(
        ctx: &Arc<Context>,
        mode: PoolingMode,
        nan_prop: NanPropagation,
        window: [usize; 2],
        pad: [usize; 2],
        stride: [usize; 2],
    ) -> Result<Self> {
        if window[0] == 0 || window[1] == 0 || stride[0] == 0 || stride[1] == 0 {
            return Err(Error::Invalid("pooling window and stride must be non-zero"));
        }
        sys::load()?;
        let wh = checked_i32(window[0], "pooling window_h exceeds i32")?;
        let ww = checked_i32(window[1], "pooling window_w exceeds i32")?;
        let ph = checked_i32(pad[0], "pooling pad_h exceeds i32")?;
        let pw = checked_i32(pad[1], "pooling pad_w exceeds i32")?;
        let sh = checked_i32(stride[0], "pooling stride_h exceeds i32")?;
        let sw = checked_i32(stride[1], "pooling stride_w exceeds i32")?;

        let mut raw: sys::cudnnPoolingDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("cudnnCreatePoolingDescriptor", unsafe { sys::cudnnCreatePoolingDescriptor(&mut raw) })?;
        // SAFETY: `raw` was initialized above.
        let status = unsafe {
            sys::cudnnSetPooling2dDescriptor(
                raw,
                mode.to_sys(),
                nan_prop.to_sys(),
                wh,
                ww,
                ph,
                pw,
                sh,
                sw,
            )
        };
        if let Err(err) = check("cudnnSetPooling2dDescriptor", status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyPoolingDescriptor(raw));
            }
            return Err(err);
        }
        Ok(Self { raw, ctx: ctx.clone() })
    }

    /// Creates an N-dimensional pooling descriptor (`1..=6` spatial dimensions).
    pub fn new_nd(
        ctx: &Arc<Context>,
        mode: PoolingMode,
        nan_prop: NanPropagation,
        window: &[usize],
        pad: &[usize],
        stride: &[usize],
    ) -> Result<Self> {
        if window.len() != pad.len() || window.len() != stride.len() {
            return Err(Error::Invalid("pooling window, pad, and stride ranks must match"));
        }
        if window.contains(&0) || stride.contains(&0) {
            return Err(Error::Invalid("pooling window and stride must be non-zero"));
        }
        sys::load()?;
        let mut win_a = [0i32; MAX_DIMS];
        let mut pad_a = [0i32; MAX_DIMS];
        let mut stride_a = [0i32; MAX_DIMS];
        let rank = copy_i32_dims(window, &mut win_a, "pooling window exceeds i32")?;
        let _ = copy_i32_dims(pad, &mut pad_a, "pooling pad exceeds i32")?;
        let _ = copy_i32_dims(stride, &mut stride_a, "pooling stride exceeds i32")?;

        let mut raw: sys::cudnnPoolingDescriptor_t = ptr::null_mut();
        // SAFETY: `raw` is a valid out-pointer.
        check("cudnnCreatePoolingDescriptor", unsafe { sys::cudnnCreatePoolingDescriptor(&mut raw) })?;
        // SAFETY: `raw` is live and all three arrays hold `rank` initialized `i32`s.
        let status = unsafe {
            sys::cudnnSetPoolingNdDescriptor(
                raw,
                mode.to_sys(),
                nan_prop.to_sys(),
                rank,
                win_a.as_ptr(),
                pad_a.as_ptr(),
                stride_a.as_ptr(),
            )
        };
        if let Err(err) = check("cudnnSetPoolingNdDescriptor", status) {
            // SAFETY: clean up descriptor on failure.
            unsafe {
                record_teardown(ctx, sys::cudnnDestroyPoolingDescriptor(raw));
            }
            return Err(err);
        }
        Ok(Self { raw, ctx: ctx.clone() })
    }

    /// Computes output `[N, C, H, W]` dimensions for a 2D pooling operation.
    pub fn output_dim_4d(&self, input: &TensorDescriptor) -> Result<[usize; 4]> {
        if !Arc::ptr_eq(&self.ctx, input.context()) {
            return Err(Error::Invalid("pooling and input descriptors belong to different contexts"));
        }
        let (mut n, mut c, mut h, mut w) = (0i32, 0i32, 0i32, 0i32);
        // SAFETY: descriptors are live and out-pointers point to stack integers.
        let status = unsafe {
            sys::cudnnGetPooling2dForwardOutputDim(
                self.raw,
                input.as_raw(),
                &mut n,
                &mut c,
                &mut h,
                &mut w,
            )
        };
        check("cudnnGetPooling2dForwardOutputDim", status)?;
        Ok([
            usize::try_from(n).map_err(|_| Error::Invalid("negative pooling output n"))?,
            usize::try_from(c).map_err(|_| Error::Invalid("negative pooling output c"))?,
            usize::try_from(h).map_err(|_| Error::Invalid("negative pooling output h"))?,
            usize::try_from(w).map_err(|_| Error::Invalid("negative pooling output w"))?,
        ])
    }

    /// Raw `cudnnPoolingDescriptor_t`.
    pub fn as_raw(&self) -> sys::cudnnPoolingDescriptor_t {
        self.raw
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }
}

impl Drop for PoolingDescriptor {
    fn drop(&mut self) {
        // SAFETY: `self.raw` was created by `cudnnCreatePoolingDescriptor` and dropped once.
        let code = unsafe { sys::cudnnDestroyPoolingDescriptor(self.raw) };
        record_teardown(&self.ctx, code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_rank_and_enum_validation() {
        let mut out = [0i32; MAX_DIMS];
        assert!(copy_i32_dims(&[], &mut out, "dim").is_err());
        assert!(copy_i32_dims(&[1; MAX_DIMS + 1], &mut out, "dim").is_err());
        assert_eq!(copy_i32_dims(&[1, 3, 224, 224], &mut out, "dim").unwrap(), 4);
        assert_eq!(out[..4], [1, 3, 224, 224]);

        assert_eq!(
            ConvMode::CrossCorrelation.to_sys(),
            sys::cudnnConvolutionMode_t_CUDNN_CROSS_CORRELATION
        );
        assert_eq!(
            ActivationMode::Relu.to_sys(),
            sys::cudnnActivationMode_t_CUDNN_ACTIVATION_RELU
        );
        assert_eq!(PoolingMode::Max.to_sys(), sys::cudnnPoolingMode_t_CUDNN_POOLING_MAX);
    }
}
