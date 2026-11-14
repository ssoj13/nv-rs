//! Forward cuDNN neural-network operators: convolution, activation, pooling, softmax, and
//! batch-normalization inference.
//!
//! Every operator validates CUDA context affinity across handles, descriptors, streams, and device
//! buffers, enforces minimum buffer byte spans from `cudnnGetTensorSizeInBytes` /
//! `cudnnGetFilterSizeInBytes`, selects `f32` vs `f64` scaling factors on the stack per cuDNN ABI
//! rules, and executes with zero steady-state host heap allocations.

use std::ffi::c_void;
use std::sync::Arc;

use cuda_core::{DeviceBuffer, Stream};
use cudnn_sys as sys;

use crate::descriptor::{
    ActivationDescriptor, ConvFwdAlgo, ConvolutionDescriptor, FilterDescriptor, PoolingDescriptor,
    TensorDescriptor,
};
use crate::handle::Handle;
use crate::scratch::Scratch;
use crate::{DataType, DeviceSpan, Error, Result, check, record_teardown, same_ctx};

/// Softmax algorithm selection (`cudnnSoftmaxAlgorithm_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoftmaxAlgo {
    /// Unshifted exponentiation (`CUDNN_SOFTMAX_FAST`).
    Fast,
    /// Log-sum-exp stabilized softmax (`CUDNN_SOFTMAX_ACCURATE`).
    Accurate,
    /// Log-softmax (`CUDNN_SOFTMAX_LOG`).
    Log,
}

impl SoftmaxAlgo {
    /// Converts to the underlying `cudnnSoftmaxAlgorithm_t`.
    pub const fn to_sys(self) -> sys::cudnnSoftmaxAlgorithm_t {
        match self {
            Self::Fast => sys::cudnnSoftmaxAlgorithm_t_CUDNN_SOFTMAX_FAST,
            Self::Accurate => sys::cudnnSoftmaxAlgorithm_t_CUDNN_SOFTMAX_ACCURATE,
            Self::Log => sys::cudnnSoftmaxAlgorithm_t_CUDNN_SOFTMAX_LOG,
        }
    }
}

/// Softmax normalization scope (`cudnnSoftmaxMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoftmaxMode {
    /// Normalize across all `C * H * W` per batch item `N` (`CUDNN_SOFTMAX_MODE_INSTANCE`).
    Instance,
    /// Normalize across channels `C` at each spatial location (`CUDNN_SOFTMAX_MODE_CHANNEL`).
    Channel,
}

impl SoftmaxMode {
    /// Converts to the underlying `cudnnSoftmaxMode_t`.
    pub const fn to_sys(self) -> sys::cudnnSoftmaxMode_t {
        match self {
            Self::Instance => sys::cudnnSoftmaxMode_t_CUDNN_SOFTMAX_MODE_INSTANCE,
            Self::Channel => sys::cudnnSoftmaxMode_t_CUDNN_SOFTMAX_MODE_CHANNEL,
        }
    }
}

/// Batch normalization mode (`cudnnBatchNormMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BatchNormMode {
    /// Per-activation (`1 x C x H x W`, normalized over `N`).
    PerActivation,
    /// Spatial (`1 x C x 1 x 1`, normalized over `N x H x W`).
    Spatial,
    /// Spatial persistent (`1 x C x 1 x 1`, faster atomic implementation).
    SpatialPersistent,
}

impl BatchNormMode {
    /// Converts to the underlying `cudnnBatchNormMode_t`.
    pub const fn to_sys(self) -> sys::cudnnBatchNormMode_t {
        match self {
            Self::PerActivation => sys::cudnnBatchNormMode_t_CUDNN_BATCHNORM_PER_ACTIVATION,
            Self::Spatial => sys::cudnnBatchNormMode_t_CUDNN_BATCHNORM_SPATIAL,
            Self::SpatialPersistent => sys::cudnnBatchNormMode_t_CUDNN_BATCHNORM_SPATIAL_PERSISTENT,
        }
    }
}

/// Stack-allocated scaling pair respecting cuDNN's ABI rule: `f64` for `DataType::F64`, `f32` for
/// all other tensor element types (including `F16` and `Bf16`).
enum ScalePair {
    F32([f32; 2]),
    F64([f64; 2]),
}

impl ScalePair {
    fn new(dtype: DataType, alpha: f64, beta: f64) -> Self {
        match dtype {
            DataType::F64 => Self::F64([alpha, beta]),
            _ => Self::F32([alpha as f32, beta as f32]),
        }
    }

