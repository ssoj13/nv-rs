//! Safe stream-ordered wrappers for NVIDIA nvCOMP batched compression/decompression across
//! `LZ4`, `Snappy`, `Zstd`, `Cascaded`, `Gdeflate`, and `Bitcomp`, plus native Bitcomp plans and
//! a stateful [`CompressionManager`].
//!
//! Note on C ABI scope: `nvcomp` 6 C headers expose the batched C APIs (`nvcompBatched*`) and the
//! native Bitcomp C API (`bitcomp*`); the high-level `nvcompManager` API is C++-only. This crate
//! exposes both the explicit batched async APIs and [`CompressionManager`] built cleanly on top of
//! the C batched APIs with two-phase `temp_bytes` sizing, explicit byte budgets, reusable
//! [`Scratch`]/[`BatchStaging`] buffers, and zero steady-state allocations.

/// Stream-ordered batched compression, decompressed-size queries, and decompression.
pub mod batched;
/// Native Bitcomp single-buffer lossless/lossy plan API.
pub mod bitcomp;
/// Explicit codec configuration enums (`LZ4`, `Snappy`, `Zstd`, `Cascaded`, `Gdeflate`, `Bitcomp`).
pub mod format;
/// Stateful [`CompressionManager`] and self-describing [`CompressedBuffer`].
pub mod manager;
/// Reusable temporary workspace (`Scratch`) and event-fenced staging tables (`BatchStaging`).
pub mod scratch;

use std::ffi::CStr;
use std::fmt;
use std::sync::Arc;

use cuda_core::{Context, Stream};
use nvcomp_sys as sys;
use thiserror::Error;

pub use batched::{
    BatchedCompressReceipt, BatchedDecompressReceipt, compress_batched_async,
    decompress_batched_async, query_decompress_sizes_async,
};
pub use bitcomp::{BitcompAlgo, BitcompDataType, BitcompPlan};
pub use format::{
    Alignments, BitcompMode, BitcompOpts, BitshuffleMode, CascadedMode, CascadedOpts, DataType,
    DecompressBackend, Format, FormatKind, GdeflateOpts, Lz4Opts, SnappyOpts, ZstdOpts,
};
pub use manager::{CompressedBuffer, CompressedLayout, CompressionManager};
pub use scratch::{BatchStaging, Scratch};

/// Crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Raw nvCOMP status code (`nvcompStatus_t`) wrapped as a `Copy` integer newtype.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NvcompStatus(
    /// Raw `nvcompStatus_t` integer value.
    pub u32,
);

impl NvcompStatus {
    /// `nvcompSuccess`.
    pub const SUCCESS: Self = Self(sys::nvcompStatus_t_nvcompSuccess);
    /// `nvcompErrorInvalidValue`.
    pub const INVALID_VALUE: Self = Self(sys::nvcompStatus_t_nvcompErrorInvalidValue);
    /// `nvcompErrorNotSupported`.
    pub const NOT_SUPPORTED: Self = Self(sys::nvcompStatus_t_nvcompErrorNotSupported);

    /// Whether this status represents success.
    pub const fn is_success(self) -> bool {
        self.0 == Self::SUCCESS.0
    }
}

impl fmt::Debug for NvcompStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NvcompStatus({}, {self})", self.0)
    }
}

impl fmt::Display for NvcompStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if sys::load().is_ok() {
            // SAFETY: `nvcompGetStatusString` returns a static string pointer or null.
            let ptr = unsafe { sys::nvcompGetStatusString(self.0) };
            if !ptr.is_null() {
                // SAFETY: non-null return from `nvcompGetStatusString` is a valid static C string.
                let msg = unsafe { CStr::from_ptr(ptr) }.to_string_lossy();
                return f.write_str(&msg);
            }
        }
        write!(f, "nvcompStatus_t({})", self.0)
    }
}

/// Raw native Bitcomp status code (`bitcompResult_t`) wrapped as a `Copy` integer newtype.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BitcompStatus(
    /// Raw `bitcompResult_t` integer value.
    pub i32,
);

impl BitcompStatus {
    /// `BITCOMP_SUCCESS`.
    pub const SUCCESS: Self = Self(sys::bitcompResult_t_BITCOMP_SUCCESS);

    /// Whether this status represents success.
    pub const fn is_success(self) -> bool {
        self.0 == Self::SUCCESS.0
    }
}

impl fmt::Debug for BitcompStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitcompStatus({})", self.0)
    }
}

impl fmt::Display for BitcompStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bitcompResult_t({})", self.0)
    }
}

