//! Stream-ordered batched async compression and decompression over packed GPU chunk spans.
//!
//! Every batched call runs the strict two-phase sequence:
//! 1. Query required temporary size + alignments + worst-case output chunk capacity;
//! 2. Verify combined device growth against the caller's byte budget, reserve [`BatchStaging`],
//!    upload chunk pointer/size tables, and enqueue the nvCOMP kernel on `stream`.

use std::ffi::c_void;
use std::ops::Range;
use std::sync::Arc;

use cuda_core::{DeviceBuffer, Stream};
use nvcomp_sys as sys;

use crate::format::{DecompressBackend, Format};
use crate::scratch::{BatchStaging, StagingTables};
use crate::{Error, NvcompStatus, Result, check, same_stream};

/// Metadata produced by [`compress_batched_async`] describing packed output slots inside the
/// destination [`DeviceBuffer<u8>`].
#[derive(Clone, Debug)]
pub struct BatchedCompressReceipt {
    /// Codec format used to produce these chunks.
    pub format: Format,
    /// Number of chunks launched.
    pub num_chunks: usize,
    /// Byte offset stride between consecutive compressed chunk slots in the destination buffer.
    pub slot_stride: usize,
    /// Byte lengths of the original uncompressed chunks.
    pub uncompressed_chunk_bytes: Vec<usize>,
}

impl BatchedCompressReceipt {
    /// Synchronizes `stream`, checks per-chunk compression statuses, and reads back the actual
    /// compressed byte lengths for each chunk using `staging`'s pinned buffer (zero allocation on
    /// repeated calls if `out_sizes` is reused).
    pub fn collect_sizes(
        &self,
        stream: &Arc<Stream>,
        staging: &mut BatchStaging,
        out_sizes: &mut Vec<usize>,
    ) -> Result<()> {
        same_stream(stream, staging.stream())?;
        if self.num_chunks == 0 {
            out_sizes.clear();
            return Ok(());
        }
        let (tables, _) = staging.tables_mut()?;
        BatchStaging::download_prefix(
            &mut tables.host_out_bytes,
            &tables.dev_out_bytes,
            self.num_chunks,
            stream,
        )?;
        BatchStaging::download_prefix(
            &mut tables.host_statuses,
            &tables.dev_statuses,
            self.num_chunks,
            stream,
        )?;
        stream.synchronize()?;

        for &raw_status in &tables.host_statuses.as_slice()[..self.num_chunks] {
            if !NvcompStatus(raw_status).is_success() {
                return check("nvcompBatchedCompressAsync[chunk]", raw_status);
            }
        }
        out_sizes.clear();
        out_sizes.extend_from_slice(&tables.host_out_bytes.as_slice()[..self.num_chunks]);
        Ok(())
    }
}

/// Metadata produced by [`decompress_batched_async`].
#[derive(Clone, Copy, Debug)]
pub struct BatchedDecompressReceipt {
    /// Codec format used to decompress.
    pub format: Format,
    /// Number of chunks decompressed.
    pub num_chunks: usize,
}

impl BatchedDecompressReceipt {
    /// Synchronizes `stream` and verifies every chunk reported `nvcompSuccess`, filling `out_sizes`
    /// with the actual decompressed byte lengths written per chunk.
    pub fn collect_sizes(
        &self,
        stream: &Arc<Stream>,
        staging: &mut BatchStaging,
        out_sizes: &mut Vec<usize>,
    ) -> Result<()> {
        same_stream(stream, staging.stream())?;
        if self.num_chunks == 0 {
            out_sizes.clear();
            return Ok(());
        }
        let (tables, _) = staging.tables_mut()?;
        BatchStaging::download_prefix(
            &mut tables.host_out_bytes,
            &tables.dev_out_bytes,
            self.num_chunks,
            stream,
        )?;
        BatchStaging::download_prefix(
            &mut tables.host_statuses,
            &tables.dev_statuses,
            self.num_chunks,
            stream,
        )?;
        stream.synchronize()?;

        for &raw_status in &tables.host_statuses.as_slice()[..self.num_chunks] {
            check("nvcompBatchedDecompressAsync[chunk]", raw_status)?;
        }
        out_sizes.clear();
        out_sizes.extend_from_slice(&tables.host_out_bytes.as_slice()[..self.num_chunks]);
        Ok(())
    }
}

