//! Safe Rust wrappers for NVIDIA GPUDirect Storage (`cuFile`).
//!
//! Provides RAII [`DriverSession`] (`cuFileDriverOpen` / `cuFileDriverClose_v2`), registered file
//! handles ([`FileHandle`]), pinned device/host buffer registration ([`RegisteredBuffer`]), stream
//! hint registration ([`RegisteredStream`]), synchronous transfers, and zero-steady-state-alloc
//! stream-ordered async transfers via [`AsyncIoSlot`].
//!
//! Explicit [`IoMode`] policy:
//! - [`IoMode::GdsOnly`] requires GPUDirect Storage (`cuFile`) and **never** silently downgrades
//!   to host-staged POSIX I/O if GDS initialization, handle registration, or transfer fails.
//! - [`IoMode::PosixFallback`] must be selected explicitly by the caller and stages transfers
//!   through reusable pinned host buffers plus CUDA driver memory copies.
//!
//! Platform note: `cufile-sys` exists only on Linux. This crate compiles cleanly on both Windows
//! and Linux; GDS-only entry points return [`Error::Unsupported`] on non-Linux hosts while types,
//! status decoding, and explicit [`IoMode::PosixFallback`] remain usable and testable everywhere.

/// RAII driver session, registered file/buffer/stream guards, and sync/async transfers.
pub mod handle;

use std::fmt;
use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, DeviceBufferView};
use thiserror::Error;

pub use handle::{
    AsyncIoSlot, BufRegisterFlags, DriverProperties, DriverSession, FileHandle, NvfsProperties,
    RegisteredBuffer, RegisteredStream, StreamFlags,
};

/// Crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Base offset of cuFile error codes (`CUFILEOP_BASE_ERR` in `cufile.h`).
pub const CUFILE_BASE_ERR: u32 = 5000;

/// Raw `CUfileOpError` status code preserved as a `Copy` integer newtype.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CufileStatus(
    /// Raw `CUfileOpError` integer value.
    pub u32,
);

