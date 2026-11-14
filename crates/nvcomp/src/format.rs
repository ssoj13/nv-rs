//! Explicit nvCOMP compression formats, per-codec options, and uniform C batched dispatch.
//!
//! A caller always selects a concrete [`Format`] (never implicitly downgraded to another codec).
//! Every batched C call (`GetRequiredAlignments`, `GetTempSize`, `GetMaxOutputChunkSize`,
//! `CompressAsync`, `GetDecompressSizeAsync`, `DecompressAsync`) routes through `Format` so the
//! staging and budget pipeline has one single implementation shared by all 6 formats.

use std::ffi::c_void;

use cuda_core::Stream;
use nvcomp_sys as sys;

use crate::{check, Error, Result};

/// Supported nvCOMP codec family without payload options.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FormatKind {
    /// LZ4 block compression.
    Lz4,
    /// Snappy compression.
    Snappy,
    /// Zstandard compression.
    Zstd,
    /// Cascaded integer/dictionary compression (delta + RLE + bitpack).
    Cascaded,
    /// Gdeflate compression.
    Gdeflate,
    /// Bitcomp compression.
    Bitcomp,
}

/// Logical element type (`nvcompType_t`) passed to typed compressors such as LZ4, Cascaded, and Bitcomp.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DataType {
    /// 8-bit signed integer (`NVCOMP_TYPE_CHAR`).
    Char,
    /// 8-bit unsigned integer (`NVCOMP_TYPE_UCHAR`).
    UChar,
    /// 16-bit signed integer (`NVCOMP_TYPE_SHORT`).
    Short,
    /// 16-bit unsigned integer (`NVCOMP_TYPE_USHORT`).
    UShort,
    /// 32-bit signed integer (`NVCOMP_TYPE_INT`).
    Int,
    /// 32-bit unsigned integer (`NVCOMP_TYPE_UINT`).
    UInt,
    /// 64-bit signed integer (`NVCOMP_TYPE_LONGLONG`).
    LongLong,
    /// 64-bit unsigned integer (`NVCOMP_TYPE_ULONGLONG`).
    ULongLong,
    /// 16-bit float (`NVCOMP_TYPE_FLOAT16`).
    Float16,
    /// 32-bit float (`NVCOMP_TYPE_FLOAT32`).
    Float32,
    /// 64-bit float (`NVCOMP_TYPE_FLOAT64`).
    Float64,
    /// Unspecified bit stream (`NVCOMP_TYPE_BITS`).
    Bits,
}

impl DataType {
    /// Converts to the underlying `nvcompType_t`.
    pub const fn to_sys(self) -> sys::nvcompType_t {
        match self {
            Self::Char => sys::nvcompType_t_NVCOMP_TYPE_CHAR,
            Self::UChar => sys::nvcompType_t_NVCOMP_TYPE_UCHAR,
            Self::Short => sys::nvcompType_t_NVCOMP_TYPE_SHORT,
            Self::UShort => sys::nvcompType_t_NVCOMP_TYPE_USHORT,
            Self::Int => sys::nvcompType_t_NVCOMP_TYPE_INT,
            Self::UInt => sys::nvcompType_t_NVCOMP_TYPE_UINT,
            Self::LongLong => sys::nvcompType_t_NVCOMP_TYPE_LONGLONG,
            Self::ULongLong => sys::nvcompType_t_NVCOMP_TYPE_ULONGLONG,
            Self::Float16 => sys::nvcompType_t_NVCOMP_TYPE_FLOAT16,
            Self::Float32 => sys::nvcompType_t_NVCOMP_TYPE_FLOAT32,
            Self::Float64 => sys::nvcompType_t_NVCOMP_TYPE_FLOAT64,
            Self::Bits => sys::nvcompType_t_NVCOMP_TYPE_BITS,
        }
    }
}

/// Bitshuffle preprocessing mode for LZ4 (`nvcompBitshuffleMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BitshuffleMode {
    /// No bitshuffle (`NVCOMP_BITSHUFFLE_NONE`).
    None,
    /// MSB-first bitshuffle (`NVCOMP_BITSHUFFLE_MSB_FIRST`).
    MsbFirst,
    /// LSB-first bitshuffle (`NVCOMP_BITSHUFFLE_LSB_FIRST`).
    LsbFirst,
}

impl BitshuffleMode {
    /// Converts to `nvcompBitshuffleMode_t`.
    pub const fn to_sys(self) -> sys::nvcompBitshuffleMode_t {
        match self {
            Self::None => sys::nvcompBitshuffleMode_t_NVCOMP_BITSHUFFLE_NONE,
            Self::MsbFirst => sys::nvcompBitshuffleMode_t_NVCOMP_BITSHUFFLE_MSB_FIRST,
            Self::LsbFirst => sys::nvcompBitshuffleMode_t_NVCOMP_BITSHUFFLE_LSB_FIRST,
        }
    }
}

