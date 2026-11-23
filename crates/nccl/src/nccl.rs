//! Safe Rust wrappers for NVIDIA NCCL collective and point-to-point GPU communication.
//!
//! Provides RAII [`Comm`] communicators (`init_all`, `init_rank`), portable [`UniqueId`] tokens,
//! explicit [`DataType`] / [`RedOp`] enums, [`group`] / [`group_start`] / [`group_end`] fusion,
//! stream-ordered collectives (`all_reduce`, `all_reduce_in_place`, `broadcast`, `reduce`,
//! `all_gather`, `reduce_scatter`, `send`, `recv`), and async-error-aware teardown
//! (`async_error`, `finalize`, `abort`, and abort-on-fault in `Drop`; a healthy `Drop` can block,
//! see [`Comm`]).
//!
//! Platform note: `nccl-sys` exists only on Linux. This crate compiles cleanly on both Windows and
//! Linux; on non-Linux targets, runtime entry points return [`Error::Unsupported`] while types,
//! serialization, and validation remain available.

/// RAII NCCL communicators, unique rendezvous IDs, group fusion, collectives, and teardown.
pub mod comm;

use std::fmt;
use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, DeviceBufferView, Stream};
use thiserror::Error;

pub use comm::{AsyncState, Comm, GroupGuard, UniqueId, group, group_end, group_start};

/// Crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Raw NCCL status code (`ncclResult_t`) preserved as a `Copy` integer newtype so future NCCL
/// error codes survive FFI intact.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NcclStatus(
    /// Raw `ncclResult_t` integer value.
    pub u32,
);

impl NcclStatus {
    /// `ncclSuccess` (`0`).
    pub const SUCCESS: Self = Self(0);
    /// `ncclUnhandledCudaError` (`1`).
    pub const UNHANDLED_CUDA_ERROR: Self = Self(1);
    /// `ncclSystemError` (`2`).
    pub const SYSTEM_ERROR: Self = Self(2);
    /// `ncclInternalError` (`3`).
    pub const INTERNAL_ERROR: Self = Self(3);
    /// `ncclInvalidArgument` (`4`).
    pub const INVALID_ARGUMENT: Self = Self(4);
    /// `ncclInvalidUsage` (`5`).
    pub const INVALID_USAGE: Self = Self(5);
    /// `ncclRemoteError` (`6`).
    pub const REMOTE_ERROR: Self = Self(6);
    /// `ncclInProgress` (`7`).
    pub const IN_PROGRESS: Self = Self(7);
    /// `ncclTimeout` (`8`).
    pub const TIMEOUT: Self = Self(8);

    /// Whether this code represents `ncclSuccess`.
    pub const fn is_success(self) -> bool {
        self.0 == Self::SUCCESS.0
    }

    /// Whether an async communicator operation or teardown is still in progress.
    pub const fn is_in_progress(self) -> bool {
        self.0 == Self::IN_PROGRESS.0
    }
}

impl fmt::Debug for NcclStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NcclStatus({}, {self})", self.0)
    }
}

impl fmt::Display for NcclStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(target_os = "linux")]
        if nccl_sys::load().is_ok() {
            // SAFETY: `ncclGetErrorString` returns a static string pointer or null.
            let ptr = unsafe { nccl_sys::ncclGetErrorString(self.0) };
            if !ptr.is_null() {
                // SAFETY: non-null return from `ncclGetErrorString` is a valid static C string.
                let msg = unsafe { std::ffi::CStr::from_ptr(ptr) }.to_string_lossy();
                return f.write_str(&msg);
            }
        }
        write!(f, "ncclResult_t({})", self.0)
    }
}

