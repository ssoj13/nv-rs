#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegGetProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegGetProperty {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_, value) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegGetCudartProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegGetCudartProperty {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_, value) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegCreate(
    backend: nvjpegBackend_t,
    dev_allocator: *mut nvjpegDevAllocator_t,
    handle: *mut nvjpegHandle_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(backend, dev_allocator, handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegCreateSimple(
    handle: *mut nvjpegHandle_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegCreateSimple {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegCreateEx(
    backend: nvjpegBackend_t,
    dev_allocator: *mut nvjpegDevAllocator_t,
    pinned_allocator: *mut nvjpegPinnedAllocator_t,
    flags: ::std::os::raw::c_uint,
    handle: *mut nvjpegHandle_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegCreateEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            backend,
                            dev_allocator,
                            pinned_allocator,
                            flags,
                            handle,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegCreateExV2(
    backend: nvjpegBackend_t,
    dev_allocator: *mut nvjpegDevAllocatorV2_t,
    pinned_allocator: *mut nvjpegPinnedAllocatorV2_t,
    flags: ::std::os::raw::c_uint,
    handle: *mut nvjpegHandle_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegCreateExV2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            backend,
                            dev_allocator,
                            pinned_allocator,
                            flags,
                            handle,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDestroy(handle: nvjpegHandle_t) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegSetDeviceMemoryPadding(
    padding: usize,
    handle: nvjpegHandle_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegSetDeviceMemoryPadding {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(padding, handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegGetDeviceMemoryPadding(
    padding: *mut usize,
    handle: nvjpegHandle_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegGetDeviceMemoryPadding {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(padding, handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegSetPinnedMemoryPadding(
    padding: usize,
    handle: nvjpegHandle_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegSetPinnedMemoryPadding {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(padding, handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegGetPinnedMemoryPadding(
    padding: *mut usize,
    handle: nvjpegHandle_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegGetPinnedMemoryPadding {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(padding, handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegGetHardwareDecoderInfo(
    handle: nvjpegHandle_t,
    num_engines: *mut ::std::os::raw::c_uint,
    num_cores_per_engine: *mut ::std::os::raw::c_uint,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegGetHardwareDecoderInfo {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, num_engines, num_cores_per_engine) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegGetHardwareEncoderInfo(
    handle: nvjpegHandle_t,
    num_engines: *mut ::std::os::raw::c_uint,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegGetHardwareEncoderInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, num_engines) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStateCreate(
    handle: nvjpegHandle_t,
    jpeg_handle: *mut nvjpegJpegState_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStateCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, jpeg_handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStateDestroy(
    jpeg_handle: nvjpegJpegState_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStateDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(jpeg_handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegGetImageInfo(
    handle: nvjpegHandle_t,
    data: *const ::std::os::raw::c_uchar,
    length: usize,
    nComponents: *mut ::std::os::raw::c_int,
    subsampling: *mut nvjpegChromaSubsampling_t,
    widths: *mut ::std::os::raw::c_int,
    heights: *mut ::std::os::raw::c_int,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegGetImageInfo {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            data,
                            length,
                            nComponents,
                            subsampling,
                            widths,
                            heights,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecode(
    handle: nvjpegHandle_t,
    jpeg_handle: nvjpegJpegState_t,
    data: *const ::std::os::raw::c_uchar,
    length: usize,
    output_format: nvjpegOutputFormat_t,
    destination: *mut nvjpegImage_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecode {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jpeg_handle,
                            data,
                            length,
                            output_format,
                            destination,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeBatchedInitialize(
    handle: nvjpegHandle_t,
    jpeg_handle: nvjpegJpegState_t,
    batch_size: ::std::os::raw::c_int,
    max_cpu_threads: ::std::os::raw::c_int,
    output_format: nvjpegOutputFormat_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeBatchedInitialize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jpeg_handle,
                            batch_size,
                            max_cpu_threads,
                            output_format,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeBatched(
    handle: nvjpegHandle_t,
    jpeg_handle: nvjpegJpegState_t,
    data: *const *const ::std::os::raw::c_uchar,
    lengths: *const usize,
    destinations: *mut nvjpegImage_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jpeg_handle,
                            data,
                            lengths,
                            destinations,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeBatchedPreAllocate(
    handle: nvjpegHandle_t,
    jpeg_handle: nvjpegJpegState_t,
    batch_size: ::std::os::raw::c_int,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
    chroma_subsampling: nvjpegChromaSubsampling_t,
    output_format: nvjpegOutputFormat_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeBatchedPreAllocate {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jpeg_handle,
                            batch_size,
                            width,
                            height,
                            chroma_subsampling,
                            output_format,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeBatchedParseJpegTables(
    handle: nvjpegHandle_t,
    jpeg_handle: nvjpegJpegState_t,
    data: *const ::std::os::raw::c_uchar,
    length: usize,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeBatchedParseJpegTables {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, jpeg_handle, data, length) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderStateCreate(
    handle: nvjpegHandle_t,
    encoder_state: *mut nvjpegEncoderState_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderStateCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, encoder_state, stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderStateCreateWithBackend(
    handle: nvjpegHandle_t,
    encoder_state: *mut nvjpegEncoderState_t,
    backend: nvjpegEncBackend_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderStateCreateWithBackend {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, encoder_state, backend, stream) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderStateDestroy(
    encoder_state: nvjpegEncoderState_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderStateDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(encoder_state) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsCreate(
    handle: nvjpegHandle_t,
    encoder_params: *mut nvjpegEncoderParams_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, encoder_params, stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsDestroy(
    encoder_params: nvjpegEncoderParams_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(encoder_params) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsSetQuality(
    encoder_params: nvjpegEncoderParams_t,
    quality: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsSetQuality {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(encoder_params, quality, stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsSetEncoding(
    encoder_params: nvjpegEncoderParams_t,
    etype: nvjpegJpegEncoding_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsSetEncoding {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(encoder_params, etype, stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsSetOptimizedHuffman(
    encoder_params: nvjpegEncoderParams_t,
    optimized: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsSetOptimizedHuffman {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(encoder_params, optimized, stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsSetSamplingFactors(
    encoder_params: nvjpegEncoderParams_t,
    chroma_subsampling: nvjpegChromaSubsampling_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsSetSamplingFactors {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(encoder_params, chroma_subsampling, stream) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsSetRestartInterval(
    encoder_params: nvjpegEncoderParams_t,
    restart_interval: ::std::os::raw::c_uint,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsSetRestartInterval {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(encoder_params, restart_interval, stream) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncodeGetBufferSize(
    handle: nvjpegHandle_t,
    encoder_params: nvjpegEncoderParams_t,
    image_width: ::std::os::raw::c_int,
    image_height: ::std::os::raw::c_int,
    max_stream_length: *mut usize,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncodeGetBufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            encoder_params,
                            image_width,
                            image_height,
                            max_stream_length,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncodeYUV(
    handle: nvjpegHandle_t,
    encoder_state: nvjpegEncoderState_t,
    encoder_params: nvjpegEncoderParams_t,
    source: *const nvjpegImage_t,
    chroma_subsampling: nvjpegChromaSubsampling_t,
    image_width: ::std::os::raw::c_int,
    image_height: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncodeYUV {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            encoder_state,
                            encoder_params,
                            source,
                            chroma_subsampling,
                            image_width,
                            image_height,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncodeImage(
    handle: nvjpegHandle_t,
    encoder_state: nvjpegEncoderState_t,
    encoder_params: nvjpegEncoderParams_t,
    source: *const nvjpegImage_t,
    input_format: nvjpegInputFormat_t,
    image_width: ::std::os::raw::c_int,
    image_height: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncodeImage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            encoder_state,
                            encoder_params,
                            source,
                            input_format,
                            image_width,
                            image_height,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncode(
    handle: nvjpegHandle_t,
    encoder_state: nvjpegEncoderState_t,
    encoder_params: nvjpegEncoderParams_t,
    source: *const nvjpegImage_t,
    input_subsampling: nvjpegChromaSubsampling_t,
    input_format: nvjpegInputFormat_t,
    image_width: ::std::os::raw::c_int,
    image_height: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncode {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            encoder_state,
                            encoder_params,
                            source,
                            input_subsampling,
                            input_format,
                            image_width,
                            image_height,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncodeRetrieveBitstreamDevice(
    handle: nvjpegHandle_t,
    encoder_state: nvjpegEncoderState_t,
    data: *mut ::std::os::raw::c_uchar,
    length: *mut usize,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncodeRetrieveBitstreamDevice {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, encoder_state, data, length, stream) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncodeRetrieveBitstream(
    handle: nvjpegHandle_t,
    encoder_state: nvjpegEncoderState_t,
    data: *mut ::std::os::raw::c_uchar,
    length: *mut usize,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncodeRetrieveBitstream {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, encoder_state, data, length, stream) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferPinnedCreate(
    handle: nvjpegHandle_t,
    pinned_allocator: *mut nvjpegPinnedAllocator_t,
    buffer: *mut nvjpegBufferPinned_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferPinnedCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, pinned_allocator, buffer) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferPinnedCreateV2(
    handle: nvjpegHandle_t,
    pinned_allocator: *mut nvjpegPinnedAllocatorV2_t,
    buffer: *mut nvjpegBufferPinned_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferPinnedCreateV2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, pinned_allocator, buffer) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferPinnedResize(
    buffer: nvjpegBufferPinned_t,
    size: usize,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferPinnedResize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(buffer, size, stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferPinnedDestroy(
    buffer: nvjpegBufferPinned_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferPinnedDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(buffer) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferDeviceCreate(
    handle: nvjpegHandle_t,
    device_allocator: *mut nvjpegDevAllocator_t,
    buffer: *mut nvjpegBufferDevice_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferDeviceCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, device_allocator, buffer) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferDeviceCreateV2(
    handle: nvjpegHandle_t,
    device_allocator: *mut nvjpegDevAllocatorV2_t,
    buffer: *mut nvjpegBufferDevice_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferDeviceCreateV2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, device_allocator, buffer) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferDeviceResize(
    buffer: nvjpegBufferDevice_t,
    size: usize,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferDeviceResize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(buffer, size, stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferDeviceDestroy(
    buffer: nvjpegBufferDevice_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferDeviceDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(buffer) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferPinnedRetrieve(
    buffer: nvjpegBufferPinned_t,
    size: *mut usize,
    ptr: *mut *mut ::std::os::raw::c_void,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferPinnedRetrieve {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(buffer, size, ptr) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegBufferDeviceRetrieve(
    buffer: nvjpegBufferDevice_t,
    size: *mut usize,
    ptr: *mut *mut ::std::os::raw::c_void,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegBufferDeviceRetrieve {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(buffer, size, ptr) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegStateAttachPinnedBuffer(
    decoder_state: nvjpegJpegState_t,
    pinned_buffer: nvjpegBufferPinned_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegStateAttachPinnedBuffer {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(decoder_state, pinned_buffer) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegStateAttachDeviceBuffer(
    decoder_state: nvjpegJpegState_t,
    device_buffer: nvjpegBufferDevice_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegStateAttachDeviceBuffer {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(decoder_state, device_buffer) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamCreate(
    handle: nvjpegHandle_t,
    jpeg_stream: *mut nvjpegJpegStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, jpeg_stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamDestroy(
    jpeg_stream: nvjpegJpegStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(jpeg_stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamParse(
    handle: nvjpegHandle_t,
    data: *const ::std::os::raw::c_uchar,
    length: usize,
    save_metadata: ::std::os::raw::c_int,
    save_stream: ::std::os::raw::c_int,
    jpeg_stream: nvjpegJpegStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamParse {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            data,
                            length,
                            save_metadata,
                            save_stream,
                            jpeg_stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamParseHeader(
    handle: nvjpegHandle_t,
    data: *const ::std::os::raw::c_uchar,
    length: usize,
    jpeg_stream: nvjpegJpegStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamParseHeader {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, data, length, jpeg_stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamParseTables(
    handle: nvjpegHandle_t,
    data: *const ::std::os::raw::c_uchar,
    length: usize,
    jpeg_stream: nvjpegJpegStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamParseTables {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, data, length, jpeg_stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamGetJpegEncoding(
    jpeg_stream: nvjpegJpegStream_t,
    jpeg_encoding: *mut nvjpegJpegEncoding_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamGetJpegEncoding {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(jpeg_stream, jpeg_encoding) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamGetFrameDimensions(
    jpeg_stream: nvjpegJpegStream_t,
    width: *mut ::std::os::raw::c_uint,
    height: *mut ::std::os::raw::c_uint,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamGetFrameDimensions {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(jpeg_stream, width, height) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamGetComponentsNum(
    jpeg_stream: nvjpegJpegStream_t,
    components_num: *mut ::std::os::raw::c_uint,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamGetComponentsNum {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(jpeg_stream, components_num) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamGetComponentDimensions(
    jpeg_stream: nvjpegJpegStream_t,
    component: ::std::os::raw::c_uint,
    width: *mut ::std::os::raw::c_uint,
    height: *mut ::std::os::raw::c_uint,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamGetComponentDimensions {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(jpeg_stream, component, width, height) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamGetExifOrientation(
    jpeg_stream: nvjpegJpegStream_t,
    orientation_flag: *mut nvjpegExifOrientation_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamGetExifOrientation {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(jpeg_stream, orientation_flag) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamGetSamplePrecision(
    jpeg_stream: nvjpegJpegStream_t,
    precision: *mut ::std::os::raw::c_uint,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamGetSamplePrecision {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(jpeg_stream, precision) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegJpegStreamGetChromaSubsampling(
    jpeg_stream: nvjpegJpegStream_t,
    chroma_subsampling: *mut nvjpegChromaSubsampling_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegJpegStreamGetChromaSubsampling {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(jpeg_stream, chroma_subsampling) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeParamsCreate(
    handle: nvjpegHandle_t,
    decode_params: *mut nvjpegDecodeParams_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeParamsCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, decode_params) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeParamsDestroy(
    decode_params: nvjpegDecodeParams_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeParamsDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(decode_params) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeParamsSetOutputFormat(
    decode_params: nvjpegDecodeParams_t,
    output_format: nvjpegOutputFormat_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeParamsSetOutputFormat {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(decode_params, output_format) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeParamsSetROI(
    decode_params: nvjpegDecodeParams_t,
    offset_x: ::std::os::raw::c_int,
    offset_y: ::std::os::raw::c_int,
    roi_width: ::std::os::raw::c_int,
    roi_height: ::std::os::raw::c_int,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeParamsSetROI {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            decode_params,
                            offset_x,
                            offset_y,
                            roi_width,
                            roi_height,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeParamsSetAllowCMYK(
    decode_params: nvjpegDecodeParams_t,
    allow_cmyk: ::std::os::raw::c_int,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeParamsSetAllowCMYK {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(decode_params, allow_cmyk) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeParamsSetScaleFactor(
    decode_params: nvjpegDecodeParams_t,
    scale_factor: nvjpegScaleFactor_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeParamsSetScaleFactor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(decode_params, scale_factor) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeParamsSetExifOrientation(
    decode_params: nvjpegDecodeParams_t,
    orientation: nvjpegExifOrientation_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeParamsSetExifOrientation {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(decode_params, orientation) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecoderCreate(
    nvjpeg_handle: nvjpegHandle_t,
    implementation: nvjpegBackend_t,
    decoder_handle: *mut nvjpegJpegDecoder_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecoderCreate {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nvjpeg_handle, implementation, decoder_handle) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecoderDestroy(
    decoder_handle: nvjpegJpegDecoder_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecoderDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(decoder_handle) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecoderJpegSupported(
    decoder_handle: nvjpegJpegDecoder_t,
    jpeg_stream: nvjpegJpegStream_t,
    decode_params: nvjpegDecodeParams_t,
    is_supported: *mut ::std::os::raw::c_int,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecoderJpegSupported {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            decoder_handle,
                            jpeg_stream,
                            decode_params,
                            is_supported,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeBatchedSupported(
    handle: nvjpegHandle_t,
    jpeg_stream: nvjpegJpegStream_t,
    is_supported: *mut ::std::os::raw::c_int,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeBatchedSupported {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, jpeg_stream, is_supported) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeBatchedSupportedEx(
    handle: nvjpegHandle_t,
    jpeg_stream: nvjpegJpegStream_t,
    decode_params: nvjpegDecodeParams_t,
    is_supported: *mut ::std::os::raw::c_int,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeBatchedSupportedEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jpeg_stream, decode_params, is_supported)
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecoderStateCreate(
    nvjpeg_handle: nvjpegHandle_t,
    decoder_handle: nvjpegJpegDecoder_t,
    decoder_state: *mut nvjpegJpegState_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecoderStateCreate {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nvjpeg_handle, decoder_handle, decoder_state) }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeJpeg(
    handle: nvjpegHandle_t,
    decoder: nvjpegJpegDecoder_t,
    decoder_state: nvjpegJpegState_t,
    jpeg_bitstream: nvjpegJpegStream_t,
    destination: *mut nvjpegImage_t,
    decode_params: nvjpegDecodeParams_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeJpeg {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            decoder,
                            decoder_state,
                            jpeg_bitstream,
                            destination,
                            decode_params,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeJpegHost(
    handle: nvjpegHandle_t,
    decoder: nvjpegJpegDecoder_t,
    decoder_state: nvjpegJpegState_t,
    decode_params: nvjpegDecodeParams_t,
    jpeg_stream: nvjpegJpegStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeJpegHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            decoder,
                            decoder_state,
                            decode_params,
                            jpeg_stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeJpegTransferToDevice(
    handle: nvjpegHandle_t,
    decoder: nvjpegJpegDecoder_t,
    decoder_state: nvjpegJpegState_t,
    jpeg_stream: nvjpegJpegStream_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeJpegTransferToDevice {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, decoder, decoder_state, jpeg_stream, stream)
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeJpegDevice(
    handle: nvjpegHandle_t,
    decoder: nvjpegJpegDecoder_t,
    decoder_state: nvjpegJpegState_t,
    destination: *mut nvjpegImage_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeJpegDevice {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, decoder, decoder_state, destination, stream)
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegDecodeBatchedEx(
    handle: nvjpegHandle_t,
    jpeg_handle: nvjpegJpegState_t,
    data: *const *const ::std::os::raw::c_uchar,
    lengths: *const usize,
    destinations: *mut nvjpegImage_t,
    decode_params: *mut nvjpegDecodeParams_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegDecodeBatchedEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jpeg_handle,
                            data,
                            lengths,
                            destinations,
                            decode_params,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsCopyMetadata(
    encoder_state: nvjpegEncoderState_t,
    encode_params: nvjpegEncoderParams_t,
    jpeg_stream: nvjpegJpegStream_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsCopyMetadata {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(encoder_state, encode_params, jpeg_stream, stream)
                    }
                }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvjpegEncoderParamsCopyQuantizationTables(
    encode_params: nvjpegEncoderParams_t,
    jpeg_stream: nvjpegJpegStream_t,
    stream: cudaStream_t,
) -> nvjpegStatus_t {
    match nvjpeg_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvjpegEncoderParamsCopyQuantizationTables {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(encode_params, jpeg_stream, stream) }
                Err(__nv_error) => {
                    nvjpegStatus_t_NVJPEG_STATUS_IMPLEMENTATION_NOT_SUPPORTED
                }
            }
        }
        Err(__nv_error) => nvjpegStatus_t_NVJPEG_STATUS_NOT_INITIALIZED,
    }
}
