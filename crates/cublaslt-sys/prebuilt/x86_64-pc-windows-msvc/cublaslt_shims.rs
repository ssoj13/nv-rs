#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtCreate(
    lightHandle: *mut cublasLtHandle_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(lightHandle) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtDestroy(
    lightHandle: cublasLtHandle_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(lightHandle) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtGetStatusName(
    status: cublasStatus_t,
) -> *const ::std::os::raw::c_char {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtGetStatusName {
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
pub unsafe extern "C" fn cublasLtGetStatusString(
    status: cublasStatus_t,
) -> *const ::std::os::raw::c_char {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtGetStatusString {
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
pub unsafe fn cublasLtGetVersion() -> ::core::result::Result<
    usize,
    nv_loader::CallError,
> {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtGetVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)() }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cublaslt",
                            stringify!(cublasLtGetVersion),
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
pub unsafe fn cublasLtGetCudartVersion() -> ::core::result::Result<
    usize,
    nv_loader::CallError,
> {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtGetCudartVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)() }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cublaslt",
                            stringify!(cublasLtGetCudartVersion),
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
pub unsafe extern "C" fn cublasLtGetProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtGetProperty {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_, value) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtHeuristicsCacheGetCapacity(
    capacity: *mut usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtHeuristicsCacheGetCapacity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(capacity) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtHeuristicsCacheSetCapacity(
    capacity: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtHeuristicsCacheSetCapacity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(capacity) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn cublasLtDisableCpuInstructionsSetMask(
    mask: ::std::os::raw::c_uint,
) -> ::core::result::Result<::std::os::raw::c_uint, nv_loader::CallError> {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtDisableCpuInstructionsSetMask {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)(mask) }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cublaslt",
                            stringify!(cublasLtDisableCpuInstructionsSetMask),
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
pub unsafe extern "C" fn cublasLtMatmul(
    lightHandle: cublasLtHandle_t,
    computeDesc: cublasLtMatmulDesc_t,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Adesc: cublasLtMatrixLayout_t,
    B: *const ::std::os::raw::c_void,
    Bdesc: cublasLtMatrixLayout_t,
    beta: *const ::std::os::raw::c_void,
    C: *const ::std::os::raw::c_void,
    Cdesc: cublasLtMatrixLayout_t,
    D: *mut ::std::os::raw::c_void,
    Ddesc: cublasLtMatrixLayout_t,
    algo: *const cublasLtMatmulAlgo_t,
    workspace: *mut ::std::os::raw::c_void,
    workspaceSizeInBytes: usize,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmul {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            lightHandle,
                            computeDesc,
                            alpha,
                            A,
                            Adesc,
                            B,
                            Bdesc,
                            beta,
                            C,
                            Cdesc,
                            D,
                            Ddesc,
                            algo,
                            workspace,
                            workspaceSizeInBytes,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixTransform(
    lightHandle: cublasLtHandle_t,
    transformDesc: cublasLtMatrixTransformDesc_t,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Adesc: cublasLtMatrixLayout_t,
    beta: *const ::std::os::raw::c_void,
    B: *const ::std::os::raw::c_void,
    Bdesc: cublasLtMatrixLayout_t,
    C: *mut ::std::os::raw::c_void,
    Cdesc: cublasLtMatrixLayout_t,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixTransform {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            lightHandle,
                            transformDesc,
                            alpha,
                            A,
                            Adesc,
                            beta,
                            B,
                            Bdesc,
                            C,
                            Cdesc,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixLayoutInit_internal(
    matLayout: cublasLtMatrixLayout_t,
    size: usize,
    type_: cudaDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixLayoutInit_internal {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(matLayout, size, type_, rows, cols, ld) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtGroupedMatrixLayoutInit_internal(
    matLayout: cublasLtMatrixLayout_t,
    size: usize,
    type_: cudaDataType,
    groupCount: ::std::os::raw::c_int,
    rows_array: *const ::std::os::raw::c_void,
    cols_array: *const ::std::os::raw::c_void,
    ld_array: *const ::std::os::raw::c_void,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtGroupedMatrixLayoutInit_internal {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            matLayout,
                            size,
                            type_,
                            groupCount,
                            rows_array,
                            cols_array,
                            ld_array,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixLayoutCreate(
    matLayout: *mut cublasLtMatrixLayout_t,
    type_: cudaDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixLayoutCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(matLayout, type_, rows, cols, ld) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtGroupedMatrixLayoutCreate(
    matLayout: *mut cublasLtMatrixLayout_t,
    type_: cudaDataType,
    groupCount: ::std::os::raw::c_int,
    rows_array: *const ::std::os::raw::c_void,
    cols_array: *const ::std::os::raw::c_void,
    ld_array: *const ::std::os::raw::c_void,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtGroupedMatrixLayoutCreate {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            matLayout,
                            type_,
                            groupCount,
                            rows_array,
                            cols_array,
                            ld_array,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixLayoutDestroy(
    matLayout: cublasLtMatrixLayout_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixLayoutDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(matLayout) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixLayoutSetAttribute(
    matLayout: cublasLtMatrixLayout_t,
    attr: cublasLtMatrixLayoutAttribute_t,
    buf: *const ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixLayoutSetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(matLayout, attr, buf, sizeInBytes) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixLayoutGetAttribute(
    matLayout: cublasLtMatrixLayout_t,
    attr: cublasLtMatrixLayoutAttribute_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixLayoutGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(matLayout, attr, buf, sizeInBytes, sizeWritten) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulDescInit_internal(
    matmulDesc: cublasLtMatmulDesc_t,
    size: usize,
    computeType: cublasComputeType_t,
    scaleType: cudaDataType_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulDescInit_internal {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(matmulDesc, size, computeType, scaleType) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulDescCreate(
    matmulDesc: *mut cublasLtMatmulDesc_t,
    computeType: cublasComputeType_t,
    scaleType: cudaDataType_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulDescCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(matmulDesc, computeType, scaleType) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulDescDestroy(
    matmulDesc: cublasLtMatmulDesc_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulDescDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(matmulDesc) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulDescSetAttribute(
    matmulDesc: cublasLtMatmulDesc_t,
    attr: cublasLtMatmulDescAttributes_t,
    buf: *const ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulDescSetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(matmulDesc, attr, buf, sizeInBytes) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulDescGetAttribute(
    matmulDesc: cublasLtMatmulDesc_t,
    attr: cublasLtMatmulDescAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulDescGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(matmulDesc, attr, buf, sizeInBytes, sizeWritten)
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixTransformDescInit_internal(
    transformDesc: cublasLtMatrixTransformDesc_t,
    size: usize,
    scaleType: cudaDataType,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixTransformDescInit_internal {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(transformDesc, size, scaleType) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixTransformDescCreate(
    transformDesc: *mut cublasLtMatrixTransformDesc_t,
    scaleType: cudaDataType,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixTransformDescCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(transformDesc, scaleType) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixTransformDescDestroy(
    transformDesc: cublasLtMatrixTransformDesc_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixTransformDescDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(transformDesc) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixTransformDescSetAttribute(
    transformDesc: cublasLtMatrixTransformDesc_t,
    attr: cublasLtMatrixTransformDescAttributes_t,
    buf: *const ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixTransformDescSetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(transformDesc, attr, buf, sizeInBytes) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatrixTransformDescGetAttribute(
    transformDesc: cublasLtMatrixTransformDesc_t,
    attr: cublasLtMatrixTransformDescAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatrixTransformDescGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(transformDesc, attr, buf, sizeInBytes, sizeWritten)
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtEmulationDescInit_internal(
    emulationDesc: cublasLtEmulationDesc_t,
    size: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtEmulationDescInit_internal {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(emulationDesc, size) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtEmulationDescCreate(
    emulationDesc: *mut cublasLtEmulationDesc_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtEmulationDescCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(emulationDesc) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtEmulationDescDestroy(
    emulationDesc: cublasLtEmulationDesc_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtEmulationDescDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(emulationDesc) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtEmulationDescSetAttribute(
    emulationDesc: cublasLtEmulationDesc_t,
    attr: cublasLtEmulationDescAttributes_t,
    buf: *const ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtEmulationDescSetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(emulationDesc, attr, buf, sizeInBytes) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtEmulationDescGetAttribute(
    emulationDesc: cublasLtEmulationDesc_t,
    attr: cublasLtEmulationDescAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtEmulationDescGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(emulationDesc, attr, buf, sizeInBytes, sizeWritten)
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulPreferenceInit_internal(
    pref: cublasLtMatmulPreference_t,
    size: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulPreferenceInit_internal {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pref, size) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulPreferenceCreate(
    pref: *mut cublasLtMatmulPreference_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulPreferenceCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pref) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulPreferenceDestroy(
    pref: cublasLtMatmulPreference_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulPreferenceDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pref) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulPreferenceSetAttribute(
    pref: cublasLtMatmulPreference_t,
    attr: cublasLtMatmulPreferenceAttributes_t,
    buf: *const ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulPreferenceSetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pref, attr, buf, sizeInBytes) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulPreferenceGetAttribute(
    pref: cublasLtMatmulPreference_t,
    attr: cublasLtMatmulPreferenceAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulPreferenceGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pref, attr, buf, sizeInBytes, sizeWritten) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoGetHeuristic(
    lightHandle: cublasLtHandle_t,
    operationDesc: cublasLtMatmulDesc_t,
    Adesc: cublasLtMatrixLayout_t,
    Bdesc: cublasLtMatrixLayout_t,
    Cdesc: cublasLtMatrixLayout_t,
    Ddesc: cublasLtMatrixLayout_t,
    preference: cublasLtMatmulPreference_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    heuristicResultsArray: *mut cublasLtMatmulHeuristicResult_t,
    returnAlgoCount: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoGetHeuristic {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            lightHandle,
                            operationDesc,
                            Adesc,
                            Bdesc,
                            Cdesc,
                            Ddesc,
                            preference,
                            requestedAlgoCount,
                            heuristicResultsArray,
                            returnAlgoCount,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoGetHeuristicForStream(
    lightHandle: cublasLtHandle_t,
    operationDesc: cublasLtMatmulDesc_t,
    Adesc: cublasLtMatrixLayout_t,
    Bdesc: cublasLtMatrixLayout_t,
    Cdesc: cublasLtMatrixLayout_t,
    Ddesc: cublasLtMatrixLayout_t,
    preference: cublasLtMatmulPreference_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    heuristicResultsArray: *mut cublasLtMatmulHeuristicResult_t,
    returnAlgoCount: *mut ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoGetHeuristicForStream {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            lightHandle,
                            operationDesc,
                            Adesc,
                            Bdesc,
                            Cdesc,
                            Ddesc,
                            preference,
                            requestedAlgoCount,
                            heuristicResultsArray,
                            returnAlgoCount,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoGetIds(
    lightHandle: cublasLtHandle_t,
    computeType: cublasComputeType_t,
    scaleType: cudaDataType_t,
    Atype: cudaDataType_t,
    Btype: cudaDataType_t,
    Ctype: cudaDataType_t,
    Dtype: cudaDataType_t,
    requestedAlgoCount: ::std::os::raw::c_int,
    algoIdsArray: *mut ::std::os::raw::c_int,
    returnAlgoCount: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoGetIds {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            lightHandle,
                            computeType,
                            scaleType,
                            Atype,
                            Btype,
                            Ctype,
                            Dtype,
                            requestedAlgoCount,
                            algoIdsArray,
                            returnAlgoCount,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoInit(
    lightHandle: cublasLtHandle_t,
    computeType: cublasComputeType_t,
    scaleType: cudaDataType_t,
    Atype: cudaDataType_t,
    Btype: cudaDataType_t,
    Ctype: cudaDataType_t,
    Dtype: cudaDataType_t,
    algoId: ::std::os::raw::c_int,
    algo: *mut cublasLtMatmulAlgo_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoInit {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            lightHandle,
                            computeType,
                            scaleType,
                            Atype,
                            Btype,
                            Ctype,
                            Dtype,
                            algoId,
                            algo,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoCheck(
    lightHandle: cublasLtHandle_t,
    operationDesc: cublasLtMatmulDesc_t,
    Adesc: cublasLtMatrixLayout_t,
    Bdesc: cublasLtMatrixLayout_t,
    Cdesc: cublasLtMatrixLayout_t,
    Ddesc: cublasLtMatrixLayout_t,
    algo: *const cublasLtMatmulAlgo_t,
    result: *mut cublasLtMatmulHeuristicResult_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoCheck {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            lightHandle,
                            operationDesc,
                            Adesc,
                            Bdesc,
                            Cdesc,
                            Ddesc,
                            algo,
                            result,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoCheckForStream(
    lightHandle: cublasLtHandle_t,
    operationDesc: cublasLtMatmulDesc_t,
    Adesc: cublasLtMatrixLayout_t,
    Bdesc: cublasLtMatrixLayout_t,
    Cdesc: cublasLtMatrixLayout_t,
    Ddesc: cublasLtMatrixLayout_t,
    algo: *const cublasLtMatmulAlgo_t,
    result: *mut cublasLtMatmulHeuristicResult_t,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoCheckForStream {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            lightHandle,
                            operationDesc,
                            Adesc,
                            Bdesc,
                            Cdesc,
                            Ddesc,
                            algo,
                            result,
                            stream,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoCapGetAttribute(
    algo: *const cublasLtMatmulAlgo_t,
    attr: cublasLtMatmulAlgoCapAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoCapGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(algo, attr, buf, sizeInBytes, sizeWritten) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoConfigSetAttribute(
    algo: *mut cublasLtMatmulAlgo_t,
    attr: cublasLtMatmulAlgoConfigAttributes_t,
    buf: *const ::std::os::raw::c_void,
    sizeInBytes: usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoConfigSetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(algo, attr, buf, sizeInBytes) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtMatmulAlgoConfigGetAttribute(
    algo: *const cublasLtMatmulAlgo_t,
    attr: cublasLtMatmulAlgoConfigAttributes_t,
    buf: *mut ::std::os::raw::c_void,
    sizeInBytes: usize,
    sizeWritten: *mut usize,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtMatmulAlgoConfigGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(algo, attr, buf, sizeInBytes, sizeWritten) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtLoggerSetCallback(
    callback: cublasLtLoggerCallback_t,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtLoggerSetCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(callback) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtLoggerSetFile(file: *mut FILE) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtLoggerSetFile {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(file) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtLoggerOpenFile(
    logFile: *const ::std::os::raw::c_char,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtLoggerOpenFile {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(logFile) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtLoggerSetLevel(
    level: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtLoggerSetLevel {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(level) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtLoggerSetMask(
    mask: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtLoggerSetMask {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(mask) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasLtLoggerForceDisable() -> cublasStatus_t {
    match cublaslt_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLtLoggerForceDisable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