impl CufileStatus {
    /// `CU_FILE_SUCCESS` (`0`).
    pub const SUCCESS: Self = Self(0);
    /// `CU_FILE_DRIVER_NOT_INITIALIZED` (`5001`).
    pub const DRIVER_NOT_INITIALIZED: Self = Self(5001);
    /// `CU_FILE_DRIVER_INVALID_PROPS` (`5002`).
    pub const DRIVER_INVALID_PROPS: Self = Self(5002);
    /// `CU_FILE_DRIVER_UNSUPPORTED_LIMIT` (`5003`).
    pub const DRIVER_UNSUPPORTED_LIMIT: Self = Self(5003);
    /// `CU_FILE_DRIVER_VERSION_MISMATCH` (`5004`).
    pub const DRIVER_VERSION_MISMATCH: Self = Self(5004);
    /// `CU_FILE_DRIVER_VERSION_READ_ERROR` (`5005`).
    pub const DRIVER_VERSION_READ_ERROR: Self = Self(5005);
    /// `CU_FILE_DRIVER_CLOSING` (`5006`).
    pub const DRIVER_CLOSING: Self = Self(5006);
    /// `CU_FILE_PLATFORM_NOT_SUPPORTED` (`5007`).
    pub const PLATFORM_NOT_SUPPORTED: Self = Self(5007);
    /// `CU_FILE_IO_NOT_SUPPORTED` (`5008`).
    pub const IO_NOT_SUPPORTED: Self = Self(5008);
    /// `CU_FILE_DEVICE_NOT_SUPPORTED` (`5009`).
    pub const DEVICE_NOT_SUPPORTED: Self = Self(5009);
    /// `CU_FILE_NVFS_DRIVER_ERROR` (`5010`).
    pub const NVFS_DRIVER_ERROR: Self = Self(5010);
    /// `CU_FILE_CUDA_DRIVER_ERROR` (`5011`).
    pub const CUDA_DRIVER_ERROR: Self = Self(5011);
    /// `CU_FILE_CUDA_POINTER_INVALID` (`5012`).
    pub const CUDA_POINTER_INVALID: Self = Self(5012);
    /// `CU_FILE_CUDA_MEMORY_TYPE_INVALID` (`5013`).
    pub const CUDA_MEMORY_TYPE_INVALID: Self = Self(5013);
    /// `CU_FILE_CUDA_POINTER_RANGE_ERROR` (`5014`).
    pub const CUDA_POINTER_RANGE_ERROR: Self = Self(5014);
    /// `CU_FILE_CUDA_CONTEXT_MISMATCH` (`5015`).
    pub const CUDA_CONTEXT_MISMATCH: Self = Self(5015);
    /// `CU_FILE_INVALID_MAPPING_SIZE` (`5016`).
    pub const INVALID_MAPPING_SIZE: Self = Self(5016);
    /// `CU_FILE_INVALID_MAPPING_RANGE` (`5017`).
    pub const INVALID_MAPPING_RANGE: Self = Self(5017);
    /// `CU_FILE_INVALID_FILE_TYPE` (`5018`).
    pub const INVALID_FILE_TYPE: Self = Self(5018);
    /// `CU_FILE_INVALID_FILE_OPEN_FLAG` (`5019`).
    pub const INVALID_FILE_OPEN_FLAG: Self = Self(5019);
    /// `CU_FILE_DIO_NOT_SET` (`5020`).
    pub const DIO_NOT_SET: Self = Self(5020);
    /// `CU_FILE_INVALID_VALUE` (`5022`).
    pub const INVALID_VALUE: Self = Self(5022);
    /// `CU_FILE_MEMORY_ALREADY_REGISTERED` (`5023`).
    pub const MEMORY_ALREADY_REGISTERED: Self = Self(5023);
    /// `CU_FILE_MEMORY_NOT_REGISTERED` (`5024`).
    pub const MEMORY_NOT_REGISTERED: Self = Self(5024);
    /// `CU_FILE_PERMISSION_DENIED` (`5025`).
    pub const PERMISSION_DENIED: Self = Self(5025);
    /// `CU_FILE_DRIVER_ALREADY_OPEN` (`5026`).
    pub const DRIVER_ALREADY_OPEN: Self = Self(5026);
    /// `CU_FILE_HANDLE_NOT_REGISTERED` (`5027`).
    pub const HANDLE_NOT_REGISTERED: Self = Self(5027);
    /// `CU_FILE_HANDLE_ALREADY_REGISTERED` (`5028`).
    pub const HANDLE_ALREADY_REGISTERED: Self = Self(5028);
    /// `CU_FILE_DEVICE_NOT_FOUND` (`5029`).
    pub const DEVICE_NOT_FOUND: Self = Self(5029);
    /// `CU_FILE_INTERNAL_ERROR` (`5030`).
    pub const INTERNAL_ERROR: Self = Self(5030);
    /// `CU_FILE_GETNEWFD_FAILED` (`5031`).
    pub const GETNEWFD_FAILED: Self = Self(5031);
    /// `CU_FILE_NVFS_SETUP_ERROR` (`5033`).
    pub const NVFS_SETUP_ERROR: Self = Self(5033);
    /// `CU_FILE_IO_DISABLED` (`5034`).
    pub const IO_DISABLED: Self = Self(5034);
    /// `CU_FILE_BATCH_SUBMIT_FAILED` (`5035`).
    pub const BATCH_SUBMIT_FAILED: Self = Self(5035);
    /// `CU_FILE_GPU_MEMORY_PINNING_FAILED` (`5036`).
    pub const GPU_MEMORY_PINNING_FAILED: Self = Self(5036);
    /// `CU_FILE_BATCH_FULL` (`5037`).
    pub const BATCH_FULL: Self = Self(5037);
    /// `CU_FILE_ASYNC_NOT_SUPPORTED` (`5038`).
    pub const ASYNC_NOT_SUPPORTED: Self = Self(5038);
    /// `CU_FILE_INTERNAL_BATCH_SETUP_ERROR` (`5039`).
    pub const INTERNAL_BATCH_SETUP_ERROR: Self = Self(5039);
    /// `CU_FILE_INTERNAL_BATCH_SUBMIT_ERROR` (`5040`).
    pub const INTERNAL_BATCH_SUBMIT_ERROR: Self = Self(5040);
    /// `CU_FILE_INTERNAL_BATCH_GETSTATUS_ERROR` (`5041`).
    pub const INTERNAL_BATCH_GETSTATUS_ERROR: Self = Self(5041);
    /// `CU_FILE_INTERNAL_BATCH_CANCEL_ERROR` (`5042`).
    pub const INTERNAL_BATCH_CANCEL_ERROR: Self = Self(5042);
    /// `CU_FILE_NOMEM_ERROR` (`5043`).
    pub const NOMEM_ERROR: Self = Self(5043);
    /// `CU_FILE_IO_ERROR` (`5044`).
    pub const IO_ERROR: Self = Self(5044);
    /// `CU_FILE_INTERNAL_BUF_REGISTER_ERROR` (`5045`).
    pub const INTERNAL_BUF_REGISTER_ERROR: Self = Self(5045);
    /// `CU_FILE_HASH_OPR_ERROR` (`5046`).
    pub const HASH_OPR_ERROR: Self = Self(5046);
    /// `CU_FILE_INVALID_CONTEXT_ERROR` (`5047`).
    pub const INVALID_CONTEXT_ERROR: Self = Self(5047);
    /// `CU_FILE_NVFS_INTERNAL_DRIVER_ERROR` (`5048`).
    pub const NVFS_INTERNAL_DRIVER_ERROR: Self = Self(5048);
    /// `CU_FILE_BATCH_NOCOMPAT_ERROR` (`5049`).
    pub const BATCH_NOCOMPAT_ERROR: Self = Self(5049);
    /// `CU_FILE_IO_MAX_ERROR` (`5050`).
    pub const IO_MAX_ERROR: Self = Self(5050);

