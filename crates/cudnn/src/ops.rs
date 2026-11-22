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
    ActivationDescriptor, ConvBwdDataAlgo, ConvBwdFilterAlgo, ConvFwdAlgo, ConvolutionDescriptor,
    FilterDescriptor, PoolingDescriptor, TensorDescriptor,
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

/// Executes forward convolution `y = alpha * conv(x, w) + beta * y` on `stream` using raw CUDA
/// device pointers owned by an external GPU allocator (e.g. CubeCL pinned resources).
///
/// # Safety
/// - `x_ptr`, `w_ptr`, and `y_ptr` must be valid device allocations in `handle.context()` spanning
///   at least `x_bytes`, `w_bytes`, and `y_bytes` bytes respectively.
/// - The allocations must remain alive and stationary until `stream` completes.
/// - `y_ptr` must not alias `x_ptr` or `w_ptr`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct raw-pointer wrapper over cudnnConvolutionForward parameters"
)]
pub unsafe fn conv_forward_raw(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    budget: Option<usize>,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x_ptr: cuda_bindings::CUdeviceptr,
    x_bytes: usize,
    w_desc: &FilterDescriptor,
    w_ptr: cuda_bindings::CUdeviceptr,
    w_bytes: usize,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvFwdAlgo,
    beta: f64,
    y_desc: &TensorDescriptor,
    y_ptr: cuda_bindings::CUdeviceptr,
    y_bytes: usize,
) -> Result<()> {
    let ctx = handle.context().clone();
    same_ctx(stream, &ctx)?;
    if !Arc::ptr_eq(x_desc.context(), &ctx)
        || !Arc::ptr_eq(w_desc.context(), &ctx)
        || !Arc::ptr_eq(conv_desc.context(), &ctx)
        || !Arc::ptr_eq(y_desc.context(), &ctx)
    {
        return Err(Error::Invalid("convolution descriptor context mismatch"));
    }
    if x_bytes < x_desc.size_in_bytes() {
        return Err(Error::Invalid("input buffer smaller than descriptor size_in_bytes"));
    }
    if w_bytes < w_desc.size_in_bytes() {
        return Err(Error::Invalid("filter buffer smaller than descriptor size_in_bytes"));
    }
    if y_bytes < y_desc.size_in_bytes() {
        return Err(Error::Invalid("output buffer smaller than descriptor size_in_bytes"));
    }

    let needed = conv_forward_workspace_size(handle, x_desc, w_desc, conv_desc, y_desc, algo)?;
    let (ws_ptr, ws_bytes) = scratch.reserve(stream, needed, budget)?;
    handle.ensure_stream(stream)?;

    let scale = ScalePair::new(y_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();

    // SAFETY: caller upholds device pointer validity, non-aliasing, and stream lifetime; context,
    // stream, byte spans, and workspace capacity are validated above.
    let status = unsafe {
        sys::cudnnConvolutionForward(
            handle.as_raw(),
            alpha_ptr,
            x_desc.as_raw(),
            x_ptr as *const c_void,
            w_desc.as_raw(),
            w_ptr as *const c_void,
            conv_desc.as_raw(),
            algo.to_sys(),
            ws_ptr as *mut c_void,
            ws_bytes,
            beta_ptr,
            y_desc.as_raw(),
            y_ptr as *mut c_void,
        )
    };
    check("cudnnConvolutionForward", status)
}

/// Queries the backward-data convolution workspace size in bytes for `algo`.
pub fn conv_backward_data_workspace_size(
    handle: &Handle,
    w_desc: &FilterDescriptor,
    dy_desc: &TensorDescriptor,
    conv_desc: &ConvolutionDescriptor,
    dx_desc: &TensorDescriptor,
    algo: ConvBwdDataAlgo,
) -> Result<usize> {
    let ctx = handle.context();
    if !Arc::ptr_eq(w_desc.context(), ctx)
        || !Arc::ptr_eq(dy_desc.context(), ctx)
        || !Arc::ptr_eq(conv_desc.context(), ctx)
        || !Arc::ptr_eq(dx_desc.context(), ctx)
    {
        return Err(Error::Invalid("convolution descriptor context mismatch"));
    }
    ctx.bind_to_thread()?;
    let mut size_in_bytes = 0usize;
    // SAFETY: handle and descriptors are valid on `ctx` and `size_in_bytes` is a valid out-ptr.
    let status = unsafe {
        sys::cudnnGetConvolutionBackwardDataWorkspaceSize(
            handle.as_raw(),
            w_desc.as_raw(),
            dy_desc.as_raw(),
            conv_desc.as_raw(),
            dx_desc.as_raw(),
            algo.to_sys(),
            &mut size_in_bytes,
        )
    };
    check("cudnnGetConvolutionBackwardDataWorkspaceSize", status)?;
    Ok(size_in_bytes)
}

/// Executes backward-data convolution `dx = alpha * conv_bwd_data(w, dy) + beta * dx` on `stream`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct typed wrapper over cudnnConvolutionBackwardData parameters"
)]
pub fn conv_backward_data<W, DY, DX>(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    budget: Option<usize>,
    alpha: f64,
    w_desc: &FilterDescriptor,
    w: &W,
    dy_desc: &TensorDescriptor,
    dy: &DY,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvBwdDataAlgo,
    beta: f64,
    dx_desc: &TensorDescriptor,
    dx: &mut DeviceBuffer<DX>,
) -> Result<()>
where
    W: DeviceSpan + ?Sized,
    DY: DeviceSpan + ?Sized,
    DX: Sized,
{
    let ctx = handle.context().clone();
    check_filter_buf(&ctx, w_desc, w)?;
    check_tensor_buf(
        &ctx,
        dy_desc,
        dy,
        "dy buffer smaller than descriptor size_in_bytes",
    )?;
    check_mut_tensor_buf(&ctx, dx_desc, dx)?;
    // SAFETY: `w`, `dy`, and `dx` are validated `DeviceSpan`/`DeviceBuffer` allocations on `ctx`,
    // and `&mut DeviceBuffer<DX>` guarantees exclusivity against `w` and `dy`.
    unsafe {
        conv_backward_data_raw(
            handle,
            stream,
            scratch,
            budget,
            alpha,
            w_desc,
            w.cu_deviceptr(),
            w.num_bytes(),
            dy_desc,
            dy.cu_deviceptr(),
            dy.num_bytes(),
            conv_desc,
            algo,
            beta,
            dx_desc,
            dx.cu_deviceptr(),
            dx.num_bytes(),
        )
    }
}

