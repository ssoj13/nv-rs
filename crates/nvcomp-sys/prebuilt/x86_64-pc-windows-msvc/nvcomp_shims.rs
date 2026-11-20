#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSCompressGetRequiredAlignments(
    compress_opts: nvcompBatchedANSCompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSCompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSCompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedANSCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSCompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSCompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedANSCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSCompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSCompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedANSCompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSCompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSCompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedANSCompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSCompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSDecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedANSDecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSDecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSDecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedANSDecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSDecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSDecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedANSDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSDecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSGetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSGetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedANSDecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedANSDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedANSDecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompCompressGetRequiredAlignments(
    compress_opts: nvcompBatchedBitcompCompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompCompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompCompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedBitcompCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompCompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompCompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedBitcompCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompCompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompCompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedBitcompCompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompCompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompCompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedBitcompCompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompCompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompDecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedBitcompDecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompDecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompDecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedBitcompDecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompDecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompDecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedBitcompDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompDecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompGetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompGetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedBitcompDecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedBitcompDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedBitcompDecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedCompressGetRequiredAlignments(
    compress_opts: nvcompBatchedCascadedCompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedCompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedCompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedCascadedCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedCompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedCompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedCascadedCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedCompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedCompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedCascadedCompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedCompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedCompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedCascadedCompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedCompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedDecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedCascadedDecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedDecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedDecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedCascadedDecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedDecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedDecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedCascadedDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedDecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedGetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedGetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCascadedDecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedCascadedDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCascadedDecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCRC32Async(
    device_input_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_input_chunk_bytes: *const usize,
    num_chunks: usize,
    device_crc32_ptr: *mut u32,
    opts: nvcompBatchedCRC32Opts_t,
    segment_kind: nvcompCRC32SegmentKind_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCRC32Async {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_input_chunk_ptrs,
                            device_input_chunk_bytes,
                            num_chunks,
                            device_crc32_ptr,
                            opts,
                            segment_kind,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCRC32GetHeuristicConf(
    device_input_chunk_bytes: *const usize,
    num_chunks: usize,
    kernel_conf: *mut nvcompCRC32KernelConf_t,
    max_input_chunk_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCRC32GetHeuristicConf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_input_chunk_bytes,
                            num_chunks,
                            kernel_conf,
                            max_input_chunk_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedCRC32SearchConf(
    device_input_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_input_chunk_bytes: *const usize,
    num_chunks: usize,
    device_crc32_ptr: *mut u32,
    spec: nvcompCRC32Spec_t,
    kernel_conf: *mut nvcompCRC32KernelConf_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedCRC32SearchConf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_input_chunk_ptrs,
                            device_input_chunk_bytes,
                            num_chunks,
                            device_crc32_ptr,
                            spec,
                            kernel_conf,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateCompressGetRequiredAlignments(
    compress_opts: nvcompBatchedDeflateCompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateCompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateCompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedDeflateCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateCompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateCompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedDeflateCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateCompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateCompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedDeflateCompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateCompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateCompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedDeflateCompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateCompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateDecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedDeflateDecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateDecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateDecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedDeflateDecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateDecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateDecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedDeflateDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateDecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateGetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateGetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedDeflateDecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedDeflateDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedDeflateDecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateCompressGetRequiredAlignments(
    compress_opts: nvcompBatchedGdeflateCompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateCompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateCompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedGdeflateCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateCompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateCompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedGdeflateCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateCompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateCompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedGdeflateCompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateCompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateCompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedGdeflateCompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateCompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateDecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedGdeflateDecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateDecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateDecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedGdeflateDecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateDecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateDecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedGdeflateDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateDecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateGetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateGetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGdeflateDecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedGdeflateDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGdeflateDecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipCompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedGzipCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipCompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipCompressGetRequiredAlignments(
    compress_opts: nvcompBatchedGzipCompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipCompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipCompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedGzipCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipCompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipCompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedGzipCompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipCompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipCompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedGzipCompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipCompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipDecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedGzipDecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipDecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipDecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedGzipDecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipDecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipDecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedGzipDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipDecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipGetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipGetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedGzipDecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedGzipDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedGzipDecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4CompressGetRequiredAlignments(
    compress_opts: nvcompBatchedLZ4CompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4CompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4CompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedLZ4CompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4CompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4CompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedLZ4CompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4CompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4CompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedLZ4CompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4CompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4CompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedLZ4CompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4CompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4DecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedLZ4DecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4DecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4DecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedLZ4DecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4DecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4DecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedLZ4DecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4DecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4GetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4GetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedLZ4DecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedLZ4DecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedLZ4DecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyCompressGetRequiredAlignments(
    compress_opts: nvcompBatchedSnappyCompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyCompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyCompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedSnappyCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyCompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyCompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedSnappyCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyCompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyCompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedSnappyCompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyCompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyCompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedSnappyCompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyCompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyDecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedSnappyDecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyDecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyDecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedSnappyDecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyDecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyDecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedSnappyDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyDecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyGetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyGetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedSnappyDecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedSnappyDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedSnappyDecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdCompressGetRequiredAlignments(
    compress_opts: nvcompBatchedZstdCompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdCompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(compress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdCompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedZstdCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdCompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdCompressGetTempSizeSync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedZstdCompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdCompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdCompressGetMaxOutputChunkSize(
    max_uncompressed_chunk_bytes: usize,
    compress_opts: nvcompBatchedZstdCompressOpts_t,
    max_compressed_chunk_bytes: *mut usize,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdCompressGetMaxOutputChunkSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            max_uncompressed_chunk_bytes,
                            compress_opts,
                            max_compressed_chunk_bytes,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdCompressAsync(
    device_uncompressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_uncompressed_chunk_bytes: *const usize,
    max_uncompressed_chunk_bytes: usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_compressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *mut usize,
    compress_opts: nvcompBatchedZstdCompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdCompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_uncompressed_chunk_ptrs,
                            device_uncompressed_chunk_bytes,
                            max_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            compress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdDecompressGetRequiredAlignments(
    decompress_opts: nvcompBatchedZstdDecompressOpts_t,
    alignment_requirements: *mut nvcompAlignmentRequirements_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdDecompressGetRequiredAlignments {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(decompress_opts, alignment_requirements) }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdDecompressGetTempSize(
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    decompress_opts: nvcompBatchedZstdDecompressOpts_t,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdDecompressGetTempSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            decompress_opts,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdDecompressGetTempSizeSync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    num_chunks: usize,
    max_uncompressed_chunk_bytes: usize,
    temp_bytes: *mut usize,
    max_total_uncompressed_bytes: usize,
    decompress_opts: nvcompBatchedZstdDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdDecompressGetTempSizeSync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            num_chunks,
                            max_uncompressed_chunk_bytes,
                            temp_bytes,
                            max_total_uncompressed_bytes,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdGetDecompressSizeAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdGetDecompressSizeAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompBatchedZstdDecompressAsync(
    device_compressed_chunk_ptrs: *const *const ::std::os::raw::c_void,
    device_compressed_chunk_bytes: *const usize,
    device_uncompressed_buffer_bytes: *const usize,
    device_uncompressed_chunk_bytes: *mut usize,
    num_chunks: usize,
    device_temp_ptr: *mut ::std::os::raw::c_void,
    temp_bytes: usize,
    device_uncompressed_chunk_ptrs: *const *mut ::std::os::raw::c_void,
    decompress_opts: nvcompBatchedZstdDecompressOpts_t,
    device_statuses: *mut nvcompStatus_t,
    stream: cudaStream_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompBatchedZstdDecompressAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device_compressed_chunk_ptrs,
                            device_compressed_chunk_bytes,
                            device_uncompressed_buffer_bytes,
                            device_uncompressed_chunk_bytes,
                            num_chunks,
                            device_temp_ptr,
                            temp_bytes,
                            device_uncompressed_chunk_ptrs,
                            decompress_opts,
                            device_statuses,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompGetProperties(
    properties: *mut nvcompProperties_t,
) -> nvcompStatus_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompGetProperties {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(properties) }
                Err(__nv_error) => nvcompStatus_t_nvcompErrorNotSupported,
            }
        }
        Err(__nv_error) => nvcompStatus_t_nvcompErrorInternal,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvcompGetStatusString(
    status: nvcompStatus_t,
) -> *const ::std::os::raw::c_char {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvcompGetStatusString {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(status) }
                Err(__nv_error) => ::core::ptr::null(),
            }
        }
        Err(__nv_error) => ::core::ptr::null(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompCreatePlan(
    handle: *mut bitcompHandle_t,
    n: usize,
    dataType: bitcompDataType_t,
    mode: bitcompMode_t,
    algo: bitcompAlgorithm_t,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompCreatePlan {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, dataType, mode, algo) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompCreatePlanFromCompressedData(
    handle: *mut bitcompHandle_t,
    data: *const ::std::os::raw::c_void,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompCreatePlanFromCompressedData {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, data) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompDestroyPlan(handle: bitcompHandle_t) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompDestroyPlan {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompSetStream(
    handle: bitcompHandle_t,
    stream: cudaStream_t,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompSetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, stream) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompAccelerateRemoteCompression(
    handle: bitcompHandle_t,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompAccelerateRemoteCompression {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompCompressLossy_fp16(
    handle: bitcompHandle_t,
    input: *const half,
    output: *mut ::std::os::raw::c_void,
    delta: half,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompCompressLossy_fp16 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output, delta) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompCompressLossy_fp32(
    handle: bitcompHandle_t,
    input: *const f32,
    output: *mut ::std::os::raw::c_void,
    delta: f32,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompCompressLossy_fp32 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output, delta) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompCompressLossy_fp64(
    handle: bitcompHandle_t,
    input: *const f64,
    output: *mut ::std::os::raw::c_void,
    delta: f64,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompCompressLossy_fp64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output, delta) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompCompressLossless(
    handle: bitcompHandle_t,
    input: *const ::std::os::raw::c_void,
    output: *mut ::std::os::raw::c_void,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompCompressLossless {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompUncompress(
    handle: bitcompHandle_t,
    input: *const ::std::os::raw::c_void,
    output: *mut ::std::os::raw::c_void,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompUncompress {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompPartialUncompress(
    handle: bitcompHandle_t,
    input: *const ::std::os::raw::c_void,
    output: *mut ::std::os::raw::c_void,
    start: usize,
    length: usize,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompPartialUncompress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, input, output, start, length) }
                }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompHostCompressLossy_fp16(
    handle: bitcompHandle_t,
    input: *const half,
    output: *mut ::std::os::raw::c_void,
    delta: half,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompHostCompressLossy_fp16 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output, delta) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompHostCompressLossy_fp32(
    handle: bitcompHandle_t,
    input: *const f32,
    output: *mut ::std::os::raw::c_void,
    delta: f32,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompHostCompressLossy_fp32 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output, delta) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompHostCompressLossy_fp64(
    handle: bitcompHandle_t,
    input: *const f64,
    output: *mut ::std::os::raw::c_void,
    delta: f64,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompHostCompressLossy_fp64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output, delta) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompHostCompressLossless(
    handle: bitcompHandle_t,
    input: *const ::std::os::raw::c_void,
    output: *mut ::std::os::raw::c_void,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompHostCompressLossless {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompHostUncompress(
    handle: bitcompHandle_t,
    input: *const ::std::os::raw::c_void,
    output: *mut ::std::os::raw::c_void,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompHostUncompress {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, input, output) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompHostPartialUncompress(
    handle: bitcompHandle_t,
    input: *const ::std::os::raw::c_void,
    output: *mut ::std::os::raw::c_void,
    start: usize,
    length: usize,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompHostPartialUncompress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, input, output, start, length) }
                }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn bitcompMaxBuflen(
    nbytes: usize,
) -> ::core::result::Result<usize, nv_loader::CallError> {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompMaxBuflen {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)(nbytes) }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "nvcomp",
                            stringify!(bitcompMaxBuflen),
                            Some(__nv_error),
                        ),
                    )
                }
            }
        }
        Err(__nv_error) => {
            ::core::result::Result::Err(nv_loader::CallError::Load(__nv_error))
        }
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompGetCompressedSize(
    compressedData: *const ::std::os::raw::c_void,
    size: *mut usize,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompGetCompressedSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(compressedData, size) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompGetCompressedSizeAsync(
    compressedData: *const ::std::os::raw::c_void,
    size: *mut usize,
    stream: cudaStream_t,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompGetCompressedSizeAsync {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(compressedData, size, stream) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompGetUncompressedSize(
    compressedData: *const ::std::os::raw::c_void,
    size: *mut usize,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompGetUncompressedSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(compressedData, size) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompGetUncompressedSizeFromHandle(
    handle: bitcompHandle_t,
    bytes: *mut usize,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompGetUncompressedSizeFromHandle {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, bytes) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompGetDataTypeFromHandle(
    handle: bitcompHandle_t,
    dataType: *mut bitcompDataType_t,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompGetDataTypeFromHandle {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, dataType) }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn bitcompGetCompressedInfo(
    compressedData: *const ::std::os::raw::c_void,
    compressedDataSize: *mut usize,
    uncompressedSize: *mut usize,
    dataType: *mut bitcompDataType_t,
    mode: *mut bitcompMode_t,
    algo: *mut bitcompAlgorithm_t,
) -> bitcompResult_t {
    match nvcomp_api() {
        Ok(__nv_api) => {
            match &__nv_api.bitcompGetCompressedInfo {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            compressedData,
                            compressedDataSize,
                            uncompressedSize,
                            dataType,
                            mode,
                            algo,
                        )
                    }
                }
                Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
            }
        }
        Err(__nv_error) => bitcompResult_t_BITCOMP_UNKNOWN_ERROR,
    }
}
