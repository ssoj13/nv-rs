//! Stateful chunked compression/decompression manager built over nvCOMP's batched C APIs.
//!
//! Because nvCOMP 6 C headers expose only the batched C APIs (`nvcompBatched*`) while the HLIF
//! manager is C++-only, [`CompressionManager`] provides single-buffer and preallocated
//! destination manager semantics cleanly over [`compress_batched_async`] and
//! [`decompress_batched_async`], reusing internal staging tables across calls so repeated
//! compress/decompress cycles perform zero steady-state allocations.

use std::ops::Range;
use std::sync::Arc;

use cuda_core::{Context, DeviceBuffer, Stream};

use crate::batched::{aligned_stride, compress_batched_async, decompress_batched_async};
use crate::format::{DecompressBackend, Format};
use crate::scratch::BatchStaging;
use crate::{Error, Result, same_stream, within};

/// Chunk layout metadata describing a compressed payload stored in a caller-owned buffer.
#[derive(Clone, Debug)]
pub struct CompressedLayout {
    /// Codec format that produced the compressed chunks.
    pub format: Format,
    /// Original total uncompressed size in bytes.
    pub uncompressed_bytes: usize,
    /// Stride in bytes between consecutive compressed chunk slots.
    pub slot_stride: usize,
    /// Uncompressed chunk ranges in the original buffer.
    pub uncompressed_ranges: Vec<Range<usize>>,
    /// Compressed chunk ranges inside the compressed buffer.
    pub compressed_ranges: Vec<Range<usize>>,
}

/// Owned GPU buffer holding chunked nvCOMP-compressed data and its exact chunk layout.
pub struct CompressedBuffer {
    /// Compressed chunk bytes stored in device memory.
    pub data: DeviceBuffer<u8>,
    /// Exact chunk offsets and format metadata.
    pub layout: CompressedLayout,
}

/// Reusable manager orchestrating two-phase sizing, byte-budget checks, and batched GPU
/// compression/decompression with an explicit [`Format`].
pub struct CompressionManager {
    ctx: Arc<Context>,
    format: Format,
    chunk_size: usize,
    backend: DecompressBackend,
    budget: Option<usize>,
    staging: BatchStaging,
    in_ranges_scratch: Vec<Range<usize>>,
    out_ranges_scratch: Vec<Range<usize>>,
    sizes_scratch: Vec<usize>,
}

impl CompressionManager {
    /// Creates a new manager bound to `stream` for `format`, splitting inputs into chunks of up
    /// to `chunk_size` bytes under an optional device byte `budget`.
    pub fn new(
        stream: &Arc<Stream>,
        format: Format,
        chunk_size: usize,
        budget: Option<usize>,
    ) -> Result<Self> {
        format.validate()?;
        if chunk_size == 0 {
            return Err(Error::Invalid("chunk_size must be non-zero"));
        }
        Ok(Self {
            ctx: stream.context().clone(),
            format,
            chunk_size,
            backend: DecompressBackend::Default,
            budget,
            staging: BatchStaging::new(stream)?,
            in_ranges_scratch: Vec::new(),
            out_ranges_scratch: Vec::new(),
            sizes_scratch: Vec::new(),
        })
    }

    /// Overrides the decompression backend (default [`DecompressBackend::Default`]).
    pub fn set_decompress_backend(&mut self, backend: DecompressBackend) {
        self.backend = backend;
    }

    /// Configured explicit format.
    pub const fn format(&self) -> Format {
        self.format
    }

    /// Configured chunk size in bytes.
    pub const fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Owning CUDA context.
    pub fn context(&self) -> &Arc<Context> {
        &self.ctx
    }

    fn plan_chunks(
        total_bytes: usize,
        chunk_size: usize,
        input_align: usize,
        out: &mut Vec<Range<usize>>,
    ) -> Result<()> {
        if total_bytes == 0 {
            return Err(Error::Invalid("cannot compress an empty buffer"));
        }
        if !chunk_size.is_multiple_of(input_align.max(1)) && total_bytes > chunk_size {
            return Err(Error::Invalid(
                "chunk_size must be a multiple of nvCOMP input alignment",
            ));
        }
        out.clear();
        let mut offset = 0usize;
        while offset < total_bytes {
            let end = offset.saturating_add(chunk_size).min(total_bytes);
            out.push(offset..end);
            offset = end;
        }
        Ok(())
    }