pub(crate) fn aligned_stride(bytes: usize, align: usize) -> Result<usize> {
    let a = align.max(1);
    let rem = bytes % a;
    if rem == 0 {
        Ok(bytes)
    } else {
        bytes
            .checked_add(a - rem)
            .ok_or(Error::Invalid("aligned stride overflow"))
    }
}

fn validate_chunk_ranges(ranges: &[Range<usize>], total_bytes: usize) -> Result<(usize, usize)> {
    if ranges.is_empty() {
        return Err(Error::Invalid(
            "batched operation requires at least one chunk",
        ));
    }
    let mut max_chunk = 0usize;
    let mut total = 0usize;
    for r in ranges {
        if r.start >= r.end || r.end > total_bytes {
            return Err(Error::Invalid("chunk range out of bounds or empty"));
        }
        let len = r.end - r.start;
        max_chunk = max_chunk.max(len);
        total = total
            .checked_add(len)
            .ok_or(Error::Invalid("total chunk byte count overflow"))?;
    }
    Ok((max_chunk, total))
}

/// Launches batched stream-ordered compression of `input_ranges` inside `input` into packed,
/// alignment-padded slots inside `output`.
///
/// The caller must ensure `output.num_bytes() >= input_ranges.len() * slot_stride`, where
/// `slot_stride = aligned_stride(format.max_output_chunk_bytes(max_chunk)?, alignments.output)?`.
pub fn compress_batched_async(
    stream: &Arc<Stream>,
    staging: &mut BatchStaging,
    format: Format,
    input: &DeviceBuffer<u8>,
    input_ranges: &[Range<usize>],
    output: &mut DeviceBuffer<u8>,
    budget: Option<usize>,
) -> Result<BatchedCompressReceipt> {
    same_stream(stream, staging.stream())?;
    if !Arc::ptr_eq(input.context(), staging.context())
        || !Arc::ptr_eq(output.context(), staging.context())
    {
        return Err(Error::Invalid(
            "input/output buffer belongs to another CUDA context",
        ));
    }
    let (max_chunk, total_bytes) = validate_chunk_ranges(input_ranges, input.num_bytes())?;
    let alignments = format.compress_alignments()?;
    let max_out_chunk = format.max_output_chunk_bytes(max_chunk)?;
    let slot_stride = aligned_stride(max_out_chunk, alignments.output)?;
    let required_output = input_ranges
        .len()
        .checked_mul(slot_stride)
        .ok_or(Error::Invalid("compressed output buffer size overflow"))?;
    if output.num_bytes() < required_output {
        return Err(Error::Invalid(
            "output device buffer smaller than required worst-case batch span",
        ));
    }

    let in_base = input.cu_deviceptr();
    let out_base = output.cu_deviceptr();
    let in_align = u64::try_from(alignments.input).unwrap_or(u64::MAX);
    let out_align = u64::try_from(alignments.output).unwrap_or(u64::MAX);
    if !out_base.is_multiple_of(out_align) {
        return Err(Error::Invalid(
            "output buffer base pointer does not satisfy nvCOMP alignment",
        ));
    }
    for r in input_ranges {
        let addr = in_base
            .checked_add(r.start as u64)
            .ok_or(Error::Invalid("input ptr overflow"))?;
        if !addr.is_multiple_of(in_align) {
            return Err(Error::Invalid(
                "input chunk offset does not satisfy nvCOMP alignment",
            ));
        }
    }

    let temp_bytes =
        format.compress_temp_bytes(input_ranges.len(), max_chunk, total_bytes, stream)?;
    staging.prepare(
        stream,
        input_ranges.len(),
        temp_bytes,
        alignments.temp,
        budget,
    )?;

    let num_chunks = input_ranges.len();
    let mut uncomp_sizes = Vec::with_capacity(num_chunks);
    {
        let (tables, _) = staging.tables_mut()?;
        let host_in_ptrs = tables.host_in_ptrs.as_mut_slice();
        let host_in_bytes = tables.host_in_bytes.as_mut_slice();
        let host_out_ptrs = tables.host_out_ptrs.as_mut_slice();

        for (i, r) in input_ranges.iter().enumerate() {
            let len = r.end - r.start;
            host_in_ptrs[i] = in_base + r.start as u64;
            host_in_bytes[i] = len;
            host_out_ptrs[i] = out_base + (i * slot_stride) as u64;
            uncomp_sizes.push(len);
        }
    }

    upload_compress_tables_and_launch(stream, staging, format, num_chunks, max_chunk)?;

    Ok(BatchedCompressReceipt {
        format,
        num_chunks,
        slot_stride,
        uncompressed_chunk_bytes: uncomp_sizes,
    })
}