/// Errors produced by safe `nccl` operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to locate or open `libnccl`.
    #[error("failed to load NCCL: {0}")]
    Load(#[from] &'static nv_loader::LoadError),
    /// Failed to resolve an unshimmed NCCL symbol.
    #[error("NCCL symbol call failed: {0}")]
    Call(#[from] nv_loader::CallError),
    /// CUDA driver error from context, stream, or device memory operations.
    #[error(transparent)]
    Driver(#[from] cuda_core::DriverError),
    /// NCCL returned a non-success status code.
    #[error("{call} failed: {code} ({})", code.0)]
    Status {
        /// Failing NCCL C function name.
        call: &'static str,
        /// Returned `ncclResult_t` status code.
        code: NcclStatus,
    },
    /// Invalid rank, peer, root, buffer length, context mismatch, or byte budget overrun.
    #[error("{0}")]
    Invalid(&'static str),
    /// NCCL requires Linux (`nccl-sys` is unavailable on this host OS) or an unsupported mode.
    #[error("{0}")]
    Unsupported(&'static str),
}

/// Converts a raw `ncclResult_t` integer into `Result<()>`.
pub fn check(call: &'static str, raw: u32) -> Result<()> {
    let code = NcclStatus(raw);
    if code.is_success() {
        Ok(())
    } else {
        Err(Error::Status { call, code })
    }
}

pub(crate) fn same_ctx(stream: &Stream, ctx: &Arc<Context>) -> Result<()> {
    if !Arc::ptr_eq(stream.context(), ctx) {
        return Err(Error::Invalid("stream belongs to a different CUDA context"));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub(crate) fn record_teardown(ctx: &Context, raw: u32) {
    if raw != 0 {
        ctx.record_err(Err::<(), _>(cuda_core::DriverError(
            cuda_bindings::cudaError_enum_CUDA_ERROR_UNKNOWN,
        )));
    }
}

/// Element data types supported by NCCL collectives (`ncclDataType_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    /// 8-bit signed integer (`ncclInt8`).
    I8,
    /// 8-bit unsigned integer (`ncclUint8`).
    U8,
    /// 32-bit signed integer (`ncclInt32`).
    I32,
    /// 32-bit unsigned integer (`ncclUint32`).
    U32,
    /// 64-bit signed integer (`ncclInt64`).
    I64,
    /// 64-bit unsigned integer (`ncclUint64`).
    U64,
    /// 16-bit IEEE float (`ncclFloat16`).
    F16,
    /// 32-bit IEEE float (`ncclFloat32`).
    F32,
    /// 64-bit IEEE float (`ncclFloat64`).
    F64,
    /// 16-bit brain float (`ncclBfloat16`).
    Bf16,
    /// 8-bit FP8 E4M3 (`ncclFloat8e4m3`).
    F8E4M3,
    /// 8-bit FP8 E5M2 (`ncclFloat8e5m2`).
    F8E5M2,
}

impl DataType {
    /// Numeric `ncclDataType_t` discriminant matching `nccl.h`.
    pub const fn to_raw(self) -> u32 {
        match self {
            Self::I8 => 0,
            Self::U8 => 1,
            Self::I32 => 2,
            Self::U32 => 3,
            Self::I64 => 4,
            Self::U64 => 5,
            Self::F16 => 6,
            Self::F32 => 7,
            Self::F64 => 8,
            Self::Bf16 => 9,
            Self::F8E4M3 => 10,
            Self::F8E5M2 => 11,
        }
    }

    /// Size of a single element in bytes.
    pub const fn elem_bytes(self) -> usize {
        match self {
            Self::I8 | Self::U8 | Self::F8E4M3 | Self::F8E5M2 => 1,
            Self::F16 | Self::Bf16 => 2,
            Self::I32 | Self::U32 | Self::F32 => 4,
            Self::I64 | Self::U64 | Self::F64 => 8,
        }
    }

    #[cfg(target_os = "linux")]
    pub(crate) const fn to_sys(self) -> nccl_sys::ncclDataType_t {
        match self {
            Self::I8 => nccl_sys::ncclDataType_t_ncclInt8,
            Self::U8 => nccl_sys::ncclDataType_t_ncclUint8,
            Self::I32 => nccl_sys::ncclDataType_t_ncclInt32,
            Self::U32 => nccl_sys::ncclDataType_t_ncclUint32,
            Self::I64 => nccl_sys::ncclDataType_t_ncclInt64,
            Self::U64 => nccl_sys::ncclDataType_t_ncclUint64,
            Self::F16 => nccl_sys::ncclDataType_t_ncclFloat16,
            Self::F32 => nccl_sys::ncclDataType_t_ncclFloat32,
            Self::F64 => nccl_sys::ncclDataType_t_ncclFloat64,
            Self::Bf16 => nccl_sys::ncclDataType_t_ncclBfloat16,
            Self::F8E4M3 => nccl_sys::ncclDataType_t_ncclFloat8e4m3,
            Self::F8E5M2 => nccl_sys::ncclDataType_t_ncclFloat8e5m2,
        }
    }
}

/// Built-in collective reduction operators (`ncclRedOp_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RedOp {
    /// Element-wise sum (`ncclSum`).
    Sum,
    /// Element-wise product (`ncclProd`).
    Prod,
    /// Element-wise maximum (`ncclMax`).
    Max,
    /// Element-wise minimum (`ncclMin`).
    Min,
    /// Element-wise average across ranks (`ncclAvg`).
    Avg,
}

impl RedOp {
    /// Numeric `ncclRedOp_t` discriminant matching `nccl.h`.
    pub const fn to_raw(self) -> u32 {
        match self {
            Self::Sum => 0,
            Self::Prod => 1,
            Self::Max => 2,
            Self::Min => 3,
            Self::Avg => 4,
        }
    }

    #[cfg(target_os = "linux")]
    pub(crate) const fn to_sys(self) -> nccl_sys::ncclRedOp_t {
        match self {
            Self::Sum => nccl_sys::ncclRedOp_t_ncclSum,
            Self::Prod => nccl_sys::ncclRedOp_t_ncclProd,
            Self::Max => nccl_sys::ncclRedOp_t_ncclMax,
            Self::Min => nccl_sys::ncclRedOp_t_ncclMin,
            Self::Avg => nccl_sys::ncclRedOp_t_ncclAvg,
        }
    }
}

mod sealed {
    use super::*;

    pub trait Sealed {}

    impl<T> Sealed for DeviceBuffer<T> {}
    impl<T> Sealed for DeviceBufferView<'_, T> {}
}

/// Read-only device span accepted as collective send input (`DeviceBuffer<T>` or `DeviceBufferView<T>`).
///
/// # Safety Invariant & Obligations
/// This trait is sealed via the internal [`sealed::Sealed`] supertrait and cannot be implemented
/// by external or downstream crates. This guarantees that:
/// - **Allocation validity**: [`cu_deviceptr`](DeviceSpan::cu_deviceptr) points to a genuinely
///   allocated device memory region in [`context`](DeviceSpan::context).
/// - **Extent bounds**: The allocation contains at least [`num_bytes`](DeviceSpan::num_bytes)
///   contiguous, allocated bytes accessible to collective communication kernels.
/// - **Lifetime**: The allocation remains alive and stationary for the duration of any collective
///   or point-to-point transfer referencing this span.
/// - **Context & stream synchronization**: [`context`](DeviceSpan::context) is the owning CUDA
///   context of the communicator; callers ensure stream dependencies are synchronized before enqueue.
///
/// Downstream code cannot fabricate untrusted device spans with arbitrary addresses or lengths:
/// ```compile_fail
/// struct FabricatedSpan;
/// impl nccl::DeviceSpan for FabricatedSpan {
///     fn context(&self) -> &std::sync::Arc<cuda_core::Context> { unimplemented!() }
///     fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr { 0xdeadbeef }
///     fn num_bytes(&self) -> usize { 1024 * 1024 }
/// }
/// ```
pub trait DeviceSpan: sealed::Sealed {
    /// Owning CUDA context.
    fn context(&self) -> &Arc<Context>;
    /// Base device pointer.
    fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr;
    /// Byte length of the span.
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

/// Returns the loaded NCCL library version integer (`MAJOR * 10000 + MINOR * 100 + PATCH` on 2.9+).
pub fn version() -> Result<i32> {
    #[cfg(target_os = "linux")]
    {
        nccl_sys::load()?;
        let mut ver = 0i32;
        // SAFETY: `ver` is a valid stack out-pointer.
        let status = unsafe { nccl_sys::ncclGetVersion(&mut ver) };
        check("ncclGetVersion", status)?;
        Ok(ver)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(Error::Unsupported("NCCL is only supported on Linux"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_status_dtype_redop_invariants() {
        assert!(NcclStatus::SUCCESS.is_success());
        assert!(!NcclStatus::IN_PROGRESS.is_success());
        assert!(NcclStatus::IN_PROGRESS.is_in_progress());
        assert!(check("ok", 0).is_ok());
        assert!(matches!(
            check("ncclAllReduce", NcclStatus::INVALID_ARGUMENT.0),
            Err(Error::Status {
                call: "ncclAllReduce",
                code: NcclStatus::INVALID_ARGUMENT
            })
        ));

        assert_eq!(DataType::I8.to_raw(), 0);
        assert_eq!(DataType::F32.to_raw(), 7);
        assert_eq!(DataType::F32.elem_bytes(), 4);
        assert_eq!(DataType::Bf16.elem_bytes(), 2);
        assert_eq!(DataType::F8E4M3.elem_bytes(), 1);
        assert_eq!(DataType::F64.elem_bytes(), 8);

        assert_eq!(RedOp::Sum.to_raw(), 0);
        assert_eq!(RedOp::Avg.to_raw(), 4);
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn t0_non_linux_explicit_unsupported() {
        assert!(matches!(version(), Err(Error::Unsupported(_))));
        assert!(matches!(UniqueId::new(), Err(Error::Unsupported(_))));
        assert!(matches!(group_start(), Err(Error::Unsupported(_))));
        assert!(matches!(group_end(), Err(Error::Unsupported(_))));
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[ignore = "requires Linux CUDA GPU(s) and libnccl"]
    fn t2_single_rank_collectives() {
        let ctx = Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();
        let comms = Comm::init_all(std::slice::from_ref(&ctx)).unwrap();
        assert_eq!(comms.len(), 1);
        let comm = &comms[0];
        assert_eq!(comm.count(), 1);
        assert_eq!(comm.user_rank(), 0);
        // NCCL-6: the communicator reports the CUDA device ordinal, not the `CUdevice` handle.
        assert_eq!(usize::try_from(comm.cu_device()).unwrap(), ctx.ordinal());

        let send = DeviceBuffer::from_host(&stream, &[1.5f32, 2.5, 3.5, 4.5]).unwrap();
        let mut recv = DeviceBuffer::<f32>::zeroed(&stream, 4).unwrap();
        group(|| {
            comm.all_reduce(&send, &mut recv, 4, DataType::F32, RedOp::Sum, &stream)?;
            Ok(())
        })
        .unwrap();
        stream.synchronize().unwrap();
        assert_eq!(comm.async_error().unwrap(), AsyncState::Ok);
        assert_eq!(
            recv.to_host_vec(&stream).unwrap(),
            vec![1.5f32, 2.5, 3.5, 4.5]
        );
    }

    #[test]
    #[ignore = "requires 2+ Linux CUDA GPUs and libnccl"]
    fn t3_multi_rank_all_reduce() {
        let count = Context::device_count().unwrap();
        if count < 2 {
            eprintln!("skipping t3_multi_rank_all_reduce: requires >= 2 GPUs, found {count}");
            return;
        }
        let ctx0 = Context::new(0).unwrap();
        let ctx1 = Context::new(1).unwrap();
        let s0 = ctx0.new_stream().unwrap();
        let s1 = ctx1.new_stream().unwrap();
        let comms = Comm::init_all(&[ctx0.clone(), ctx1.clone()]).unwrap();
        assert_eq!(comms.len(), 2);

        let tx0 = DeviceBuffer::from_host(&s0, &[10.0f32, 20.0]).unwrap();
        let tx1 = DeviceBuffer::from_host(&s1, &[1.0f32, 2.0]).unwrap();
        let mut rx0 = DeviceBuffer::<f32>::zeroed(&s0, 2).unwrap();
        let mut rx1 = DeviceBuffer::<f32>::zeroed(&s1, 2).unwrap();

        group(|| {
            comms[0].all_reduce(&tx0, &mut rx0, 2, DataType::F32, RedOp::Sum, &s0)?;
            comms[1].all_reduce(&tx1, &mut rx1, 2, DataType::F32, RedOp::Sum, &s1)?;
            Ok(())
        })
        .unwrap();
        s0.synchronize().unwrap();
        s1.synchronize().unwrap();
        assert_eq!(rx0.to_host_vec(&s0).unwrap(), vec![11.0f32, 22.0]);
        assert_eq!(rx1.to_host_vec(&s1).unwrap(), vec![11.0f32, 22.0]);
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[ignore = "requires Linux CUDA GPU(s) and libnccl"]
    fn t2_single_rank_all_reduce_in_place() {
        // NCCL-7: in-place reduction (`sendbuff == recvbuff`) through one exclusive borrow.
        let ctx = Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();
        let comms = Comm::init_all(std::slice::from_ref(&ctx)).unwrap();
        let mut buf = DeviceBuffer::from_host(&stream, &[1.0f32, 2.0, 3.0]).unwrap();
        comms[0]
            .all_reduce_in_place(&mut buf, 3, DataType::F32, RedOp::Sum, &stream)
            .unwrap();
        stream.synchronize().unwrap();
        assert_eq!(comms[0].async_error().unwrap(), AsyncState::Ok);
        assert_eq!(buf.to_host_vec(&stream).unwrap(), vec![1.0f32, 2.0, 3.0]);
    }

    #[test]
    #[ignore = "requires 2+ Linux CUDA GPUs and libnccl"]
    fn t3_drop_waits_for_unmatched_peer_work() {
        // NCCL-2 (deciding test): `Comm::drop` calls `ncclCommDestroy` on a healthy communicator.
        // Rank 0 enqueues a send that rank 1 has not matched; this asserts the documented claim
        // that dropping rank 0 then blocks until the peer participates.
        let count = Context::device_count().unwrap();
        if count < 2 {
            eprintln!("skipping t3_drop_waits_for_unmatched_peer_work: requires >= 2 GPUs");
            return;
        }
        let ctx0 = Context::new(0).unwrap();
        let ctx1 = Context::new(1).unwrap();
        let s0 = ctx0.new_stream().unwrap();
        let s1 = ctx1.new_stream().unwrap();
        let mut comms = Comm::init_all(&[ctx0.clone(), ctx1.clone()])
            .unwrap()
            .into_iter();
        let c0 = comms.next().unwrap();
        let c1 = comms.next().unwrap();
        let tx = DeviceBuffer::from_host(&s0, &[1.0f32, 2.0]).unwrap();
        let mut rx = DeviceBuffer::<f32>::zeroed(&s1, 2).unwrap();
        c0.send(&tx, 2, DataType::F32, 1, &s0).unwrap();

        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let dropper = std::thread::spawn(move || {
            drop(c0);
            done_tx.send(()).unwrap();
        });
        let blocked = done_rx
            .recv_timeout(std::time::Duration::from_secs(10))
            .is_err();
        if blocked {
            // Match the send so rank 0's destroy can complete, then tear rank 1 down too.
            c1.recv(&mut rx, 2, DataType::F32, 0, &s1).unwrap();
            s1.synchronize().unwrap();
            drop(c1);
        } else {
            // Rank 0 is gone; a matching recv could never complete.
            c1.abort().unwrap();
        }
        dropper.join().unwrap();
        assert!(
            blocked,
            "Comm::drop returned while a peer had not matched an enqueued send"
        );
    }

    #[test]
    fn t0_device_span_sealed_and_internal_impls() {
        fn assert_device_span<T: DeviceSpan>() {}
        assert_device_span::<DeviceBuffer<f32>>();
        assert_device_span::<DeviceBufferView<'_, f32>>();
    }
}