    /// Computes worst-case compressed output buffer capacity in bytes for an input of `total_bytes`.
    pub fn max_compressed_bytes(&self, total_bytes: usize) -> Result<usize> {
        if total_bytes == 0 {
            return Err(Error::Invalid("input byte count must be non-zero"));
        }
        let alignments = self.format.compress_alignments()?;
        let max_chunk = self.chunk_size.min(total_bytes);
        let max_out_chunk = self.format.max_output_chunk_bytes(max_chunk)?;
        let slot_stride = aligned_stride(max_out_chunk, alignments.output)?;
        let chunks = total_bytes.div_ceil(self.chunk_size);
        chunks
            .checked_mul(slot_stride)
            .ok_or(Error::Invalid("compressed output capacity overflow"))
    }

    /// Compresses `input` asynchronously into a newly allocated [`CompressedBuffer`], enforcing
    /// `budget` against combined output + staging + temp workspace growth.
    pub fn compress_async(
        &mut self,
        stream: &Arc<Stream>,
        input: &DeviceBuffer<u8>,
    ) -> Result<CompressedBuffer> {
        same_stream(stream, self.staging.stream())?;
        let total = input.num_bytes();
        let alignments = self.format.compress_alignments()?;
        Self::plan_chunks(
            total,
            self.chunk_size,
            alignments.input,
            &mut self.in_ranges_scratch,
        )?;

        let max_chunk = self.chunk_size.min(total);
        let max_out_chunk = self.format.max_output_chunk_bytes(max_chunk)?;
        let slot_stride = aligned_stride(max_out_chunk, alignments.output)?;
        let num_chunks = self.in_ranges_scratch.len();
        let out_bytes = num_chunks
            .checked_mul(slot_stride)
            .ok_or(Error::Invalid("compressed output size overflow"))?;
        let temp_bytes = self
            .format
            .compress_temp_bytes(num_chunks, max_chunk, total, stream)?;
        let staging_grow = self.staging.growth(num_chunks, temp_bytes);
        within(self.budget, out_bytes.saturating_add(staging_grow))?;

        // SAFETY: `output` slots are populated by `compress_batched_async` before any read.
        let mut output = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, out_bytes) }?;
        let remaining_budget = self.budget.map(|b| b.saturating_sub(out_bytes));
        let layout = self.compress_into_inner(stream, input, &mut output, remaining_budget)?;
        Ok(CompressedBuffer {
            data: output,
            layout,
        })
    }

    /// Compresses `input` into a caller-owned `output` buffer (zero device allocations once
    /// `self` and `output` are sized).
    pub fn compress_into_async(
        &mut self,
        stream: &Arc<Stream>,
        input: &DeviceBuffer<u8>,
        output: &mut DeviceBuffer<u8>,
    ) -> Result<CompressedLayout> {
        same_stream(stream, self.staging.stream())?;
        let alignments = self.format.compress_alignments()?;
        Self::plan_chunks(
            input.num_bytes(),
            self.chunk_size,
            alignments.input,
            &mut self.in_ranges_scratch,
        )?;
        self.compress_into_inner(stream, input, output, self.budget)
    }

    fn compress_into_inner(
        &mut self,
        stream: &Arc<Stream>,
        input: &DeviceBuffer<u8>,
        output: &mut DeviceBuffer<u8>,
        budget: Option<usize>,
    ) -> Result<CompressedLayout> {
        let receipt = compress_batched_async(
            stream,
            &mut self.staging,
            self.format,
            input,
            &self.in_ranges_scratch,
            output,
            budget,
        )?;
        receipt.collect_sizes(stream, &mut self.staging, &mut self.sizes_scratch)?;
        self.out_ranges_scratch.clear();
        for (i, &sz) in self.sizes_scratch.iter().enumerate() {
            let start = i * receipt.slot_stride;
            self.out_ranges_scratch.push(start..(start + sz));
        }
        Ok(CompressedLayout {
            format: self.format,
            uncompressed_bytes: input.num_bytes(),
            slot_stride: receipt.slot_stride,
            uncompressed_ranges: self.in_ranges_scratch.clone(),
            compressed_ranges: self.out_ranges_scratch.clone(),
        })
    }

    /// Decompresses `compressed` into a caller-provided `output` buffer (zero output allocation),
    /// returning the number of uncompressed bytes written.
    pub fn decompress_into_async(
        &mut self,
        stream: &Arc<Stream>,
        compressed: &CompressedBuffer,
        output: &mut DeviceBuffer<u8>,
    ) -> Result<usize> {
        self.decompress_into_with_budget_async(stream, compressed, output, self.budget)
    }

    fn decompress_into_with_budget_async(
        &mut self,
        stream: &Arc<Stream>,
        compressed: &CompressedBuffer,
        output: &mut DeviceBuffer<u8>,
        budget: Option<usize>,
    ) -> Result<usize> {
        same_stream(stream, self.staging.stream())?;
        if compressed.layout.format.kind() != self.format.kind() {
            return Err(Error::Invalid(
                "compressed payload format does not match manager format",
            ));
        }
        if output.num_bytes() < compressed.layout.uncompressed_bytes {
            return Err(Error::Invalid(
                "decompression output buffer smaller than uncompressed_bytes",
            ));
        }
        let receipt = decompress_batched_async(
            stream,
            &mut self.staging,
            self.format,
            self.backend,
            &compressed.data,
            &compressed.layout.compressed_ranges,
            output,
            &compressed.layout.uncompressed_ranges,
            budget,
        )?;
        receipt.collect_sizes(stream, &mut self.staging, &mut self.sizes_scratch)?;
        Ok(self.sizes_scratch.iter().copied().sum())
    }

    /// Allocates an exact-sized output buffer (respecting `budget`) and decompresses `compressed`.
    pub fn decompress_to_buffer_async(
        &mut self,
        stream: &Arc<Stream>,
        compressed: &CompressedBuffer,
    ) -> Result<DeviceBuffer<u8>> {
        same_stream(stream, self.staging.stream())?;
        if compressed.layout.format.kind() != self.format.kind() {
            return Err(Error::Invalid(
                "compressed payload format does not match manager format",
            ));
        }
        let num_chunks = compressed.layout.compressed_ranges.len();
        let max_chunk = compressed
            .layout
            .uncompressed_ranges
            .iter()
            .map(|r| r.end - r.start)
            .max()
            .unwrap_or(0);
        let temp_bytes = self.format.decompress_temp_bytes(
            num_chunks,
            max_chunk,
            compressed.layout.uncompressed_bytes,
            self.backend,
            stream,
        )?;
        let out_bytes = compressed.layout.uncompressed_bytes;
        let total_grow = out_bytes.saturating_add(self.staging.growth(num_chunks, temp_bytes));
        within(self.budget, total_grow)?;

        // SAFETY: `output` is completely overwritten by `decompress_into_with_budget_async` before return.
        let mut output = unsafe { DeviceBuffer::<u8>::uninitialized_async(stream, out_bytes) }?;
        let remaining_budget = self.budget.map(|b| b.saturating_sub(out_bytes));
        let written = self.decompress_into_with_budget_async(
            stream,
            compressed,
            &mut output,
            remaining_budget,
        )?;
        if written != compressed.layout.uncompressed_bytes {
            return Err(Error::Invalid("decompressed byte count mismatch"));
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_chunk_planning_and_alignment() {
        let mut ranges = Vec::new();
        CompressionManager::plan_chunks(100, 32, 4, &mut ranges).unwrap();
        assert_eq!(ranges, vec![0..32, 32..64, 64..96, 96..100]);

        // Unaligned chunk_size rejected when multiple chunks exist.
        assert!(CompressionManager::plan_chunks(100, 30, 4, &mut ranges).is_err());
        assert!(CompressionManager::plan_chunks(0, 32, 4, &mut ranges).is_err());
    }
}