fn upload_compress_tables_and_launch(
    stream: &Stream,
    staging: &mut BatchStaging,
    format: Format,
    num_chunks: usize,
    max_chunk: usize,
) -> Result<()> {
    {
        let (tables, _) = staging.tables_mut()?;
        let cu_stream = stream;
        let StagingTables {
            host_in_ptrs,
            host_in_bytes,
            host_out_ptrs,
            dev_in_ptrs,
            dev_in_bytes,
            dev_out_ptrs,
            ..
        } = tables;
        BatchStaging::upload_prefix(host_in_ptrs, dev_in_ptrs, num_chunks, cu_stream)?;
        BatchStaging::upload_prefix(host_in_bytes, dev_in_bytes, num_chunks, cu_stream)?;
        BatchStaging::upload_prefix(host_out_ptrs, dev_out_ptrs, num_chunks, cu_stream)?;
    }
    staging.arm_dma_fence(stream)?;

    let res = {
        let (tables, scratch) = staging.tables_mut()?;
        let (temp_ptr, temp_bytes) = scratch.as_raw_parts();
        // SAFETY: all device pointer/size/status tables hold `>= num_chunks` entries and scratch holds
        // `>= temp_bytes` aligned workspace.
        unsafe {
            format.launch_compress_async(
                tables.dev_in_ptrs.cu_deviceptr() as *const *const c_void,
                tables.dev_in_bytes.cu_deviceptr() as *const usize,
                max_chunk,
                num_chunks,
                temp_ptr as *mut c_void,
                temp_bytes,
                tables.dev_out_ptrs.cu_deviceptr() as *const *mut c_void,
                tables.dev_out_bytes.cu_deviceptr() as *mut usize,
                tables.dev_statuses.cu_deviceptr() as *mut sys::nvcompStatus_t,
                stream.cu_stream(),
            )
        }
    };
    // Advance fence past kernel completion (tables + scratch remain protected until done).
    staging.arm_dma_fence(stream)?;
    res
}