    fn ptrs(&self) -> (*const c_void, *const c_void) {
        match self {
            Self::F32(v) => (
                (&v[0] as *const f32).cast::<c_void>(),
                (&v[1] as *const f32).cast::<c_void>(),
            ),
            Self::F64(v) => (
                (&v[0] as *const f64).cast::<c_void>(),
                (&v[1] as *const f64).cast::<c_void>(),
            ),
        }
    }
}

fn check_tensor_buf<B: DeviceSpan + ?Sized>(
    ctx: &Arc<cuda_core::Context>,
    desc: &TensorDescriptor,
    buf: &B,
    label: &'static str,
) -> Result<()> {
    if !Arc::ptr_eq(desc.context(), ctx) || !Arc::ptr_eq(buf.context(), ctx) {
        return Err(Error::Invalid(
            "tensor descriptor or device buffer belongs to another context",
        ));
    }
    if buf.num_bytes() < desc.size_in_bytes() {
        return Err(Error::Invalid(label));
    }
    Ok(())
}

fn check_filter_buf<B: DeviceSpan + ?Sized>(
    ctx: &Arc<cuda_core::Context>,
    desc: &FilterDescriptor,
    buf: &B,
) -> Result<()> {
    if !Arc::ptr_eq(desc.context(), ctx) || !Arc::ptr_eq(buf.context(), ctx) {
        return Err(Error::Invalid(
            "filter descriptor or weight buffer belongs to another context",
        ));
    }
    if buf.num_bytes() < desc.size_in_bytes() {
        return Err(Error::Invalid(
            "filter buffer smaller than descriptor size_in_bytes",
        ));
    }
    Ok(())
}

fn check_mut_tensor_buf<T>(
    ctx: &Arc<cuda_core::Context>,
    desc: &TensorDescriptor,
    buf: &DeviceBuffer<T>,
) -> Result<()> {
    check_tensor_buf(
        ctx,
        desc,
        buf,
        "output buffer smaller than descriptor size_in_bytes",
    )
}

/// Queries the forward convolution workspace size in bytes for `algo`.
pub fn conv_forward_workspace_size(
    handle: &Handle,
    x_desc: &TensorDescriptor,
    w_desc: &FilterDescriptor,
    conv_desc: &ConvolutionDescriptor,
    y_desc: &TensorDescriptor,
    algo: ConvFwdAlgo,
) -> Result<usize> {
    let ctx = handle.context();
    if !Arc::ptr_eq(x_desc.context(), ctx)
        || !Arc::ptr_eq(w_desc.context(), ctx)
        || !Arc::ptr_eq(conv_desc.context(), ctx)
        || !Arc::ptr_eq(y_desc.context(), ctx)
    {
        return Err(Error::Invalid("convolution descriptor context mismatch"));
    }
    ctx.bind_to_thread()?;
    let mut size_in_bytes = 0usize;
    // SAFETY: handle and descriptors are valid on `ctx` and `size_in_bytes` is a valid out-ptr.
    let status = unsafe {
        sys::cudnnGetConvolutionForwardWorkspaceSize(
            handle.as_raw(),
            x_desc.as_raw(),
            w_desc.as_raw(),
            conv_desc.as_raw(),
            y_desc.as_raw(),
            algo.to_sys(),
            &mut size_in_bytes,
        )
    };
    check("cudnnGetConvolutionForwardWorkspaceSize", status)?;
    Ok(size_in_bytes)
}

