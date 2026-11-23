//! Safe stream-ordered wrappers for NVIDIA nvCOMP batched compression/decompression across
//! `LZ4`, `Snappy`, `Zstd`, `Cascaded`, `Gdeflate`, and `Bitcomp`, plus native Bitcomp plans and
//! a stateful [`CompressionManager`].
//!
//! Note on C ABI scope: `nvcomp` 6 C headers expose the batched C APIs (`nvcompBatched*`) and the
//! native Bitcomp C API (`bitcomp*`); the high-level `nvcompManager` API is C++-only. This crate
//! exposes both the explicit batched async APIs and [`CompressionManager`] built cleanly on top of
//! the C batched APIs with two-phase `temp_bytes` sizing, explicit byte budgets, reusable
//! [`Scratch`]/[`BatchStaging`] buffers, and zero steady-state device staging allocations.
//! [`CompressionManager`] calls block until their stream completes; the batched functions only
//! enqueue, and their receipts collect results later.

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

    const CHUNK: usize = 16 * 1024;

    fn gpu_fixture() -> (Arc<Context>, Arc<Stream>, Vec<u8>, DeviceBuffer<u8>) {
        let ctx = Context::new(0).unwrap();
        let stream = ctx.new_stream().unwrap();
        let host: Vec<u8> = (0..4 * CHUNK).map(|i| ((i / 3) % 251) as u8).collect();
        let dev = DeviceBuffer::from_host(&stream, &host).unwrap();
        (ctx, stream, host, dev)
    }

    fn slot_stride(format: Format) -> usize {
        let alignments = format.compress_alignments().unwrap();
        batched::aligned_stride(
            format.max_output_chunk_bytes(CHUNK).unwrap(),
            alignments.output,
        )
        .unwrap()
    }

    #[test]
    #[ignore = "requires CUDA GPU and nvCOMP runtime"]
    fn t2_stale_receipt_rejected_and_back_to_back_batches_roundtrip() {
        let (_ctx, stream, host, input) = gpu_fixture();
        let format = Format::default_for(FormatKind::Lz4);
        let stride = slot_stride(format);
        let mut staging = BatchStaging::new(&stream).unwrap();
        let ranges_a = std::slice::from_ref(&(0..CHUNK));
        let ranges_b = [0..CHUNK, CHUNK..2 * CHUNK];
        let mut out_a = DeviceBuffer::<u8>::zeroed(&stream, stride).unwrap();
        let mut out_b = DeviceBuffer::<u8>::zeroed(&stream, 2 * stride).unwrap();

        // B is prepared while A's kernel may still run: only A's uploads are fenced (CMP-5).
        let a = compress_batched_async(
            &stream,
            &mut staging,
            format,
            &input,
            ranges_a,
            &mut out_a,
            None,
        )
        .unwrap();
        let b = compress_batched_async(
            &stream,
            &mut staging,
            format,
            &input,
            &ranges_b,
            &mut out_b,
            None,
        )
        .unwrap();

        // A's result tables were overwritten by B: the stale receipt must not report them (CMP-1).
        let mut sizes = Vec::new();
        assert!(matches!(
            a.collect_sizes(&stream, &mut staging, &mut sizes),
            Err(Error::Invalid(_))
        ));
        b.collect_sizes(&stream, &mut staging, &mut sizes).unwrap();
        assert_eq!(sizes.len(), 2);

        let comp_ranges = sizes
            .iter()
            .enumerate()
            .map(|(i, &len)| i * stride..i * stride + len)
            .collect::<Vec<_>>();
        let mut dec = DeviceBuffer::<u8>::zeroed(&stream, 2 * CHUNK).unwrap();
        let receipt = decompress_batched_async(
            &stream,
            &mut staging,
            format,
            DecompressBackend::Default,
            &out_b,
            &comp_ranges,
            &mut dec,
            &ranges_b,
            None,
        )
        .unwrap();
        receipt
            .collect_sizes(&stream, &mut staging, &mut sizes)
            .unwrap();
        assert_eq!(
            dec.to_host_vec(&stream).unwrap(),
            host[..2 * CHUNK].to_vec()
        );
    }

    #[test]
    #[ignore = "requires CUDA GPU and nvCOMP runtime"]
    fn t2_decompress_rejects_overlapping_output_ranges() {
        let (_ctx, stream, _host, input) = gpu_fixture();
        let format = Format::default_for(FormatKind::Lz4);
        let mut staging = BatchStaging::new(&stream).unwrap();
        let mut out = DeviceBuffer::<u8>::zeroed(&stream, 2 * CHUNK).unwrap();
        let res = decompress_batched_async(
            &stream,
            &mut staging,
            format,
            DecompressBackend::Default,
            &input,
            &[0..256, 256..512],
            &mut out,
            &[0..CHUNK, CHUNK / 2..CHUNK + CHUNK / 2],
            None,
        );
        assert!(matches!(res, Err(Error::Invalid(_))));
    }

    #[test]
    #[ignore = "requires CUDA GPU and nvCOMP runtime"]
    fn t2_decompress_uses_payload_format_options() {
        let (_ctx, stream, host, input) = gpu_fixture();
        let shuffled = Format::Lz4(Lz4Opts {
            data_type: DataType::UInt,
            bitshuffle: BitshuffleMode::MsbFirst,
            sort_before_hw_decompress: false,
        });
        let mut encoder = CompressionManager::new(&stream, shuffled, CHUNK, None).unwrap();
        let mut decoder =
            CompressionManager::new(&stream, Format::default_for(FormatKind::Lz4), CHUNK, None)
                .unwrap();
        let compressed = encoder.compress_async(&stream, &input).unwrap();
        let out = decoder
            .decompress_to_buffer_async(&stream, &compressed)
            .unwrap();
        assert_eq!(out.to_host_vec(&stream).unwrap(), host);
    }

    #[test]
    #[ignore = "requires CUDA GPU and nvCOMP runtime"]
    fn t2_manager_rejects_chunk_above_format_limit() {
        let (_ctx, stream, _host, _input) = gpu_fixture();
        let lz4 = Format::default_for(FormatKind::Lz4);
        assert!(matches!(
            CompressionManager::new(&stream, lz4, (1 << 24) + 1, None),
            Err(Error::Invalid(_))
        ));
        assert!(CompressionManager::new(&stream, lz4, 1 << 24, None).is_ok());
    }

    #[test]
    #[ignore = "requires CUDA GPU and nvCOMP runtime"]
    fn t2_bitcomp_lossy_plan_rejects_lossless_compress() {
        let (ctx, stream, _host, _input) = gpu_fixture();
        let n = 4 * CHUNK;
        let mut plan = BitcompPlan::new(
            &ctx,
            n,
            BitcompDataType::F32,
            BitcompMode::LossyFpToSigned,
            BitcompAlgo::Default,
        )
        .unwrap();
        let input = DeviceBuffer::<u8>::zeroed(&stream, n).unwrap();
        let mut output =
            DeviceBuffer::<u8>::zeroed(&stream, BitcompPlan::max_buflen(n).unwrap()).unwrap();
        assert!(matches!(
            plan.compress_lossless_async(&stream, &input, &mut output, None),
            Err(Error::Unsupported(_))
        ));
    }

    /// CMP-4 regression: `bitcompUncompress` trusts the stream header and faulted (CUDA error 700)
    /// on a truncated input. The wrapper must reject every truncation before queuing the kernel,
    /// so the context stays usable: a valid round trip in the same context must still succeed.
    #[test]
    #[ignore = "requires CUDA GPU and nvCOMP runtime"]
    fn t2_bitcomp_uncompress_rejects_truncated_input() {
        let (ctx, stream, host, input) = gpu_fixture();
        let n = host.len();
        let mut plan = BitcompPlan::new(
            &ctx,
            n,
            BitcompDataType::U8,
            BitcompMode::Lossless,
            BitcompAlgo::Default,
        )
        .unwrap();
        let mut packed =
            DeviceBuffer::<u8>::zeroed(&stream, BitcompPlan::max_buflen(n).unwrap()).unwrap();
        plan.compress_lossless_async(&stream, &input, &mut packed, None)
            .unwrap();
        stream.synchronize().unwrap();
        let packed_host = packed.to_host_vec(&stream).unwrap();
        let mut packed_len = 0usize;
        // SAFETY: `packed_host` is a host copy of a complete Bitcomp stream.
        let status =
            unsafe { sys::bitcompGetCompressedSize(packed_host.as_ptr().cast(), &mut packed_len) };
        check_bitcomp("bitcompGetCompressedSize", status).unwrap();
        assert!(packed_len > bitcomp::BITCOMP_HEADER_BYTES);

        let mut out = DeviceBuffer::<u8>::zeroed(&stream, n).unwrap();
        // Shorter than the header, header only, and one byte short of the declared size.
        for len in [8, bitcomp::BITCOMP_HEADER_BYTES, packed_len - 1] {
            let truncated = DeviceBuffer::from_host(&stream, &packed_host[..len]).unwrap();
            let res = plan.uncompress_async(&stream, &truncated, &mut out);
            assert!(
                matches!(res, Err(Error::Invalid(_))),
                "truncated Bitcomp input of {len} bytes was not rejected: {res:?}"
            );
            stream.synchronize().unwrap();
        }

        // A plan whose size differs from the stream's declared size is rejected too.
        let mut other = BitcompPlan::new(
            &ctx,
            n / 2,
            BitcompDataType::U8,
            BitcompMode::Lossless,
            BitcompAlgo::Default,
        )
        .unwrap();
        assert!(matches!(
            other.uncompress_async(&stream, &packed, &mut out),
            Err(Error::Invalid(_))
        ));

        // The context is still usable: the exact-length stream round-trips.
        let exact = DeviceBuffer::from_host(&stream, &packed_host[..packed_len]).unwrap();
        plan.uncompress_async(&stream, &exact, &mut out).unwrap();
        stream.synchronize().unwrap();
        assert_eq!(out.to_host_vec(&stream).unwrap(), host);
    }
}