/// Errors returned by safe `nvcomp` operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to locate or open the nvCOMP dynamic library.
    #[error("failed to load nvCOMP: {0}")]
    Load(#[from] &'static nv_loader::LoadError),
    /// Failed to resolve an unshimmed nvCOMP/Bitcomp symbol.
    #[error("nvCOMP symbol call failed: {0}")]
    Call(#[from] nv_loader::CallError),
    /// CUDA driver error from context, stream, event, or device allocation operations.
    #[error(transparent)]
    Driver(#[from] cuda_core::DriverError),
    /// nvCOMP returned a non-success status code.
    #[error("{call} failed: {code} ({})", code.0)]
    Status {
        /// Name of the failing nvCOMP C entry point.
        call: &'static str,
        /// Returned `nvcompStatus_t` value.
        code: NvcompStatus,
    },
    /// Native Bitcomp returned a non-success result code.
    #[error("{call} failed: {code}")]
    BitcompStatus {
        /// Name of the failing Bitcomp C entry point.
        call: &'static str,
        /// Returned `bitcompResult_t` value.
        code: BitcompStatus,
    },
    /// Required temporary or staging memory growth exceeds the caller's byte budget.
    #[error("needs {requested} device bytes, budget admits {available}")]
    Budget {
        /// Additional device bytes requested.
        requested: usize,
        /// Caller-imposed byte budget.
        available: usize,
    },
    /// Invalid chunk sizes, buffer spans, alignment, format mismatch, or context mismatch.
    #[error("{0}")]
    Invalid(&'static str),
    /// Requested codec option or backend is unsupported.
    #[error("{0}")]
    Unsupported(&'static str),
}

/// Converts a raw `nvcompStatus_t` into `Result<()>`.
pub fn check(call: &'static str, raw: sys::nvcompStatus_t) -> Result<()> {
    let code = NvcompStatus(raw);
    if code.is_success() {
        Ok(())
    } else {
        Err(Error::Status { call, code })
    }
}

/// Converts a raw `bitcompResult_t` into `Result<()>`.
pub fn check_bitcomp(call: &'static str, raw: sys::bitcompResult_t) -> Result<()> {
    let code = BitcompStatus(raw);
    if code.is_success() {
        Ok(())
    } else {
        Err(Error::BitcompStatus { call, code })
    }
}

pub(crate) fn same_ctx(stream: &Stream, ctx: &Arc<Context>) -> Result<()> {
    if !Arc::ptr_eq(stream.context(), ctx) {
        return Err(Error::Invalid("stream belongs to a different CUDA context"));
    }
    Ok(())
}

pub(crate) fn same_stream(stream: &Arc<Stream>, expected: &Arc<Stream>) -> Result<()> {
    if !Arc::ptr_eq(stream, expected) {
        return Err(Error::Invalid(
            "staging/scratch buffer belongs to a different CUDA stream",
        ));
    }
    Ok(())
}

pub(crate) fn within(budget: Option<usize>, peak: usize) -> Result<()> {
    match budget {
        Some(available) if peak > available => Err(Error::Budget {
            requested: peak,
            available,
        }),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cuda_core::DeviceBuffer;

    #[test]
    fn t0_status_and_budget_checks() {
        assert!(NvcompStatus::SUCCESS.is_success());
        assert!(!NvcompStatus::INVALID_VALUE.is_success());
        assert!(check("ok", sys::nvcompStatus_t_nvcompSuccess).is_ok());
        assert!(matches!(
            check(
                "nvcompBatchedLZ4CompressAsync",
                sys::nvcompStatus_t_nvcompErrorAlignment
            ),
            Err(Error::Status {
                call: "nvcompBatchedLZ4CompressAsync",
                ..
            })
        ));

        assert!(BitcompStatus::SUCCESS.is_success());
        assert!(!BitcompStatus(-1).is_success());
        assert!(check_bitcomp("bitcompCreatePlan", sys::bitcompResult_t_BITCOMP_SUCCESS).is_ok());
        assert!(matches!(
            check_bitcomp("bitcompCreatePlan", -1),
            Err(Error::BitcompStatus {
                call: "bitcompCreatePlan",
                code: BitcompStatus(-1)
            })
        ));

        assert!(within(None, usize::MAX).is_ok());
        assert!(within(Some(4096), 4096).is_ok());
        assert!(matches!(
            within(Some(4096), 8192),
            Err(Error::Budget {
                requested: 8192,
                available: 4096
            })
        ));
    }

    #[test]
    #[ignore = "requires CUDA GPU and nvCOMP runtime"]
    fn t2_all_formats_roundtrip_bit_exact() {
        let ctx = Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();

        // Structured pattern (compressible + non-trivial byte distribution) across 4-byte aligned length.
        let mut host_in = Vec::with_capacity(64 * 1024);
        for i in 0..(16 * 1024usize) {
            host_in.extend_from_slice(&((i as u32).wrapping_mul(17) & 0x00FF_0FFF).to_le_bytes());
        }
        let dev_in = DeviceBuffer::from_host(&stream, &host_in).unwrap();

        for kind in [
            FormatKind::Lz4,
            FormatKind::Snappy,
            FormatKind::Zstd,
            FormatKind::Cascaded,
            FormatKind::Gdeflate,
            FormatKind::Bitcomp,
        ] {
            let format = Format::default_for(kind);
            let mut mgr =
                CompressionManager::new(&stream, format, 16 * 1024, Some(512 * 1024 * 1024))
                    .unwrap();
            let compressed = mgr.compress_async(&stream, &dev_in).unwrap();
            let dev_out = mgr
                .decompress_to_buffer_async(&stream, &compressed)
                .unwrap();
            stream.synchronize().unwrap();
            let host_out = dev_out.to_host_vec(&stream).unwrap();
            assert_eq!(host_out, host_in, "roundtrip mismatch for {kind:?}");
        }
    }
}
