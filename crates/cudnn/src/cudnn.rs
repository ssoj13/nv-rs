//! Safe wrappers for cuDNN handles, tensor/layer descriptors, workspace budgeting, and
//! forward neural-network primitives (`Conv`, `Activation`, `Pooling`, `Softmax`, `BatchNorm`).
//!
//! Every native handle and descriptor is owned by a single RAII type holding an
//! `Arc<cuda_core::Context>`; `Drop` destroys the C object and records any teardown error on
//! `Context` rather than panicking. Forward operations size workspaces up front, enforce an
//! explicit byte budget before growing a caller-owned [`Scratch`] buffer, and execute on a
//! bound CUDA stream with zero steady-state host allocations.

/// Layer, tensor, filter, convolution, activation, and pooling descriptors.
pub mod descriptor;
/// RAII cuDNN handle and stream binding.
pub mod handle;
/// Forward convolution, activation, pooling, softmax, and batch-normalization primitives.
pub mod ops;
/// Reusable device scratch workspace and byte-budget helpers.
pub mod scratch;

use std::ffi::CStr;
use std::fmt;
use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, DeviceBufferView, Stream};
use cudnn_sys as sys;
use thiserror::Error;

pub use descriptor::{
    ActivationDescriptor, ActivationMode, ConvBwdDataAlgo, ConvBwdFilterAlgo, ConvFwdAlgo,
    ConvMode, ConvolutionDescriptor, FilterDescriptor, MAX_DIMS, MathType, PoolingDescriptor,
    PoolingMode, TensorDescriptor,
};
pub use handle::Handle;
pub use ops::{
    BatchNormMode, SoftmaxAlgo, SoftmaxMode, activation_forward, batch_norm_forward_inference,
    conv_backward_data, conv_backward_data_on_bound_stream, conv_backward_data_raw,
    conv_backward_data_workspace_size, conv_backward_filter, conv_backward_filter_on_bound_stream,
    conv_backward_filter_raw, conv_backward_filter_workspace_size, conv_forward,
    conv_forward_on_bound_stream, conv_forward_raw, conv_forward_workspace_size,
    derive_bn_tensor_desc, pooling_forward, softmax_forward,
};
pub use scratch::Scratch;

/// Crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Raw cuDNN status code (`cudnnStatus_t`), preserved as a `Copy` integer newtype so status codes
/// added in newer cuDNN releases remain lossless across FFI boundaries.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CudnnStatus(
    /// Raw `cudnnStatus_t` integer value.
    pub u32,
);

impl CudnnStatus {
    /// `CUDNN_STATUS_SUCCESS`.
    pub const SUCCESS: Self = Self(sys::cudnnStatus_t_CUDNN_STATUS_SUCCESS);
    /// `CUDNN_STATUS_NOT_INITIALIZED`.
    pub const NOT_INITIALIZED: Self = Self(sys::cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED);
    /// `CUDNN_STATUS_BAD_PARAM`.
    pub const BAD_PARAM: Self = Self(sys::cudnnStatus_t_CUDNN_STATUS_BAD_PARAM);
    /// `CUDNN_STATUS_NOT_SUPPORTED`.
    pub const NOT_SUPPORTED: Self = Self(sys::cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED);

    /// Whether this code represents success.
    pub const fn is_success(self) -> bool {
        self.0 == Self::SUCCESS.0
    }
}

impl fmt::Debug for CudnnStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CudnnStatus({}, {self})", self.0)
    }
}

impl fmt::Display for CudnnStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if sys::load().is_ok() {
            // SAFETY: `cudnnGetErrorString` returns a static NUL-terminated string or null.
            let ptr = unsafe { sys::cudnnGetErrorString(self.0) };
            if !ptr.is_null() {
                // SAFETY: `cudnnGetErrorString` returns a valid static C string when non-null.
                let msg = unsafe { CStr::from_ptr(ptr) }.to_string_lossy();
                return f.write_str(&msg);
            }
        }
        write!(f, "cudnnStatus_t({})", self.0)
    }
}