/// Queries the uncompressed byte sizes stored inside compressed chunks asynchronously on `stream`,
/// returning the per-chunk sizes after synchronizing `stream`.
pub fn query_decompress_sizes_async(
    stream: &Arc<Stream>,
    staging: &mut BatchStaging,
    format: Format,
    compressed: &DeviceBuffer<u8>,
    compressed_ranges: &[Range<usize>],
    budget: Option<usize>,
) -> Result<Vec<usize>> {
    same_stream(stream, staging.stream())?;
    if !Arc::ptr_eq(compressed.context(), staging.context()) {
        return Err(Error::Invalid(
            "compressed buffer belongs to another CUDA context",
        ));
    }
    let (_, _) = validate_chunk_ranges(compressed_ranges, compressed.num_bytes())?;
    let alignments = format.decompress_alignments(DecompressBackend::Default)?;
    let in_align = u64::try_from(alignments.input).unwrap_or(u64::MAX);
    let base = compressed.cu_deviceptr();
    for r in compressed_ranges {
        let addr = base
            .checked_add(r.start as u64)
            .ok_or(Error::Invalid("compressed ptr overflow"))?;
        if !addr.is_multiple_of(in_align) {
            return Err(Error::Invalid(
                "compressed chunk offset violates nvCOMP input alignment",
            ));
        }
    }

    let num_chunks = compressed_ranges.len();
    staging.prepare(stream, num_chunks, 0, alignments.temp, budget)?;
    {
        let (tables, _) = staging.tables_mut()?;
        let host_in_ptrs = tables.host_in_ptrs.as_mut_slice();
        let host_in_bytes = tables.host_in_bytes.as_mut_slice();
        for (i, r) in compressed_ranges.iter().enumerate() {
            host_in_ptrs[i] = base + r.start as u64;
            host_in_bytes[i] = r.end - r.start;
        }
        let StagingTables {
            host_in_ptrs,
            host_in_bytes,
            dev_in_ptrs,
            dev_in_bytes,
            ..
        } = tables;
        BatchStaging::upload_prefix(host_in_ptrs, dev_in_ptrs, num_chunks, stream)?;
        BatchStaging::upload_prefix(host_in_bytes, dev_in_bytes, num_chunks, stream)?;
    }
    staging.arm_dma_fence(stream)?;
    {
        let (tables, _) = staging.tables_mut()?;
        // SAFETY: `dev_in_ptrs`, `dev_in_bytes`, and `dev_out_bytes` hold `>= num_chunks` slots.
        unsafe {
            format.launch_get_decompress_size_async(
                tables.dev_in_ptrs.cu_deviceptr() as *const *const c_void,
                tables.dev_in_bytes.cu_deviceptr() as *const usize,
                tables.dev_out_bytes.cu_deviceptr() as *mut usize,
                num_chunks,
                stream.cu_stream(),
            )?;
        }
        BatchStaging::download_prefix(
            &mut tables.host_out_bytes,
            &tables.dev_out_bytes,
            num_chunks,
            stream,
        )?;
    }
    stream.synchronize()?;
    let (tables, _) = staging.tables_mut()?;
    Ok(tables.host_out_bytes.as_slice()[..num_chunks].to_vec())
}