    /// Whether this status code represents `CU_FILE_SUCCESS`.
    pub const fn is_success(self) -> bool {
        self.0 == Self::SUCCESS.0
    }

    /// Whether this status indicates an underlying CUDA driver error in `cu_err`.
    pub const fn is_cuda_driver_error(self) -> bool {
        self.0 == Self::CUDA_DRIVER_ERROR.0
    }

    /// Human-readable description matching `cufileop_status_error` in `cufile.h`.
    pub const fn description(self) -> &'static str {
        match self.0 {
            0 => "cufile success",
            5001 => "nvidia-fs driver is not loaded",
            5002 => "invalid property",
            5003 => "property range error",
            5004 => "nvidia-fs driver version mismatch",
            5005 => "nvidia-fs driver version read error",
            5006 => "driver shutdown in progress",
            5007 => "GPUDirect Storage not supported on current platform",
            5008 => "GPUDirect Storage not supported on current file",
            5009 => "GPUDirect Storage not supported on current GPU",
            5010 => "nvidia-fs driver ioctl error",
            5011 => "CUDA Driver API error",
            5012 => "invalid device pointer",
            5013 => "invalid pointer memory type",
            5014 => "pointer range exceeds allocated address range",
            5015 => "cuda context mismatch",
            5016 => "access beyond maximum pinned size",
            5017 => "access beyond mapped size",
            5018 => "unsupported file type",
            5019 => "unsupported file open flags",
            5020 => "fd direct IO not set",
            5022 => "invalid arguments",
            5023 => "device pointer already registered",
            5024 => "device pointer lookup failure",
            5025 => "driver or file access error",
            5026 => "driver is already open",
            5027 => "file descriptor is not registered",
            5028 => "file descriptor is already registered",
            5029 => "GPU device not found",
            5030 => "internal error",
            5031 => "failed to obtain new file descriptor",
            5033 => "NVFS driver initialization error",
            5034 => "GPUDirect Storage disabled by config on current file",
            5035 => "failed to submit batch operation",
            5036 => "failed to allocate pinned GPU Memory",
            5037 => "queue full for batch operation",
            5038 => "cuFile stream operation not supported",
            5039 => "batch setup internal error",
            5040 => "batch submit internal error",
            5041 => "batch get status internal error",
            5042 => "batch cancel internal error",
            5043 => "cufile no memory error",
            5044 => "cufile io error",
            5045 => "cufile buf registration error",
            5046 => "cufile hash operation error",
            5047 => "cufile invalid context error",
            5048 => "nvfs internal driver error",
            5049 => "compat mode off error",
            5050 => "GPUDirect Storage Max Error",
            _ => "unknown cufile error",
        }
    }
}