/// Errors produced by safe cuDNN wrappers.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to locate or open the cuDNN dynamic library.
    #[error("failed to load cuDNN: {0}")]
    Load(#[from] &'static nv_loader::LoadError),
    /// Failed to resolve an unshimmed cuDNN symbol.
    #[error("cuDNN symbol call failed: {0}")]
    Call(#[from] nv_loader::CallError),
    /// CUDA driver error from context, stream, or device buffer operations.
    #[error(transparent)]
    Driver(#[from] cuda_core::DriverError),
    /// cuDNN returned a non-success status code.
    #[error("{call} failed: {code} ({})", code.0)]
    Status {
        /// Name of the cuDNN C entry point that failed.
        call: &'static str,
        /// Returned `cudnnStatus_t` status code.
        code: CudnnStatus,
    },
    /// Operation workspace growth exceeds the caller's explicit byte budget.
    #[error("needs {requested} device bytes, budget admits {available}")]
    Budget {
        /// Additional device bytes required by the operation.
        requested: usize,
        /// Caller-imposed maximum byte budget.
        available: usize,
    },
    /// Invalid descriptor shapes, strides, buffer sizes, or mismatched contexts.
    #[error("{0}")]
    Invalid(&'static str),
    /// Requested capability or mode is not supported by this wrapper/library.
    #[error("{0}")]
    Unsupported(&'static str),
}

/// Converts a raw `cudnnStatus_t` into `Result<()>`.
pub fn check(call: &'static str, raw: sys::cudnnStatus_t) -> Result<()> {
    let code = CudnnStatus(raw);
    if code.is_success() {
        Ok(())
    } else {
        Err(Error::Status { call, code })
    }
}

/// Verifies `stream` belongs to `ctx`.
pub(crate) fn same_ctx(stream: &Stream, ctx: &Arc<Context>) -> Result<()> {
    if !Arc::ptr_eq(stream.context(), ctx) {
        return Err(Error::Invalid("stream belongs to a different CUDA context"));
    }
    Ok(())
}

/// Verifies `stream` matches `expected` stream identity.
pub(crate) fn same_stream(stream: &Arc<Stream>, expected: &Arc<Stream>) -> Result<()> {
    if !Arc::ptr_eq(stream, expected) {
        return Err(Error::Invalid(
            "scratch workspace belongs to a different CUDA stream",
        ));
    }
    Ok(())
}

/// Rejects an allocation growth or peak byte count above `budget`.
pub(crate) fn within(budget: Option<usize>, peak: usize) -> Result<()> {
    match budget {
        Some(available) if peak > available => Err(Error::Budget {
            requested: peak,
            available,
        }),
        _ => Ok(()),
    }
}

/// Records a C teardown failure onto `ctx` without panicking.
pub(crate) fn record_teardown(ctx: &Context, code: sys::cudnnStatus_t) {
    if code != sys::cudnnStatus_t_CUDNN_STATUS_SUCCESS {
        ctx.record_err(Err::<(), _>(cuda_core::DriverError(
            cuda_bindings::cudaError_enum_CUDA_ERROR_UNKNOWN,
        )));
    }
}

/// Converts a `usize` count or dimension into `i32` for cuDNN C APIs.
pub(crate) fn checked_i32(value: usize, what: &'static str) -> Result<i32> {
    i32::try_from(value).map_err(|_| Error::Invalid(what))
}

/// Data types supported by cuDNN tensor and filter descriptors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    /// 32-bit IEEE floating point (`CUDNN_DATA_FLOAT`).
    F32,
    /// 64-bit IEEE floating point (`CUDNN_DATA_DOUBLE`).
    F64,
    /// 16-bit IEEE floating point (`CUDNN_DATA_HALF`).
    F16,
    /// 16-bit brain floating point (`CUDNN_DATA_BFLOAT16`).
    Bf16,
    /// 8-bit signed integer (`CUDNN_DATA_INT8`).
    I8,
    /// 8-bit unsigned integer (`CUDNN_DATA_UINT8`).
    U8,
    /// 32-bit signed integer (`CUDNN_DATA_INT32`).
    I32,
    /// 64-bit signed integer (`CUDNN_DATA_INT64`).
    I64,
}

impl DataType {
    /// Converts to the underlying `cudnnDataType_t`.
    pub const fn to_sys(self) -> sys::cudnnDataType_t {
        match self {
            Self::F32 => sys::cudnnDataType_t_CUDNN_DATA_FLOAT,
            Self::F64 => sys::cudnnDataType_t_CUDNN_DATA_DOUBLE,
            Self::F16 => sys::cudnnDataType_t_CUDNN_DATA_HALF,
            Self::Bf16 => sys::cudnnDataType_t_CUDNN_DATA_BFLOAT16,
            Self::I8 => sys::cudnnDataType_t_CUDNN_DATA_INT8,
            Self::U8 => sys::cudnnDataType_t_CUDNN_DATA_UINT8,
            Self::I32 => sys::cudnnDataType_t_CUDNN_DATA_INT32,
            Self::I64 => sys::cudnnDataType_t_CUDNN_DATA_INT64,
        }
    }

    /// Reconstructs from a raw `cudnnDataType_t`, returning an error on unrecognized variants.
    pub fn from_sys(raw: sys::cudnnDataType_t) -> Result<Self> {
        match raw {
            sys::cudnnDataType_t_CUDNN_DATA_FLOAT => Ok(Self::F32),
            sys::cudnnDataType_t_CUDNN_DATA_DOUBLE => Ok(Self::F64),
            sys::cudnnDataType_t_CUDNN_DATA_HALF => Ok(Self::F16),
            sys::cudnnDataType_t_CUDNN_DATA_BFLOAT16 => Ok(Self::Bf16),
            sys::cudnnDataType_t_CUDNN_DATA_INT8 => Ok(Self::I8),
            sys::cudnnDataType_t_CUDNN_DATA_UINT8 => Ok(Self::U8),
            sys::cudnnDataType_t_CUDNN_DATA_INT32 => Ok(Self::I32),
            sys::cudnnDataType_t_CUDNN_DATA_INT64 => Ok(Self::I64),
            _ => Err(Error::Unsupported("unrecognized cudnnDataType_t variant")),
        }
    }

    /// Size of one scalar element in bytes.
    pub const fn elem_bytes(self) -> usize {
        match self {
            Self::I8 | Self::U8 => 1,
            Self::F16 | Self::Bf16 => 2,
            Self::F32 | Self::I32 => 4,
            Self::F64 | Self::I64 => 8,
        }
    }
}

/// Memory layout formats for cuDNN tensor and filter descriptors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TensorFormat {
    /// Channel-major contiguous layout (`CUDNN_TENSOR_NCHW`).
    Nchw,
    /// Interleaved channel layout (`CUDNN_TENSOR_NHWC`).
    Nhwc,
    /// Vectorized channel layout (`CUDNN_TENSOR_NCHW_VECT_C`).
    NchwVectC,
}

impl TensorFormat {
    /// Converts to the underlying `cudnnTensorFormat_t`.
    pub const fn to_sys(self) -> sys::cudnnTensorFormat_t {
        match self {
            Self::Nchw => sys::cudnnTensorFormat_t_CUDNN_TENSOR_NCHW,
            Self::Nhwc => sys::cudnnTensorFormat_t_CUDNN_TENSOR_NHWC,
            Self::NchwVectC => sys::cudnnTensorFormat_t_CUDNN_TENSOR_NCHW_VECT_C,
        }
    }
}

/// NaN propagation policy for activation and pooling descriptors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NanPropagation {
    /// Suppress NaNs where possible (`CUDNN_NOT_PROPAGATE_NAN`).
    NotPropagate,
    /// Propagate NaNs through the operation (`CUDNN_PROPAGATE_NAN`).
    Propagate,
}

impl NanPropagation {
    /// Converts to the underlying `cudnnNanPropagation_t`.
    pub const fn to_sys(self) -> sys::cudnnNanPropagation_t {
        match self {
            Self::NotPropagate => sys::cudnnNanPropagation_t_CUDNN_NOT_PROPAGATE_NAN,
            Self::Propagate => sys::cudnnNanPropagation_t_CUDNN_PROPAGATE_NAN,
        }
    }
}

mod sealed {
    use super::*;

    pub trait Sealed {}

    impl<T> Sealed for DeviceBuffer<T> {}
    impl<T> Sealed for DeviceBufferView<'_, T> {}
}

/// Read-only device memory view accepted by cuDNN forward operators, allowing typed
/// [`DeviceBuffer<T>`] and [`DeviceBufferView<T>`] slices without extra wrappers.
///
/// # Safety Invariant & Obligations
/// This trait is sealed via the internal [`sealed::Sealed`] supertrait and cannot be implemented
/// by external or downstream crates. This guarantees that:
/// - **Allocation validity**: [`cu_deviceptr`](DeviceSpan::cu_deviceptr) points to a genuinely
///   allocated device memory region in [`context`](DeviceSpan::context).
/// - **Extent bounds**: The allocation contains at least [`num_bytes`](DeviceSpan::num_bytes)
///   contiguous, allocated bytes accessible to CUDA driver operations.
/// - **Lifetime**: The allocation remains alive and stationary for the duration of any operation
///   referencing this span.
/// - **Context & stream synchronization**: [`context`](DeviceSpan::context) is the owning CUDA
///   context matching the operation stream; callers ensure prior stream dependencies are synchronized.
///
/// Downstream code cannot fabricate untrusted device spans with arbitrary addresses or lengths:
/// ```compile_fail
/// struct FabricatedSpan;
/// impl cudnn::DeviceSpan for FabricatedSpan {
///     fn context(&self) -> &std::sync::Arc<cuda_core::Context> { unimplemented!() }
///     fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr { 0xdeadbeef }
///     fn num_bytes(&self) -> usize { 1024 * 1024 }
/// }
/// ```
pub trait DeviceSpan: sealed::Sealed {
    /// Owning CUDA context.
    fn context(&self) -> &Arc<Context>;
    /// Base device address.
    fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr;
    /// Extent in bytes.
    fn num_bytes(&self) -> usize;
}

impl<T> DeviceSpan for DeviceBuffer<T> {
    fn context(&self) -> &Arc<Context> {
        DeviceBuffer::context(self)
    }

    fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr {
        DeviceBuffer::cu_deviceptr(self)
    }

    fn num_bytes(&self) -> usize {
        DeviceBuffer::num_bytes(self)
    }
}

impl<T> DeviceSpan for DeviceBufferView<'_, T> {
    fn context(&self) -> &Arc<Context> {
        DeviceBufferView::context(self)
    }

    fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr {
        DeviceBufferView::cu_deviceptr(self)
    }

    fn num_bytes(&self) -> usize {
        DeviceBufferView::num_bytes(self)
    }
}