/// Executes backward-data convolution `dx = alpha * conv_bwd_data(w, dy) + beta * dx` on `stream`
/// using raw CUDA device pointers owned by an external GPU allocator.
///
/// # Safety
/// - `w_ptr`, `dy_ptr`, and `dx_ptr` must be valid device allocations in `handle.context()` spanning
///   at least `w_bytes`, `dy_bytes`, and `dx_bytes` bytes respectively.
/// - Allocations must remain alive and stationary until `stream` completes; `dx_ptr` must not alias
///   `w_ptr` or `dy_ptr`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct raw-pointer wrapper over cudnnConvolutionBackwardData parameters"
)]
pub unsafe fn conv_backward_data_raw(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    budget: Option<usize>,
    alpha: f64,
    w_desc: &FilterDescriptor,
    w_ptr: cuda_bindings::CUdeviceptr,
    w_bytes: usize,
    dy_desc: &TensorDescriptor,
    dy_ptr: cuda_bindings::CUdeviceptr,
    dy_bytes: usize,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvBwdDataAlgo,
    beta: f64,
    dx_desc: &TensorDescriptor,
    dx_ptr: cuda_bindings::CUdeviceptr,
    dx_bytes: usize,
) -> Result<()> {
    let ctx = handle.context().clone();
    same_ctx(stream, &ctx)?;
    if !Arc::ptr_eq(w_desc.context(), &ctx)
        || !Arc::ptr_eq(dy_desc.context(), &ctx)
        || !Arc::ptr_eq(conv_desc.context(), &ctx)
        || !Arc::ptr_eq(dx_desc.context(), &ctx)
    {
        return Err(Error::Invalid("convolution descriptor context mismatch"));
    }
    if w_bytes < w_desc.size_in_bytes() {
        return Err(Error::Invalid("filter buffer smaller than descriptor size_in_bytes"));
    }
    if dy_bytes < dy_desc.size_in_bytes() {
        return Err(Error::Invalid("dy buffer smaller than descriptor size_in_bytes"));
    }
    if dx_bytes < dx_desc.size_in_bytes() {
        return Err(Error::Invalid("dx buffer smaller than descriptor size_in_bytes"));
    }

    let needed =
        conv_backward_data_workspace_size(handle, w_desc, dy_desc, conv_desc, dx_desc, algo)?;
    let (ws_ptr, ws_bytes) = scratch.reserve(stream, needed, budget)?;
    handle.ensure_stream(stream)?;

    let scale = ScalePair::new(dx_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();

    // SAFETY: caller upholds device pointer validity, non-aliasing, and stream lifetime; context,
    // stream, byte spans, and workspace capacity are validated above.
    let status = unsafe {
        sys::cudnnConvolutionBackwardData(
            handle.as_raw(),
            alpha_ptr,
            w_desc.as_raw(),
            w_ptr as *const c_void,
            dy_desc.as_raw(),
            dy_ptr as *const c_void,
            conv_desc.as_raw(),
            algo.to_sys(),
            ws_ptr as *mut c_void,
            ws_bytes,
            beta_ptr,
            dx_desc.as_raw(),
            dx_ptr as *mut c_void,
        )
    };
    check("cudnnConvolutionBackwardData", status)
}

/// Queries the backward-filter convolution workspace size in bytes for `algo`.
pub fn conv_backward_filter_workspace_size(
    handle: &Handle,
    x_desc: &TensorDescriptor,
    dy_desc: &TensorDescriptor,
    conv_desc: &ConvolutionDescriptor,
    dw_desc: &FilterDescriptor,
    algo: ConvBwdFilterAlgo,
) -> Result<usize> {
    let ctx = handle.context();
    if !Arc::ptr_eq(x_desc.context(), ctx)
        || !Arc::ptr_eq(dy_desc.context(), ctx)
        || !Arc::ptr_eq(conv_desc.context(), ctx)
        || !Arc::ptr_eq(dw_desc.context(), ctx)
    {
        return Err(Error::Invalid("convolution descriptor context mismatch"));
    }
    ctx.bind_to_thread()?;
    let mut size_in_bytes = 0usize;
    // SAFETY: handle and descriptors are valid on `ctx` and `size_in_bytes` is a valid out-ptr.
    let status = unsafe {
        sys::cudnnGetConvolutionBackwardFilterWorkspaceSize(
            handle.as_raw(),
            x_desc.as_raw(),
            dy_desc.as_raw(),
            conv_desc.as_raw(),
            dw_desc.as_raw(),
            algo.to_sys(),
            &mut size_in_bytes,
        )
    };
    check("cudnnGetConvolutionBackwardFilterWorkspaceSize", status)?;
    Ok(size_in_bytes)
}

/// Executes backward-filter convolution `dw = alpha * conv_bwd_filter(x, dy) + beta * dw` on `stream`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct typed wrapper over cudnnConvolutionBackwardFilter parameters"
)]
pub fn conv_backward_filter<X, DY, DW>(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    budget: Option<usize>,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x: &X,
    dy_desc: &TensorDescriptor,
    dy: &DY,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvBwdFilterAlgo,
    beta: f64,
    dw_desc: &FilterDescriptor,
    dw: &mut DeviceBuffer<DW>,
) -> Result<()>
where
    X: DeviceSpan + ?Sized,
    DY: DeviceSpan + ?Sized,
    DW: Sized,
{
    let ctx = handle.context().clone();
    check_tensor_buf(
        &ctx,
        x_desc,
        x,
        "input buffer smaller than descriptor size_in_bytes",
    )?;
    check_tensor_buf(
        &ctx,
        dy_desc,
        dy,
        "dy buffer smaller than descriptor size_in_bytes",
    )?;
    check_filter_buf(&ctx, dw_desc, dw)?;
    // SAFETY: `x`, `dy`, and `dw` are validated `DeviceSpan`/`DeviceBuffer` allocations on `ctx`,
    // and `&mut DeviceBuffer<DW>` guarantees exclusivity against `x` and `dy`.
    unsafe {
        conv_backward_filter_raw(
            handle,
            stream,
            scratch,
            budget,
            alpha,
            x_desc,
            x.cu_deviceptr(),
            x.num_bytes(),
            dy_desc,
            dy.cu_deviceptr(),
            dy.num_bytes(),
            conv_desc,
            algo,
            beta,
            dw_desc,
            dw.cu_deviceptr(),
            dw.num_bytes(),
        )
    }
}