/// Decompression execution backend (`nvcompDecompressBackend_t`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DecompressBackend {
    /// Library-chosen default backend (`NVCOMP_DECOMPRESS_BACKEND_DEFAULT`).
    #[default]
    Default,
    /// Dedicated hardware decompression engine (`NVCOMP_DECOMPRESS_BACKEND_HARDWARE`).
    Hardware,
    /// CUDA compute kernels (`NVCOMP_DECOMPRESS_BACKEND_CUDA`).
    Cuda,
}

impl DecompressBackend {
    /// Converts to `nvcompDecompressBackend_t`.
    pub const fn to_sys(self) -> sys::nvcompDecompressBackend_t {
        match self {
            Self::Default => sys::nvcompDecompressBackend_t_NVCOMP_DECOMPRESS_BACKEND_DEFAULT,
            Self::Hardware => sys::nvcompDecompressBackend_t_NVCOMP_DECOMPRESS_BACKEND_HARDWARE,
            Self::Cuda => sys::nvcompDecompressBackend_t_NVCOMP_DECOMPRESS_BACKEND_CUDA,
        }
    }
}

/// Cascaded symmetry mode (`nvcompCascadedMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CascadedMode {
    /// Unspecified (valid for universal decompression only).
    Unspecified,
    /// Asymmetric mode (faster decompression, broader integer support).
    Asymmetric,
    /// Symmetric mode.
    Symmetric,
}

impl CascadedMode {
    /// Converts to `nvcompCascadedMode_t`.
    pub const fn to_sys(self) -> sys::nvcompCascadedMode_t {
        match self {
            Self::Unspecified => sys::nvcompCascadedMode_t_NVCOMP_CASCADED_MODE_UNSPECIFIED,
            Self::Asymmetric => sys::nvcompCascadedMode_t_NVCOMP_CASCADED_MODE_ASYMMETRIC,
            Self::Symmetric => sys::nvcompCascadedMode_t_NVCOMP_CASCADED_MODE_SYMMETRIC,
        }
    }
}

/// Bitcomp compression mode (`bitcompMode_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BitcompMode {
    /// Bit-exact lossless mode (`BITCOMP_LOSSLESS`).
    Lossless,
    /// Lossy floating-point quantization to signed integers (`BITCOMP_LOSSY_FP_TO_SIGNED`).
    LossyFpToSigned,
    /// Lossy floating-point quantization to unsigned integers (`BITCOMP_LOSSY_FP_TO_UNSIGNED`).
    LossyFpToUnsigned,
}

impl BitcompMode {
    /// Converts to `bitcompMode_t`.
    pub const fn to_sys(self) -> sys::bitcompMode_t {
        match self {
            Self::Lossless => sys::bitcompMode_t_BITCOMP_LOSSLESS,
            Self::LossyFpToSigned => sys::bitcompMode_t_BITCOMP_LOSSY_FP_TO_SIGNED,
            Self::LossyFpToUnsigned => sys::bitcompMode_t_BITCOMP_LOSSY_FP_TO_UNSIGNED,
        }
    }
}

/// LZ4 compression configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lz4Opts {
    /// Element granularity used by bitshuffle preprocessing.
    pub data_type: DataType,
    /// Optional bitshuffle transform.
    pub bitshuffle: BitshuffleMode,
    /// Sort chunks before hardware decompression when supported.
    pub sort_before_hw_decompress: bool,
}

impl Default for Lz4Opts {
    fn default() -> Self {
        Self {
            data_type: DataType::Char,
            bitshuffle: BitshuffleMode::None,
            sort_before_hw_decompress: false,
        }
    }
}

impl Lz4Opts {
    fn to_compress_sys(self) -> sys::nvcompBatchedLZ4CompressOpts_t {
        sys::nvcompBatchedLZ4CompressOpts_t {
            data_type: self.data_type.to_sys(),
            bitshuffle_mode: self.bitshuffle.to_sys(),
            ..Default::default()
        }
    }

    fn to_decompress_sys(self, backend: DecompressBackend) -> sys::nvcompBatchedLZ4DecompressOpts_t {
        sys::nvcompBatchedLZ4DecompressOpts_t {
            backend: backend.to_sys(),
            sort_before_hw_decompress: i32::from(self.sort_before_hw_decompress),
            data_type: self.data_type.to_sys(),
            bitshuffle_mode: self.bitshuffle.to_sys(),
            ..Default::default()
        }
    }
}