/// Launches batched stream-ordered decompression of `compressed_ranges` inside `compressed` into
/// `output_ranges` inside `output`.
#[expect(
    clippy::too_many_arguments,
    reason = "Explicit stream, staging, codec format, backend, compressed/output ranges, and budget"
)]
pub fn decompress_batched_async(
    stream: &Arc<Stream>,
    staging: &mut BatchStaging,
    format: Format,
    backend: DecompressBackend,
    compressed: &DeviceBuffer<u8>,
    compressed_ranges: &[Range<usize>],
    output: &mut DeviceBuffer<u8>,
    output_ranges: &[Range<usize>],
    budget: Option<usize>,
) -> Result<BatchedDecompressReceipt> {
    same_stream(stream, staging.stream())?;
    if !Arc::ptr_eq(compressed.context(), staging.context())
        || !Arc::ptr_eq(output.context(), staging.context())
    {
        return Err(Error::Invalid(
            "compressed/output buffer belongs to another CUDA context",
        ));
    }
    if compressed_ranges.len() != output_ranges.len() {
        return Err(Error::Invalid(
            "compressed and output range counts must match",
        ));
    }
    let _ = validate_chunk_ranges(compressed_ranges, compressed.num_bytes())?;
    let (max_uncomp_chunk, total_uncomp_bytes) =
        validate_chunk_ranges(output_ranges, output.num_bytes())?;

    let alignments = format.decompress_alignments(backend)?;
    let in_align = u64::try_from(alignments.input).unwrap_or(u64::MAX);
    let out_align = u64::try_from(alignments.output).unwrap_or(u64::MAX);
    let in_base = compressed.cu_deviceptr();
    let out_base = output.cu_deviceptr();

    for r in compressed_ranges {
        let addr = in_base
            .checked_add(r.start as u64)
            .ok_or(Error::Invalid("compressed ptr overflow"))?;
        if !addr.is_multiple_of(in_align) {
            return Err(Error::Invalid(
                "compressed chunk offset violates nvCOMP input alignment",
            ));
        }
    }
    for r in output_ranges {
        let addr = out_base
            .checked_add(r.start as u64)
            .ok_or(Error::Invalid("output ptr overflow"))?;
        if !addr.is_multiple_of(out_align) {
            return Err(Error::Invalid(
                "output chunk offset violates nvCOMP output alignment",
            ));
        }
    }

    let num_chunks = compressed_ranges.len();
    let temp_bytes = format.decompress_temp_bytes(
        num_chunks,
        max_uncomp_chunk,
        total_uncomp_bytes,
        backend,
        stream,
    )?;
    staging.prepare(stream, num_chunks, temp_bytes, alignments.temp, budget)?;

    {
        let (tables, _) = staging.tables_mut()?;
        let host_in_ptrs = tables.host_in_ptrs.as_mut_slice();
        let host_in_bytes = tables.host_in_bytes.as_mut_slice();
        let host_out_ptrs = tables.host_out_ptrs.as_mut_slice();
        let host_buf_bytes = tables.host_buf_bytes.as_mut_slice();

        for (i, (cr, or)) in compressed_ranges
            .iter()
            .zip(output_ranges.iter())
            .enumerate()
        {
            host_in_ptrs[i] = in_base + cr.start as u64;
            host_in_bytes[i] = cr.end - cr.start;
            host_out_ptrs[i] = out_base + or.start as u64;
            host_buf_bytes[i] = or.end - or.start;
        }
        let StagingTables {
            host_in_ptrs,
            host_in_bytes,
            host_out_ptrs,
            host_buf_bytes,
            dev_in_ptrs,
            dev_in_bytes,
            dev_out_ptrs,
            dev_buf_bytes,
            ..
        } = tables;
        BatchStaging::upload_prefix(host_in_ptrs, dev_in_ptrs, num_chunks, stream)?;
        BatchStaging::upload_prefix(host_in_bytes, dev_in_bytes, num_chunks, stream)?;
        BatchStaging::upload_prefix(host_out_ptrs, dev_out_ptrs, num_chunks, stream)?;
        BatchStaging::upload_prefix(host_buf_bytes, dev_buf_bytes, num_chunks, stream)?;
    }
    staging.arm_dma_fence(stream)?;

    let launch_res = {
        let (tables, scratch) = staging.tables_mut()?;
        let (temp_ptr, cap_bytes) = scratch.as_raw_parts();
        // SAFETY: all tables hold `>= num_chunks` items, output chunks satisfy alignment and
        // capacity bounds, and scratch satisfies temp bytes + alignment.
        unsafe {
            format.launch_decompress_async(
                tables.dev_in_ptrs.cu_deviceptr() as *const *const c_void,
                tables.dev_in_bytes.cu_deviceptr() as *const usize,
                tables.dev_buf_bytes.cu_deviceptr() as *const usize,
                tables.dev_out_bytes.cu_deviceptr() as *mut usize,
                num_chunks,
                temp_ptr as *mut c_void,
                cap_bytes,
                tables.dev_out_ptrs.cu_deviceptr() as *const *mut c_void,
                backend,
                tables.dev_statuses.cu_deviceptr() as *mut sys::nvcompStatus_t,
                stream.cu_stream(),
            )
        }
    };
    // Advance fence past kernel completion (tables + scratch remain protected until done).
    staging.arm_dma_fence(stream)?;
    launch_res?;

    Ok(BatchedDecompressReceipt { format, num_chunks })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0_aligned_stride_and_range_validation() {
        assert_eq!(aligned_stride(0, 8).unwrap(), 0);
        assert_eq!(aligned_stride(1, 8).unwrap(), 8);
        assert_eq!(aligned_stride(16, 8).unwrap(), 16);
        assert_eq!(aligned_stride(17, 16).unwrap(), 32);
        assert!(aligned_stride(usize::MAX, 16).is_err());

        assert!(validate_chunk_ranges(&[], 1024).is_err());
        assert!(validate_chunk_ranges(std::slice::from_ref(&(0..0)), 1024).is_err());
        assert!(validate_chunk_ranges(std::slice::from_ref(&(512..1025)), 1024).is_err());
        let (max_c, tot) = validate_chunk_ranges(&[0..512, 512..768], 1024).unwrap();
        assert_eq!((max_c, tot), (512, 768));
    }
}