/// Returns the loaded cuDNN library version number (`MAJOR * 10000 + MINOR * 100 + PATCH`).
pub fn version() -> Result<usize> {
    sys::load()?;
    // SAFETY: `cudnnGetVersion` takes no arguments and returns a version integer.
    Ok(unsafe { sys::cudnnGetVersion()? })
}

/// Returns the maximum CUDA driver/runtime version supported by the loaded cuDNN library.
pub fn max_device_version() -> Result<usize> {
    sys::load()?;
    // SAFETY: `cudnnGetMaxDeviceVersion` takes no arguments and returns a version integer.
    Ok(unsafe { sys::cudnnGetMaxDeviceVersion()? })
}

/// Returns the CUDA toolkit version cuDNN was compiled against.
pub fn cudart_version() -> Result<usize> {
    sys::load()?;
    // SAFETY: `cudnnGetCudartVersion` takes no arguments and returns a version integer.
    Ok(unsafe { sys::cudnnGetCudartVersion()? })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_status_and_datatype_mapping() {
        assert!(CudnnStatus::SUCCESS.is_success());
        assert!(!CudnnStatus::BAD_PARAM.is_success());
        assert_eq!(CudnnStatus(0), CudnnStatus::SUCCESS);
        let unknown = CudnnStatus(99_999);
        assert!(!unknown.is_success());

        assert!(check("noop", sys::cudnnStatus_t_CUDNN_STATUS_SUCCESS).is_ok());
        let err = check("cudnnCreate", sys::cudnnStatus_t_CUDNN_STATUS_BAD_PARAM).unwrap_err();
        assert!(matches!(
            err,
            Error::Status {
                call: "cudnnCreate",
                code: CudnnStatus::BAD_PARAM
            }
        ));

        for (dt, bytes) in [
            (DataType::F32, 4usize),
            (DataType::F64, 8),
            (DataType::F16, 2),
            (DataType::Bf16, 2),
            (DataType::I8, 1),
            (DataType::U8, 1),
            (DataType::I32, 4),
            (DataType::I64, 8),
        ] {
            assert_eq!(dt.elem_bytes(), bytes);
            assert_eq!(DataType::from_sys(dt.to_sys()).unwrap(), dt);
        }
        assert!(DataType::from_sys(999).is_err());
    }

    #[test]
    fn t0_budget_and_i32_bounds() {
        assert!(within(None, usize::MAX).is_ok());
        assert!(within(Some(1024), 1024).is_ok());
        let err = within(Some(1024), 1025).unwrap_err();
        assert!(matches!(
            err,
            Error::Budget {
                requested: 1025,
                available: 1024
            }
        ));

        assert_eq!(checked_i32(42, "dim").unwrap(), 42);
        assert!(checked_i32(usize::MAX, "dim").is_err());
    }

    #[test]
    #[ignore = "requires CUDA GPU and cuDNN runtime"]
    fn t2_conv_and_relu_forward() {
        let ctx = Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();
        let mut handle = Handle::new(&ctx).unwrap();
        handle.set_stream(&stream).unwrap();

        // 1x1x3x3 input, 1x1x2x2 ones filter -> 1x1x2x2 output; ReLU clamps negatives.
        let x_desc =
            TensorDescriptor::new_4d(&ctx, TensorFormat::Nchw, DataType::F32, [1, 1, 3, 3])
                .unwrap();
        let w_desc =
            FilterDescriptor::new_4d(&ctx, TensorFormat::Nchw, DataType::F32, [1, 1, 2, 2])
                .unwrap();
        let conv_desc = ConvolutionDescriptor::new_2d(
            &ctx,
            [0, 0],
            [1, 1],
            [1, 1],
            ConvMode::CrossCorrelation,
            DataType::F32,
        )
        .unwrap();
        let out_dims = conv_desc.output_dim_4d(&x_desc, &w_desc).unwrap();
        assert_eq!(out_dims, [1, 1, 2, 2]);
        let y_desc =
            TensorDescriptor::new_4d(&ctx, TensorFormat::Nchw, DataType::F32, out_dims).unwrap();

        let x_host: [f32; 9] = [-10.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let w_host: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let x_dev = DeviceBuffer::from_host(&stream, &x_host).unwrap();
        let w_dev = DeviceBuffer::from_host(&stream, &w_host).unwrap();
        let mut y_dev = DeviceBuffer::<f32>::zeroed(&stream, 4).unwrap();
        let mut z_dev = DeviceBuffer::<f32>::zeroed(&stream, 4).unwrap();

        let mut scratch = Scratch::new(&stream);
        conv_forward(
            &mut handle,
            &stream,
            &mut scratch,
            Some(64 * 1024 * 1024),
            1.0,
            &x_desc,
            &x_dev,
            &w_desc,
            &w_dev,
            &conv_desc,
            ConvFwdAlgo::ImplicitGemm,
            0.0,
            &y_desc,
            &mut y_dev,
        )
        .unwrap();

        let act = ActivationDescriptor::new(
            &ctx,
            ActivationMode::Relu,
            NanPropagation::NotPropagate,
            0.0,
        )
        .unwrap();
        activation_forward(
            &mut handle,
            &stream,
            &act,
            1.0,
            &y_desc,
            &y_dev,
            0.0,
            &y_desc,
            &mut z_dev,
        )
        .unwrap();

        stream.synchronize().unwrap();
        let out = z_dev.to_host_vec(&stream).unwrap();
        // Top-left window sum: -10 + 1 + 3 + 4 = -2 -> ReLU = 0; others: 12, 20, 24.
        assert_eq!(out, vec![0.0, 12.0, 20.0, 24.0]);
    }

    #[test]
    fn t0_device_span_sealed_and_internal_impls() {
        fn assert_device_span<T: DeviceSpan>() {}
        assert_device_span::<DeviceBuffer<f32>>();
        assert_device_span::<DeviceBufferView<'_, f32>>();
    }
}