/// Executes backward-filter convolution `dw = alpha * conv_bwd_filter(x, dy) + beta * dw` on `stream`
/// using raw CUDA device pointers owned by an external GPU allocator.
///
/// # Safety
/// - `x_ptr`, `dy_ptr`, and `dw_ptr` must be valid device allocations in `handle.context()` spanning
///   at least `x_bytes`, `dy_bytes`, and `dw_bytes` bytes respectively.
/// - Allocations must remain alive and stationary until `stream` completes; `dw_ptr` must not alias
///   `x_ptr` or `dy_ptr`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct raw-pointer wrapper over cudnnConvolutionBackwardFilter parameters"
)]
pub unsafe fn conv_backward_filter_raw(
    handle: &mut Handle,
    stream: &Arc<Stream>,
    scratch: &mut Scratch,
    budget: Option<usize>,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x_ptr: cuda_bindings::CUdeviceptr,
    x_bytes: usize,
    dy_desc: &TensorDescriptor,
    dy_ptr: cuda_bindings::CUdeviceptr,
    dy_bytes: usize,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvBwdFilterAlgo,
    beta: f64,
    dw_desc: &FilterDescriptor,
    dw_ptr: cuda_bindings::CUdeviceptr,
    dw_bytes: usize,
) -> Result<()> {
    let ctx = handle.context().clone();
    same_ctx(stream, &ctx)?;
    if !Arc::ptr_eq(x_desc.context(), &ctx)
        || !Arc::ptr_eq(dy_desc.context(), &ctx)
        || !Arc::ptr_eq(conv_desc.context(), &ctx)
        || !Arc::ptr_eq(dw_desc.context(), &ctx)
    {
        return Err(Error::Invalid("convolution descriptor context mismatch"));
    }
    if x_bytes < x_desc.size_in_bytes() {
        return Err(Error::Invalid("input buffer smaller than descriptor size_in_bytes"));
    }
    if dy_bytes < dy_desc.size_in_bytes() {
        return Err(Error::Invalid("dy buffer smaller than descriptor size_in_bytes"));
    }
    if dw_bytes < dw_desc.size_in_bytes() {
        return Err(Error::Invalid("dw buffer smaller than descriptor size_in_bytes"));
    }

    let needed =
        conv_backward_filter_workspace_size(handle, x_desc, dy_desc, conv_desc, dw_desc, algo)?;
    let (ws_ptr, ws_bytes) = scratch.reserve(stream, needed, budget)?;
    handle.ensure_stream(stream)?;

    let scale = ScalePair::new(dw_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();

    // SAFETY: caller upholds device pointer validity, non-aliasing, and stream lifetime; context,
    // stream, byte spans, and workspace capacity are validated above.
    let status = unsafe {
        sys::cudnnConvolutionBackwardFilter(
            handle.as_raw(),
            alpha_ptr,
            x_desc.as_raw(),
            x_ptr as *const c_void,
            dy_desc.as_raw(),
            dy_ptr as *const c_void,
            conv_desc.as_raw(),
            algo.to_sys(),
            ws_ptr as *mut c_void,
            ws_bytes,
            beta_ptr,
            dw_desc.as_raw(),
            dw_ptr as *mut c_void,
        )
    };
    check("cudnnConvolutionBackwardFilter", status)
}

/// Zero-overhead forward convolution on the stream already bound to `handle` (via
/// [`Handle::set_stream`] or [`Handle::set_raw_stream`]), using pre-queried workspace memory
/// `(ws_ptr, ws_bytes)`. Performs zero descriptor creation, zero workspace queries, and zero host
/// synchronization.
///
/// # Safety
/// - `x_ptr`, `w_ptr`, `y_ptr`, and (when `ws_bytes > 0`) `ws_ptr` must be valid device allocations
///   in `handle.context()` spanning at least their descriptor/workspace byte sizes.
/// - All buffers must remain alive until the bound CUDA stream completes; `y_ptr` must not alias
///   `x_ptr`, `w_ptr`, or `ws_ptr`.
#[expect(
    clippy::too_many_arguments,
    reason = "Direct zero-overhead wrapper over cudnnConvolutionForward on a bound stream"
)]
pub unsafe fn conv_forward_on_bound_stream(
    handle: &mut Handle,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x_ptr: cuda_bindings::CUdeviceptr,
    w_desc: &FilterDescriptor,
    w_ptr: cuda_bindings::CUdeviceptr,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvFwdAlgo,
    ws_ptr: cuda_bindings::CUdeviceptr,
    ws_bytes: usize,
    beta: f64,
    y_desc: &TensorDescriptor,
    y_ptr: cuda_bindings::CUdeviceptr,
) -> Result<()> {
    let scale = ScalePair::new(y_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();
    // SAFETY: caller guarantees context binding, stream binding, buffer validity, non-aliasing, and
    // workspace capacity.
    let status = unsafe {
        sys::cudnnConvolutionForward(
            handle.as_raw(),
            alpha_ptr,
            x_desc.as_raw(),
            x_ptr as *const c_void,
            w_desc.as_raw(),
            w_ptr as *const c_void,
            conv_desc.as_raw(),
            algo.to_sys(),
            ws_ptr as *mut c_void,
            ws_bytes,
            beta_ptr,
            y_desc.as_raw(),
            y_ptr as *mut c_void,
        )
    };
    check("cudnnConvolutionForward", status)
}

/// Zero-overhead backward-data convolution on the stream already bound to `handle`.
///
/// # Safety
/// Same pointer validity, non-aliasing, workspace, and stream-lifetime obligations as
/// [`conv_forward_on_bound_stream`].
#[expect(
    clippy::too_many_arguments,
    reason = "Direct zero-overhead wrapper over cudnnConvolutionBackwardData on a bound stream"
)]
pub unsafe fn conv_backward_data_on_bound_stream(
    handle: &mut Handle,
    alpha: f64,
    w_desc: &FilterDescriptor,
    w_ptr: cuda_bindings::CUdeviceptr,
    dy_desc: &TensorDescriptor,
    dy_ptr: cuda_bindings::CUdeviceptr,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvBwdDataAlgo,
    ws_ptr: cuda_bindings::CUdeviceptr,
    ws_bytes: usize,
    beta: f64,
    dx_desc: &TensorDescriptor,
    dx_ptr: cuda_bindings::CUdeviceptr,
) -> Result<()> {
    let scale = ScalePair::new(dx_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();
    // SAFETY: caller guarantees context binding, stream binding, buffer validity, non-aliasing, and
    // workspace capacity.
    let status = unsafe {
        sys::cudnnConvolutionBackwardData(
            handle.as_raw(),
            alpha_ptr,
            w_desc.as_raw(),
            w_ptr as *const c_void,
            dy_desc.as_raw(),
            dy_ptr as *const c_void,
            conv_desc.as_raw(),
            algo.to_sys(),
            ws_ptr as *mut c_void,
            ws_bytes,
            beta_ptr,
            dx_desc.as_raw(),
            dx_ptr as *mut c_void,
        )
    };
    check("cudnnConvolutionBackwardData", status)
}

/// Zero-overhead backward-filter convolution on the stream already bound to `handle`.
///
/// # Safety
/// Same pointer validity, non-aliasing, workspace, and stream-lifetime obligations as
/// [`conv_forward_on_bound_stream`].
#[expect(
    clippy::too_many_arguments,
    reason = "Direct zero-overhead wrapper over cudnnConvolutionBackwardFilter on a bound stream"
)]
pub unsafe fn conv_backward_filter_on_bound_stream(
    handle: &mut Handle,
    alpha: f64,
    x_desc: &TensorDescriptor,
    x_ptr: cuda_bindings::CUdeviceptr,
    dy_desc: &TensorDescriptor,
    dy_ptr: cuda_bindings::CUdeviceptr,
    conv_desc: &ConvolutionDescriptor,
    algo: ConvBwdFilterAlgo,
    ws_ptr: cuda_bindings::CUdeviceptr,
    ws_bytes: usize,
    beta: f64,
    dw_desc: &FilterDescriptor,
    dw_ptr: cuda_bindings::CUdeviceptr,
) -> Result<()> {
    let scale = ScalePair::new(dw_desc.data_type(), alpha, beta);
    let (alpha_ptr, beta_ptr) = scale.ptrs();
    // SAFETY: caller guarantees context binding, stream binding, buffer validity, non-aliasing, and
    // workspace capacity.
    let status = unsafe {
        sys::cudnnConvolutionBackwardFilter(
            handle.as_raw(),
            alpha_ptr,
            x_desc.as_raw(),
            x_ptr as *const c_void,
            dy_desc.as_raw(),
            dy_ptr as *const c_void,
            conv_desc.as_raw(),
            algo.to_sys(),
            ws_ptr as *mut c_void,
            ws_bytes,
            beta_ptr,
            dw_desc.as_raw(),
            dw_ptr as *mut c_void,
        )
    };
    check("cudnnConvolutionBackwardFilter", status)
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

    #[test]
    #[ignore = "requires CUDA GPU and cuDNN runtime"]
    fn t2_conv_backward_data_and_filter() {
        let dev = cuda_core::Device::new(0).unwrap();
        let ctx = dev.primary_context().unwrap();
        let stream = ctx.new_stream().unwrap();
        let mut handle = Handle::new(&ctx).unwrap();
        let mut scratch = Scratch::new(&stream);

        let (n, c_in, c_out, h, w_in, k) = (2usize, 3, 4, 5, 5, 3);
        let x_host: Vec<f32> = (0..n * c_in * h * w_in)
            .map(|i| (((i * 29 + 7) % 97) as f32 / 97.0 - 0.5) * 1.5)
            .collect();
        let w_host: Vec<f32> = (0..c_out * c_in * k * k)
            .map(|i| (((i * 41 + 13) % 89) as f32 / 89.0 - 0.5) * 1.2)
            .collect();
        let dy_host: Vec<f32> = (0..n * c_out * h * w_in)
            .map(|i| (((i * 19 + 5) % 83) as f32 / 83.0 - 0.5) * 1.0)
            .collect();

        let x_desc = TensorDescriptor::new_4d(
            &ctx,
            crate::TensorFormat::Nchw,
            DataType::F32,
            [n, c_in, h, w_in],
        )
        .unwrap();
        let w_desc = FilterDescriptor::new_4d(
            &ctx,
            crate::TensorFormat::Nchw,
            DataType::F32,
            [c_out, c_in, k, k],
        )
        .unwrap();
        let conv_desc = ConvolutionDescriptor::new_2d(
            &ctx,
            [1, 1],
            [1, 1],
            [1, 1],
            crate::ConvMode::CrossCorrelation,
            DataType::F32,
        )
        .unwrap();
        let y_dims = conv_desc.output_dim_4d(&x_desc, &w_desc).unwrap();
        assert_eq!(y_dims, [n, c_out, h, w_in]);
        let y_desc = TensorDescriptor::new_4d(
            &ctx,
            crate::TensorFormat::Nchw,
            DataType::F32,
            y_dims,
        )
        .unwrap();

        let x_dev = DeviceBuffer::from_host(&stream, &x_host).unwrap();
        let w_dev = DeviceBuffer::from_host(&stream, &w_host).unwrap();
        let dy_dev = DeviceBuffer::from_host(&stream, &dy_host).unwrap();
        let mut dx_dev = DeviceBuffer::<f32>::zeroed(&stream, x_host.len()).unwrap();
        let mut dw_dev = DeviceBuffer::<f32>::zeroed(&stream, w_host.len()).unwrap();

        conv_backward_data(
            &mut handle,
            &stream,
            &mut scratch,
            None,
            1.0,
            &w_desc,
            &w_dev,
            &y_desc,
            &dy_dev,
            &conv_desc,
            ConvBwdDataAlgo::Algo1,
            0.0,
            &x_desc,
            &mut dx_dev,
        )
        .unwrap();
        conv_backward_filter(
            &mut handle,
            &stream,
            &mut scratch,
            None,
            1.0,
            &x_desc,
            &x_dev,
            &y_desc,
            &dy_dev,
            &conv_desc,
            ConvBwdFilterAlgo::Algo1,
            0.0,
            &w_desc,
            &mut dw_dev,
        )
        .unwrap();
        stream.synchronize().unwrap();
        let dx = dx_dev.to_host_vec(&stream).unwrap();
        let dw = dw_dev.to_host_vec(&stream).unwrap();

        // Central finite-difference check on sampled x and w entries.
        let eps = 1e-3_f32;
        let mut y_buf = DeviceBuffer::<f32>::zeroed(&stream, dy_host.len()).unwrap();
        let mut eval_loss = |xv: &[f32], wv: &[f32]| -> f64 {
            let xd = DeviceBuffer::from_host(&stream, xv).unwrap();
            let wd = DeviceBuffer::from_host(&stream, wv).unwrap();
            conv_forward(
                &mut handle,
                &stream,
                &mut scratch,
                None,
                1.0,
                &x_desc,
                &xd,
                &w_desc,
                &wd,
                &conv_desc,
                ConvFwdAlgo::ImplicitPrecompGemm,
                0.0,
                &y_desc,
                &mut y_buf,
            )
            .unwrap();
            stream.synchronize().unwrap();
            let yv = y_buf.to_host_vec(&stream).unwrap();
            yv.iter()
                .zip(&dy_host)
                .map(|(&y, &dy)| f64::from(y) * f64::from(dy))
                .sum()
        };

        for idx in [0, 17, 63, x_host.len() - 1] {
            let mut xp = x_host.clone();
            let mut xm = x_host.clone();
            xp[idx] += eps;
            xm[idx] -= eps;
            let num = (eval_loss(&xp, &w_host) - eval_loss(&xm, &w_host)) / (2.0 * f64::from(eps));
            assert!(
                (num - f64::from(dx[idx])).abs() < 5e-4,
                "dx[{idx}] mismatch: cudnn={} num={num}",
                dx[idx]
            );
        }
        for idx in [0, 11, 55, w_host.len() - 1] {
            let mut wp = w_host.clone();
            let mut wm = w_host.clone();
            wp[idx] += eps;
            wm[idx] -= eps;
            let num = (eval_loss(&x_host, &wp) - eval_loss(&x_host, &wm)) / (2.0 * f64::from(eps));
            assert!(
                (num - f64::from(dw[idx])).abs() < 5e-4,
                "dw[{idx}] mismatch: cudnn={} num={num}",
                dw[idx]
            );
        }
    }
}