impl fmt::Debug for CufileStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CufileStatus({}, {self})", self.0)
    }
}

impl fmt::Display for CufileStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

/// Explicit I/O execution policy for [`FileHandle`].
///
/// Never silently downgraded: [`IoMode::GdsOnly`] always fails cleanly rather than switching to
/// host-staged POSIX reads/writes behind the caller's back.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IoMode {
    /// Strict GPUDirect Storage path via `libcufile`; never falls back to host POSIX I/O.
    GdsOnly,
    /// Explicit host-staged POSIX fallback (`std::fs::File` + pinned host buffer + CUDA memcpy).
    PosixFallback,
}

/// Errors produced by safe `cufile` operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to locate or open `libcufile`.
    #[error("failed to load cuFile: {0}")]
    Load(#[from] &'static nv_loader::LoadError),
    /// Failed to resolve an unshimmed cuFile symbol.
    #[error("cuFile symbol call failed: {0}")]
    Call(#[from] nv_loader::CallError),
    /// CUDA driver error from context, stream, event, or device memory operations.
    #[error(transparent)]
    Driver(#[from] cuda_core::DriverError),
    /// Filesystem / OS I/O error (POSIX `errno` on `-1` returns or explicit `PosixFallback`).
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// cuFile returned a structured non-success status code.
    #[error("{call} failed: {code} ({}, cuda={cuda_err:?})", code.0)]
    Status {
        /// Failing cuFile C entry point name.
        call: &'static str,
        /// Returned `CUfileOpError` status.
        code: CufileStatus,
        /// Accompanying CUDA driver status when `code` is [`CufileStatus::CUDA_DRIVER_ERROR`].
        cuda_err: cuda_bindings::CUresult,
    },
    /// Invalid buffer range, offset overflow, context mismatch, or already-consumed handle.
    #[error("{0}")]
    Invalid(&'static str),
    /// cuFile GDS requires Linux or an unsupported runtime configuration was requested.
    #[error("{0}")]
    Unsupported(&'static str),
}

#[cfg(target_os = "linux")]
pub(crate) fn check_status(call: &'static str, raw: cufile_sys::CUfileError_t) -> Result<()> {
    let code = CufileStatus(raw.err);
    if code.is_success() {
        Ok(())
    } else {
        Err(Error::Status {
            call,
            code,
            cuda_err: raw.cu_err,
        })
    }
}

/// Decodes the `ssize_t` return convention shared by `cuFileRead`, `cuFileWrite`,
/// `cuFileReadAsync`, and `cuFileWriteAsync`:
/// - `>= 0`: byte count transferred
/// - `-1`: OS `errno` captured via `last_os_error`
/// - `< -1`: negated `CUfileOpError` status code
pub fn decode_io_ret(call: &'static str, ret: isize) -> Result<usize> {
    decode_io_ret_with(call, ret, std::io::Error::last_os_error)
}

pub(crate) fn decode_io_ret_with(
    call: &'static str,
    ret: isize,
    last_err: impl FnOnce() -> std::io::Error,
) -> Result<usize> {
    if ret >= 0 {
        return Ok(ret as usize);
    }
    if ret == -1 {
        return Err(Error::Io(last_err()));
    }
    let raw = ret
        .checked_neg()
        .and_then(|pos| u32::try_from(pos).ok())
        .ok_or(Error::Invalid("cuFile I/O return value out of range"))?;
    Err(Error::Status {
        call,
        code: CufileStatus(raw),
        cuda_err: cuda_bindings::cudaError_enum_CUDA_SUCCESS,
    })
}

pub(crate) fn record_teardown_driver_err(ctx: &Context, err: cuda_bindings::CUresult) {
    let cu = if err == cuda_bindings::cudaError_enum_CUDA_SUCCESS {
        cuda_bindings::cudaError_enum_CUDA_ERROR_UNKNOWN
    } else {
        err
    };
    ctx.record_err(Err::<(), _>(cuda_core::DriverError(cu)));
}

#[cfg(target_os = "linux")]
pub(crate) fn record_teardown_cufile(ctx: &Context, raw: cufile_sys::CUfileError_t) {
    if raw.err != 0 {
        record_teardown_driver_err(ctx, raw.cu_err);
    }
}

/// Read-only device span accepted by cuFile write operations (`DeviceBuffer<T>`,
/// `DeviceBufferView<'_, T>`, or [`RegisteredBuffer`]).
pub trait DeviceSpan {
    /// Owning CUDA context.
    fn context(&self) -> &Arc<Context>;
    /// Base CUDA device pointer.
    fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr;
    /// Byte length of the device region.
    fn num_bytes(&self) -> usize;
}

/// Mutable device span accepted by cuFile read operations (`DeviceBuffer<T>`,
/// `DeviceBufferView<'_, T>`, or [`RegisteredBuffer`]).
pub trait DeviceSpanMut: DeviceSpan {}

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

impl<T> DeviceSpanMut for DeviceBuffer<T> {}

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

impl<T> DeviceSpanMut for DeviceBufferView<'_, T> {}

impl<T: DeviceSpan + ?Sized> DeviceSpan for &mut T {
    fn context(&self) -> &Arc<Context> {
        (**self).context()
    }

    fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr {
        (**self).cu_deviceptr()
    }

    fn num_bytes(&self) -> usize {
        (**self).num_bytes()
    }
}

impl<T: DeviceSpanMut + ?Sized> DeviceSpanMut for &mut T {}

impl<T: DeviceSpan + ?Sized> DeviceSpan for Arc<T> {
    fn context(&self) -> &Arc<Context> {
        (**self).context()
    }

    fn cu_deviceptr(&self) -> cuda_bindings::CUdeviceptr {
        (**self).cu_deviceptr()
    }

    fn num_bytes(&self) -> usize {
        (**self).num_bytes()
    }
}

impl<T: DeviceSpanMut + ?Sized> DeviceSpanMut for Arc<T> {}

/// Returns the loaded `libcufile` library version integer.
pub fn version() -> Result<i32> {
    #[cfg(target_os = "linux")]
    {
        cufile_sys::load()?;
        let mut ver = 0i32;
        // SAFETY: `ver` is a valid aligned output integer.
        let status = unsafe { cufile_sys::cuFileGetVersion(&mut ver) };
        check_status("cuFileGetVersion", status)?;
        Ok(ver)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(Error::Unsupported("cuFile requires Linux"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_status_and_io_ret_decoding() {
        assert!(CufileStatus::SUCCESS.is_success());
        assert!(!CufileStatus::IO_NOT_SUPPORTED.is_success());
        assert!(CufileStatus::CUDA_DRIVER_ERROR.is_cuda_driver_error());
        assert_eq!(CufileStatus::DIO_NOT_SET.0, CUFILE_BASE_ERR + 20);
        assert_eq!(CufileStatus::IO_MAX_ERROR.0, CUFILE_BASE_ERR + 50);
        assert!(!CufileStatus::NVFS_DRIVER_ERROR.description().is_empty());

        assert_eq!(decode_io_ret("cuFileRead", 0).unwrap(), 0);
        assert_eq!(decode_io_ret("cuFileRead", 4096).unwrap(), 4096);

        let err_os = decode_io_ret_with("cuFileRead", -1, || {
            std::io::Error::from(std::io::ErrorKind::PermissionDenied)
        });
        assert!(
            matches!(err_os, Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::PermissionDenied)
        );

        let err_cufile = decode_io_ret("cuFileWrite", -5020);
        assert!(matches!(
            err_cufile,
            Err(Error::Status {
                call: "cuFileWrite",
                code: CufileStatus::DIO_NOT_SET,
                ..
            })
        ));
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn t0_non_linux_gds_explicit_unsupported_never_downgrades() {
        assert!(matches!(version(), Err(Error::Unsupported(_))));
        assert!(matches!(
            DriverSession::use_count(),
            Err(Error::Unsupported(_))
        ));
    }

    #[test]
    #[ignore = "requires Linux CUDA GPU (with or without nvidia-fs driver)"]
    fn t2_cufile_register_rw_roundtrip() {
        let ctx = Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();
        let path = std::env::temp_dir().join("nv_rs_cufile_t2_roundtrip.bin");
        let payload: Vec<u8> = (0..4096u32)
            .map(|i| (i.wrapping_mul(17) & 0xff) as u8)
            .collect();

        // Try GDS session first; under PosixFallback verify explicit fallback works regardless.
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();
        let fh = FileHandle::register(&ctx, file, IoMode::PosixFallback).unwrap();

        let src_dev = Arc::new(DeviceBuffer::from_host(&stream, &payload).unwrap());
        let dst_dev = Arc::new(DeviceBuffer::<u8>::zeroed(&stream, payload.len()).unwrap());
        stream.synchronize().unwrap();

        let wrote = fh.write(0, &src_dev, 0, payload.len()).unwrap();
        assert_eq!(wrote, payload.len());
        let read = fh.read(0, &dst_dev, 0, payload.len()).unwrap();
        assert_eq!(read, payload.len());

        let host = dst_dev.to_host_vec(&stream).unwrap();
        assert_eq!(host, payload);

        // Async stream-ordered path with reusable slot (both read_async and write_async).
        let async_file_buf = Arc::new(DeviceBuffer::<u8>::zeroed(&stream, payload.len()).unwrap());
        let mut slot = AsyncIoSlot::new(&ctx).unwrap();
        fh.write_async(&stream, 4096, &src_dev, 0, payload.len(), &mut slot)
            .unwrap();
        assert_eq!(slot.wait().unwrap(), payload.len());

        fh.read_async(&stream, 4096, &async_file_buf, 0, payload.len(), &mut slot)
            .unwrap();
        assert_eq!(slot.wait().unwrap(), payload.len());
        assert_eq!(async_file_buf.to_host_vec(&stream).unwrap(), payload);

        drop(FileHandle::deregister(fh).unwrap());
        let _ = std::fs::remove_file(path);
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[ignore = "requires Linux GPUDirect Storage / libcufile"]
    fn t2_gds_session_register_roundtrip() {
        let ctx = Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();
        let _session = DriverSession::open(&ctx).unwrap();
        let path = std::env::temp_dir().join("nv_rs_cufile_gds_t2.bin");
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();

        let payload: Vec<u8> = (0..4096u32).map(|i| (i & 0xff) as u8).collect();
        let src = DeviceBuffer::from_host(&stream, &payload).unwrap();
        let dst = DeviceBuffer::<u8>::zeroed(&stream, payload.len()).unwrap();
        stream.synchronize().unwrap();

        let reg_src = RegisteredBuffer::register(src, BufRegisterFlags::NONE).unwrap();
        let reg_dst = RegisteredBuffer::register(dst, BufRegisterFlags::NONE).unwrap();
        let _reg_stream = RegisteredStream::register(&stream, StreamFlags::NONE).unwrap();
        let fh = FileHandle::register(&ctx, file, IoMode::GdsOnly).unwrap();

        let wrote = fh.write(0, &reg_src, 0, payload.len()).unwrap();
        assert_eq!(wrote, payload.len());
        let read = fh.read(0, &reg_dst, 0, payload.len()).unwrap();
        assert_eq!(read, payload.len());

        let dst_inner = RegisteredBuffer::deregister(reg_dst).unwrap();
        assert_eq!(dst_inner.to_host_vec(&stream).unwrap(), payload);
        drop(reg_src);
        drop(FileHandle::deregister(fh).unwrap());
        let _ = std::fs::remove_file(path);
    }
}