/// Snappy compression configuration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SnappyOpts {
    /// Sort chunks before hardware decompression when supported.
    pub sort_before_hw_decompress: bool,
}

impl SnappyOpts {
    fn to_compress_sys(self) -> sys::nvcompBatchedSnappyCompressOpts_t {
        sys::nvcompBatchedSnappyCompressOpts_t::default()
    }

    fn to_decompress_sys(
        self,
        backend: DecompressBackend,
    ) -> sys::nvcompBatchedSnappyDecompressOpts_t {
        sys::nvcompBatchedSnappyDecompressOpts_t {
            backend: backend.to_sys(),
            sort_before_hw_decompress: i32::from(self.sort_before_hw_decompress),
            ..Default::default()
        }
    }
}

/// Zstandard compression configuration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ZstdOpts;

impl ZstdOpts {
    fn to_compress_sys(self) -> sys::nvcompBatchedZstdCompressOpts_t {
        sys::nvcompBatchedZstdCompressOpts_t::default()
    }

    fn to_decompress_sys(self, backend: DecompressBackend) -> sys::nvcompBatchedZstdDecompressOpts_t {
        sys::nvcompBatchedZstdDecompressOpts_t { backend: backend.to_sys(), ..Default::default() }
    }
}

/// Cascaded compression configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CascadedOpts {
    /// Logical integer element type.
    pub data_type: DataType,
    /// Asymmetric vs symmetric coding mode.
    pub mode: CascadedMode,
    /// Compression effort level (`0..=7`, default `3`).
    pub compression_level: u8,
}

impl Default for CascadedOpts {
    fn default() -> Self {
        Self {
            data_type: DataType::UInt,
            mode: CascadedMode::Asymmetric,
            compression_level: 3,
        }
    }
}

impl CascadedOpts {
    fn to_compress_sys(self) -> sys::nvcompBatchedCascadedCompressOpts_t {
        sys::nvcompBatchedCascadedCompressOpts_t {
            common_opts: sys::nvcompCascadedCommonOpts_t {
                data_type: self.data_type.to_sys(),
                mode: self.mode.to_sys(),
                terminal_codec_flags:
                    sys::nvcompCascadedTerminalCodec_t_NVCOMP_CASCADED_TERMINAL_CODEC_CASCADED_BITPACK
                        as u64,
                coarse_grained_encoding_flags:
                    sys::nvcompCascadedCoarseGrainedEncoding_t_NVCOMP_CASCADED_COARSE_GRAINED_ENCODING_NONE
                        as u64,
            },
            fine_grained_encoding_flags:
                sys::nvcompCascadedFineGrainedEncoding_t_NVCOMP_CASCADED_FINE_GRAINED_ENCODING_ALL
                    as u64,
            compression_level: self.compression_level,
            ..Default::default()
        }
    }

    fn to_decompress_sys(
        self,
        backend: DecompressBackend,
    ) -> sys::nvcompBatchedCascadedDecompressOpts_t {
        sys::nvcompBatchedCascadedDecompressOpts_t {
            backend: backend.to_sys(),
            common_opts: sys::nvcompCascadedCommonOpts_t {
                data_type: DataType::Bits.to_sys(),
                mode: CascadedMode::Unspecified.to_sys(),
                terminal_codec_flags:
                    sys::nvcompCascadedTerminalCodec_t_NVCOMP_CASCADED_TERMINAL_CODEC_UNSPECIFIED
                        as u64,
                coarse_grained_encoding_flags:
                    sys::nvcompCascadedCoarseGrainedEncoding_t_NVCOMP_CASCADED_COARSE_GRAINED_ENCODING_NONE
                        as u64,
            },
            ..Default::default()
        }
    }
}

/// Gdeflate compression configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GdeflateOpts {
    /// Gdeflate compression algorithm level (`0..=5`, default `1`).
    pub algorithm: i32,
}

impl Default for GdeflateOpts {
    fn default() -> Self {
        Self { algorithm: 1 }
    }
}

impl GdeflateOpts {
    fn to_compress_sys(self) -> sys::nvcompBatchedGdeflateCompressOpts_t {
        sys::nvcompBatchedGdeflateCompressOpts_t { algorithm: self.algorithm, ..Default::default() }
    }

    fn to_decompress_sys(
        self,
        backend: DecompressBackend,
    ) -> sys::nvcompBatchedGdeflateDecompressOpts_t {
        sys::nvcompBatchedGdeflateDecompressOpts_t {
            backend: backend.to_sys(),
            ..Default::default()
        }
    }
}