/// Executes forward convolution `y = alpha * conv(x, w) + beta * y` on `stream`.
///
/// Queries the required workspace size for `algo`, enforces `budget` on any growth of `scratch`,
/// and binds `stream` to `handle` when needed.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct typed wrapper over cudnnConvolutionForward parameters"
)]
pub fn conv_forward<X, W, Y>(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    budget: Option<usize>,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x: &X,
    w_desc: &FilterDescriptor,
    w: &W,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvFwdAlgo,
    beta: f64,
    y_desc: &TensorDescriptor,
    y: &mut DeviceBuffer<Y>,
) -> Result<()>
where
    X: DeviceSpan + ?Sized,
    W: DeviceSpan + ?Sized,
    Y: Sized,
{
    let ctx = handle.context().clone();
    same_ctx(stream, &ctx)?;
    if !Arc::ptr_eq(conv_desc.context(), &ctx) {
        return Err(Error::Invalid(
            "convolution descriptor belongs to another context",
        ));
    }
    check_tensor_buf(
        &ctx,
        x_desc,
        x,
        "input buffer smaller than descriptor size_in_bytes",
    )?;
    check_filter_buf(&ctx, w_desc, w)?;
    check_mut_tensor_buf(&ctx, y_desc, y)?;

    let needed = conv_forward_workspace_size(handle, x_desc, w_desc, conv_desc, y_desc, algo)?;
    let (ws_ptr, ws_bytes) = scratch.reserve(stream, needed, budget)?;
    handle.ensure_stream(stream)?;

    let scale = ScalePair::new(y_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();

    // SAFETY: context is bound, stream is set on `handle`, all descriptor/device buffer byte spans
    // were validated, and `ws_ptr` spans at least `needed` workspace bytes (or null when 0).
    let status = unsafe {
        sys::cudnnConvolutionForward(
            handle.as_raw(),
            alpha_ptr,
            x_desc.as_raw(),
            x.cu_deviceptr() as *const c_void,
            w_desc.as_raw(),
            w.cu_deviceptr() as *const c_void,
            conv_desc.as_raw(),
            algo.to_sys(),
            ws_ptr as *mut c_void,
            ws_bytes,
            beta_ptr,
            y_desc.as_raw(),
            y.cu_deviceptr() as *mut c_void,
        )
    };
    check("cudnnConvolutionForward", status)
}

/// Executes elementwise activation `y = alpha * act(x) + beta * y` on `stream`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct typed wrapper over cudnnActivationForward parameters"
)]
pub fn activation_forward<X, Y>(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    act_desc: &ActivationDescriptor,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x: &X,
    beta: f64,
    y_desc: &TensorDescriptor,
    y: &mut DeviceBuffer<Y>,
) -> Result<()>
where
    X: DeviceSpan + ?Sized,
    Y: Sized,
{
    let ctx = handle.context().clone();
    same_ctx(stream, &ctx)?;
    if !Arc::ptr_eq(act_desc.context(), &ctx) {
        return Err(Error::Invalid(
            "activation descriptor belongs to another context",
        ));
    }
    check_tensor_buf(
        &ctx,
        x_desc,
        x,
        "input buffer smaller than descriptor size_in_bytes",
    )?;
    check_mut_tensor_buf(&ctx, y_desc, y)?;
    handle.ensure_stream(stream)?;

    let scale = ScalePair::new(y_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();

    // SAFETY: context is bound, stream is set, and tensor spans were verified.
    let status = unsafe {
        sys::cudnnActivationForward(
            handle.as_raw(),
            act_desc.as_raw(),
            alpha_ptr,
            x_desc.as_raw(),
            x.cu_deviceptr() as *const c_void,
            beta_ptr,
            y_desc.as_raw(),
            y.cu_deviceptr() as *mut c_void,
        )
    };
    check("cudnnActivationForward", status)
}

/// Executes spatial pooling `y = alpha * pool(x) + beta * y` on `stream`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct typed wrapper over cudnnPoolingForward parameters"
)]
pub fn pooling_forward<X, Y>(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    pool_desc: &PoolingDescriptor,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x: &X,
    beta: f64,
    y_desc: &TensorDescriptor,
    y: &mut DeviceBuffer<Y>,
) -> Result<()>
where
    X: DeviceSpan + ?Sized,
    Y: Sized,
{
    let ctx = handle.context().clone();
    same_ctx(stream, &ctx)?;
    if !Arc::ptr_eq(pool_desc.context(), &ctx) {
        return Err(Error::Invalid(
            "pooling descriptor belongs to another context",
        ));
    }
    check_tensor_buf(
        &ctx,
        x_desc,
        x,
        "input buffer smaller than descriptor size_in_bytes",
    )?;
    check_mut_tensor_buf(&ctx, y_desc, y)?;
    handle.ensure_stream(stream)?;

    let scale = ScalePair::new(y_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();

    // SAFETY: context is bound, stream is set, and tensor spans were verified.
    let status = unsafe {
        sys::cudnnPoolingForward(
            handle.as_raw(),
            pool_desc.as_raw(),
            alpha_ptr,
            x_desc.as_raw(),
            x.cu_deviceptr() as *const c_void,
            beta_ptr,
            y_desc.as_raw(),
            y.cu_deviceptr() as *mut c_void,
        )
    };
    check("cudnnPoolingForward", status)
}

/// Executes forward softmax `y = alpha * softmax(x) + beta * y` on `stream`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct typed wrapper over cudnnSoftmaxForward parameters"
)]
pub fn softmax_forward<X, Y>(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    algo: SoftmaxAlgo,
    mode: SoftmaxMode,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x: &X,
    beta: f64,
    y_desc: &TensorDescriptor,
    y: &mut DeviceBuffer<Y>,
) -> Result<()>
where
    X: DeviceSpan + ?Sized,
    Y: Sized,
{
    let ctx = handle.context().clone();
    same_ctx(stream, &ctx)?;
    check_tensor_buf(
        &ctx,
        x_desc,
        x,
        "input buffer smaller than descriptor size_in_bytes",
    )?;
    check_mut_tensor_buf(&ctx, y_desc, y)?;
    handle.ensure_stream(stream)?;

    let scale = ScalePair::new(y_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();

    // SAFETY: context is bound, stream is set, and tensor spans were verified.
    let status = unsafe {
        sys::cudnnSoftmaxForward(
            handle.as_raw(),
            algo.to_sys(),
            mode.to_sys(),
            alpha_ptr,
            x_desc.as_raw(),
            x.cu_deviceptr() as *const c_void,
            beta_ptr,
            y_desc.as_raw(),
            y.cu_deviceptr() as *mut c_void,
        )
    };
    check("cudnnSoftmaxForward", status)
}

/// Derives the shared scale/bias/mean/variance tensor descriptor for batch normalization from `x_desc`.
pub fn derive_bn_tensor_desc(
    x_desc: &TensorDescriptor,
    mode: BatchNormMode,
) -> Result<TensorDescriptor> {
    let ctx = x_desc.context();
    let raw = TensorDescriptor::create_empty()?;
    // SAFETY: `raw` is a fresh empty tensor descriptor and `x_desc` is live.
    let status = unsafe { sys::cudnnDeriveBNTensorDescriptor(raw, x_desc.as_raw(), mode.to_sys()) };
    if let Err(err) = check("cudnnDeriveBNTensorDescriptor", status) {
        // SAFETY: clean up descriptor on failure.
        unsafe {
            record_teardown(ctx, sys::cudnnDestroyTensorDescriptor(raw));
        }
        return Err(err);
    }
    let param_dtype = match x_desc.data_type() {
        DataType::F64 => DataType::F64,
        _ => DataType::F32,
    };
    TensorDescriptor::finish(ctx, raw, param_dtype)
}

/// Executes batch-normalization inference on `stream`:
/// `y = alpha * (scale * (x - mean) / sqrt(var + epsilon) + bias) + beta * y`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct typed wrapper over cudnnBatchNormalizationForwardInference parameters"
)]
pub fn batch_norm_forward_inference<X, Y, P>(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    mode: BatchNormMode,
    alpha: f64,
    beta: f64,
    x_desc: &TensorDescriptor,
    x: &X,
    y_desc: &TensorDescriptor,
    y: &mut DeviceBuffer<Y>,
    bn_param_desc: &TensorDescriptor,
    scale: &P,
    bias: &P,
    estimated_mean: &P,
    estimated_variance: &P,
    epsilon: f64,
) -> Result<()>
where
    X: DeviceSpan + ?Sized,
    Y: Sized,
    P: DeviceSpan + ?Sized,
{
    if !epsilon.is_finite() || epsilon < 0.0 {
        return Err(Error::Invalid("batch norm epsilon must be finite and >= 0"));
    }
    let ctx = handle.context().clone();
    same_ctx(stream, &ctx)?;
    check_tensor_buf(
        &ctx,
        x_desc,
        x,
        "input buffer smaller than descriptor size_in_bytes",
    )?;
    check_mut_tensor_buf(&ctx, y_desc, y)?;
    check_tensor_buf(
        &ctx,
        bn_param_desc,
        scale,
        "bn scale buffer smaller than descriptor",
    )?;
    check_tensor_buf(
        &ctx,
        bn_param_desc,
        bias,
        "bn bias buffer smaller than descriptor",
    )?;
    check_tensor_buf(
        &ctx,
        bn_param_desc,
        estimated_mean,
        "bn mean buffer smaller than descriptor",
    )?;
    check_tensor_buf(
        &ctx,
        bn_param_desc,
        estimated_variance,
        "bn variance buffer smaller than descriptor",
    )?;
    handle.ensure_stream(stream)?;

    let scale_pair = ScalePair::new(y_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale_pair.ptrs();

    // SAFETY: context is bound, stream is set, and all input/output/parameter spans match
    // their descriptors and `ctx`.
    let status = unsafe {
        sys::cudnnBatchNormalizationForwardInference(
            handle.as_raw(),
            mode.to_sys(),
            alpha_ptr,
            beta_ptr,
            x_desc.as_raw(),
            x.cu_deviceptr() as *const c_void,
            y_desc.as_raw(),
            y.cu_deviceptr() as *mut c_void,
            bn_param_desc.as_raw(),
            scale.cu_deviceptr() as *const c_void,
            bias.cu_deviceptr() as *const c_void,
            estimated_mean.cu_deviceptr() as *const c_void,
            estimated_variance.cu_deviceptr() as *const c_void,
            epsilon,
        )
    };
    check("cudnnBatchNormalizationForwardInference", status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_scale_pair_abi_selection() {
        let f32_scale = ScalePair::new(DataType::F16, 1.5, 0.25);
        assert!(matches!(f32_scale, ScalePair::F32([a, b]) if a == 1.5 && b == 0.25));

        let f64_scale = ScalePair::new(DataType::F64, 1.5, 0.25);
        assert!(matches!(f64_scale, ScalePair::F64([a, b]) if a == 1.5 && b == 0.25));

        assert_eq!(
            SoftmaxAlgo::Accurate.to_sys(),
            sys::cudnnSoftmaxAlgorithm_t_CUDNN_SOFTMAX_ACCURATE
        );
        assert_eq!(
            SoftmaxMode::Channel.to_sys(),
            sys::cudnnSoftmaxMode_t_CUDNN_SOFTMAX_MODE_CHANNEL
        );
        assert_eq!(
            BatchNormMode::Spatial.to_sys(),
            sys::cudnnBatchNormMode_t_CUDNN_BATCHNORM_SPATIAL
        );
    }

    #[test]
    #[ignore = "requires CUDA GPU and cuDNN runtime"]
    fn t2_pooling_softmax_batchnorm_forward() {
        let ctx = cuda_core::Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();
        let mut handle = Handle::new(&ctx).unwrap();
        handle.set_stream(&stream).unwrap();

        let x_desc =
            TensorDescriptor::new_4d(&ctx, crate::TensorFormat::Nchw, DataType::F32, [1, 2, 2, 2])
                .unwrap();
        let pool_desc = PoolingDescriptor::new_2d(
            &ctx,
            crate::PoolingMode::Max,
            crate::NanPropagation::NotPropagate,
            [2, 2],
            [0, 0],
            [2, 2],
        )
        .unwrap();
        let pool_dims = pool_desc.output_dim_4d(&x_desc).unwrap();
        assert_eq!(pool_dims, [1, 2, 1, 1]);
        let y_pool_desc =
            TensorDescriptor::new_4d(&ctx, crate::TensorFormat::Nchw, DataType::F32, pool_dims)
                .unwrap();

        let x_host: [f32; 8] = [1.0, 4.0, 2.0, 3.0, -1.0, 0.0, 5.0, 2.0];
        let x_dev = DeviceBuffer::from_host(&stream, &x_host).unwrap();
        let mut pool_out = DeviceBuffer::<f32>::zeroed(&stream, 2).unwrap();
        pooling_forward(
            &mut handle,
            &stream,
            &pool_desc,
            1.0,
            &x_desc,
            &x_dev,
            0.0,
            &y_pool_desc,
            &mut pool_out,
        )
        .unwrap();

        let mut sm_out = DeviceBuffer::<f32>::zeroed(&stream, 2).unwrap();
        softmax_forward(
            &mut handle,
            &stream,
            SoftmaxAlgo::Accurate,
            SoftmaxMode::Channel,
            1.0,
            &y_pool_desc,
            &pool_out,
            0.0,
            &y_pool_desc,
            &mut sm_out,
        )
        .unwrap();

        let bn_desc = derive_bn_tensor_desc(&x_desc, BatchNormMode::Spatial).unwrap();
        let scale = DeviceBuffer::from_host(&stream, &[1.0f32, 1.0f32]).unwrap();
        let bias = DeviceBuffer::from_host(&stream, &[0.0f32, 0.0f32]).unwrap();
        let mean = DeviceBuffer::from_host(&stream, &[0.0f32, 0.0f32]).unwrap();
        let var = DeviceBuffer::from_host(&stream, &[1.0f32, 1.0f32]).unwrap();
        let mut bn_out = DeviceBuffer::<f32>::zeroed(&stream, 8).unwrap();
        batch_norm_forward_inference(
            &mut handle,
            &stream,
            BatchNormMode::Spatial,
            1.0,
            0.0,
            &x_desc,
            &x_dev,
            &x_desc,
            &mut bn_out,
            &bn_desc,
            &scale,
            &bias,
            &mean,
            &var,
            1e-5,
        )
        .unwrap();

        stream.synchronize().unwrap();
        let pooled = pool_out.to_host_vec(&stream).unwrap();
        assert_eq!(pooled, vec![4.0, 5.0]);
        let sm = sm_out.to_host_vec(&stream).unwrap();
        assert!((sm[0] + sm[1] - 1.0).abs() < 1e-5);
        assert!(sm[1] > sm[0]);
    }
}
