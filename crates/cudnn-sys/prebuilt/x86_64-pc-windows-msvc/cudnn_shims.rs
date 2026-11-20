#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn cudnnGetVersion() -> ::core::result::Result<usize, nv_loader::CallError> {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)() }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cudnn",
                            stringify!(cudnnGetVersion),
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
pub unsafe fn cudnnGetMaxDeviceVersion() -> ::core::result::Result<
    usize,
    nv_loader::CallError,
> {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetMaxDeviceVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)() }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cudnn",
                            stringify!(cudnnGetMaxDeviceVersion),
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
pub unsafe fn cudnnGetCudartVersion() -> ::core::result::Result<
    usize,
    nv_loader::CallError,
> {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetCudartVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)() }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cudnn",
                            stringify!(cudnnGetCudartVersion),
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
pub unsafe extern "C" fn cudnnGetErrorString(
    status: cudnnStatus_t,
) -> *const ::std::os::raw::c_char {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetErrorString {
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
pub unsafe fn cudnnGetLastErrorString(
    message: *mut ::std::os::raw::c_char,
    max_size: usize,
) -> ::core::result::Result<(), nv_loader::CallError> {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetLastErrorString {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(message, max_size) };
                    ::core::result::Result::Ok(())
                }
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cudnn",
                            stringify!(cudnnGetLastErrorString),
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
pub unsafe extern "C" fn cudnnQueryRuntimeError(
    handle: cudnnHandle_t,
    rstatus: *mut cudnnStatus_t,
    mode: cudnnErrQueryMode_t,
    tag: *mut cudnnRuntimeTag_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnQueryRuntimeError {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, rstatus, mode, tag) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetProperty {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_, value) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreate(handle: *mut cudnnHandle_t) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroy(handle: cudnnHandle_t) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetStream(
    handle: cudnnHandle_t,
    streamId: cudaStream_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetStream(
    handle: cudnnHandle_t,
    streamId: *mut cudaStream_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetCallback(
    mask: ::std::os::raw::c_uint,
    udata: *mut ::std::os::raw::c_void,
    fptr: cudnnCallback_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(mask, udata, fptr) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetCallback(
    mask: *mut ::std::os::raw::c_uint,
    udata: *mut *mut ::std::os::raw::c_void,
    fptr: *mut cudnnCallback_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(mask, udata, fptr) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGraphVersionCheck() -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGraphVersionCheck {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendCreateDescriptor(
    descriptorType: cudnnBackendDescriptorType_t,
    descriptor: *mut cudnnBackendDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendCreateDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descriptorType, descriptor) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendDestroyDescriptor(
    descriptor: cudnnBackendDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendDestroyDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descriptor) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendInitialize(
    descriptor: cudnnBackendDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendInitialize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descriptor) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendFinalize(
    descriptor: cudnnBackendDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendFinalize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descriptor) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendSetAttribute(
    descriptor: cudnnBackendDescriptor_t,
    attributeName: cudnnBackendAttributeName_t,
    attributeType: cudnnBackendAttributeType_t,
    elementCount: i64,
    arrayOfElements: *const ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendSetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            descriptor,
                            attributeName,
                            attributeType,
                            elementCount,
                            arrayOfElements,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendGetAttribute(
    descriptor: cudnnBackendDescriptor_t,
    attributeName: cudnnBackendAttributeName_t,
    attributeType: cudnnBackendAttributeType_t,
    requestedElementCount: i64,
    elementCount: *mut i64,
    arrayOfElements: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            descriptor,
                            attributeName,
                            attributeType,
                            requestedElementCount,
                            elementCount,
                            arrayOfElements,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetExecutionPlanWorkspaceSize(
    handle: cudnnHandle_t,
    executionPlan: cudnnBackendDescriptor_t,
    variantPack: cudnnBackendDescriptor_t,
    workspaceSizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetExecutionPlanWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            executionPlan,
                            variantPack,
                            workspaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendExecute(
    handle: cudnnHandle_t,
    executionPlan: cudnnBackendDescriptor_t,
    variantPack: cudnnBackendDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendExecute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, executionPlan, variantPack) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendPopulateCudaGraph(
    handle: cudnnHandle_t,
    executionPlan: cudnnBackendDescriptor_t,
    variantPack: cudnnBackendDescriptor_t,
    graph: cudaGraph_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendPopulateCudaGraph {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, executionPlan, variantPack, graph) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBackendUpdateCudaGraph(
    handle: cudnnHandle_t,
    executionPlan: cudnnBackendDescriptor_t,
    variantPack: cudnnBackendDescriptor_t,
    graph: cudaGraph_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBackendUpdateCudaGraph {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, executionPlan, variantPack, graph) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateTensorDescriptor(
    tensorDesc: *mut cudnnTensorDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateTensorDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(tensorDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetTensor4dDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
    format: cudnnTensorFormat_t,
    dataType: cudnnDataType_t,
    n: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetTensor4dDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(tensorDesc, format, dataType, n, c, h, w) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetTensor4dDescriptorEx(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: cudnnDataType_t,
    n: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
    nStride: ::std::os::raw::c_int,
    cStride: ::std::os::raw::c_int,
    hStride: ::std::os::raw::c_int,
    wStride: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetTensor4dDescriptorEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            tensorDesc,
                            dataType,
                            n,
                            c,
                            h,
                            w,
                            nStride,
                            cStride,
                            hStride,
                            wStride,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetTensor4dDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: *mut cudnnDataType_t,
    n: *mut ::std::os::raw::c_int,
    c: *mut ::std::os::raw::c_int,
    h: *mut ::std::os::raw::c_int,
    w: *mut ::std::os::raw::c_int,
    nStride: *mut ::std::os::raw::c_int,
    cStride: *mut ::std::os::raw::c_int,
    hStride: *mut ::std::os::raw::c_int,
    wStride: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetTensor4dDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            tensorDesc,
                            dataType,
                            n,
                            c,
                            h,
                            w,
                            nStride,
                            cStride,
                            hStride,
                            wStride,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetTensorNdDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
    dataType: cudnnDataType_t,
    nbDims: ::std::os::raw::c_int,
    dimA: *const ::std::os::raw::c_int,
    strideA: *const ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetTensorNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(tensorDesc, dataType, nbDims, dimA, strideA) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetTensorNdDescriptorEx(
    tensorDesc: cudnnTensorDescriptor_t,
    format: cudnnTensorFormat_t,
    dataType: cudnnDataType_t,
    nbDims: ::std::os::raw::c_int,
    dimA: *const ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetTensorNdDescriptorEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(tensorDesc, format, dataType, nbDims, dimA) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetTensorNdDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
    nbDimsRequested: ::std::os::raw::c_int,
    dataType: *mut cudnnDataType_t,
    nbDims: *mut ::std::os::raw::c_int,
    dimA: *mut ::std::os::raw::c_int,
    strideA: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetTensorNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            tensorDesc,
                            nbDimsRequested,
                            dataType,
                            nbDims,
                            dimA,
                            strideA,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetTensorSizeInBytes(
    tensorDesc: cudnnTensorDescriptor_t,
    size: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetTensorSizeInBytes {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(tensorDesc, size) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyTensorDescriptor(
    tensorDesc: cudnnTensorDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyTensorDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(tensorDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnInitTransformDest(
    transformDesc: cudnnTensorTransformDescriptor_t,
    srcDesc: cudnnTensorDescriptor_t,
    destDesc: cudnnTensorDescriptor_t,
    destSizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnInitTransformDest {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(transformDesc, srcDesc, destDesc, destSizeInBytes)
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateTensorTransformDescriptor(
    transformDesc: *mut cudnnTensorTransformDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateTensorTransformDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(transformDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetTensorTransformDescriptor(
    transformDesc: cudnnTensorTransformDescriptor_t,
    nbDims: u32,
    destFormat: cudnnTensorFormat_t,
    padBeforeA: *const i32,
    padAfterA: *const i32,
    foldA: *const u32,
    direction: cudnnFoldingDirection_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetTensorTransformDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            transformDesc,
                            nbDims,
                            destFormat,
                            padBeforeA,
                            padAfterA,
                            foldA,
                            direction,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetTensorTransformDescriptor(
    transformDesc: cudnnTensorTransformDescriptor_t,
    nbDimsRequested: u32,
    destFormat: *mut cudnnTensorFormat_t,
    padBeforeA: *mut i32,
    padAfterA: *mut i32,
    foldA: *mut u32,
    direction: *mut cudnnFoldingDirection_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetTensorTransformDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            transformDesc,
                            nbDimsRequested,
                            destFormat,
                            padBeforeA,
                            padAfterA,
                            foldA,
                            direction,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyTensorTransformDescriptor(
    transformDesc: cudnnTensorTransformDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyTensorTransformDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(transformDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnTransformTensor(
    handle: cudnnHandle_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnTransformTensor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, alpha, xDesc, x, beta, yDesc, y) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnTransformTensorEx(
    handle: cudnnHandle_t,
    transDesc: cudnnTensorTransformDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    srcDesc: cudnnTensorDescriptor_t,
    srcData: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    destDesc: cudnnTensorDescriptor_t,
    destData: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnTransformTensorEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transDesc,
                            alpha,
                            srcDesc,
                            srcData,
                            beta,
                            destDesc,
                            destData,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnAddTensor(
    handle: cudnnHandle_t,
    alpha: *const ::std::os::raw::c_void,
    aDesc: cudnnTensorDescriptor_t,
    A: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    cDesc: cudnnTensorDescriptor_t,
    C: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnAddTensor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, alpha, aDesc, A, beta, cDesc, C) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateOpTensorDescriptor(
    opTensorDesc: *mut cudnnOpTensorDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateOpTensorDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(opTensorDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetOpTensorDescriptor(
    opTensorDesc: cudnnOpTensorDescriptor_t,
    opTensorOp: cudnnOpTensorOp_t,
    opTensorCompType: cudnnDataType_t,
    opTensorNanOpt: cudnnNanPropagation_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetOpTensorDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            opTensorDesc,
                            opTensorOp,
                            opTensorCompType,
                            opTensorNanOpt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetOpTensorDescriptor(
    opTensorDesc: cudnnOpTensorDescriptor_t,
    opTensorOp: *mut cudnnOpTensorOp_t,
    opTensorCompType: *mut cudnnDataType_t,
    opTensorNanOpt: *mut cudnnNanPropagation_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetOpTensorDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            opTensorDesc,
                            opTensorOp,
                            opTensorCompType,
                            opTensorNanOpt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyOpTensorDescriptor(
    opTensorDesc: cudnnOpTensorDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyOpTensorDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(opTensorDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnOpTensor(
    handle: cudnnHandle_t,
    opTensorDesc: cudnnOpTensorDescriptor_t,
    alpha1: *const ::std::os::raw::c_void,
    aDesc: cudnnTensorDescriptor_t,
    A: *const ::std::os::raw::c_void,
    alpha2: *const ::std::os::raw::c_void,
    bDesc: cudnnTensorDescriptor_t,
    B: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    cDesc: cudnnTensorDescriptor_t,
    C: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnOpTensor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opTensorDesc,
                            alpha1,
                            aDesc,
                            A,
                            alpha2,
                            bDesc,
                            B,
                            beta,
                            cDesc,
                            C,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateReduceTensorDescriptor(
    reduceTensorDesc: *mut cudnnReduceTensorDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateReduceTensorDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(reduceTensorDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    reduceTensorOp: cudnnReduceTensorOp_t,
    reduceTensorCompType: cudnnDataType_t,
    reduceTensorNanOpt: cudnnNanPropagation_t,
    reduceTensorIndices: cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: cudnnIndicesType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetReduceTensorDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            reduceTensorDesc,
                            reduceTensorOp,
                            reduceTensorCompType,
                            reduceTensorNanOpt,
                            reduceTensorIndices,
                            reduceTensorIndicesType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    reduceTensorOp: *mut cudnnReduceTensorOp_t,
    reduceTensorCompType: *mut cudnnDataType_t,
    reduceTensorNanOpt: *mut cudnnNanPropagation_t,
    reduceTensorIndices: *mut cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: *mut cudnnIndicesType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetReduceTensorDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            reduceTensorDesc,
                            reduceTensorOp,
                            reduceTensorCompType,
                            reduceTensorNanOpt,
                            reduceTensorIndices,
                            reduceTensorIndicesType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyReduceTensorDescriptor(
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyReduceTensorDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(reduceTensorDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetReductionIndicesSize(
    handle: cudnnHandle_t,
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    aDesc: cudnnTensorDescriptor_t,
    cDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetReductionIndicesSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, reduceTensorDesc, aDesc, cDesc, sizeInBytes)
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetReductionWorkspaceSize(
    handle: cudnnHandle_t,
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    aDesc: cudnnTensorDescriptor_t,
    cDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetReductionWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, reduceTensorDesc, aDesc, cDesc, sizeInBytes)
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnReduceTensor(
    handle: cudnnHandle_t,
    reduceTensorDesc: cudnnReduceTensorDescriptor_t,
    indices: *mut ::std::os::raw::c_void,
    indicesSizeInBytes: usize,
    workspace: *mut ::std::os::raw::c_void,
    workspaceSizeInBytes: usize,
    alpha: *const ::std::os::raw::c_void,
    aDesc: cudnnTensorDescriptor_t,
    A: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    cDesc: cudnnTensorDescriptor_t,
    C: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnReduceTensor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            reduceTensorDesc,
                            indices,
                            indicesSizeInBytes,
                            workspace,
                            workspaceSizeInBytes,
                            alpha,
                            aDesc,
                            A,
                            beta,
                            cDesc,
                            C,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetTensor(
    handle: cudnnHandle_t,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
    valuePtr: *const ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetTensor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, yDesc, y, valuePtr) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnScaleTensor(
    handle: cudnnHandle_t,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
    alpha: *const ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnScaleTensor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, yDesc, y, alpha) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateFilterDescriptor(
    filterDesc: *mut cudnnFilterDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateFilterDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(filterDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetFilter4dDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
    dataType: cudnnDataType_t,
    format: cudnnTensorFormat_t,
    k: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetFilter4dDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(filterDesc, dataType, format, k, c, h, w) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetFilter4dDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
    dataType: *mut cudnnDataType_t,
    format: *mut cudnnTensorFormat_t,
    k: *mut ::std::os::raw::c_int,
    c: *mut ::std::os::raw::c_int,
    h: *mut ::std::os::raw::c_int,
    w: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetFilter4dDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(filterDesc, dataType, format, k, c, h, w) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetFilterNdDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
    dataType: cudnnDataType_t,
    format: cudnnTensorFormat_t,
    nbDims: ::std::os::raw::c_int,
    filterDimA: *const ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetFilterNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(filterDesc, dataType, format, nbDims, filterDimA)
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetFilterNdDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
    nbDimsRequested: ::std::os::raw::c_int,
    dataType: *mut cudnnDataType_t,
    format: *mut cudnnTensorFormat_t,
    nbDims: *mut ::std::os::raw::c_int,
    filterDimA: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetFilterNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            filterDesc,
                            nbDimsRequested,
                            dataType,
                            format,
                            nbDims,
                            filterDimA,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetFilterSizeInBytes(
    filterDesc: cudnnFilterDescriptor_t,
    size: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetFilterSizeInBytes {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(filterDesc, size) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnTransformFilter(
    handle: cudnnHandle_t,
    transDesc: cudnnTensorTransformDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    srcDesc: cudnnFilterDescriptor_t,
    srcData: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    destDesc: cudnnFilterDescriptor_t,
    destData: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnTransformFilter {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transDesc,
                            alpha,
                            srcDesc,
                            srcData,
                            beta,
                            destDesc,
                            destData,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyFilterDescriptor(
    filterDesc: cudnnFilterDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyFilterDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(filterDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSoftmaxForward(
    handle: cudnnHandle_t,
    algo: cudnnSoftmaxAlgorithm_t,
    mode: cudnnSoftmaxMode_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSoftmaxForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, algo, mode, alpha, xDesc, x, beta, yDesc, y)
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreatePoolingDescriptor(
    poolingDesc: *mut cudnnPoolingDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreatePoolingDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(poolingDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetPooling2dDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    windowHeight: ::std::os::raw::c_int,
    windowWidth: ::std::os::raw::c_int,
    verticalPadding: ::std::os::raw::c_int,
    horizontalPadding: ::std::os::raw::c_int,
    verticalStride: ::std::os::raw::c_int,
    horizontalStride: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetPooling2dDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            poolingDesc,
                            mode,
                            maxpoolingNanOpt,
                            windowHeight,
                            windowWidth,
                            verticalPadding,
                            horizontalPadding,
                            verticalStride,
                            horizontalStride,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetPooling2dDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: *mut cudnnPoolingMode_t,
    maxpoolingNanOpt: *mut cudnnNanPropagation_t,
    windowHeight: *mut ::std::os::raw::c_int,
    windowWidth: *mut ::std::os::raw::c_int,
    verticalPadding: *mut ::std::os::raw::c_int,
    horizontalPadding: *mut ::std::os::raw::c_int,
    verticalStride: *mut ::std::os::raw::c_int,
    horizontalStride: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetPooling2dDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            poolingDesc,
                            mode,
                            maxpoolingNanOpt,
                            windowHeight,
                            windowWidth,
                            verticalPadding,
                            horizontalPadding,
                            verticalStride,
                            horizontalStride,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetPoolingNdDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    mode: cudnnPoolingMode_t,
    maxpoolingNanOpt: cudnnNanPropagation_t,
    nbDims: ::std::os::raw::c_int,
    windowDimA: *const ::std::os::raw::c_int,
    paddingA: *const ::std::os::raw::c_int,
    strideA: *const ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetPoolingNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            poolingDesc,
                            mode,
                            maxpoolingNanOpt,
                            nbDims,
                            windowDimA,
                            paddingA,
                            strideA,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetPoolingNdDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
    nbDimsRequested: ::std::os::raw::c_int,
    mode: *mut cudnnPoolingMode_t,
    maxpoolingNanOpt: *mut cudnnNanPropagation_t,
    nbDims: *mut ::std::os::raw::c_int,
    windowDimA: *mut ::std::os::raw::c_int,
    paddingA: *mut ::std::os::raw::c_int,
    strideA: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetPoolingNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            poolingDesc,
                            nbDimsRequested,
                            mode,
                            maxpoolingNanOpt,
                            nbDims,
                            windowDimA,
                            paddingA,
                            strideA,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetPoolingNdForwardOutputDim(
    poolingDesc: cudnnPoolingDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    nbDims: ::std::os::raw::c_int,
    outputTensorDimA: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetPoolingNdForwardOutputDim {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            poolingDesc,
                            inputTensorDesc,
                            nbDims,
                            outputTensorDimA,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetPooling2dForwardOutputDim(
    poolingDesc: cudnnPoolingDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    n: *mut ::std::os::raw::c_int,
    c: *mut ::std::os::raw::c_int,
    h: *mut ::std::os::raw::c_int,
    w: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetPooling2dForwardOutputDim {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(poolingDesc, inputTensorDesc, n, c, h, w) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyPoolingDescriptor(
    poolingDesc: cudnnPoolingDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyPoolingDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(poolingDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnPoolingForward(
    handle: cudnnHandle_t,
    poolingDesc: cudnnPoolingDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnPoolingForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, poolingDesc, alpha, xDesc, x, beta, yDesc, y)
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateActivationDescriptor(
    activationDesc: *mut cudnnActivationDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateActivationDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(activationDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetActivationDescriptor(
    activationDesc: cudnnActivationDescriptor_t,
    mode: cudnnActivationMode_t,
    reluNanOpt: cudnnNanPropagation_t,
    coef: f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetActivationDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(activationDesc, mode, reluNanOpt, coef) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetActivationDescriptor(
    activationDesc: cudnnActivationDescriptor_t,
    mode: *mut cudnnActivationMode_t,
    reluNanOpt: *mut cudnnNanPropagation_t,
    coef: *mut f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetActivationDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(activationDesc, mode, reluNanOpt, coef) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetActivationDescriptorSwishBeta(
    activationDesc: cudnnActivationDescriptor_t,
    swish_beta: f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetActivationDescriptorSwishBeta {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(activationDesc, swish_beta) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetActivationDescriptorSwishBeta(
    activationDesc: cudnnActivationDescriptor_t,
    swish_beta: *mut f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetActivationDescriptorSwishBeta {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(activationDesc, swish_beta) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyActivationDescriptor(
    activationDesc: cudnnActivationDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyActivationDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(activationDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnActivationForward(
    handle: cudnnHandle_t,
    activationDesc: cudnnActivationDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnActivationForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            activationDesc,
                            alpha,
                            xDesc,
                            x,
                            beta,
                            yDesc,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateLRNDescriptor(
    normDesc: *mut cudnnLRNDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateLRNDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(normDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetLRNDescriptor(
    normDesc: cudnnLRNDescriptor_t,
    lrnN: ::std::os::raw::c_uint,
    lrnAlpha: f64,
    lrnBeta: f64,
    lrnK: f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetLRNDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(normDesc, lrnN, lrnAlpha, lrnBeta, lrnK) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetLRNDescriptor(
    normDesc: cudnnLRNDescriptor_t,
    lrnN: *mut ::std::os::raw::c_uint,
    lrnAlpha: *mut f64,
    lrnBeta: *mut f64,
    lrnK: *mut f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetLRNDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(normDesc, lrnN, lrnAlpha, lrnBeta, lrnK) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyLRNDescriptor(
    lrnDesc: cudnnLRNDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyLRNDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(lrnDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnLRNCrossChannelForward(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    lrnMode: cudnnLRNMode_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnLRNCrossChannelForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            normDesc,
                            lrnMode,
                            alpha,
                            xDesc,
                            x,
                            beta,
                            yDesc,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDivisiveNormalizationForward(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    mode: cudnnDivNormMode_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    means: *const ::std::os::raw::c_void,
    temp: *mut ::std::os::raw::c_void,
    temp2: *mut ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDivisiveNormalizationForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            normDesc,
                            mode,
                            alpha,
                            xDesc,
                            x,
                            means,
                            temp,
                            temp2,
                            beta,
                            yDesc,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDeriveBNTensorDescriptor(
    derivedBnDesc: cudnnTensorDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    mode: cudnnBatchNormMode_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDeriveBNTensorDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(derivedBnDesc, xDesc, mode) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBatchNormalizationForwardInference(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alpha: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: *const ::std::os::raw::c_void,
    bnBias: *const ::std::os::raw::c_void,
    estimatedMean: *const ::std::os::raw::c_void,
    estimatedVariance: *const ::std::os::raw::c_void,
    epsilon: f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBatchNormalizationForwardInference {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            alpha,
                            beta,
                            xDesc,
                            x,
                            yDesc,
                            y,
                            bnScaleBiasMeanVarDesc,
                            bnScale,
                            bnBias,
                            estimatedMean,
                            estimatedVariance,
                            epsilon,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDeriveNormTensorDescriptor(
    derivedNormScaleBiasDesc: cudnnTensorDescriptor_t,
    derivedNormMeanVarDesc: cudnnTensorDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    mode: cudnnNormMode_t,
    groupCnt: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDeriveNormTensorDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            derivedNormScaleBiasDesc,
                            derivedNormMeanVarDesc,
                            xDesc,
                            mode,
                            groupCnt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnNormalizationForwardInference(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alpha: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    normScale: *const ::std::os::raw::c_void,
    normBias: *const ::std::os::raw::c_void,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    estimatedMean: *const ::std::os::raw::c_void,
    estimatedVariance: *const ::std::os::raw::c_void,
    zDesc: cudnnTensorDescriptor_t,
    z: *const ::std::os::raw::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
    epsilon: f64,
    groupCnt: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnNormalizationForwardInference {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            normOps,
                            algo,
                            alpha,
                            beta,
                            xDesc,
                            x,
                            normScaleBiasDesc,
                            normScale,
                            normBias,
                            normMeanVarDesc,
                            estimatedMean,
                            estimatedVariance,
                            zDesc,
                            z,
                            activationDesc,
                            yDesc,
                            y,
                            epsilon,
                            groupCnt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateSpatialTransformerDescriptor(
    stDesc: *mut cudnnSpatialTransformerDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateSpatialTransformerDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(stDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetSpatialTransformerNdDescriptor(
    stDesc: cudnnSpatialTransformerDescriptor_t,
    samplerType: cudnnSamplerType_t,
    dataType: cudnnDataType_t,
    nbDims: ::std::os::raw::c_int,
    dimA: *const ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetSpatialTransformerNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(stDesc, samplerType, dataType, nbDims, dimA) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroySpatialTransformerDescriptor(
    stDesc: cudnnSpatialTransformerDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroySpatialTransformerDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(stDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSpatialTfGridGeneratorForward(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    theta: *const ::std::os::raw::c_void,
    grid: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSpatialTfGridGeneratorForward {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, stDesc, theta, grid) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSpatialTfSamplerForward(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    grid: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSpatialTfSamplerForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, stDesc, alpha, xDesc, x, grid, beta, yDesc, y)
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateDropoutDescriptor(
    dropoutDesc: *mut cudnnDropoutDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateDropoutDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dropoutDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyDropoutDescriptor(
    dropoutDesc: cudnnDropoutDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyDropoutDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dropoutDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDropoutGetStatesSize(
    handle: cudnnHandle_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDropoutGetStatesSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, sizeInBytes) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDropoutGetReserveSpaceSize(
    xdesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDropoutGetReserveSpaceSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(xdesc, sizeInBytes) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetDropoutDescriptor(
    dropoutDesc: cudnnDropoutDescriptor_t,
    handle: cudnnHandle_t,
    dropout: f32,
    states: *mut ::std::os::raw::c_void,
    stateSizeInBytes: usize,
    seed: ::std::os::raw::c_ulonglong,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetDropoutDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            dropoutDesc,
                            handle,
                            dropout,
                            states,
                            stateSizeInBytes,
                            seed,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnRestoreDropoutDescriptor(
    dropoutDesc: cudnnDropoutDescriptor_t,
    handle: cudnnHandle_t,
    dropout: f32,
    states: *mut ::std::os::raw::c_void,
    stateSizeInBytes: usize,
    seed: ::std::os::raw::c_ulonglong,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnRestoreDropoutDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            dropoutDesc,
                            handle,
                            dropout,
                            states,
                            stateSizeInBytes,
                            seed,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetDropoutDescriptor(
    dropoutDesc: cudnnDropoutDescriptor_t,
    handle: cudnnHandle_t,
    dropout: *mut f32,
    states: *mut *mut ::std::os::raw::c_void,
    seed: *mut ::std::os::raw::c_ulonglong,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetDropoutDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(dropoutDesc, handle, dropout, states, seed) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDropoutForward(
    handle: cudnnHandle_t,
    dropoutDesc: cudnnDropoutDescriptor_t,
    xdesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    ydesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
    reserveSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDropoutForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dropoutDesc,
                            xdesc,
                            x,
                            ydesc,
                            y,
                            reserveSpace,
                            reserveSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnOpsVersionCheck() -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnOpsVersionCheck {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSoftmaxBackward(
    handle: cudnnHandle_t,
    algo: cudnnSoftmaxAlgorithm_t,
    mode: cudnnSoftmaxMode_t,
    alpha: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSoftmaxBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            mode,
                            alpha,
                            yDesc,
                            y,
                            dyDesc,
                            dy,
                            beta,
                            dxDesc,
                            dx,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnPoolingBackward(
    handle: cudnnHandle_t,
    poolingDesc: cudnnPoolingDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnPoolingBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            poolingDesc,
                            alpha,
                            yDesc,
                            y,
                            dyDesc,
                            dy,
                            xDesc,
                            x,
                            beta,
                            dxDesc,
                            dx,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnActivationBackward(
    handle: cudnnHandle_t,
    activationDesc: cudnnActivationDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnActivationBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            activationDesc,
                            alpha,
                            yDesc,
                            y,
                            dyDesc,
                            dy,
                            xDesc,
                            x,
                            beta,
                            dxDesc,
                            dx,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnLRNCrossChannelBackward(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    lrnMode: cudnnLRNMode_t,
    alpha: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnLRNCrossChannelBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            normDesc,
                            lrnMode,
                            alpha,
                            yDesc,
                            y,
                            dyDesc,
                            dy,
                            xDesc,
                            x,
                            beta,
                            dxDesc,
                            dx,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDivisiveNormalizationBackward(
    handle: cudnnHandle_t,
    normDesc: cudnnLRNDescriptor_t,
    mode: cudnnDivNormMode_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    means: *const ::std::os::raw::c_void,
    dy: *const ::std::os::raw::c_void,
    temp: *mut ::std::os::raw::c_void,
    temp2: *mut ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    dXdMeansDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
    dMeans: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDivisiveNormalizationBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            normDesc,
                            mode,
                            alpha,
                            xDesc,
                            x,
                            means,
                            dy,
                            temp,
                            temp2,
                            beta,
                            dXdMeansDesc,
                            dx,
                            dMeans,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    xDesc: cudnnTensorDescriptor_t,
    zDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            bnOps,
                            xDesc,
                            zDesc,
                            yDesc,
                            bnScaleBiasMeanVarDesc,
                            activationDesc,
                            sizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    xDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    dzDesc: cudnnTensorDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    dBnScaleBiasDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetBatchNormalizationBackwardExWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            bnOps,
                            xDesc,
                            yDesc,
                            dyDesc,
                            dzDesc,
                            dxDesc,
                            dBnScaleBiasDesc,
                            activationDesc,
                            sizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    activationDesc: cudnnActivationDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetBatchNormalizationTrainingExReserveSpaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            bnOps,
                            activationDesc,
                            xDesc,
                            sizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBatchNormalizationForwardTraining(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alpha: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: *const ::std::os::raw::c_void,
    bnBias: *const ::std::os::raw::c_void,
    exponentialAverageFactor: f64,
    resultRunningMean: *mut ::std::os::raw::c_void,
    resultRunningVariance: *mut ::std::os::raw::c_void,
    epsilon: f64,
    resultSaveMean: *mut ::std::os::raw::c_void,
    resultSaveInvVariance: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBatchNormalizationForwardTraining {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            alpha,
                            beta,
                            xDesc,
                            x,
                            yDesc,
                            y,
                            bnScaleBiasMeanVarDesc,
                            bnScale,
                            bnBias,
                            exponentialAverageFactor,
                            resultRunningMean,
                            resultRunningVariance,
                            epsilon,
                            resultSaveMean,
                            resultSaveInvVariance,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBatchNormalizationForwardTrainingEx(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    alpha: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    xData: *const ::std::os::raw::c_void,
    zDesc: cudnnTensorDescriptor_t,
    zData: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    yData: *mut ::std::os::raw::c_void,
    bnScaleBiasMeanVarDesc: cudnnTensorDescriptor_t,
    bnScale: *const ::std::os::raw::c_void,
    bnBias: *const ::std::os::raw::c_void,
    exponentialAverageFactor: f64,
    resultRunningMean: *mut ::std::os::raw::c_void,
    resultRunningVariance: *mut ::std::os::raw::c_void,
    epsilon: f64,
    resultSaveMean: *mut ::std::os::raw::c_void,
    resultSaveInvVariance: *mut ::std::os::raw::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    workspace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBatchNormalizationForwardTrainingEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            bnOps,
                            alpha,
                            beta,
                            xDesc,
                            xData,
                            zDesc,
                            zData,
                            yDesc,
                            yData,
                            bnScaleBiasMeanVarDesc,
                            bnScale,
                            bnBias,
                            exponentialAverageFactor,
                            resultRunningMean,
                            resultRunningVariance,
                            epsilon,
                            resultSaveMean,
                            resultSaveInvVariance,
                            activationDesc,
                            workspace,
                            workSpaceSizeInBytes,
                            reserveSpace,
                            reserveSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBatchNormalizationBackward(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    alphaDataDiff: *const ::std::os::raw::c_void,
    betaDataDiff: *const ::std::os::raw::c_void,
    alphaParamDiff: *const ::std::os::raw::c_void,
    betaParamDiff: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
    dBnScaleBiasDesc: cudnnTensorDescriptor_t,
    bnScale: *const ::std::os::raw::c_void,
    dBnScaleResult: *mut ::std::os::raw::c_void,
    dBnBiasResult: *mut ::std::os::raw::c_void,
    epsilon: f64,
    savedMean: *const ::std::os::raw::c_void,
    savedInvVariance: *const ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBatchNormalizationBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            alphaDataDiff,
                            betaDataDiff,
                            alphaParamDiff,
                            betaParamDiff,
                            xDesc,
                            x,
                            dyDesc,
                            dy,
                            dxDesc,
                            dx,
                            dBnScaleBiasDesc,
                            bnScale,
                            dBnScaleResult,
                            dBnBiasResult,
                            epsilon,
                            savedMean,
                            savedInvVariance,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBatchNormalizationBackwardEx(
    handle: cudnnHandle_t,
    mode: cudnnBatchNormMode_t,
    bnOps: cudnnBatchNormOps_t,
    alphaDataDiff: *const ::std::os::raw::c_void,
    betaDataDiff: *const ::std::os::raw::c_void,
    alphaParamDiff: *const ::std::os::raw::c_void,
    betaParamDiff: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    xData: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    yData: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dyData: *const ::std::os::raw::c_void,
    dzDesc: cudnnTensorDescriptor_t,
    dzData: *mut ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dxData: *mut ::std::os::raw::c_void,
    dBnScaleBiasDesc: cudnnTensorDescriptor_t,
    bnScaleData: *const ::std::os::raw::c_void,
    bnBiasData: *const ::std::os::raw::c_void,
    dBnScaleData: *mut ::std::os::raw::c_void,
    dBnBiasData: *mut ::std::os::raw::c_void,
    epsilon: f64,
    savedMean: *const ::std::os::raw::c_void,
    savedInvVariance: *const ::std::os::raw::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBatchNormalizationBackwardEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            bnOps,
                            alphaDataDiff,
                            betaDataDiff,
                            alphaParamDiff,
                            betaParamDiff,
                            xDesc,
                            xData,
                            yDesc,
                            yData,
                            dyDesc,
                            dyData,
                            dzDesc,
                            dzData,
                            dxDesc,
                            dxData,
                            dBnScaleBiasDesc,
                            bnScaleData,
                            bnBiasData,
                            dBnScaleData,
                            dBnBiasData,
                            epsilon,
                            savedMean,
                            savedInvVariance,
                            activationDesc,
                            workSpace,
                            workSpaceSizeInBytes,
                            reserveSpace,
                            reserveSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetNormalizationForwardTrainingWorkspaceSize(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    xDesc: cudnnTensorDescriptor_t,
    zDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
    groupCnt: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetNormalizationForwardTrainingWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            normOps,
                            algo,
                            xDesc,
                            zDesc,
                            yDesc,
                            normScaleBiasDesc,
                            activationDesc,
                            normMeanVarDesc,
                            sizeInBytes,
                            groupCnt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetNormalizationBackwardWorkspaceSize(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    xDesc: cudnnTensorDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    dzDesc: cudnnTensorDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    dNormScaleBiasDesc: cudnnTensorDescriptor_t,
    activationDesc: cudnnActivationDescriptor_t,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
    groupCnt: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetNormalizationBackwardWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            normOps,
                            algo,
                            xDesc,
                            yDesc,
                            dyDesc,
                            dzDesc,
                            dxDesc,
                            dNormScaleBiasDesc,
                            activationDesc,
                            normMeanVarDesc,
                            sizeInBytes,
                            groupCnt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetNormalizationTrainingReserveSpaceSize(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    activationDesc: cudnnActivationDescriptor_t,
    xDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
    groupCnt: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetNormalizationTrainingReserveSpaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            normOps,
                            algo,
                            activationDesc,
                            xDesc,
                            sizeInBytes,
                            groupCnt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnNormalizationForwardTraining(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alpha: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    xData: *const ::std::os::raw::c_void,
    normScaleBiasDesc: cudnnTensorDescriptor_t,
    normScale: *const ::std::os::raw::c_void,
    normBias: *const ::std::os::raw::c_void,
    exponentialAverageFactor: f64,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    resultRunningMean: *mut ::std::os::raw::c_void,
    resultRunningVariance: *mut ::std::os::raw::c_void,
    epsilon: f64,
    resultSaveMean: *mut ::std::os::raw::c_void,
    resultSaveInvVariance: *mut ::std::os::raw::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    zDesc: cudnnTensorDescriptor_t,
    zData: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    yData: *mut ::std::os::raw::c_void,
    workspace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
    groupCnt: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnNormalizationForwardTraining {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            normOps,
                            algo,
                            alpha,
                            beta,
                            xDesc,
                            xData,
                            normScaleBiasDesc,
                            normScale,
                            normBias,
                            exponentialAverageFactor,
                            normMeanVarDesc,
                            resultRunningMean,
                            resultRunningVariance,
                            epsilon,
                            resultSaveMean,
                            resultSaveInvVariance,
                            activationDesc,
                            zDesc,
                            zData,
                            yDesc,
                            yData,
                            workspace,
                            workSpaceSizeInBytes,
                            reserveSpace,
                            reserveSpaceSizeInBytes,
                            groupCnt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnNormalizationBackward(
    handle: cudnnHandle_t,
    mode: cudnnNormMode_t,
    normOps: cudnnNormOps_t,
    algo: cudnnNormAlgo_t,
    alphaDataDiff: *const ::std::os::raw::c_void,
    betaDataDiff: *const ::std::os::raw::c_void,
    alphaParamDiff: *const ::std::os::raw::c_void,
    betaParamDiff: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    xData: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    yData: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dyData: *const ::std::os::raw::c_void,
    dzDesc: cudnnTensorDescriptor_t,
    dzData: *mut ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dxData: *mut ::std::os::raw::c_void,
    dNormScaleBiasDesc: cudnnTensorDescriptor_t,
    normScaleData: *const ::std::os::raw::c_void,
    normBiasData: *const ::std::os::raw::c_void,
    dNormScaleData: *mut ::std::os::raw::c_void,
    dNormBiasData: *mut ::std::os::raw::c_void,
    epsilon: f64,
    normMeanVarDesc: cudnnTensorDescriptor_t,
    savedMean: *const ::std::os::raw::c_void,
    savedInvVariance: *const ::std::os::raw::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
    groupCnt: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnNormalizationBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mode,
                            normOps,
                            algo,
                            alphaDataDiff,
                            betaDataDiff,
                            alphaParamDiff,
                            betaParamDiff,
                            xDesc,
                            xData,
                            yDesc,
                            yData,
                            dyDesc,
                            dyData,
                            dzDesc,
                            dzData,
                            dxDesc,
                            dxData,
                            dNormScaleBiasDesc,
                            normScaleData,
                            normBiasData,
                            dNormScaleData,
                            dNormBiasData,
                            epsilon,
                            normMeanVarDesc,
                            savedMean,
                            savedInvVariance,
                            activationDesc,
                            workSpace,
                            workSpaceSizeInBytes,
                            reserveSpace,
                            reserveSpaceSizeInBytes,
                            groupCnt,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSpatialTfGridGeneratorBackward(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    dgrid: *const ::std::os::raw::c_void,
    dtheta: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSpatialTfGridGeneratorBackward {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, stDesc, dgrid, dtheta) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSpatialTfSamplerBackward(
    handle: cudnnHandle_t,
    stDesc: cudnnSpatialTransformerDescriptor_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
    alphaDgrid: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    grid: *const ::std::os::raw::c_void,
    betaDgrid: *const ::std::os::raw::c_void,
    dgrid: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSpatialTfSamplerBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            stDesc,
                            alpha,
                            xDesc,
                            x,
                            beta,
                            dxDesc,
                            dx,
                            alphaDgrid,
                            dyDesc,
                            dy,
                            grid,
                            betaDgrid,
                            dgrid,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDropoutBackward(
    handle: cudnnHandle_t,
    dropoutDesc: cudnnDropoutDescriptor_t,
    dydesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    dxdesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
    reserveSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDropoutBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dropoutDesc,
                            dydesc,
                            dy,
                            dxdesc,
                            dx,
                            reserveSpace,
                            reserveSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateRNNDescriptor(
    rnnDesc: *mut cudnnRNNDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateRNNDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(rnnDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyRNNDescriptor(
    rnnDesc: cudnnRNNDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyRNNDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(rnnDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetRNNDescriptor_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    algo: cudnnRNNAlgo_t,
    cellMode: cudnnRNNMode_t,
    biasMode: cudnnRNNBiasMode_t,
    dirMode: cudnnDirectionMode_t,
    inputMode: cudnnRNNInputMode_t,
    dataType: cudnnDataType_t,
    mathPrec: cudnnDataType_t,
    mathType: cudnnMathType_t,
    inputSize: i32,
    hiddenSize: i32,
    projSize: i32,
    numLayers: i32,
    dropoutDesc: cudnnDropoutDescriptor_t,
    auxFlags: u32,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetRNNDescriptor_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            rnnDesc,
                            algo,
                            cellMode,
                            biasMode,
                            dirMode,
                            inputMode,
                            dataType,
                            mathPrec,
                            mathType,
                            inputSize,
                            hiddenSize,
                            projSize,
                            numLayers,
                            dropoutDesc,
                            auxFlags,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetRNNDescriptor_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    algo: *mut cudnnRNNAlgo_t,
    cellMode: *mut cudnnRNNMode_t,
    biasMode: *mut cudnnRNNBiasMode_t,
    dirMode: *mut cudnnDirectionMode_t,
    inputMode: *mut cudnnRNNInputMode_t,
    dataType: *mut cudnnDataType_t,
    mathPrec: *mut cudnnDataType_t,
    mathType: *mut cudnnMathType_t,
    inputSize: *mut i32,
    hiddenSize: *mut i32,
    projSize: *mut i32,
    numLayers: *mut i32,
    dropoutDesc: *mut cudnnDropoutDescriptor_t,
    auxFlags: *mut u32,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetRNNDescriptor_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            rnnDesc,
                            algo,
                            cellMode,
                            biasMode,
                            dirMode,
                            inputMode,
                            dataType,
                            mathPrec,
                            mathType,
                            inputSize,
                            hiddenSize,
                            projSize,
                            numLayers,
                            dropoutDesc,
                            auxFlags,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnRNNSetClip_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    clipMode: cudnnRNNClipMode_t,
    clipNanOpt: cudnnNanPropagation_t,
    lclip: f64,
    rclip: f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnRNNSetClip_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rnnDesc, clipMode, clipNanOpt, lclip, rclip) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnRNNSetClip_v9(
    rnnDesc: cudnnRNNDescriptor_t,
    clipMode: cudnnRNNClipMode_t,
    lclip: f64,
    rclip: f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnRNNSetClip_v9 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(rnnDesc, clipMode, lclip, rclip) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnRNNGetClip_v8(
    rnnDesc: cudnnRNNDescriptor_t,
    clipMode: *mut cudnnRNNClipMode_t,
    clipNanOpt: *mut cudnnNanPropagation_t,
    lclip: *mut f64,
    rclip: *mut f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnRNNGetClip_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rnnDesc, clipMode, clipNanOpt, lclip, rclip) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnRNNGetClip_v9(
    rnnDesc: cudnnRNNDescriptor_t,
    clipMode: *mut cudnnRNNClipMode_t,
    lclip: *mut f64,
    rclip: *mut f64,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnRNNGetClip_v9 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(rnnDesc, clipMode, lclip, rclip) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnBuildRNNDynamic(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    miniBatch: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnBuildRNNDynamic {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, rnnDesc, miniBatch) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetRNNTempSpaceSizes(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    fwdMode: cudnnForwardMode_t,
    xDesc: cudnnRNNDataDescriptor_t,
    workSpaceSize: *mut usize,
    reserveSpaceSize: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetRNNTempSpaceSizes {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            rnnDesc,
                            fwdMode,
                            xDesc,
                            workSpaceSize,
                            reserveSpaceSize,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetRNNWeightSpaceSize(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    weightSpaceSize: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetRNNWeightSpaceSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, rnnDesc, weightSpaceSize) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetRNNWeightParams(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    pseudoLayer: i32,
    weightSpaceSize: usize,
    weightSpace: *const ::std::os::raw::c_void,
    linLayerID: i32,
    mDesc: cudnnTensorDescriptor_t,
    mAddr: *mut *mut ::std::os::raw::c_void,
    bDesc: cudnnTensorDescriptor_t,
    bAddr: *mut *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetRNNWeightParams {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            rnnDesc,
                            pseudoLayer,
                            weightSpaceSize,
                            weightSpace,
                            linLayerID,
                            mDesc,
                            mAddr,
                            bDesc,
                            bAddr,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateRNNDataDescriptor(
    rnnDataDesc: *mut cudnnRNNDataDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateRNNDataDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(rnnDataDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyRNNDataDescriptor(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyRNNDataDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(rnnDataDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetRNNDataDescriptor(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    dataType: cudnnDataType_t,
    layout: cudnnRNNDataLayout_t,
    maxSeqLength: ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    vectorSize: ::std::os::raw::c_int,
    seqLengthArray: *const ::std::os::raw::c_int,
    paddingFill: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetRNNDataDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            rnnDataDesc,
                            dataType,
                            layout,
                            maxSeqLength,
                            batchSize,
                            vectorSize,
                            seqLengthArray,
                            paddingFill,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetRNNDataDescriptor(
    rnnDataDesc: cudnnRNNDataDescriptor_t,
    dataType: *mut cudnnDataType_t,
    layout: *mut cudnnRNNDataLayout_t,
    maxSeqLength: *mut ::std::os::raw::c_int,
    batchSize: *mut ::std::os::raw::c_int,
    vectorSize: *mut ::std::os::raw::c_int,
    arrayLengthRequested: ::std::os::raw::c_int,
    seqLengthArray: *mut ::std::os::raw::c_int,
    paddingFill: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetRNNDataDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            rnnDataDesc,
                            dataType,
                            layout,
                            maxSeqLength,
                            batchSize,
                            vectorSize,
                            arrayLengthRequested,
                            seqLengthArray,
                            paddingFill,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnRNNForward(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    fwdMode: cudnnForwardMode_t,
    devSeqLengths: *const i32,
    xDesc: cudnnRNNDataDescriptor_t,
    x: *const ::std::os::raw::c_void,
    yDesc: cudnnRNNDataDescriptor_t,
    y: *mut ::std::os::raw::c_void,
    hDesc: cudnnTensorDescriptor_t,
    hx: *const ::std::os::raw::c_void,
    hy: *mut ::std::os::raw::c_void,
    cDesc: cudnnTensorDescriptor_t,
    cx: *const ::std::os::raw::c_void,
    cy: *mut ::std::os::raw::c_void,
    weightSpaceSize: usize,
    weightSpace: *const ::std::os::raw::c_void,
    workSpaceSize: usize,
    workSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSize: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnRNNForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            rnnDesc,
                            fwdMode,
                            devSeqLengths,
                            xDesc,
                            x,
                            yDesc,
                            y,
                            hDesc,
                            hx,
                            hy,
                            cDesc,
                            cx,
                            cy,
                            weightSpaceSize,
                            weightSpace,
                            workSpaceSize,
                            workSpace,
                            reserveSpaceSize,
                            reserveSpace,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateSeqDataDescriptor(
    seqDataDesc: *mut cudnnSeqDataDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateSeqDataDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(seqDataDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroySeqDataDescriptor(
    seqDataDesc: cudnnSeqDataDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroySeqDataDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(seqDataDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetSeqDataDescriptor(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    dataType: cudnnDataType_t,
    nbDims: ::std::os::raw::c_int,
    dimA: *const ::std::os::raw::c_int,
    axes: *const cudnnSeqDataAxis_t,
    seqLengthArraySize: usize,
    seqLengthArray: *const ::std::os::raw::c_int,
    paddingFill: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetSeqDataDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            seqDataDesc,
                            dataType,
                            nbDims,
                            dimA,
                            axes,
                            seqLengthArraySize,
                            seqLengthArray,
                            paddingFill,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetSeqDataDescriptor(
    seqDataDesc: cudnnSeqDataDescriptor_t,
    dataType: *mut cudnnDataType_t,
    nbDims: *mut ::std::os::raw::c_int,
    nbDimsRequested: ::std::os::raw::c_int,
    dimA: *mut ::std::os::raw::c_int,
    axes: *mut cudnnSeqDataAxis_t,
    seqLengthArraySize: *mut usize,
    seqLengthSizeRequested: usize,
    seqLengthArray: *mut ::std::os::raw::c_int,
    paddingFill: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetSeqDataDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            seqDataDesc,
                            dataType,
                            nbDims,
                            nbDimsRequested,
                            dimA,
                            axes,
                            seqLengthArraySize,
                            seqLengthSizeRequested,
                            seqLengthArray,
                            paddingFill,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateAttnDescriptor(
    attnDesc: *mut cudnnAttnDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateAttnDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(attnDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyAttnDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(attnDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
    attnMode: ::std::os::raw::c_uint,
    nHeads: ::std::os::raw::c_int,
    smScaler: f64,
    dataType: cudnnDataType_t,
    computePrec: cudnnDataType_t,
    mathType: cudnnMathType_t,
    attnDropoutDesc: cudnnDropoutDescriptor_t,
    postDropoutDesc: cudnnDropoutDescriptor_t,
    qSize: ::std::os::raw::c_int,
    kSize: ::std::os::raw::c_int,
    vSize: ::std::os::raw::c_int,
    qProjSize: ::std::os::raw::c_int,
    kProjSize: ::std::os::raw::c_int,
    vProjSize: ::std::os::raw::c_int,
    oProjSize: ::std::os::raw::c_int,
    qoMaxSeqLength: ::std::os::raw::c_int,
    kvMaxSeqLength: ::std::os::raw::c_int,
    maxBatchSize: ::std::os::raw::c_int,
    maxBeamSize: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetAttnDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            attnDesc,
                            attnMode,
                            nHeads,
                            smScaler,
                            dataType,
                            computePrec,
                            mathType,
                            attnDropoutDesc,
                            postDropoutDesc,
                            qSize,
                            kSize,
                            vSize,
                            qProjSize,
                            kProjSize,
                            vProjSize,
                            oProjSize,
                            qoMaxSeqLength,
                            kvMaxSeqLength,
                            maxBatchSize,
                            maxBeamSize,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetAttnDescriptor(
    attnDesc: cudnnAttnDescriptor_t,
    attnMode: *mut ::std::os::raw::c_uint,
    nHeads: *mut ::std::os::raw::c_int,
    smScaler: *mut f64,
    dataType: *mut cudnnDataType_t,
    computePrec: *mut cudnnDataType_t,
    mathType: *mut cudnnMathType_t,
    attnDropoutDesc: *mut cudnnDropoutDescriptor_t,
    postDropoutDesc: *mut cudnnDropoutDescriptor_t,
    qSize: *mut ::std::os::raw::c_int,
    kSize: *mut ::std::os::raw::c_int,
    vSize: *mut ::std::os::raw::c_int,
    qProjSize: *mut ::std::os::raw::c_int,
    kProjSize: *mut ::std::os::raw::c_int,
    vProjSize: *mut ::std::os::raw::c_int,
    oProjSize: *mut ::std::os::raw::c_int,
    qoMaxSeqLength: *mut ::std::os::raw::c_int,
    kvMaxSeqLength: *mut ::std::os::raw::c_int,
    maxBatchSize: *mut ::std::os::raw::c_int,
    maxBeamSize: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetAttnDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            attnDesc,
                            attnMode,
                            nHeads,
                            smScaler,
                            dataType,
                            computePrec,
                            mathType,
                            attnDropoutDesc,
                            postDropoutDesc,
                            qSize,
                            kSize,
                            vSize,
                            qProjSize,
                            kProjSize,
                            vProjSize,
                            oProjSize,
                            qoMaxSeqLength,
                            kvMaxSeqLength,
                            maxBatchSize,
                            maxBeamSize,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetMultiHeadAttnBuffers(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    weightSizeInBytes: *mut usize,
    workSpaceSizeInBytes: *mut usize,
    reserveSpaceSizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetMultiHeadAttnBuffers {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            attnDesc,
                            weightSizeInBytes,
                            workSpaceSizeInBytes,
                            reserveSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetMultiHeadAttnWeights(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    wKind: cudnnMultiHeadAttnWeightKind_t,
    weightSizeInBytes: usize,
    weights: *const ::std::os::raw::c_void,
    wDesc: cudnnTensorDescriptor_t,
    wAddr: *mut *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetMultiHeadAttnWeights {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            attnDesc,
                            wKind,
                            weightSizeInBytes,
                            weights,
                            wDesc,
                            wAddr,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnMultiHeadAttnForward(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    currIdx: ::std::os::raw::c_int,
    loWinIdx: *const ::std::os::raw::c_int,
    hiWinIdx: *const ::std::os::raw::c_int,
    devSeqLengthsQO: *const ::std::os::raw::c_int,
    devSeqLengthsKV: *const ::std::os::raw::c_int,
    qDesc: cudnnSeqDataDescriptor_t,
    queries: *const ::std::os::raw::c_void,
    residuals: *const ::std::os::raw::c_void,
    kDesc: cudnnSeqDataDescriptor_t,
    keys: *const ::std::os::raw::c_void,
    vDesc: cudnnSeqDataDescriptor_t,
    values: *const ::std::os::raw::c_void,
    oDesc: cudnnSeqDataDescriptor_t,
    out: *mut ::std::os::raw::c_void,
    weightSizeInBytes: usize,
    weights: *const ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    workSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnMultiHeadAttnForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            attnDesc,
                            currIdx,
                            loWinIdx,
                            hiWinIdx,
                            devSeqLengthsQO,
                            devSeqLengthsKV,
                            qDesc,
                            queries,
                            residuals,
                            kDesc,
                            keys,
                            vDesc,
                            values,
                            oDesc,
                            out,
                            weightSizeInBytes,
                            weights,
                            workSpaceSizeInBytes,
                            workSpace,
                            reserveSpaceSizeInBytes,
                            reserveSpace,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnAdvVersionCheck() -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnAdvVersionCheck {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnRNNBackwardData_v8(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    devSeqLengths: *const i32,
    yDesc: cudnnRNNDataDescriptor_t,
    y: *const ::std::os::raw::c_void,
    dy: *const ::std::os::raw::c_void,
    xDesc: cudnnRNNDataDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
    hDesc: cudnnTensorDescriptor_t,
    hx: *const ::std::os::raw::c_void,
    dhy: *const ::std::os::raw::c_void,
    dhx: *mut ::std::os::raw::c_void,
    cDesc: cudnnTensorDescriptor_t,
    cx: *const ::std::os::raw::c_void,
    dcy: *const ::std::os::raw::c_void,
    dcx: *mut ::std::os::raw::c_void,
    weightSpaceSize: usize,
    weightSpace: *const ::std::os::raw::c_void,
    workSpaceSize: usize,
    workSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSize: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnRNNBackwardData_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            rnnDesc,
                            devSeqLengths,
                            yDesc,
                            y,
                            dy,
                            xDesc,
                            dx,
                            hDesc,
                            hx,
                            dhy,
                            dhx,
                            cDesc,
                            cx,
                            dcy,
                            dcx,
                            weightSpaceSize,
                            weightSpace,
                            workSpaceSize,
                            workSpace,
                            reserveSpaceSize,
                            reserveSpace,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnRNNBackwardWeights_v8(
    handle: cudnnHandle_t,
    rnnDesc: cudnnRNNDescriptor_t,
    addGrad: cudnnWgradMode_t,
    devSeqLengths: *const i32,
    xDesc: cudnnRNNDataDescriptor_t,
    x: *const ::std::os::raw::c_void,
    hDesc: cudnnTensorDescriptor_t,
    hx: *const ::std::os::raw::c_void,
    yDesc: cudnnRNNDataDescriptor_t,
    y: *const ::std::os::raw::c_void,
    weightSpaceSize: usize,
    dweightSpace: *mut ::std::os::raw::c_void,
    workSpaceSize: usize,
    workSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSize: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnRNNBackwardWeights_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            rnnDesc,
                            addGrad,
                            devSeqLengths,
                            xDesc,
                            x,
                            hDesc,
                            hx,
                            yDesc,
                            y,
                            weightSpaceSize,
                            dweightSpace,
                            workSpaceSize,
                            workSpace,
                            reserveSpaceSize,
                            reserveSpace,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnMultiHeadAttnBackwardData(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    loWinIdx: *const ::std::os::raw::c_int,
    hiWinIdx: *const ::std::os::raw::c_int,
    devSeqLengthsDQDO: *const ::std::os::raw::c_int,
    devSeqLengthsDKDV: *const ::std::os::raw::c_int,
    doDesc: cudnnSeqDataDescriptor_t,
    dout: *const ::std::os::raw::c_void,
    dqDesc: cudnnSeqDataDescriptor_t,
    dqueries: *mut ::std::os::raw::c_void,
    queries: *const ::std::os::raw::c_void,
    dkDesc: cudnnSeqDataDescriptor_t,
    dkeys: *mut ::std::os::raw::c_void,
    keys: *const ::std::os::raw::c_void,
    dvDesc: cudnnSeqDataDescriptor_t,
    dvalues: *mut ::std::os::raw::c_void,
    values: *const ::std::os::raw::c_void,
    weightSizeInBytes: usize,
    weights: *const ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    workSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnMultiHeadAttnBackwardData {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            attnDesc,
                            loWinIdx,
                            hiWinIdx,
                            devSeqLengthsDQDO,
                            devSeqLengthsDKDV,
                            doDesc,
                            dout,
                            dqDesc,
                            dqueries,
                            queries,
                            dkDesc,
                            dkeys,
                            keys,
                            dvDesc,
                            dvalues,
                            values,
                            weightSizeInBytes,
                            weights,
                            workSpaceSizeInBytes,
                            workSpace,
                            reserveSpaceSizeInBytes,
                            reserveSpace,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnMultiHeadAttnBackwardWeights(
    handle: cudnnHandle_t,
    attnDesc: cudnnAttnDescriptor_t,
    addGrad: cudnnWgradMode_t,
    qDesc: cudnnSeqDataDescriptor_t,
    queries: *const ::std::os::raw::c_void,
    kDesc: cudnnSeqDataDescriptor_t,
    keys: *const ::std::os::raw::c_void,
    vDesc: cudnnSeqDataDescriptor_t,
    values: *const ::std::os::raw::c_void,
    doDesc: cudnnSeqDataDescriptor_t,
    dout: *const ::std::os::raw::c_void,
    weightSizeInBytes: usize,
    weights: *const ::std::os::raw::c_void,
    dweights: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    workSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnMultiHeadAttnBackwardWeights {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            attnDesc,
                            addGrad,
                            qDesc,
                            queries,
                            kDesc,
                            keys,
                            vDesc,
                            values,
                            doDesc,
                            dout,
                            weightSizeInBytes,
                            weights,
                            dweights,
                            workSpaceSizeInBytes,
                            workSpace,
                            reserveSpaceSizeInBytes,
                            reserveSpace,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateCTCLossDescriptor(
    ctcLossDesc: *mut cudnnCTCLossDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateCTCLossDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(ctcLossDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetCTCLossDescriptor(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetCTCLossDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(ctcLossDesc, compType) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetCTCLossDescriptorEx(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: cudnnDataType_t,
    normMode: cudnnLossNormalizationMode_t,
    gradMode: cudnnNanPropagation_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetCTCLossDescriptorEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(ctcLossDesc, compType, normMode, gradMode) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetCTCLossDescriptor_v8(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: cudnnDataType_t,
    normMode: cudnnLossNormalizationMode_t,
    gradMode: cudnnNanPropagation_t,
    maxLabelLength: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetCTCLossDescriptor_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            ctcLossDesc,
                            compType,
                            normMode,
                            gradMode,
                            maxLabelLength,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetCTCLossDescriptor_v9(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: cudnnDataType_t,
    normMode: cudnnLossNormalizationMode_t,
    ctcGradMode: cudnnCTCGradMode_t,
    maxLabelLength: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetCTCLossDescriptor_v9 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            ctcLossDesc,
                            compType,
                            normMode,
                            ctcGradMode,
                            maxLabelLength,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetCTCLossDescriptor(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: *mut cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetCTCLossDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(ctcLossDesc, compType) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetCTCLossDescriptorEx(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: *mut cudnnDataType_t,
    normMode: *mut cudnnLossNormalizationMode_t,
    gradMode: *mut cudnnNanPropagation_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetCTCLossDescriptorEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(ctcLossDesc, compType, normMode, gradMode) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetCTCLossDescriptor_v8(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: *mut cudnnDataType_t,
    normMode: *mut cudnnLossNormalizationMode_t,
    gradMode: *mut cudnnNanPropagation_t,
    maxLabelLength: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetCTCLossDescriptor_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            ctcLossDesc,
                            compType,
                            normMode,
                            gradMode,
                            maxLabelLength,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetCTCLossDescriptor_v9(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    compType: *mut cudnnDataType_t,
    normMode: *mut cudnnLossNormalizationMode_t,
    ctcGradMode: *mut cudnnCTCGradMode_t,
    maxLabelLength: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetCTCLossDescriptor_v9 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            ctcLossDesc,
                            compType,
                            normMode,
                            ctcGradMode,
                            maxLabelLength,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyCTCLossDescriptor(
    ctcLossDesc: cudnnCTCLossDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyCTCLossDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(ctcLossDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCTCLoss(
    handle: cudnnHandle_t,
    probsDesc: cudnnTensorDescriptor_t,
    probs: *const ::std::os::raw::c_void,
    hostLabels: *const ::std::os::raw::c_int,
    hostLabelLengths: *const ::std::os::raw::c_int,
    hostInputLengths: *const ::std::os::raw::c_int,
    costs: *mut ::std::os::raw::c_void,
    gradientsDesc: cudnnTensorDescriptor_t,
    gradients: *mut ::std::os::raw::c_void,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    workspace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCTCLoss {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            probsDesc,
                            probs,
                            hostLabels,
                            hostLabelLengths,
                            hostInputLengths,
                            costs,
                            gradientsDesc,
                            gradients,
                            algo,
                            ctcLossDesc,
                            workspace,
                            workSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCTCLoss_v8(
    handle: cudnnHandle_t,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    probsDesc: cudnnTensorDescriptor_t,
    probs: *const ::std::os::raw::c_void,
    labels: *const ::std::os::raw::c_int,
    labelLengths: *const ::std::os::raw::c_int,
    inputLengths: *const ::std::os::raw::c_int,
    costs: *mut ::std::os::raw::c_void,
    gradientsDesc: cudnnTensorDescriptor_t,
    gradients: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    workspace: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCTCLoss_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            ctcLossDesc,
                            probsDesc,
                            probs,
                            labels,
                            labelLengths,
                            inputLengths,
                            costs,
                            gradientsDesc,
                            gradients,
                            workSpaceSizeInBytes,
                            workspace,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetCTCLossWorkspaceSize(
    handle: cudnnHandle_t,
    probsDesc: cudnnTensorDescriptor_t,
    gradientsDesc: cudnnTensorDescriptor_t,
    labels: *const ::std::os::raw::c_int,
    labelLengths: *const ::std::os::raw::c_int,
    inputLengths: *const ::std::os::raw::c_int,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetCTCLossWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            probsDesc,
                            gradientsDesc,
                            labels,
                            labelLengths,
                            inputLengths,
                            algo,
                            ctcLossDesc,
                            sizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetCTCLossWorkspaceSize_v8(
    handle: cudnnHandle_t,
    algo: cudnnCTCLossAlgo_t,
    ctcLossDesc: cudnnCTCLossDescriptor_t,
    probsDesc: cudnnTensorDescriptor_t,
    gradientsDesc: cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetCTCLossWorkspaceSize_v8 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            ctcLossDesc,
                            probsDesc,
                            gradientsDesc,
                            sizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateConvolutionDescriptor(
    convDesc: *mut cudnnConvolutionDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateConvolutionDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(convDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyConvolutionDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyConvolutionDescriptor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(convDesc) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetConvolutionMathType(
    convDesc: cudnnConvolutionDescriptor_t,
    mathType: cudnnMathType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetConvolutionMathType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(convDesc, mathType) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionMathType(
    convDesc: cudnnConvolutionDescriptor_t,
    mathType: *mut cudnnMathType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionMathType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(convDesc, mathType) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetConvolutionGroupCount(
    convDesc: cudnnConvolutionDescriptor_t,
    groupCount: ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetConvolutionGroupCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(convDesc, groupCount) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionGroupCount(
    convDesc: cudnnConvolutionDescriptor_t,
    groupCount: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionGroupCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(convDesc, groupCount) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetConvolutionReorderType(
    convDesc: cudnnConvolutionDescriptor_t,
    reorderType: cudnnReorderType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetConvolutionReorderType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(convDesc, reorderType) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionReorderType(
    convDesc: cudnnConvolutionDescriptor_t,
    reorderType: *mut cudnnReorderType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionReorderType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(convDesc, reorderType) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetConvolution2dDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    pad_h: ::std::os::raw::c_int,
    pad_w: ::std::os::raw::c_int,
    u: ::std::os::raw::c_int,
    v: ::std::os::raw::c_int,
    dilation_h: ::std::os::raw::c_int,
    dilation_w: ::std::os::raw::c_int,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetConvolution2dDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            convDesc,
                            pad_h,
                            pad_w,
                            u,
                            v,
                            dilation_h,
                            dilation_w,
                            mode,
                            computeType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolution2dDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    pad_h: *mut ::std::os::raw::c_int,
    pad_w: *mut ::std::os::raw::c_int,
    u: *mut ::std::os::raw::c_int,
    v: *mut ::std::os::raw::c_int,
    dilation_h: *mut ::std::os::raw::c_int,
    dilation_w: *mut ::std::os::raw::c_int,
    mode: *mut cudnnConvolutionMode_t,
    computeType: *mut cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolution2dDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            convDesc,
                            pad_h,
                            pad_w,
                            u,
                            v,
                            dilation_h,
                            dilation_w,
                            mode,
                            computeType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetConvolutionNdDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    arrayLength: ::std::os::raw::c_int,
    padA: *const ::std::os::raw::c_int,
    filterStrideA: *const ::std::os::raw::c_int,
    dilationA: *const ::std::os::raw::c_int,
    mode: cudnnConvolutionMode_t,
    computeType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetConvolutionNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            convDesc,
                            arrayLength,
                            padA,
                            filterStrideA,
                            dilationA,
                            mode,
                            computeType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionNdDescriptor(
    convDesc: cudnnConvolutionDescriptor_t,
    arrayLengthRequested: ::std::os::raw::c_int,
    arrayLength: *mut ::std::os::raw::c_int,
    padA: *mut ::std::os::raw::c_int,
    strideA: *mut ::std::os::raw::c_int,
    dilationA: *mut ::std::os::raw::c_int,
    mode: *mut cudnnConvolutionMode_t,
    computeType: *mut cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionNdDescriptor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            convDesc,
                            arrayLengthRequested,
                            arrayLength,
                            padA,
                            strideA,
                            dilationA,
                            mode,
                            computeType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolution2dForwardOutputDim(
    convDesc: cudnnConvolutionDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    n: *mut ::std::os::raw::c_int,
    c: *mut ::std::os::raw::c_int,
    h: *mut ::std::os::raw::c_int,
    w: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolution2dForwardOutputDim {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(convDesc, inputTensorDesc, filterDesc, n, c, h, w)
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionNdForwardOutputDim(
    convDesc: cudnnConvolutionDescriptor_t,
    inputTensorDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    nbDims: ::std::os::raw::c_int,
    tensorOuputDimA: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionNdForwardOutputDim {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            convDesc,
                            inputTensorDesc,
                            filterDesc,
                            nbDims,
                            tensorOuputDimA,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionForwardAlgorithmMaxCount(
    handle: cudnnHandle_t,
    count: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionForwardAlgorithmMaxCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, count) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionForwardAlgorithm_v7(
    handle: cudnnHandle_t,
    srcDesc: cudnnTensorDescriptor_t,
    filterDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    destDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionForwardAlgorithm_v7 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            srcDesc,
                            filterDesc,
                            convDesc,
                            destDesc,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFindConvolutionForwardAlgorithm(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFindConvolutionForwardAlgorithm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            xDesc,
                            wDesc,
                            convDesc,
                            yDesc,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFindConvolutionForwardAlgorithmEx(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::std::os::raw::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionFwdAlgoPerf_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFindConvolutionForwardAlgorithmEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            xDesc,
                            x,
                            wDesc,
                            w,
                            convDesc,
                            yDesc,
                            y,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                            workSpace,
                            workSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnIm2Col(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    colBuffer: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnIm2Col {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, xDesc, x, wDesc, convDesc, colBuffer) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnReorderFilterAndBias(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    reorderType: cudnnReorderType_t,
    filterData: *const ::std::os::raw::c_void,
    reorderedFilterData: *mut ::std::os::raw::c_void,
    reorderBias: ::std::os::raw::c_int,
    biasData: *const ::std::os::raw::c_void,
    reorderedBiasData: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnReorderFilterAndBias {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            filterDesc,
                            reorderType,
                            filterData,
                            reorderedFilterData,
                            reorderBias,
                            biasData,
                            reorderedBiasData,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionForwardWorkspaceSize(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    wDesc: cudnnFilterDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    algo: cudnnConvolutionFwdAlgo_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionForwardWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            xDesc,
                            wDesc,
                            convDesc,
                            yDesc,
                            algo,
                            sizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnConvolutionForward(
    handle: cudnnHandle_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::std::os::raw::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionFwdAlgo_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::std::os::raw::c_void,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnConvolutionForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            alpha,
                            xDesc,
                            x,
                            wDesc,
                            w,
                            convDesc,
                            algo,
                            workSpace,
                            workSpaceSizeInBytes,
                            beta,
                            yDesc,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnConvolutionBiasActivationForward(
    handle: cudnnHandle_t,
    alpha1: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::std::os::raw::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionFwdAlgo_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    alpha2: *const ::std::os::raw::c_void,
    zDesc: cudnnTensorDescriptor_t,
    z: *const ::std::os::raw::c_void,
    biasDesc: cudnnTensorDescriptor_t,
    bias: *const ::std::os::raw::c_void,
    activationDesc: cudnnActivationDescriptor_t,
    yDesc: cudnnTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnConvolutionBiasActivationForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            alpha1,
                            xDesc,
                            x,
                            wDesc,
                            w,
                            convDesc,
                            algo,
                            workSpace,
                            workSpaceSizeInBytes,
                            alpha2,
                            zDesc,
                            z,
                            biasDesc,
                            bias,
                            activationDesc,
                            yDesc,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(
    handle: cudnnHandle_t,
    count: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionBackwardDataAlgorithmMaxCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, count) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFindConvolutionBackwardDataAlgorithm(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFindConvolutionBackwardDataAlgorithm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            wDesc,
                            dyDesc,
                            convDesc,
                            dxDesc,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFindConvolutionBackwardDataAlgorithmEx(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFindConvolutionBackwardDataAlgorithmEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            wDesc,
                            w,
                            dyDesc,
                            dy,
                            convDesc,
                            dxDesc,
                            dx,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                            workSpace,
                            workSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnTensorDescriptor_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionBwdDataAlgoPerf_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionBackwardDataAlgorithm_v7 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            filterDesc,
                            diffDesc,
                            convDesc,
                            gradDesc,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardDataWorkspaceSize(
    handle: cudnnHandle_t,
    wDesc: cudnnFilterDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dxDesc: cudnnTensorDescriptor_t,
    algo: cudnnConvolutionBwdDataAlgo_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionBackwardDataWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            wDesc,
                            dyDesc,
                            convDesc,
                            dxDesc,
                            algo,
                            sizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnConvolutionBackwardData(
    handle: cudnnHandle_t,
    alpha: *const ::std::os::raw::c_void,
    wDesc: cudnnFilterDescriptor_t,
    w: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionBwdDataAlgo_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::std::os::raw::c_void,
    dxDesc: cudnnTensorDescriptor_t,
    dx: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnConvolutionBackwardData {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            alpha,
                            wDesc,
                            w,
                            dyDesc,
                            dy,
                            convDesc,
                            algo,
                            workSpace,
                            workSpaceSizeInBytes,
                            beta,
                            dxDesc,
                            dx,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetFoldedConvBackwardDataDescriptors(
    handle: cudnnHandle_t,
    filterDesc: cudnnFilterDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnTensorDescriptor_t,
    transformFormat: cudnnTensorFormat_t,
    foldedFilterDesc: cudnnFilterDescriptor_t,
    paddedDiffDesc: cudnnTensorDescriptor_t,
    foldedConvDesc: cudnnConvolutionDescriptor_t,
    foldedGradDesc: cudnnTensorDescriptor_t,
    filterFoldTransDesc: cudnnTensorTransformDescriptor_t,
    diffPadTransDesc: cudnnTensorTransformDescriptor_t,
    gradFoldTransDesc: cudnnTensorTransformDescriptor_t,
    gradUnfoldTransDesc: cudnnTensorTransformDescriptor_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetFoldedConvBackwardDataDescriptors {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            filterDesc,
                            diffDesc,
                            convDesc,
                            gradDesc,
                            transformFormat,
                            foldedFilterDesc,
                            paddedDiffDesc,
                            foldedConvDesc,
                            foldedGradDesc,
                            filterFoldTransDesc,
                            diffPadTransDesc,
                            gradFoldTransDesc,
                            gradUnfoldTransDesc,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCnnVersionCheck() -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCnnVersionCheck {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(
    handle: cudnnHandle_t,
    count: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionBackwardFilterAlgorithmMaxCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, count) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFindConvolutionBackwardFilterAlgorithm(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFindConvolutionBackwardFilterAlgorithm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            xDesc,
                            dyDesc,
                            convDesc,
                            dwDesc,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFindConvolutionBackwardFilterAlgorithmEx(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    y: *const ::std::os::raw::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    dwDesc: cudnnFilterDescriptor_t,
    dw: *mut ::std::os::raw::c_void,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFindConvolutionBackwardFilterAlgorithmEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            xDesc,
                            x,
                            dyDesc,
                            y,
                            convDesc,
                            dwDesc,
                            dw,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                            workSpace,
                            workSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
    handle: cudnnHandle_t,
    srcDesc: cudnnTensorDescriptor_t,
    diffDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnFilterDescriptor_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    returnedAlgoCount: *mut ::std::os::raw::c_int,
    perfResults: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionBackwardFilterAlgorithm_v7 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            srcDesc,
                            diffDesc,
                            convDesc,
                            gradDesc,
                            requestedAlgoCount,
                            returnedAlgoCount,
                            perfResults,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
    handle: cudnnHandle_t,
    xDesc: cudnnTensorDescriptor_t,
    dyDesc: cudnnTensorDescriptor_t,
    convDesc: cudnnConvolutionDescriptor_t,
    gradDesc: cudnnFilterDescriptor_t,
    algo: cudnnConvolutionBwdFilterAlgo_t,
    sizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetConvolutionBackwardFilterWorkspaceSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            xDesc,
                            dyDesc,
                            convDesc,
                            gradDesc,
                            algo,
                            sizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnConvolutionBackwardFilter(
    handle: cudnnHandle_t,
    alpha: *const ::std::os::raw::c_void,
    xDesc: cudnnTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    convDesc: cudnnConvolutionDescriptor_t,
    algo: cudnnConvolutionBwdFilterAlgo_t,
    workSpace: *mut ::std::os::raw::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::std::os::raw::c_void,
    dwDesc: cudnnFilterDescriptor_t,
    dw: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnConvolutionBackwardFilter {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            alpha,
                            xDesc,
                            x,
                            dyDesc,
                            dy,
                            convDesc,
                            algo,
                            workSpace,
                            workSpaceSizeInBytes,
                            beta,
                            dwDesc,
                            dw,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnConvolutionBackwardBias(
    handle: cudnnHandle_t,
    alpha: *const ::std::os::raw::c_void,
    dyDesc: cudnnTensorDescriptor_t,
    dy: *const ::std::os::raw::c_void,
    beta: *const ::std::os::raw::c_void,
    dbDesc: cudnnTensorDescriptor_t,
    db: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnConvolutionBackwardBias {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, alpha, dyDesc, dy, beta, dbDesc, db) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateFusedOpsConstParamPack(
    constPack: *mut cudnnFusedOpsConstParamPack_t,
    ops: cudnnFusedOps_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateFusedOpsConstParamPack {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(constPack, ops) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyFusedOpsConstParamPack(
    constPack: cudnnFusedOpsConstParamPack_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyFusedOpsConstParamPack {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(constPack) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetFusedOpsConstParamPackAttribute(
    constPack: cudnnFusedOpsConstParamPack_t,
    paramLabel: cudnnFusedOpsConstParamLabel_t,
    param: *const ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetFusedOpsConstParamPackAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(constPack, paramLabel, param) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetFusedOpsConstParamPackAttribute(
    constPack: cudnnFusedOpsConstParamPack_t,
    paramLabel: cudnnFusedOpsConstParamLabel_t,
    param: *mut ::std::os::raw::c_void,
    isNULL: *mut ::std::os::raw::c_int,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetFusedOpsConstParamPackAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(constPack, paramLabel, param, isNULL) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateFusedOpsVariantParamPack(
    varPack: *mut cudnnFusedOpsVariantParamPack_t,
    ops: cudnnFusedOps_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateFusedOpsVariantParamPack {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(varPack, ops) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyFusedOpsVariantParamPack(
    varPack: cudnnFusedOpsVariantParamPack_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyFusedOpsVariantParamPack {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(varPack) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSetFusedOpsVariantParamPackAttribute(
    varPack: cudnnFusedOpsVariantParamPack_t,
    paramLabel: cudnnFusedOpsVariantParamLabel_t,
    ptr: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSetFusedOpsVariantParamPackAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(varPack, paramLabel, ptr) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGetFusedOpsVariantParamPackAttribute(
    varPack: cudnnFusedOpsVariantParamPack_t,
    paramLabel: cudnnFusedOpsVariantParamLabel_t,
    ptr: *mut ::std::os::raw::c_void,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGetFusedOpsVariantParamPackAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(varPack, paramLabel, ptr) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCreateFusedOpsPlan(
    plan: *mut cudnnFusedOpsPlan_t,
    ops: cudnnFusedOps_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCreateFusedOpsPlan {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, ops) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnDestroyFusedOpsPlan(
    plan: cudnnFusedOpsPlan_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnDestroyFusedOpsPlan {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnMakeFusedOpsPlan(
    handle: cudnnHandle_t,
    plan: cudnnFusedOpsPlan_t,
    constPack: cudnnFusedOpsConstParamPack_t,
    workspaceSizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnMakeFusedOpsPlan {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, plan, constPack, workspaceSizeInBytes) }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFusedOpsExecute(
    handle: cudnnHandle_t,
    plan: cudnnFusedOpsPlan_t,
    varPack: cudnnFusedOpsVariantParamPack_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFusedOpsExecute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, plan, varPack) }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnSubquadraticOpsVersionCheck() -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnSubquadraticOpsVersionCheck {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCausalConv1dForward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weight: *const ::std::os::raw::c_void,
    bias: *const ::std::os::raw::c_void,
    y: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    activation: cudnnCausalConv1dActivation_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCausalConv1dForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weight,
                            bias,
                            y,
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                            activation,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCausalConv1dBackward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weight: *const ::std::os::raw::c_void,
    bias: *const ::std::os::raw::c_void,
    dy: *const ::std::os::raw::c_void,
    dx: *mut ::std::os::raw::c_void,
    dweight: *mut ::std::os::raw::c_void,
    dbias: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    dwDataType: cudnnDataType_t,
    activation: cudnnCausalConv1dActivation_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCausalConv1dBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weight,
                            bias,
                            dy,
                            dx,
                            dweight,
                            dbias,
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                            dwDataType,
                            activation,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCausalConv1dNwhForward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weight: *const ::std::os::raw::c_void,
    bias: *const ::std::os::raw::c_void,
    y: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    activation: cudnnCausalConv1dActivation_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCausalConv1dNwhForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weight,
                            bias,
                            y,
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                            activation,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnCausalConv1dNwhBackward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weight: *const ::std::os::raw::c_void,
    bias: *const ::std::os::raw::c_void,
    dy: *const ::std::os::raw::c_void,
    dx: *mut ::std::os::raw::c_void,
    dweight: *mut ::std::os::raw::c_void,
    dbias: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    dwDataType: cudnnDataType_t,
    activation: cudnnCausalConv1dActivation_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnCausalConv1dNwhBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weight,
                            bias,
                            dy,
                            dx,
                            dweight,
                            dbias,
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                            dwDataType,
                            activation,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnB2BCausalConv1dForward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weightsProj: *const ::std::os::raw::c_void,
    weightsMixer: *const ::std::os::raw::c_void,
    skipBias: *const ::std::os::raw::c_void,
    y: *mut ::std::os::raw::c_void,
    yGated: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSizeProj: ::std::os::raw::c_int,
    kernelSizeMixer: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnB2BCausalConv1dForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weightsProj,
                            weightsMixer,
                            skipBias,
                            y,
                            yGated,
                            batch,
                            dim,
                            seqLen,
                            kernelSizeProj,
                            kernelSizeMixer,
                            dataType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnB2BCausalConv1dBackward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weightsProj: *const ::std::os::raw::c_void,
    weightsMixer: *const ::std::os::raw::c_void,
    skipBias: *const ::std::os::raw::c_void,
    y: *const ::std::os::raw::c_void,
    dy: *const ::std::os::raw::c_void,
    dx: *mut ::std::os::raw::c_void,
    dweightsProj: *mut ::std::os::raw::c_void,
    dweightsMixer: *mut ::std::os::raw::c_void,
    dskipBias: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSizeProj: ::std::os::raw::c_int,
    kernelSizeMixer: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    dwDataType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnB2BCausalConv1dBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weightsProj,
                            weightsMixer,
                            skipBias,
                            y,
                            dy,
                            dx,
                            dweightsProj,
                            dweightsMixer,
                            dskipBias,
                            batch,
                            dim,
                            seqLen,
                            kernelSizeProj,
                            kernelSizeMixer,
                            dataType,
                            dwDataType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFFTCausalConv1dForward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weight: *const ::std::os::raw::c_void,
    y: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFFTCausalConv1dForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weight,
                            y,
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnFFTCausalConv1dBackward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weight: *const ::std::os::raw::c_void,
    dy: *const ::std::os::raw::c_void,
    dx: *mut ::std::os::raw::c_void,
    dweight: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnFFTCausalConv1dBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weight,
                            dy,
                            dx,
                            dweight,
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnLongFFTCausalConv1dGetBufferSizes(
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    workspaceSizeInBytes: *mut usize,
    reserveSpaceSizeInBytes: *mut usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnLongFFTCausalConv1dGetBufferSizes {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                            workspaceSizeInBytes,
                            reserveSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnLongFFTCausalConv1dForward(
    stream: cudaStream_t,
    x: *const ::std::os::raw::c_void,
    weight: *const ::std::os::raw::c_void,
    y: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    workspace: *mut ::std::os::raw::c_void,
    workspaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnLongFFTCausalConv1dForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            x,
                            weight,
                            y,
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                            workspace,
                            workspaceSizeInBytes,
                            reserveSpace,
                            reserveSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnLongFFTCausalConv1dBackward(
    stream: cudaStream_t,
    dy: *const ::std::os::raw::c_void,
    dx: *mut ::std::os::raw::c_void,
    dweight: *mut ::std::os::raw::c_void,
    batch: ::std::os::raw::c_int,
    dim: ::std::os::raw::c_int,
    seqLen: ::std::os::raw::c_int,
    kernelSize: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    workspace: *mut ::std::os::raw::c_void,
    workspaceSizeInBytes: usize,
    reserveSpace: *mut ::std::os::raw::c_void,
    reserveSpaceSizeInBytes: usize,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnLongFFTCausalConv1dBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            dy,
                            dx,
                            dweight,
                            batch,
                            dim,
                            seqLen,
                            kernelSize,
                            dataType,
                            workspace,
                            workspaceSizeInBytes,
                            reserveSpace,
                            reserveSpaceSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGnnAggSimpleForward(
    stream: cudaStream_t,
    graph: *const cudnnGnnCscGraph_t,
    nodeFeatures: *const ::std::os::raw::c_void,
    edgeFeatures: *const ::std::os::raw::c_void,
    concatFeatures: *const ::std::os::raw::c_void,
    output: *mut ::std::os::raw::c_void,
    outPositions: *mut ::std::os::raw::c_void,
    nodeFeatDim: ::std::os::raw::c_int,
    edgeFeatDim: ::std::os::raw::c_int,
    concatFeatDim: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    aggOp: cudnnGnnAggOp_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGnnAggSimpleForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            graph,
                            nodeFeatures,
                            edgeFeatures,
                            concatFeatures,
                            output,
                            outPositions,
                            nodeFeatDim,
                            edgeFeatDim,
                            concatFeatDim,
                            dataType,
                            aggOp,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGnnAggSimpleBackward(
    stream: cudaStream_t,
    graph: *const cudnnGnnCscGraph_t,
    gradOutput: *const ::std::os::raw::c_void,
    outPositions: *const ::std::os::raw::c_void,
    gradNodeFeatures: *mut ::std::os::raw::c_void,
    gradEdgeFeatures: *mut ::std::os::raw::c_void,
    gradConcatFeatures: *mut ::std::os::raw::c_void,
    nodeFeatDim: ::std::os::raw::c_int,
    edgeFeatDim: ::std::os::raw::c_int,
    concatFeatDim: ::std::os::raw::c_int,
    dataType: cudnnDataType_t,
    aggOp: cudnnGnnAggOp_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGnnAggSimpleBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            graph,
                            gradOutput,
                            outPositions,
                            gradNodeFeatures,
                            gradEdgeFeatures,
                            gradConcatFeatures,
                            nodeFeatDim,
                            edgeFeatDim,
                            concatFeatDim,
                            dataType,
                            aggOp,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGnnMhaGatForward(
    stream: cudaStream_t,
    graph: *const cudnnGnnCscGraph_t,
    srcFeatures: *const ::std::os::raw::c_void,
    dstFeatures: *const ::std::os::raw::c_void,
    edgeFeatures: *const ::std::os::raw::c_void,
    attnWeights: *const ::std::os::raw::c_void,
    dropoutMask: *const f32,
    output: *mut ::std::os::raw::c_void,
    smScores: *mut ::std::os::raw::c_void,
    dimNode: ::std::os::raw::c_int,
    dimEdge: ::std::os::raw::c_int,
    params: *const cudnnGnnMhaParams_t,
    dataType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGnnMhaGatForward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            graph,
                            srcFeatures,
                            dstFeatures,
                            edgeFeatures,
                            attnWeights,
                            dropoutMask,
                            output,
                            smScores,
                            dimNode,
                            dimEdge,
                            params,
                            dataType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGnnMhaGatBackward(
    stream: cudaStream_t,
    graph: *const cudnnGnnCscGraph_t,
    gradOut: *const ::std::os::raw::c_void,
    srcFeatures: *const ::std::os::raw::c_void,
    dstFeatures: *const ::std::os::raw::c_void,
    edgeFeatures: *const ::std::os::raw::c_void,
    attnWeights: *const ::std::os::raw::c_void,
    smScores: *const ::std::os::raw::c_void,
    dropoutMask: *const f32,
    gradAttention: *const f32,
    gradSrcFeatures: *mut ::std::os::raw::c_void,
    gradDstFeatures: *mut ::std::os::raw::c_void,
    gradEdgeFeatures: *mut ::std::os::raw::c_void,
    gradWeights: *mut ::std::os::raw::c_void,
    gradSmScores: *mut ::std::os::raw::c_void,
    gradWspaceFeat: *mut ::std::os::raw::c_void,
    gradWspaceWeights: *mut ::std::os::raw::c_void,
    dimNode: ::std::os::raw::c_int,
    dimEdge: ::std::os::raw::c_int,
    params: *const cudnnGnnMhaParams_t,
    dataType: cudnnDataType_t,
    gradDataType: cudnnDataType_t,
    gradWeightType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGnnMhaGatBackward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            graph,
                            gradOut,
                            srcFeatures,
                            dstFeatures,
                            edgeFeatures,
                            attnWeights,
                            smScores,
                            dropoutMask,
                            gradAttention,
                            gradSrcFeatures,
                            gradDstFeatures,
                            gradEdgeFeatures,
                            gradWeights,
                            gradSmScores,
                            gradWspaceFeat,
                            gradWspaceWeights,
                            dimNode,
                            dimEdge,
                            params,
                            dataType,
                            gradDataType,
                            gradWeightType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGnnMhaGatV2Forward(
    stream: cudaStream_t,
    graph: *const cudnnGnnCscGraph_t,
    srcFeatures: *const ::std::os::raw::c_void,
    dstFeatures: *const ::std::os::raw::c_void,
    edgeFeatures: *const ::std::os::raw::c_void,
    attnWeights: *const ::std::os::raw::c_void,
    dropoutMask: *const f32,
    output: *mut ::std::os::raw::c_void,
    smScores: *mut ::std::os::raw::c_void,
    actScores: *mut ::std::os::raw::c_void,
    dimNode: ::std::os::raw::c_int,
    params: *const cudnnGnnMhaParams_t,
    dataType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGnnMhaGatV2Forward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            graph,
                            srcFeatures,
                            dstFeatures,
                            edgeFeatures,
                            attnWeights,
                            dropoutMask,
                            output,
                            smScores,
                            actScores,
                            dimNode,
                            params,
                            dataType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cudnnGnnMhaGatV2Backward(
    stream: cudaStream_t,
    graph: *const cudnnGnnCscGraph_t,
    gradOut: *const ::std::os::raw::c_void,
    srcFeatures: *const ::std::os::raw::c_void,
    dstFeatures: *const ::std::os::raw::c_void,
    edgeFeatures: *const ::std::os::raw::c_void,
    attnWeights: *const ::std::os::raw::c_void,
    smScores: *const ::std::os::raw::c_void,
    actScores: *const ::std::os::raw::c_void,
    dropoutMask: *const f32,
    gradAttention: *const f32,
    gradSrcFeatures: *mut ::std::os::raw::c_void,
    gradDstFeatures: *mut ::std::os::raw::c_void,
    gradEdgeFeatures: *mut ::std::os::raw::c_void,
    gradWeights: *mut ::std::os::raw::c_void,
    gradSmScores: *mut ::std::os::raw::c_void,
    gradWspaceFeat: *mut ::std::os::raw::c_void,
    gradWspaceWeights: *mut ::std::os::raw::c_void,
    dimNode: ::std::os::raw::c_int,
    params: *const cudnnGnnMhaParams_t,
    dataType: cudnnDataType_t,
    gradType: cudnnDataType_t,
) -> cudnnStatus_t {
    match cudnn_api() {
        Ok(__nv_api) => {
            match &__nv_api.cudnnGnnMhaGatV2Backward {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            stream,
                            graph,
                            gradOut,
                            srcFeatures,
                            dstFeatures,
                            edgeFeatures,
                            attnWeights,
                            smScores,
                            actScores,
                            dropoutMask,
                            gradAttention,
                            gradSrcFeatures,
                            gradDstFeatures,
                            gradEdgeFeatures,
                            gradWeights,
                            gradSmScores,
                            gradWspaceFeat,
                            gradWspaceWeights,
                            dimNode,
                            params,
                            dataType,
                            gradType,
                        )
                    }
                }
                Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cudnnStatus_t_CUDNN_STATUS_NOT_INITIALIZED,
    }
}