/// Bitcomp compression configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitcompOpts {
    /// Native Bitcomp algorithm (`0` = default, `1` = sparse).
    pub algorithm: i32,
    /// Element data type (`UChar` default for byte streams).
    pub data_type: DataType,
    /// Lossy quantization step when `mode != BitcompMode::Lossless`.
    pub delta: f64,
    /// Lossless vs floating-point lossy mode.
    pub mode: BitcompMode,
}

impl Default for BitcompOpts {
    fn default() -> Self {
        Self {
            algorithm: 0,
            data_type: DataType::UChar,
            delta: 0.0,
            mode: BitcompMode::Lossless,
        }
    }
}

impl BitcompOpts {
    fn to_compress_sys(self) -> sys::nvcompBatchedBitcompCompressOpts_t {
        sys::nvcompBatchedBitcompCompressOpts_t {
            algorithm: self.algorithm,
            data_type: self.data_type.to_sys(),
            delta: self.delta,
            mode: self.mode.to_sys(),
            ..Default::default()
        }
    }

    fn to_decompress_sys(
        self,
        backend: DecompressBackend,
    ) -> sys::nvcompBatchedBitcompDecompressOpts_t {
        sys::nvcompBatchedBitcompDecompressOpts_t {
            backend: backend.to_sys(),
            ..Default::default()
        }
    }
}

/// Explicit compression format and codec options. Selection is strictly honored with zero silent
/// codec downgrades.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    /// LZ4 batched codec.
    Lz4(Lz4Opts),
    /// Snappy batched codec.
    Snappy(SnappyOpts),
    /// Zstandard batched codec.
    Zstd(ZstdOpts),
    /// Cascaded batched codec.
    Cascaded(CascadedOpts),
    /// Gdeflate batched codec.
    Gdeflate(GdeflateOpts),
    /// Bitcomp batched codec.
    Bitcomp(BitcompOpts),
}

/// Minimum GPU buffer alignment requirements reported by nvCOMP.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Alignments {
    /// Minimum byte alignment for each input chunk.
    pub input: usize,
    /// Minimum byte alignment for each output chunk.
    pub output: usize,
    /// Minimum byte alignment for the temporary workspace buffer.
    pub temp: usize,
}

impl From<sys::nvcompAlignmentRequirements_t> for Alignments {
    fn from(raw: sys::nvcompAlignmentRequirements_t) -> Self {
        Self { input: raw.input.max(1), output: raw.output.max(1), temp: raw.temp.max(1) }
    }
}

impl Format {
    /// Returns the default configuration for `kind`.
    pub fn default_for(kind: FormatKind) -> Self {
        match kind {
            FormatKind::Lz4 => Self::Lz4(Lz4Opts::default()),
            FormatKind::Snappy => Self::Snappy(SnappyOpts::default()),
            FormatKind::Zstd => Self::Zstd(ZstdOpts),
            FormatKind::Cascaded => Self::Cascaded(CascadedOpts::default()),
            FormatKind::Gdeflate => Self::Gdeflate(GdeflateOpts::default()),
            FormatKind::Bitcomp => Self::Bitcomp(BitcompOpts::default()),
        }
    }

    /// Returns the codec family tag of `self`.
    pub const fn kind(&self) -> FormatKind {
        match self {
            Self::Lz4(_) => FormatKind::Lz4,
            Self::Snappy(_) => FormatKind::Snappy,
            Self::Zstd(_) => FormatKind::Zstd,
            Self::Cascaded(_) => FormatKind::Cascaded,
            Self::Gdeflate(_) => FormatKind::Gdeflate,
            Self::Bitcomp(_) => FormatKind::Bitcomp,
        }
    }

    /// Validates user-supplied format options before invoking FFI.
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Gdeflate(opts) if !(0..=5).contains(&opts.algorithm) => {
                Err(Error::Invalid("gdeflate algorithm must be in 0..=5"))
            }
            Self::Bitcomp(opts)
                if opts.mode != BitcompMode::Lossless
                    && (!opts.delta.is_finite() || opts.delta <= 0.0) =>
            {
                Err(Error::Invalid("lossy bitcomp delta must be finite and > 0"))
            }
            _ => Ok(()),
        }
    }

    /// Queries compression input/output/temp alignment requirements.
    pub fn compress_alignments(&self) -> Result<Alignments> {
        self.validate()?;
        sys::load()?;
        let mut req = sys::nvcompAlignmentRequirements_t::default();
        // SAFETY: `req` is a valid out-pointer to C POD struct.
        let (call, status) = unsafe {
            match self {
                Self::Lz4(o) => (
                    "nvcompBatchedLZ4CompressGetRequiredAlignments",
                    sys::nvcompBatchedLZ4CompressGetRequiredAlignments(o.to_compress_sys(), &mut req),
                ),
                Self::Snappy(o) => (
                    "nvcompBatchedSnappyCompressGetRequiredAlignments",
                    sys::nvcompBatchedSnappyCompressGetRequiredAlignments(
                        o.to_compress_sys(),
                        &mut req,
                    ),
                ),
                Self::Zstd(o) => (
                    "nvcompBatchedZstdCompressGetRequiredAlignments",
                    sys::nvcompBatchedZstdCompressGetRequiredAlignments(
                        o.to_compress_sys(),
                        &mut req,
                    ),
                ),
                Self::Cascaded(o) => (
                    "nvcompBatchedCascadedCompressGetRequiredAlignments",
                    sys::nvcompBatchedCascadedCompressGetRequiredAlignments(
                        o.to_compress_sys(),
                        &mut req,
                    ),
                ),
                Self::Gdeflate(o) => (
                    "nvcompBatchedGdeflateCompressGetRequiredAlignments",
                    sys::nvcompBatchedGdeflateCompressGetRequiredAlignments(
                        o.to_compress_sys(),
                        &mut req,
                    ),
                ),
                Self::Bitcomp(o) => (
                    "nvcompBatchedBitcompCompressGetRequiredAlignments",
                    sys::nvcompBatchedBitcompCompressGetRequiredAlignments(
                        o.to_compress_sys(),
                        &mut req,
                    ),
                ),
            }
        };
        check(call, status)?;
        Ok(Alignments::from(req))
    }

    /// Queries decompression input/output/temp alignment requirements for `backend`.
    pub fn decompress_alignments(&self, backend: DecompressBackend) -> Result<Alignments> {
        self.validate()?;
        sys::load()?;
        let mut req = sys::nvcompAlignmentRequirements_t::default();
        // SAFETY: `req` is a valid out-pointer to C POD struct.
        let (call, status) = unsafe {
            match self {
                Self::Lz4(o) => (
                    "nvcompBatchedLZ4DecompressGetRequiredAlignments",
                    sys::nvcompBatchedLZ4DecompressGetRequiredAlignments(
                        o.to_decompress_sys(backend),
                        &mut req,
                    ),
                ),
                Self::Snappy(o) => (
                    "nvcompBatchedSnappyDecompressGetRequiredAlignments",
                    sys::nvcompBatchedSnappyDecompressGetRequiredAlignments(
                        o.to_decompress_sys(backend),
                        &mut req,
                    ),
                ),
                Self::Zstd(o) => (
                    "nvcompBatchedZstdDecompressGetRequiredAlignments",
                    sys::nvcompBatchedZstdDecompressGetRequiredAlignments(
                        o.to_decompress_sys(backend),
                        &mut req,
                    ),
                ),
                Self::Cascaded(o) => (
                    "nvcompBatchedCascadedDecompressGetRequiredAlignments",
                    sys::nvcompBatchedCascadedDecompressGetRequiredAlignments(
                        o.to_decompress_sys(backend),
                        &mut req,
                    ),
                ),
                Self::Gdeflate(o) => (
                    "nvcompBatchedGdeflateDecompressGetRequiredAlignments",
                    sys::nvcompBatchedGdeflateDecompressGetRequiredAlignments(
                        o.to_decompress_sys(backend),
                        &mut req,
                    ),
                ),
                Self::Bitcomp(o) => (
                    "nvcompBatchedBitcompDecompressGetRequiredAlignments",
                    sys::nvcompBatchedBitcompDecompressGetRequiredAlignments(
                        o.to_decompress_sys(backend),
                        &mut req,
                    ),
                ),
            }
        };
        check(call, status)?;
        Ok(Alignments::from(req))
    }

    /// Queries the worst-case compressed output size in bytes for one chunk of at most
    /// `max_uncompressed_chunk_bytes`.
    pub fn max_output_chunk_bytes(&self, max_uncompressed_chunk_bytes: usize) -> Result<usize> {
        if max_uncompressed_chunk_bytes == 0 {
            return Err(Error::Invalid("chunk size must be non-zero"));
        }
        self.validate()?;
        sys::load()?;
        let mut max_out = 0usize;
        // SAFETY: `max_out` is a valid stack out-pointer.
        let (call, status) = unsafe {
            match self {
                Self::Lz4(o) => (
                    "nvcompBatchedLZ4CompressGetMaxOutputChunkSize",
                    sys::nvcompBatchedLZ4CompressGetMaxOutputChunkSize(
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut max_out,
                    ),
                ),
                Self::Snappy(o) => (
                    "nvcompBatchedSnappyCompressGetMaxOutputChunkSize",
                    sys::nvcompBatchedSnappyCompressGetMaxOutputChunkSize(
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut max_out,
                    ),
                ),
                Self::Zstd(o) => (
                    "nvcompBatchedZstdCompressGetMaxOutputChunkSize",
                    sys::nvcompBatchedZstdCompressGetMaxOutputChunkSize(
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut max_out,
                    ),
                ),
                Self::Cascaded(o) => (
                    "nvcompBatchedCascadedCompressGetMaxOutputChunkSize",
                    sys::nvcompBatchedCascadedCompressGetMaxOutputChunkSize(
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut max_out,
                    ),
                ),
                Self::Gdeflate(o) => (
                    "nvcompBatchedGdeflateCompressGetMaxOutputChunkSize",
                    sys::nvcompBatchedGdeflateCompressGetMaxOutputChunkSize(
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut max_out,
                    ),
                ),
                Self::Bitcomp(o) => (
                    "nvcompBatchedBitcompCompressGetMaxOutputChunkSize",
                    sys::nvcompBatchedBitcompCompressGetMaxOutputChunkSize(
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut max_out,
                    ),
                ),
            }
        };
        check(call, status)?;
        Ok(max_out)
    }

    /// Queries the temporary GPU workspace size in bytes required to compress a batch.
    pub fn compress_temp_bytes(
        &self,
        num_chunks: usize,
        max_uncompressed_chunk_bytes: usize,
        max_total_uncompressed_bytes: usize,
        stream: &Stream,
    ) -> Result<usize> {
        self.validate()?;
        sys::load()?;
        stream.context().bind_to_thread()?;
        let mut temp_bytes = 0usize;
        let cu_stream = stream.cu_stream();
        // SAFETY: context is bound and `temp_bytes` is a valid out-pointer.
        let (call, status) = unsafe {
            match self {
                Self::Lz4(o) => (
                    "nvcompBatchedLZ4CompressGetTempSize",
                    sys::nvcompBatchedLZ4CompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Snappy(o) => (
                    "nvcompBatchedSnappyCompressGetTempSize",
                    sys::nvcompBatchedSnappyCompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Zstd(o) => (
                    "nvcompBatchedZstdCompressGetTempSize",
                    sys::nvcompBatchedZstdCompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Cascaded(o) => (
                    "nvcompBatchedCascadedCompressGetTempSize",
                    sys::nvcompBatchedCascadedCompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Gdeflate(o) => (
                    "nvcompBatchedGdeflateCompressGetTempSize",
                    sys::nvcompBatchedGdeflateCompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Bitcomp(o) => (
                    "nvcompBatchedBitcompCompressGetTempSize",
                    sys::nvcompBatchedBitcompCompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_compress_sys(),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
            }
        };
        check(call, status)?;
        Ok(temp_bytes)
    }

    /// Queries the temporary GPU workspace size in bytes required to decompress a batch.
    pub fn decompress_temp_bytes(
        &self,
        num_chunks: usize,
        max_uncompressed_chunk_bytes: usize,
        max_total_uncompressed_bytes: usize,
        backend: DecompressBackend,
        stream: &Stream,
    ) -> Result<usize> {
        self.validate()?;
        sys::load()?;
        stream.context().bind_to_thread()?;
        let mut temp_bytes = 0usize;
        let cu_stream = stream.cu_stream();
        // SAFETY: context is bound and `temp_bytes` is a valid out-pointer.
        let (call, status) = unsafe {
            match self {
                Self::Lz4(o) => (
                    "nvcompBatchedLZ4DecompressGetTempSize",
                    sys::nvcompBatchedLZ4DecompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_decompress_sys(backend),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Snappy(o) => (
                    "nvcompBatchedSnappyDecompressGetTempSize",
                    sys::nvcompBatchedSnappyDecompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_decompress_sys(backend),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Zstd(o) => (
                    "nvcompBatchedZstdDecompressGetTempSize",
                    sys::nvcompBatchedZstdDecompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_decompress_sys(backend),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Cascaded(o) => (
                    "nvcompBatchedCascadedDecompressGetTempSize",
                    sys::nvcompBatchedCascadedDecompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_decompress_sys(backend),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Gdeflate(o) => (
                    "nvcompBatchedGdeflateDecompressGetTempSize",
                    sys::nvcompBatchedGdeflateDecompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_decompress_sys(backend),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
                Self::Bitcomp(o) => (
                    "nvcompBatchedBitcompDecompressGetTempSize",
                    sys::nvcompBatchedBitcompDecompressGetTempSize(
                        num_chunks,
                        max_uncompressed_chunk_bytes,
                        o.to_decompress_sys(backend),
                        &mut temp_bytes,
                        max_total_uncompressed_bytes,
                        cu_stream,
                    ),
                ),
            }
        };
        check(call, status)?;
        Ok(temp_bytes)
    }

    /// Launches batched asynchronous compression on `stream`.
    ///
    /// # Safety
    ///
    /// Every device pointer array and output/temp buffer must remain valid until `stream` reaches
    /// this launch, hold at least `num_chunks` entries, and satisfy the alignment requirements of
    /// `self`.
    #[expect(
        clippy::too_many_arguments,
        reason = "Direct dispatch wrapper matching nvcompBatched*CompressAsync C ABI"
    )]
    pub(crate) unsafe fn launch_compress_async(
        &self,
        dev_in_ptrs: *const *const c_void,
        dev_in_bytes: *const usize,
        max_uncompressed_chunk_bytes: usize,
        num_chunks: usize,
        dev_temp_ptr: *mut c_void,
        temp_bytes: usize,
        dev_out_ptrs: *const *mut c_void,
        dev_out_bytes: *mut usize,
        dev_statuses: *mut sys::nvcompStatus_t,
        stream: sys::cudaStream_t,
    ) -> Result<()> {
        // SAFETY: caller upholds pointer array, buffer extent, alignment, and stream lifetime.
        let (call, status) = unsafe {
            match self {
                Self::Lz4(o) => (
                    "nvcompBatchedLZ4CompressAsync",
                    sys::nvcompBatchedLZ4CompressAsync(
                        dev_in_ptrs,
                        dev_in_bytes,
                        max_uncompressed_chunk_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_out_ptrs,
                        dev_out_bytes,
                        o.to_compress_sys(),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Snappy(o) => (
                    "nvcompBatchedSnappyCompressAsync",
                    sys::nvcompBatchedSnappyCompressAsync(
                        dev_in_ptrs,
                        dev_in_bytes,
                        max_uncompressed_chunk_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_out_ptrs,
                        dev_out_bytes,
                        o.to_compress_sys(),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Zstd(o) => (
                    "nvcompBatchedZstdCompressAsync",
                    sys::nvcompBatchedZstdCompressAsync(
                        dev_in_ptrs,
                        dev_in_bytes,
                        max_uncompressed_chunk_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_out_ptrs,
                        dev_out_bytes,
                        o.to_compress_sys(),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Cascaded(o) => (
                    "nvcompBatchedCascadedCompressAsync",
                    sys::nvcompBatchedCascadedCompressAsync(
                        dev_in_ptrs,
                        dev_in_bytes,
                        max_uncompressed_chunk_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_out_ptrs,
                        dev_out_bytes,
                        o.to_compress_sys(),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Gdeflate(o) => (
                    "nvcompBatchedGdeflateCompressAsync",
                    sys::nvcompBatchedGdeflateCompressAsync(
                        dev_in_ptrs,
                        dev_in_bytes,
                        max_uncompressed_chunk_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_out_ptrs,
                        dev_out_bytes,
                        o.to_compress_sys(),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Bitcomp(o) => (
                    "nvcompBatchedBitcompCompressAsync",
                    sys::nvcompBatchedBitcompCompressAsync(
                        dev_in_ptrs,
                        dev_in_bytes,
                        max_uncompressed_chunk_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_out_ptrs,
                        dev_out_bytes,
                        o.to_compress_sys(),
                        dev_statuses,
                        stream,
                    ),
                ),
            }
        };
        check(call, status)
    }

    /// Launches `nvcompBatched*GetDecompressSizeAsync` on `stream`.
    ///
    /// # Safety
    ///
    /// All device pointer/size arrays must be valid on `stream` for at least `num_chunks` items.
    pub(crate) unsafe fn launch_get_decompress_size_async(
        &self,
        dev_comp_ptrs: *const *const c_void,
        dev_comp_bytes: *const usize,
        dev_uncomp_bytes: *mut usize,
        num_chunks: usize,
        stream: sys::cudaStream_t,
    ) -> Result<()> {
        // SAFETY: caller upholds device pointer and size array invariants.
        let (call, status) = unsafe {
            match self {
                Self::Lz4(_) => (
                    "nvcompBatchedLZ4GetDecompressSizeAsync",
                    sys::nvcompBatchedLZ4GetDecompressSizeAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        stream,
                    ),
                ),
                Self::Snappy(_) => (
                    "nvcompBatchedSnappyGetDecompressSizeAsync",
                    sys::nvcompBatchedSnappyGetDecompressSizeAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        stream,
                    ),
                ),
                Self::Zstd(_) => (
                    "nvcompBatchedZstdGetDecompressSizeAsync",
                    sys::nvcompBatchedZstdGetDecompressSizeAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        stream,
                    ),
                ),
                Self::Cascaded(_) => (
                    "nvcompBatchedCascadedGetDecompressSizeAsync",
                    sys::nvcompBatchedCascadedGetDecompressSizeAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        stream,
                    ),
                ),
                Self::Gdeflate(_) => (
                    "nvcompBatchedGdeflateGetDecompressSizeAsync",
                    sys::nvcompBatchedGdeflateGetDecompressSizeAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        stream,
                    ),
                ),
                Self::Bitcomp(_) => (
                    "nvcompBatchedBitcompGetDecompressSizeAsync",
                    sys::nvcompBatchedBitcompGetDecompressSizeAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        stream,
                    ),
                ),
            }
        };
        check(call, status)
    }

    /// Launches `nvcompBatched*DecompressAsync` on `stream`.
    ///
    /// # Safety
    ///
    /// Every device pointer/size/status array and temporary buffer must satisfy alignment,
    /// allocation extent, and stream lifetime invariants for `num_chunks`.
    #[expect(
        clippy::too_many_arguments,
        reason = "Direct dispatch wrapper matching nvcompBatched*DecompressAsync C ABI"
    )]
    pub(crate) unsafe fn launch_decompress_async(
        &self,
        dev_comp_ptrs: *const *const c_void,
        dev_comp_bytes: *const usize,
        dev_uncomp_buf_bytes: *const usize,
        dev_uncomp_bytes: *mut usize,
        num_chunks: usize,
        dev_temp_ptr: *mut c_void,
        temp_bytes: usize,
        dev_uncomp_ptrs: *const *mut c_void,
        backend: DecompressBackend,
        dev_statuses: *mut sys::nvcompStatus_t,
        stream: sys::cudaStream_t,
    ) -> Result<()> {
        // SAFETY: caller upholds pointer array, extent, alignment, and stream lifetime.
        let (call, status) = unsafe {
            match self {
                Self::Lz4(o) => (
                    "nvcompBatchedLZ4DecompressAsync",
                    sys::nvcompBatchedLZ4DecompressAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_buf_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_uncomp_ptrs,
                        o.to_decompress_sys(backend),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Snappy(o) => (
                    "nvcompBatchedSnappyDecompressAsync",
                    sys::nvcompBatchedSnappyDecompressAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_buf_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_uncomp_ptrs,
                        o.to_decompress_sys(backend),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Zstd(o) => (
                    "nvcompBatchedZstdDecompressAsync",
                    sys::nvcompBatchedZstdDecompressAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_buf_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_uncomp_ptrs,
                        o.to_decompress_sys(backend),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Cascaded(o) => (
                    "nvcompBatchedCascadedDecompressAsync",
                    sys::nvcompBatchedCascadedDecompressAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_buf_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_uncomp_ptrs,
                        o.to_decompress_sys(backend),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Gdeflate(o) => (
                    "nvcompBatchedGdeflateDecompressAsync",
                    sys::nvcompBatchedGdeflateDecompressAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_buf_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_uncomp_ptrs,
                        o.to_decompress_sys(backend),
                        dev_statuses,
                        stream,
                    ),
                ),
                Self::Bitcomp(o) => (
                    "nvcompBatchedBitcompDecompressAsync",
                    sys::nvcompBatchedBitcompDecompressAsync(
                        dev_comp_ptrs,
                        dev_comp_bytes,
                        dev_uncomp_buf_bytes,
                        dev_uncomp_bytes,
                        num_chunks,
                        dev_temp_ptr,
                        temp_bytes,
                        dev_uncomp_ptrs,
                        o.to_decompress_sys(backend),
                        dev_statuses,
                        stream,
                    ),
                ),
            }
        };
        check(call, status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_format_defaults_and_validation() {
        for kind in [
            FormatKind::Lz4,
            FormatKind::Snappy,
            FormatKind::Zstd,
            FormatKind::Cascaded,
            FormatKind::Gdeflate,
            FormatKind::Bitcomp,
        ] {
            let f = Format::default_for(kind);
            assert_eq!(f.kind(), kind);
            assert!(f.validate().is_ok());
        }

        let bad_gdeflate = Format::Gdeflate(GdeflateOpts { algorithm: 9 });
        assert!(bad_gdeflate.validate().is_err());

        let bad_bitcomp = Format::Bitcomp(BitcompOpts {
            mode: BitcompMode::LossyFpToSigned,
            delta: 0.0,
            ..Default::default()
        });
        assert!(bad_bitcomp.validate().is_err());

        let cascaded = CascadedOpts::default().to_compress_sys();
        assert_eq!(cascaded.compression_level, 3);
        assert_eq!(cascaded.common_opts.data_type, sys::nvcompType_t_NVCOMP_TYPE_UINT);
    }
}
