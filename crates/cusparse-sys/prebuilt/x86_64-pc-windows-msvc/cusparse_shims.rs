#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreate(
    handle: *mut cusparseHandle_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroy(handle: cusparseHandle_t) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseGetVersion(
    handle: cusparseHandle_t,
    version: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, version) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseGetProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetProperty {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_, value) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseGetErrorName(
    status: cusparseStatus_t,
) -> *const ::std::os::raw::c_char {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetErrorName {
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
pub unsafe extern "C" fn cusparseGetErrorString(
    status: cusparseStatus_t,
) -> *const ::std::os::raw::c_char {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetErrorString {
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
pub unsafe extern "C" fn cusparseSetStream(
    handle: cusparseHandle_t,
    streamId: cudaStream_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseGetStream(
    handle: cusparseHandle_t,
    streamId: *mut cudaStream_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseGetPointerMode(
    handle: cusparseHandle_t,
    mode: *mut cusparsePointerMode_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetPointerMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSetPointerMode(
    handle: cusparseHandle_t,
    mode: cusparsePointerMode_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSetPointerMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseLoggerSetCallback(
    callback: cusparseLoggerCallback_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseLoggerSetCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(callback) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseLoggerSetFile(file: *mut FILE) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseLoggerSetFile {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(file) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseLoggerOpenFile(
    logFile: *const ::std::os::raw::c_char,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseLoggerOpenFile {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(logFile) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseLoggerSetLevel(
    level: ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseLoggerSetLevel {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(level) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseLoggerSetMask(
    mask: ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseLoggerSetMask {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(mask) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseLoggerForceDisable() -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseLoggerForceDisable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateMatDescr(
    descrA: *mut cusparseMatDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateMatDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descrA) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyMatDescr(
    descrA: cusparseMatDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyMatDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descrA) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSetMatType(
    descrA: cusparseMatDescr_t,
    type_: cusparseMatrixType_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSetMatType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descrA, type_) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn cusparseGetMatType(
    descrA: cusparseMatDescr_t,
) -> ::core::result::Result<cusparseMatrixType_t, nv_loader::CallError> {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetMatType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)(descrA) }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cusparse",
                            stringify!(cusparseGetMatType),
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
pub unsafe extern "C" fn cusparseSetMatFillMode(
    descrA: cusparseMatDescr_t,
    fillMode: cusparseFillMode_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSetMatFillMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descrA, fillMode) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn cusparseGetMatFillMode(
    descrA: cusparseMatDescr_t,
) -> ::core::result::Result<cusparseFillMode_t, nv_loader::CallError> {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetMatFillMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)(descrA) }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cusparse",
                            stringify!(cusparseGetMatFillMode),
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
pub unsafe extern "C" fn cusparseSetMatDiagType(
    descrA: cusparseMatDescr_t,
    diagType: cusparseDiagType_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSetMatDiagType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descrA, diagType) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn cusparseGetMatDiagType(
    descrA: cusparseMatDescr_t,
) -> ::core::result::Result<cusparseDiagType_t, nv_loader::CallError> {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetMatDiagType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)(descrA) }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cusparse",
                            stringify!(cusparseGetMatDiagType),
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
pub unsafe extern "C" fn cusparseSetMatIndexBase(
    descrA: cusparseMatDescr_t,
    base: cusparseIndexBase_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSetMatIndexBase {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descrA, base) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn cusparseGetMatIndexBase(
    descrA: cusparseMatDescr_t,
) -> ::core::result::Result<cusparseIndexBase_t, nv_loader::CallError> {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGetMatIndexBase {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)(descrA) }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cusparse",
                            stringify!(cusparseGetMatIndexBase),
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
pub unsafe extern "C" fn cusparseCreateCsric02Info(
    info: *mut csric02Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateCsric02Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyCsric02Info(
    info: csric02Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyCsric02Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateBsric02Info(
    info: *mut bsric02Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateBsric02Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyBsric02Info(
    info: bsric02Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyBsric02Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateCsrilu02Info(
    info: *mut csrilu02Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateCsrilu02Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyCsrilu02Info(
    info: csrilu02Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyCsrilu02Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateBsrilu02Info(
    info: *mut bsrilu02Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateBsrilu02Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyBsrilu02Info(
    info: bsrilu02Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyBsrilu02Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateBsrsv2Info(
    info: *mut bsrsv2Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateBsrsv2Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyBsrsv2Info(
    info: bsrsv2Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyBsrsv2Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateBsrsm2Info(
    info: *mut bsrsm2Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateBsrsm2Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyBsrsm2Info(
    info: bsrsm2Info_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyBsrsm2Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateCsru2csrInfo(
    info: *mut csru2csrInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateCsru2csrInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyCsru2csrInfo(
    info: csru2csrInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyCsru2csrInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateColorInfo(
    info: *mut cusparseColorInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateColorInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyColorInfo(
    info: cusparseColorInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyColorInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreatePruneInfo(
    info: *mut pruneInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreatePruneInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyPruneInfo(
    info: pruneInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyPruneInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgemvi(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    xVal: *const f32,
    xInd: *const ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    idxBase: cusparseIndexBase_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgemvi {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transA,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            nnz,
                            xVal,
                            xInd,
                            beta,
                            y,
                            idxBase,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgemvi_bufferSize(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    pBufferSize: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgemvi_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, transA, m, n, nnz, pBufferSize) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgemvi(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    xVal: *const f64,
    xInd: *const ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    idxBase: cusparseIndexBase_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgemvi {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transA,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            nnz,
                            xVal,
                            xInd,
                            beta,
                            y,
                            idxBase,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgemvi_bufferSize(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    pBufferSize: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgemvi_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, transA, m, n, nnz, pBufferSize) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgemvi(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    xVal: *const cuComplex,
    xInd: *const ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    idxBase: cusparseIndexBase_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgemvi {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transA,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            nnz,
                            xVal,
                            xInd,
                            beta,
                            y,
                            idxBase,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgemvi_bufferSize(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    pBufferSize: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgemvi_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, transA, m, n, nnz, pBufferSize) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgemvi(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    xVal: *const cuDoubleComplex,
    xInd: *const ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    idxBase: cusparseIndexBase_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgemvi {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transA,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            nnz,
                            xVal,
                            xInd,
                            beta,
                            y,
                            idxBase,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgemvi_bufferSize(
    handle: cusparseHandle_t,
    transA: cusparseOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    pBufferSize: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgemvi_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, transA, m, n, nnz, pBufferSize) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrmv(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    x: *const f32,
    beta: *const f32,
    y: *mut f32,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrmv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            x,
                            beta,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrmv(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f64,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    x: *const f64,
    beta: *const f64,
    y: *mut f64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrmv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            x,
                            beta,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrmv(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    x: *const cuComplex,
    beta: *const cuComplex,
    y: *mut cuComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrmv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            x,
                            beta,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrmv(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrmv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            x,
                            beta,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrxmv(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    sizeOfMask: ::std::os::raw::c_int,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedMaskPtrA: *const ::std::os::raw::c_int,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedEndPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    x: *const f32,
    beta: *const f32,
    y: *mut f32,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrxmv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            sizeOfMask,
                            mb,
                            nb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedMaskPtrA,
                            bsrSortedRowPtrA,
                            bsrSortedEndPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            x,
                            beta,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrxmv(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    sizeOfMask: ::std::os::raw::c_int,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f64,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedMaskPtrA: *const ::std::os::raw::c_int,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedEndPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    x: *const f64,
    beta: *const f64,
    y: *mut f64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrxmv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            sizeOfMask,
                            mb,
                            nb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedMaskPtrA,
                            bsrSortedRowPtrA,
                            bsrSortedEndPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            x,
                            beta,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrxmv(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    sizeOfMask: ::std::os::raw::c_int,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedMaskPtrA: *const ::std::os::raw::c_int,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedEndPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    x: *const cuComplex,
    beta: *const cuComplex,
    y: *mut cuComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrxmv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            sizeOfMask,
                            mb,
                            nb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedMaskPtrA,
                            bsrSortedRowPtrA,
                            bsrSortedEndPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            x,
                            beta,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrxmv(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    sizeOfMask: ::std::os::raw::c_int,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedMaskPtrA: *const ::std::os::raw::c_int,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedEndPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrxmv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            sizeOfMask,
                            mb,
                            nb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedMaskPtrA,
                            bsrSortedRowPtrA,
                            bsrSortedEndPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            x,
                            beta,
                            y,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXbsrsv2_zeroPivot(
    handle: cusparseHandle_t,
    info: bsrsv2Info_t,
    position: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXbsrsv2_zeroPivot {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, position) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrsv2_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *mut f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrsv2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrsv2_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *mut f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrsv2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrsv2_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *mut cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrsv2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrsv2_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *mut cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrsv2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *mut f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrsv2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *mut f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrsv2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *mut cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrsv2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *mut cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrsv2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrsv2_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrsv2_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrsv2_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrsv2_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrsv2_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrsv2_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrsv2_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrsv2_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrsv2_solve(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    f: *const f32,
    x: *mut f32,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrsv2_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            f,
                            x,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrsv2_solve(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f64,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    f: *const f64,
    x: *mut f64,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrsv2_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            f,
                            x,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrsv2_solve(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    f: *const cuComplex,
    x: *mut cuComplex,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrsv2_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            f,
                            x,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrsv2_solve(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrsv2Info_t,
    f: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrsv2_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            mb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            info,
                            f,
                            x,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrmm(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f32,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrmm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transB,
                            mb,
                            n,
                            kb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockSize,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrmm(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f64,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrmm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transB,
                            mb,
                            n,
                            kb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockSize,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrmm(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrmm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transB,
                            mb,
                            n,
                            kb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockSize,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrmm(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrmm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transB,
                            mb,
                            n,
                            kb,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockSize,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXbsrsm2_zeroPivot(
    handle: cusparseHandle_t,
    info: bsrsm2Info_t,
    position: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXbsrsm2_zeroPivot {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, position) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrsm2_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrsm2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrsm2_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrsm2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrsm2_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrsm2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrsm2_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrsm2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrsm2_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrsm2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transB,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrsm2_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrsm2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transB,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrsm2_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrsm2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transB,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrsm2_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transB: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrsm2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transB,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrsm2_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrsm2_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrsm2_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrsm2_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrsm2_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrsm2_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrsm2_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrsm2_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrsm2_solve(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f32,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    X: *mut f32,
    ldx: ::std::os::raw::c_int,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrsm2_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            B,
                            ldb,
                            X,
                            ldx,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrsm2_solve(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const f64,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    X: *mut f64,
    ldx: ::std::os::raw::c_int,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrsm2_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            B,
                            ldb,
                            X,
                            ldx,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrsm2_solve(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    X: *mut cuComplex,
    ldx: ::std::os::raw::c_int,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrsm2_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            B,
                            ldb,
                            X,
                            ldx,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrsm2_solve(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    transA: cusparseOperation_t,
    transXY: cusparseOperation_t,
    mb: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrsm2Info_t,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    X: *mut cuDoubleComplex,
    ldx: ::std::os::raw::c_int,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrsm2_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            transA,
                            transXY,
                            mb,
                            n,
                            nnzb,
                            alpha,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            B,
                            ldb,
                            X,
                            ldx,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsrilu02_numericBoost(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    enable_boost: ::std::os::raw::c_int,
    tol: *mut f64,
    boost_val: *mut f32,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsrilu02_numericBoost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, info, enable_boost, tol, boost_val) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsrilu02_numericBoost(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    enable_boost: ::std::os::raw::c_int,
    tol: *mut f64,
    boost_val: *mut f64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsrilu02_numericBoost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, info, enable_boost, tol, boost_val) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsrilu02_numericBoost(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    enable_boost: ::std::os::raw::c_int,
    tol: *mut f64,
    boost_val: *mut cuComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsrilu02_numericBoost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, info, enable_boost, tol, boost_val) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsrilu02_numericBoost(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    enable_boost: ::std::os::raw::c_int,
    tol: *mut f64,
    boost_val: *mut cuDoubleComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsrilu02_numericBoost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, info, enable_boost, tol, boost_val) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcsrilu02_zeroPivot(
    handle: cusparseHandle_t,
    info: csrilu02Info_t,
    position: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcsrilu02_zeroPivot {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, position) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsrilu02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsrilu02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsrilu02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsrilu02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsrilu02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsrilu02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsrilu02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsrilu02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut f32,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsrilu02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedVal,
                            csrSortedRowPtr,
                            csrSortedColInd,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut f64,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsrilu02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedVal,
                            csrSortedRowPtr,
                            csrSortedColInd,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut cuComplex,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsrilu02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedVal,
                            csrSortedRowPtr,
                            csrSortedColInd,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut cuDoubleComplex,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsrilu02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedVal,
                            csrSortedRowPtr,
                            csrSortedColInd,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsrilu02_analysis(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsrilu02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsrilu02_analysis(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsrilu02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsrilu02_analysis(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsrilu02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsrilu02_analysis(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsrilu02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsrilu02(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA_valM: *mut f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsrilu02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA_valM,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsrilu02(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA_valM: *mut f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsrilu02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA_valM,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsrilu02(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA_valM: *mut cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsrilu02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA_valM,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsrilu02(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA_valM: *mut cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsrilu02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA_valM,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrilu02_numericBoost(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    enable_boost: ::std::os::raw::c_int,
    tol: *mut f64,
    boost_val: *mut f32,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrilu02_numericBoost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, info, enable_boost, tol, boost_val) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrilu02_numericBoost(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    enable_boost: ::std::os::raw::c_int,
    tol: *mut f64,
    boost_val: *mut f64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrilu02_numericBoost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, info, enable_boost, tol, boost_val) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrilu02_numericBoost(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    enable_boost: ::std::os::raw::c_int,
    tol: *mut f64,
    boost_val: *mut cuComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrilu02_numericBoost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, info, enable_boost, tol, boost_val) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrilu02_numericBoost(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    enable_boost: ::std::os::raw::c_int,
    tol: *mut f64,
    boost_val: *mut cuDoubleComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrilu02_numericBoost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, info, enable_boost, tol, boost_val) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXbsrilu02_zeroPivot(
    handle: cusparseHandle_t,
    info: bsrilu02Info_t,
    position: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXbsrilu02_zeroPivot {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, position) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrilu02_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrilu02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrilu02_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrilu02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrilu02_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrilu02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrilu02_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrilu02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrilu02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrilu02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrilu02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrilu02_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrilu02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrilu02_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrilu02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrilu02_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrilu02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrilu02_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrilu02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrilu02_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrilu02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsrilu02(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsrilu02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsrilu02(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsrilu02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsrilu02(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsrilu02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsrilu02(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsrilu02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsrilu02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcsric02_zeroPivot(
    handle: cusparseHandle_t,
    info: csric02Info_t,
    position: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcsric02_zeroPivot {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, position) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsric02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsric02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsric02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsric02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsric02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsric02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsric02_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *mut cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsric02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut f32,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsric02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedVal,
                            csrSortedRowPtr,
                            csrSortedColInd,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut f64,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsric02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedVal,
                            csrSortedRowPtr,
                            csrSortedColInd,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut cuComplex,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsric02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedVal,
                            csrSortedRowPtr,
                            csrSortedColInd,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedVal: *mut cuDoubleComplex,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    csrSortedColInd: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsric02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedVal,
                            csrSortedRowPtr,
                            csrSortedColInd,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsric02_analysis(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsric02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsric02_analysis(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsric02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsric02_analysis(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsric02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsric02_analysis(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsric02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsric02(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA_valM: *mut f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsric02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA_valM,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsric02(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA_valM: *mut f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsric02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA_valM,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsric02(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA_valM: *mut cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsric02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA_valM,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsric02(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA_valM: *mut cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    info: csric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsric02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA_valM,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXbsric02_zeroPivot(
    handle: cusparseHandle_t,
    info: bsric02Info_t,
    position: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXbsric02_zeroPivot {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, position) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsric02_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsric02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsric02_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsric02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsric02_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsric02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsric02_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsric02_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsric02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsric02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsric02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsric02_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockSize: ::std::os::raw::c_int,
    info: bsric02Info_t,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsric02_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockSize,
                            info,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsric02_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    pInputBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsric02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pInputBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsric02_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    pInputBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsric02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pInputBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsric02_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    pInputBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsric02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pInputBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsric02_analysis(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *const cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    pInputBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsric02_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pInputBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsric02(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsric02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsric02(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsric02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsric02(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsric02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsric02(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedVal: *mut cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    info: bsric02Info_t,
    policy: cusparseSolvePolicy_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsric02 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nnzb,
                            descrA,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            blockDim,
                            info,
                            policy,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgtsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgtsv2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgtsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgtsv2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgtsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgtsv2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgtsv2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgtsv2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgtsv2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgtsv2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, pBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgtsv2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgtsv2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, pBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgtsv2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgtsv2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, pBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgtsv2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgtsv2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, pBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgtsv2_nopivot_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgtsv2_nopivot_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgtsv2_nopivot_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgtsv2_nopivot_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgtsv2_nopivot_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgtsv2_nopivot_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgtsv2_nopivot_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgtsv2_nopivot_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, bufferSizeInBytes)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgtsv2_nopivot(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgtsv2_nopivot {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, pBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgtsv2_nopivot(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgtsv2_nopivot {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, pBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgtsv2_nopivot(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgtsv2_nopivot {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, pBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgtsv2_nopivot(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgtsv2_nopivot {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, dl, d, du, B, ldb, pBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgtsv2StridedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    x: *const f32,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgtsv2StridedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            batchStride,
                            bufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgtsv2StridedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    x: *const f64,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgtsv2StridedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            batchStride,
                            bufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgtsv2StridedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    x: *const cuComplex,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgtsv2StridedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            batchStride,
                            bufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgtsv2StridedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgtsv2StridedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            batchStride,
                            bufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgtsv2StridedBatch(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    x: *mut f32,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgtsv2StridedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            batchStride,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgtsv2StridedBatch(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    x: *mut f64,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgtsv2StridedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            batchStride,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgtsv2StridedBatch(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    x: *mut cuComplex,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgtsv2StridedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            batchStride,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgtsv2StridedBatch(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    batchStride: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgtsv2StridedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            batchStride,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgtsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    x: *const f32,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgtsvInterleavedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgtsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    x: *const f64,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgtsvInterleavedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgtsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    x: *const cuComplex,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgtsvInterleavedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgtsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgtsvInterleavedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            dl,
                            d,
                            du,
                            x,
                            batchCount,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgtsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *mut f32,
    d: *mut f32,
    du: *mut f32,
    x: *mut f32,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgtsvInterleavedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, algo, m, dl, d, du, x, batchCount, pBuffer)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgtsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *mut f64,
    d: *mut f64,
    du: *mut f64,
    x: *mut f64,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgtsvInterleavedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, algo, m, dl, d, du, x, batchCount, pBuffer)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgtsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *mut cuComplex,
    d: *mut cuComplex,
    du: *mut cuComplex,
    x: *mut cuComplex,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgtsvInterleavedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, algo, m, dl, d, du, x, batchCount, pBuffer)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgtsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    dl: *mut cuDoubleComplex,
    d: *mut cuDoubleComplex,
    du: *mut cuDoubleComplex,
    x: *mut cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgtsvInterleavedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, algo, m, dl, d, du, x, batchCount, pBuffer)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgpsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    ds: *const f32,
    dl: *const f32,
    d: *const f32,
    du: *const f32,
    dw: *const f32,
    x: *const f32,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgpsvInterleavedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            ds,
                            dl,
                            d,
                            du,
                            dw,
                            x,
                            batchCount,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgpsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    ds: *const f64,
    dl: *const f64,
    d: *const f64,
    du: *const f64,
    dw: *const f64,
    x: *const f64,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgpsvInterleavedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            ds,
                            dl,
                            d,
                            du,
                            dw,
                            x,
                            batchCount,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgpsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    ds: *const cuComplex,
    dl: *const cuComplex,
    d: *const cuComplex,
    du: *const cuComplex,
    dw: *const cuComplex,
    x: *const cuComplex,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgpsvInterleavedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            ds,
                            dl,
                            d,
                            du,
                            dw,
                            x,
                            batchCount,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgpsvInterleavedBatch_bufferSizeExt(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    ds: *const cuDoubleComplex,
    dl: *const cuDoubleComplex,
    d: *const cuDoubleComplex,
    du: *const cuDoubleComplex,
    dw: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgpsvInterleavedBatch_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            ds,
                            dl,
                            d,
                            du,
                            dw,
                            x,
                            batchCount,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgpsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    ds: *mut f32,
    dl: *mut f32,
    d: *mut f32,
    du: *mut f32,
    dw: *mut f32,
    x: *mut f32,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgpsvInterleavedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            ds,
                            dl,
                            d,
                            du,
                            dw,
                            x,
                            batchCount,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgpsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    ds: *mut f64,
    dl: *mut f64,
    d: *mut f64,
    du: *mut f64,
    dw: *mut f64,
    x: *mut f64,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgpsvInterleavedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            ds,
                            dl,
                            d,
                            du,
                            dw,
                            x,
                            batchCount,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgpsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    ds: *mut cuComplex,
    dl: *mut cuComplex,
    d: *mut cuComplex,
    du: *mut cuComplex,
    dw: *mut cuComplex,
    x: *mut cuComplex,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgpsvInterleavedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            ds,
                            dl,
                            d,
                            du,
                            dw,
                            x,
                            batchCount,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgpsvInterleavedBatch(
    handle: cusparseHandle_t,
    algo: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    ds: *mut cuDoubleComplex,
    dl: *mut cuDoubleComplex,
    d: *mut cuDoubleComplex,
    du: *mut cuDoubleComplex,
    dw: *mut cuDoubleComplex,
    x: *mut cuDoubleComplex,
    batchCount: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgpsvInterleavedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            algo,
                            m,
                            ds,
                            dl,
                            d,
                            du,
                            dw,
                            x,
                            batchCount,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsrgeam2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    beta: *const f32,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedValB: *const f32,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsrgeam2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            alpha,
                            descrA,
                            nnzA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            beta,
                            descrB,
                            nnzB,
                            csrSortedValB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsrgeam2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    beta: *const f64,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedValB: *const f64,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsrgeam2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            alpha,
                            descrA,
                            nnzA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            beta,
                            descrB,
                            nnzB,
                            csrSortedValB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsrgeam2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    beta: *const cuComplex,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedValB: *const cuComplex,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const cuComplex,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsrgeam2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            alpha,
                            descrA,
                            nnzA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            beta,
                            descrB,
                            nnzB,
                            csrSortedValB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsrgeam2_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedValB: *const cuDoubleComplex,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const cuDoubleComplex,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsrgeam2_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            alpha,
                            descrA,
                            nnzA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            beta,
                            descrB,
                            nnzB,
                            csrSortedValB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcsrgeam2Nnz(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    workspace: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcsrgeam2Nnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            descrA,
                            nnzA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            descrB,
                            nnzB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedRowPtrC,
                            nnzTotalDevHostPtr,
                            workspace,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsrgeam2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    beta: *const f32,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedValB: *const f32,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f32,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsrgeam2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            alpha,
                            descrA,
                            nnzA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            beta,
                            descrB,
                            nnzB,
                            csrSortedValB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsrgeam2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    beta: *const f64,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedValB: *const f64,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f64,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsrgeam2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            alpha,
                            descrA,
                            nnzA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            beta,
                            descrB,
                            nnzB,
                            csrSortedValB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsrgeam2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    beta: *const cuComplex,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedValB: *const cuComplex,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut cuComplex,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsrgeam2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            alpha,
                            descrA,
                            nnzA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            beta,
                            descrB,
                            nnzB,
                            csrSortedValB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsrgeam2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    descrA: cusparseMatDescr_t,
    nnzA: ::std::os::raw::c_int,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    descrB: cusparseMatDescr_t,
    nnzB: ::std::os::raw::c_int,
    csrSortedValB: *const cuDoubleComplex,
    csrSortedRowPtrB: *const ::std::os::raw::c_int,
    csrSortedColIndB: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut cuDoubleComplex,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsrgeam2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            alpha,
                            descrA,
                            nnzA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            beta,
                            descrB,
                            nnzB,
                            csrSortedValB,
                            csrSortedRowPtrB,
                            csrSortedColIndB,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsrcolor(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    fractionToColor: *const f32,
    ncolors: *mut ::std::os::raw::c_int,
    coloring: *mut ::std::os::raw::c_int,
    reordering: *mut ::std::os::raw::c_int,
    info: cusparseColorInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsrcolor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            fractionToColor,
                            ncolors,
                            coloring,
                            reordering,
                            info,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsrcolor(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    fractionToColor: *const f64,
    ncolors: *mut ::std::os::raw::c_int,
    coloring: *mut ::std::os::raw::c_int,
    reordering: *mut ::std::os::raw::c_int,
    info: cusparseColorInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsrcolor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            fractionToColor,
                            ncolors,
                            coloring,
                            reordering,
                            info,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsrcolor(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    fractionToColor: *const f32,
    ncolors: *mut ::std::os::raw::c_int,
    coloring: *mut ::std::os::raw::c_int,
    reordering: *mut ::std::os::raw::c_int,
    info: cusparseColorInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsrcolor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            fractionToColor,
                            ncolors,
                            coloring,
                            reordering,
                            info,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsrcolor(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    fractionToColor: *const f64,
    ncolors: *mut ::std::os::raw::c_int,
    coloring: *mut ::std::os::raw::c_int,
    reordering: *mut ::std::os::raw::c_int,
    info: cusparseColorInfo_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsrcolor {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            fractionToColor,
                            ncolors,
                            coloring,
                            reordering,
                            info,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSnnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    nnzPerRowCol: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSnnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            A,
                            lda,
                            nnzPerRowCol,
                            nnzTotalDevHostPtr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    nnzPerRowCol: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            A,
                            lda,
                            nnzPerRowCol,
                            nnzTotalDevHostPtr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCnnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    nnzPerRowCol: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCnnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            A,
                            lda,
                            nnzPerRowCol,
                            nnzTotalDevHostPtr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZnnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    nnzPerRowCol: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZnnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            A,
                            lda,
                            nnzPerRowCol,
                            nnzTotalDevHostPtr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSnnz_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    descr: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzPerRow: *mut ::std::os::raw::c_int,
    nnzC: *mut ::std::os::raw::c_int,
    tol: f32,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSnnz_compress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            descr,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            nnzPerRow,
                            nnzC,
                            tol,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnnz_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    descr: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzPerRow: *mut ::std::os::raw::c_int,
    nnzC: *mut ::std::os::raw::c_int,
    tol: f64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnnz_compress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            descr,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            nnzPerRow,
                            nnzC,
                            tol,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCnnz_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    descr: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzPerRow: *mut ::std::os::raw::c_int,
    nnzC: *mut ::std::os::raw::c_int,
    tol: cuComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCnnz_compress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            descr,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            nnzPerRow,
                            nnzC,
                            tol,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZnnz_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    descr: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzPerRow: *mut ::std::os::raw::c_int,
    nnzC: *mut ::std::os::raw::c_int,
    tol: cuDoubleComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZnnz_compress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            descr,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            nnzPerRow,
                            nnzC,
                            tol,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsr2csr_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    nnzPerRow: *const ::std::os::raw::c_int,
    csrSortedValC: *mut f32,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    tol: f32,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsr2csr_compress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedColIndA,
                            csrSortedRowPtrA,
                            nnzA,
                            nnzPerRow,
                            csrSortedValC,
                            csrSortedColIndC,
                            csrSortedRowPtrC,
                            tol,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsr2csr_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    nnzPerRow: *const ::std::os::raw::c_int,
    csrSortedValC: *mut f64,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    tol: f64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsr2csr_compress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedColIndA,
                            csrSortedRowPtrA,
                            nnzA,
                            nnzPerRow,
                            csrSortedValC,
                            csrSortedColIndC,
                            csrSortedRowPtrC,
                            tol,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsr2csr_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    nnzPerRow: *const ::std::os::raw::c_int,
    csrSortedValC: *mut cuComplex,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    tol: cuComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsr2csr_compress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedColIndA,
                            csrSortedRowPtrA,
                            nnzA,
                            nnzPerRow,
                            csrSortedValC,
                            csrSortedColIndC,
                            csrSortedRowPtrC,
                            tol,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsr2csr_compress(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    nnzPerRow: *const ::std::os::raw::c_int,
    csrSortedValC: *mut cuDoubleComplex,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    tol: cuDoubleComplex,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsr2csr_compress {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedColIndA,
                            csrSortedRowPtrA,
                            nnzA,
                            nnzPerRow,
                            csrSortedValC,
                            csrSortedColIndC,
                            csrSortedRowPtrC,
                            tol,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcoo2csr(
    handle: cusparseHandle_t,
    cooRowInd: *const ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    csrSortedRowPtr: *mut ::std::os::raw::c_int,
    idxBase: cusparseIndexBase_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcoo2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, cooRowInd, nnz, m, csrSortedRowPtr, idxBase)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcsr2coo(
    handle: cusparseHandle_t,
    csrSortedRowPtr: *const ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    cooRowInd: *mut ::std::os::raw::c_int,
    idxBase: cusparseIndexBase_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcsr2coo {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, csrSortedRowPtr, nnz, m, cooRowInd, idxBase)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcsr2bsrNnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcsr2bsrNnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            blockDim,
                            descrC,
                            bsrSortedRowPtrC,
                            nnzTotalDevHostPtr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsr2bsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut f32,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsr2bsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            blockDim,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsr2bsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut f64,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsr2bsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            blockDim,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsr2bsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut cuComplex,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsr2bsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            blockDim,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsr2bsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut cuDoubleComplex,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsr2bsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            blockDim,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSbsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f32,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSbsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDbsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f64,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDbsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCbsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut cuComplex,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCbsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZbsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    blockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut cuDoubleComplex,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZbsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            blockDim,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgebsr2gebsc_bufferSize(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgebsr2gebsc_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgebsr2gebsc_bufferSize(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgebsr2gebsc_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgebsr2gebsc_bufferSize(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgebsr2gebsc_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgebsr2gebsc_bufferSize(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgebsr2gebsc_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgebsr2gebsc_bufferSizeExt(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgebsr2gebsc_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgebsr2gebsc_bufferSizeExt(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgebsr2gebsc_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgebsr2gebsc_bufferSizeExt(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgebsr2gebsc_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgebsr2gebsc_bufferSizeExt(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgebsr2gebsc_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgebsr2gebsc(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const f32,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    bscVal: *mut f32,
    bscRowInd: *mut ::std::os::raw::c_int,
    bscColPtr: *mut ::std::os::raw::c_int,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgebsr2gebsc {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            bscVal,
                            bscRowInd,
                            bscColPtr,
                            copyValues,
                            idxBase,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgebsr2gebsc(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const f64,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    bscVal: *mut f64,
    bscRowInd: *mut ::std::os::raw::c_int,
    bscColPtr: *mut ::std::os::raw::c_int,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgebsr2gebsc {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            bscVal,
                            bscRowInd,
                            bscColPtr,
                            copyValues,
                            idxBase,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgebsr2gebsc(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const cuComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    bscVal: *mut cuComplex,
    bscRowInd: *mut ::std::os::raw::c_int,
    bscColPtr: *mut ::std::os::raw::c_int,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgebsr2gebsc {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            bscVal,
                            bscRowInd,
                            bscColPtr,
                            copyValues,
                            idxBase,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgebsr2gebsc(
    handle: cusparseHandle_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    bsrSortedVal: *const cuDoubleComplex,
    bsrSortedRowPtr: *const ::std::os::raw::c_int,
    bsrSortedColInd: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    bscVal: *mut cuDoubleComplex,
    bscRowInd: *mut ::std::os::raw::c_int,
    bscColPtr: *mut ::std::os::raw::c_int,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgebsr2gebsc {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            mb,
                            nb,
                            nnzb,
                            bsrSortedVal,
                            bsrSortedRowPtr,
                            bsrSortedColInd,
                            rowBlockDim,
                            colBlockDim,
                            bscVal,
                            bscRowInd,
                            bscColPtr,
                            copyValues,
                            idxBase,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXgebsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXgebsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            descrC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgebsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f32,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgebsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgebsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f64,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgebsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgebsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut cuComplex,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgebsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgebsr2csr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut cuDoubleComplex,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgebsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsr2gebsr_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsr2gebsr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsr2gebsr_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsr2gebsr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsr2gebsr_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsr2gebsr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsr2gebsr_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsr2gebsr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsr2gebsr_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsr2gebsr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsr2gebsr_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsr2gebsr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsr2gebsr_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsr2gebsr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsr2gebsr_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsr2gebsr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            rowBlockDim,
                            colBlockDim,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcsr2gebsrNnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcsr2gebsrNnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            descrC,
                            bsrSortedRowPtrC,
                            rowBlockDim,
                            colBlockDim,
                            nnzTotalDevHostPtr,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsr2gebsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut f32,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsr2gebsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                            rowBlockDim,
                            colBlockDim,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsr2gebsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut f64,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsr2gebsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                            rowBlockDim,
                            colBlockDim,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsr2gebsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut cuComplex,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsr2gebsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                            rowBlockDim,
                            colBlockDim,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsr2gebsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const cuDoubleComplex,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut cuDoubleComplex,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
    rowBlockDim: ::std::os::raw::c_int,
    colBlockDim: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsr2gebsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            m,
                            n,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                            rowBlockDim,
                            colBlockDim,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgebsr2gebsr_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgebsr2gebsr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            rowBlockDimC,
                            colBlockDimC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgebsr2gebsr_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgebsr2gebsr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            rowBlockDimC,
                            colBlockDimC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgebsr2gebsr_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgebsr2gebsr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            rowBlockDimC,
                            colBlockDimC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgebsr2gebsr_bufferSize(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgebsr2gebsr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            rowBlockDimC,
                            colBlockDimC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgebsr2gebsr_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgebsr2gebsr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            rowBlockDimC,
                            colBlockDimC,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgebsr2gebsr_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgebsr2gebsr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            rowBlockDimC,
                            colBlockDimC,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgebsr2gebsr_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgebsr2gebsr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            rowBlockDimC,
                            colBlockDimC,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgebsr2gebsr_bufferSizeExt(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgebsr2gebsr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            rowBlockDimC,
                            colBlockDimC,
                            pBufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXgebsr2gebsrNnz(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXgebsr2gebsrNnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            descrC,
                            bsrSortedRowPtrC,
                            rowBlockDimC,
                            colBlockDimC,
                            nnzTotalDevHostPtr,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSgebsr2gebsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f32,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut f32,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSgebsr2gebsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                            rowBlockDimC,
                            colBlockDimC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDgebsr2gebsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const f64,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut f64,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDgebsr2gebsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                            rowBlockDimC,
                            colBlockDimC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCgebsr2gebsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut cuComplex,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCgebsr2gebsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                            rowBlockDimC,
                            colBlockDimC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZgebsr2gebsr(
    handle: cusparseHandle_t,
    dirA: cusparseDirection_t,
    mb: ::std::os::raw::c_int,
    nb: ::std::os::raw::c_int,
    nnzb: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    bsrSortedValA: *const cuDoubleComplex,
    bsrSortedRowPtrA: *const ::std::os::raw::c_int,
    bsrSortedColIndA: *const ::std::os::raw::c_int,
    rowBlockDimA: ::std::os::raw::c_int,
    colBlockDimA: ::std::os::raw::c_int,
    descrC: cusparseMatDescr_t,
    bsrSortedValC: *mut cuDoubleComplex,
    bsrSortedRowPtrC: *mut ::std::os::raw::c_int,
    bsrSortedColIndC: *mut ::std::os::raw::c_int,
    rowBlockDimC: ::std::os::raw::c_int,
    colBlockDimC: ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZgebsr2gebsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            dirA,
                            mb,
                            nb,
                            nnzb,
                            descrA,
                            bsrSortedValA,
                            bsrSortedRowPtrA,
                            bsrSortedColIndA,
                            rowBlockDimA,
                            colBlockDimA,
                            descrC,
                            bsrSortedValC,
                            bsrSortedRowPtrC,
                            bsrSortedColIndC,
                            rowBlockDimC,
                            colBlockDimC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateIdentityPermutation(
    handle: cusparseHandle_t,
    n: ::std::os::raw::c_int,
    p: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateIdentityPermutation {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, p) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcoosort_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    cooRowsA: *const ::std::os::raw::c_int,
    cooColsA: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcoosort_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            cooRowsA,
                            cooColsA,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcoosortByRow(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    cooRowsA: *mut ::std::os::raw::c_int,
    cooColsA: *mut ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcoosortByRow {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, nnz, cooRowsA, cooColsA, P, pBuffer)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcoosortByColumn(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    cooRowsA: *mut ::std::os::raw::c_int,
    cooColsA: *mut ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcoosortByColumn {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, nnz, cooRowsA, cooColsA, P, pBuffer)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcsrsort_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcsrsort_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            csrRowPtrA,
                            csrColIndA,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcsrsort(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *mut ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcsrsort {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrRowPtrA,
                            csrColIndA,
                            P,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcscsort_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    cscColPtrA: *const ::std::os::raw::c_int,
    cscRowIndA: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcscsort_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            cscColPtrA,
                            cscRowIndA,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseXcscsort(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    cscColPtrA: *const ::std::os::raw::c_int,
    cscRowIndA: *mut ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseXcscsort {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            cscColPtrA,
                            cscRowIndA,
                            P,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsru2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *mut f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsru2csr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsru2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *mut f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsru2csr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsru2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *mut cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsru2csr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsru2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *mut cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsru2csr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsru2csr(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *mut f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsru2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsru2csr(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *mut f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsru2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsru2csr(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *mut cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsru2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsru2csr(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *mut cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsru2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScsr2csru(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *mut f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScsr2csru {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDcsr2csru(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *mut f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDcsr2csru {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCcsr2csru(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *mut cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCcsr2csru {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseZcsr2csru(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *mut cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *mut ::std::os::raw::c_int,
    info: csru2csrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseZcsr2csru {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneDense2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    threshold: *const f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneDense2csr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            threshold,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneDense2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    threshold: *const f64,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneDense2csr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            threshold,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneDense2csrNnz(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    threshold: *const f32,
    descrC: cusparseMatDescr_t,
    csrRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneDense2csrNnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            threshold,
                            descrC,
                            csrRowPtrC,
                            nnzTotalDevHostPtr,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneDense2csrNnz(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    threshold: *const f64,
    descrC: cusparseMatDescr_t,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneDense2csrNnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            threshold,
                            descrC,
                            csrSortedRowPtrC,
                            nnzTotalDevHostPtr,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneDense2csr(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    threshold: *const f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneDense2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            threshold,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneDense2csr(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    threshold: *const f64,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneDense2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            threshold,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneCsr2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    threshold: *const f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneCsr2csr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            threshold,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneCsr2csr_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    threshold: *const f64,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneCsr2csr_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            threshold,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneCsr2csrNnz(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    threshold: *const f32,
    descrC: cusparseMatDescr_t,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneCsr2csrNnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            threshold,
                            descrC,
                            csrSortedRowPtrC,
                            nnzTotalDevHostPtr,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneCsr2csrNnz(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    threshold: *const f64,
    descrC: cusparseMatDescr_t,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneCsr2csrNnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            threshold,
                            descrC,
                            csrSortedRowPtrC,
                            nnzTotalDevHostPtr,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneCsr2csr(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    threshold: *const f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneCsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            threshold,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneCsr2csr(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    threshold: *const f64,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneCsr2csr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            threshold,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneDense2csrByPercentage_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneDense2csrByPercentage_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            percentage,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneDense2csrByPercentage_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneDense2csrByPercentage_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            percentage,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneDense2csrNnzByPercentage(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneDense2csrNnzByPercentage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            percentage,
                            descrC,
                            csrRowPtrC,
                            nnzTotalDevHostPtr,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneDense2csrNnzByPercentage(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneDense2csrNnzByPercentage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            percentage,
                            descrC,
                            csrRowPtrC,
                            nnzTotalDevHostPtr,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneDense2csrByPercentage(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneDense2csrByPercentage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            percentage,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneDense2csrByPercentage(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneDense2csrByPercentage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            percentage,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneCsr2csrByPercentage_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneCsr2csrByPercentage_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            percentage,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneCsr2csrByPercentage_bufferSizeExt(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *const f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *const ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBufferSizeInBytes: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneCsr2csrByPercentage_bufferSizeExt {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            percentage,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            info,
                            pBufferSizeInBytes,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneCsr2csrNnzByPercentage(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneCsr2csrNnzByPercentage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            percentage,
                            descrC,
                            csrSortedRowPtrC,
                            nnzTotalDevHostPtr,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneCsr2csrNnzByPercentage(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedRowPtrC: *mut ::std::os::raw::c_int,
    nnzTotalDevHostPtr: *mut ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneCsr2csrNnzByPercentage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            percentage,
                            descrC,
                            csrSortedRowPtrC,
                            nnzTotalDevHostPtr,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpruneCsr2csrByPercentage(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f32,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f32,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpruneCsr2csrByPercentage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            percentage,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDpruneCsr2csrByPercentage(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrSortedValA: *const f64,
    csrSortedRowPtrA: *const ::std::os::raw::c_int,
    csrSortedColIndA: *const ::std::os::raw::c_int,
    percentage: f32,
    descrC: cusparseMatDescr_t,
    csrSortedValC: *mut f64,
    csrSortedRowPtrC: *const ::std::os::raw::c_int,
    csrSortedColIndC: *mut ::std::os::raw::c_int,
    info: pruneInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDpruneCsr2csrByPercentage {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrSortedValA,
                            csrSortedRowPtrA,
                            csrSortedColIndA,
                            percentage,
                            descrC,
                            csrSortedValC,
                            csrSortedRowPtrC,
                            csrSortedColIndC,
                            info,
                            pBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCsr2cscEx2(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *const ::std::os::raw::c_void,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    cscVal: *mut ::std::os::raw::c_void,
    cscColPtr: *mut ::std::os::raw::c_int,
    cscRowInd: *mut ::std::os::raw::c_int,
    valType: cudaDataType,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    alg: cusparseCsr2CscAlg_t,
    buffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCsr2cscEx2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            cscVal,
                            cscColPtr,
                            cscRowInd,
                            valType,
                            copyValues,
                            idxBase,
                            alg,
                            buffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCsr2cscEx2_bufferSize(
    handle: cusparseHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    csrVal: *const ::std::os::raw::c_void,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    cscVal: *mut ::std::os::raw::c_void,
    cscColPtr: *mut ::std::os::raw::c_int,
    cscRowInd: *mut ::std::os::raw::c_int,
    valType: cudaDataType,
    copyValues: cusparseAction_t,
    idxBase: cusparseIndexBase_t,
    alg: cusparseCsr2CscAlg_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCsr2cscEx2_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            cscVal,
                            cscColPtr,
                            cscRowInd,
                            valType,
                            copyValues,
                            idxBase,
                            alg,
                            bufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateSpVec(
    spVecDescr: *mut cusparseSpVecDescr_t,
    size: i64,
    nnz: i64,
    indices: *mut ::std::os::raw::c_void,
    values: *mut ::std::os::raw::c_void,
    idxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateSpVec {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spVecDescr,
                            size,
                            nnz,
                            indices,
                            values,
                            idxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstSpVec(
    spVecDescr: *mut cusparseConstSpVecDescr_t,
    size: i64,
    nnz: i64,
    indices: *const ::std::os::raw::c_void,
    values: *const ::std::os::raw::c_void,
    idxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstSpVec {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spVecDescr,
                            size,
                            nnz,
                            indices,
                            values,
                            idxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroySpVec(
    spVecDescr: cusparseConstSpVecDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroySpVec {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spVecDescr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpVecGet(
    spVecDescr: cusparseSpVecDescr_t,
    size: *mut i64,
    nnz: *mut i64,
    indices: *mut *mut ::std::os::raw::c_void,
    values: *mut *mut ::std::os::raw::c_void,
    idxType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpVecGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spVecDescr,
                            size,
                            nnz,
                            indices,
                            values,
                            idxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstSpVecGet(
    spVecDescr: cusparseConstSpVecDescr_t,
    size: *mut i64,
    nnz: *mut i64,
    indices: *mut *const ::std::os::raw::c_void,
    values: *mut *const ::std::os::raw::c_void,
    idxType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstSpVecGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spVecDescr,
                            size,
                            nnz,
                            indices,
                            values,
                            idxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpVecGetIndexBase(
    spVecDescr: cusparseConstSpVecDescr_t,
    idxBase: *mut cusparseIndexBase_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpVecGetIndexBase {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spVecDescr, idxBase) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpVecGetValues(
    spVecDescr: cusparseSpVecDescr_t,
    values: *mut *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpVecGetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spVecDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstSpVecGetValues(
    spVecDescr: cusparseConstSpVecDescr_t,
    values: *mut *const ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstSpVecGetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spVecDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpVecSetValues(
    spVecDescr: cusparseSpVecDescr_t,
    values: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpVecSetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spVecDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateDnVec(
    dnVecDescr: *mut cusparseDnVecDescr_t,
    size: i64,
    values: *mut ::std::os::raw::c_void,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateDnVec {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnVecDescr, size, values, valueType) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstDnVec(
    dnVecDescr: *mut cusparseConstDnVecDescr_t,
    size: i64,
    values: *const ::std::os::raw::c_void,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstDnVec {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnVecDescr, size, values, valueType) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyDnVec(
    dnVecDescr: cusparseConstDnVecDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyDnVec {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnVecDescr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnVecGet(
    dnVecDescr: cusparseDnVecDescr_t,
    size: *mut i64,
    values: *mut *mut ::std::os::raw::c_void,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnVecGet {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnVecDescr, size, values, valueType) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstDnVecGet(
    dnVecDescr: cusparseConstDnVecDescr_t,
    size: *mut i64,
    values: *mut *const ::std::os::raw::c_void,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstDnVecGet {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnVecDescr, size, values, valueType) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnVecGetValues(
    dnVecDescr: cusparseDnVecDescr_t,
    values: *mut *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnVecGetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnVecDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstDnVecGetValues(
    dnVecDescr: cusparseConstDnVecDescr_t,
    values: *mut *const ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstDnVecGetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnVecDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnVecSetValues(
    dnVecDescr: cusparseDnVecDescr_t,
    values: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnVecSetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnVecDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroySpMat(
    spMatDescr: cusparseConstSpMatDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroySpMat {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMatGetFormat(
    spMatDescr: cusparseConstSpMatDescr_t,
    format: *mut cusparseFormat_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMatGetFormat {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr, format) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMatGetIndexBase(
    spMatDescr: cusparseConstSpMatDescr_t,
    idxBase: *mut cusparseIndexBase_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMatGetIndexBase {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr, idxBase) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMatGetValues(
    spMatDescr: cusparseSpMatDescr_t,
    values: *mut *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMatGetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstSpMatGetValues(
    spMatDescr: cusparseConstSpMatDescr_t,
    values: *mut *const ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstSpMatGetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMatSetValues(
    spMatDescr: cusparseSpMatDescr_t,
    values: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMatSetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMatGetSize(
    spMatDescr: cusparseConstSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    nnz: *mut i64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMatGetSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr, rows, cols, nnz) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMatGetStridedBatch(
    spMatDescr: cusparseConstSpMatDescr_t,
    batchCount: *mut ::std::os::raw::c_int,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMatGetStridedBatch {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr, batchCount) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCooSetStridedBatch(
    spMatDescr: cusparseSpMatDescr_t,
    batchCount: ::std::os::raw::c_int,
    batchStride: i64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCooSetStridedBatch {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spMatDescr, batchCount, batchStride) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCsrSetStridedBatch(
    spMatDescr: cusparseSpMatDescr_t,
    batchCount: ::std::os::raw::c_int,
    offsetsBatchStride: i64,
    columnsValuesBatchStride: i64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCsrSetStridedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            batchCount,
                            offsetsBatchStride,
                            columnsValuesBatchStride,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseBsrSetStridedBatch(
    spMatDescr: cusparseSpMatDescr_t,
    batchCount: ::std::os::raw::c_int,
    offsetsBatchStride: i64,
    columnsBatchStride: i64,
    ValuesBatchStride: i64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseBsrSetStridedBatch {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            batchCount,
                            offsetsBatchStride,
                            columnsBatchStride,
                            ValuesBatchStride,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMatGetAttribute(
    spMatDescr: cusparseConstSpMatDescr_t,
    attribute: cusparseSpMatAttribute_t,
    data: *mut ::std::os::raw::c_void,
    dataSize: usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMatGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(spMatDescr, attribute, data, dataSize) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMatSetAttribute(
    spMatDescr: cusparseSpMatDescr_t,
    attribute: cusparseSpMatAttribute_t,
    data: *mut ::std::os::raw::c_void,
    dataSize: usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMatSetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(spMatDescr, attribute, data, dataSize) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateCsr(
    spMatDescr: *mut cusparseSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    csrRowOffsets: *mut ::std::os::raw::c_void,
    csrColInd: *mut ::std::os::raw::c_void,
    csrValues: *mut ::std::os::raw::c_void,
    csrRowOffsetsType: cusparseIndexType_t,
    csrColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateCsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            csrRowOffsets,
                            csrColInd,
                            csrValues,
                            csrRowOffsetsType,
                            csrColIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstCsr(
    spMatDescr: *mut cusparseConstSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    csrRowOffsets: *const ::std::os::raw::c_void,
    csrColInd: *const ::std::os::raw::c_void,
    csrValues: *const ::std::os::raw::c_void,
    csrRowOffsetsType: cusparseIndexType_t,
    csrColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstCsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            csrRowOffsets,
                            csrColInd,
                            csrValues,
                            csrRowOffsetsType,
                            csrColIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateCsc(
    spMatDescr: *mut cusparseSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    cscColOffsets: *mut ::std::os::raw::c_void,
    cscRowInd: *mut ::std::os::raw::c_void,
    cscValues: *mut ::std::os::raw::c_void,
    cscColOffsetsType: cusparseIndexType_t,
    cscRowIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateCsc {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            cscColOffsets,
                            cscRowInd,
                            cscValues,
                            cscColOffsetsType,
                            cscRowIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstCsc(
    spMatDescr: *mut cusparseConstSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    cscColOffsets: *const ::std::os::raw::c_void,
    cscRowInd: *const ::std::os::raw::c_void,
    cscValues: *const ::std::os::raw::c_void,
    cscColOffsetsType: cusparseIndexType_t,
    cscRowIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstCsc {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            cscColOffsets,
                            cscRowInd,
                            cscValues,
                            cscColOffsetsType,
                            cscRowIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCsrGet(
    spMatDescr: cusparseSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    nnz: *mut i64,
    csrRowOffsets: *mut *mut ::std::os::raw::c_void,
    csrColInd: *mut *mut ::std::os::raw::c_void,
    csrValues: *mut *mut ::std::os::raw::c_void,
    csrRowOffsetsType: *mut cusparseIndexType_t,
    csrColIndType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCsrGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            csrRowOffsets,
                            csrColInd,
                            csrValues,
                            csrRowOffsetsType,
                            csrColIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstCsrGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    nnz: *mut i64,
    csrRowOffsets: *mut *const ::std::os::raw::c_void,
    csrColInd: *mut *const ::std::os::raw::c_void,
    csrValues: *mut *const ::std::os::raw::c_void,
    csrRowOffsetsType: *mut cusparseIndexType_t,
    csrColIndType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstCsrGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            csrRowOffsets,
                            csrColInd,
                            csrValues,
                            csrRowOffsetsType,
                            csrColIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCscGet(
    spMatDescr: cusparseSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    nnz: *mut i64,
    cscColOffsets: *mut *mut ::std::os::raw::c_void,
    cscRowInd: *mut *mut ::std::os::raw::c_void,
    cscValues: *mut *mut ::std::os::raw::c_void,
    cscColOffsetsType: *mut cusparseIndexType_t,
    cscRowIndType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCscGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            cscColOffsets,
                            cscRowInd,
                            cscValues,
                            cscColOffsetsType,
                            cscRowIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstCscGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    nnz: *mut i64,
    cscColOffsets: *mut *const ::std::os::raw::c_void,
    cscRowInd: *mut *const ::std::os::raw::c_void,
    cscValues: *mut *const ::std::os::raw::c_void,
    cscColOffsetsType: *mut cusparseIndexType_t,
    cscRowIndType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstCscGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            cscColOffsets,
                            cscRowInd,
                            cscValues,
                            cscColOffsetsType,
                            cscRowIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCsrSetPointers(
    spMatDescr: cusparseSpMatDescr_t,
    csrRowOffsets: *mut ::std::os::raw::c_void,
    csrColInd: *mut ::std::os::raw::c_void,
    csrValues: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCsrSetPointers {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(spMatDescr, csrRowOffsets, csrColInd, csrValues)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCscSetPointers(
    spMatDescr: cusparseSpMatDescr_t,
    cscColOffsets: *mut ::std::os::raw::c_void,
    cscRowInd: *mut ::std::os::raw::c_void,
    cscValues: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCscSetPointers {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(spMatDescr, cscColOffsets, cscRowInd, cscValues)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateBsr(
    spMatDescr: *mut cusparseSpMatDescr_t,
    brows: i64,
    bcols: i64,
    bnnz: i64,
    rowBlockSize: i64,
    colBlockSize: i64,
    bsrRowOffsets: *mut ::std::os::raw::c_void,
    bsrColInd: *mut ::std::os::raw::c_void,
    bsrValues: *mut ::std::os::raw::c_void,
    bsrRowOffsetsType: cusparseIndexType_t,
    bsrColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
    order: cusparseOrder_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateBsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            brows,
                            bcols,
                            bnnz,
                            rowBlockSize,
                            colBlockSize,
                            bsrRowOffsets,
                            bsrColInd,
                            bsrValues,
                            bsrRowOffsetsType,
                            bsrColIndType,
                            idxBase,
                            valueType,
                            order,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstBsr(
    spMatDescr: *mut cusparseConstSpMatDescr_t,
    brows: i64,
    bcols: i64,
    bnnz: i64,
    rowBlockDim: i64,
    colBlockDim: i64,
    bsrRowOffsets: *const ::std::os::raw::c_void,
    bsrColInd: *const ::std::os::raw::c_void,
    bsrValues: *const ::std::os::raw::c_void,
    bsrRowOffsetsType: cusparseIndexType_t,
    bsrColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
    order: cusparseOrder_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstBsr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            brows,
                            bcols,
                            bnnz,
                            rowBlockDim,
                            colBlockDim,
                            bsrRowOffsets,
                            bsrColInd,
                            bsrValues,
                            bsrRowOffsetsType,
                            bsrColIndType,
                            idxBase,
                            valueType,
                            order,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateCoo(
    spMatDescr: *mut cusparseSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    cooRowInd: *mut ::std::os::raw::c_void,
    cooColInd: *mut ::std::os::raw::c_void,
    cooValues: *mut ::std::os::raw::c_void,
    cooIdxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateCoo {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            cooRowInd,
                            cooColInd,
                            cooValues,
                            cooIdxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstCoo(
    spMatDescr: *mut cusparseConstSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    cooRowInd: *const ::std::os::raw::c_void,
    cooColInd: *const ::std::os::raw::c_void,
    cooValues: *const ::std::os::raw::c_void,
    cooIdxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstCoo {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            cooRowInd,
                            cooColInd,
                            cooValues,
                            cooIdxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCooGet(
    spMatDescr: cusparseSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    nnz: *mut i64,
    cooRowInd: *mut *mut ::std::os::raw::c_void,
    cooColInd: *mut *mut ::std::os::raw::c_void,
    cooValues: *mut *mut ::std::os::raw::c_void,
    idxType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCooGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            cooRowInd,
                            cooColInd,
                            cooValues,
                            idxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstCooGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    nnz: *mut i64,
    cooRowInd: *mut *const ::std::os::raw::c_void,
    cooColInd: *mut *const ::std::os::raw::c_void,
    cooValues: *mut *const ::std::os::raw::c_void,
    idxType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstCooGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            cooRowInd,
                            cooColInd,
                            cooValues,
                            idxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCooSetPointers(
    spMatDescr: cusparseSpMatDescr_t,
    cooRows: *mut ::std::os::raw::c_void,
    cooColumns: *mut ::std::os::raw::c_void,
    cooValues: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCooSetPointers {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(spMatDescr, cooRows, cooColumns, cooValues) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateBlockedEll(
    spMatDescr: *mut cusparseSpMatDescr_t,
    rows: i64,
    cols: i64,
    ellBlockSize: i64,
    ellCols: i64,
    ellColInd: *mut ::std::os::raw::c_void,
    ellValue: *mut ::std::os::raw::c_void,
    ellIdxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateBlockedEll {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            ellBlockSize,
                            ellCols,
                            ellColInd,
                            ellValue,
                            ellIdxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstBlockedEll(
    spMatDescr: *mut cusparseConstSpMatDescr_t,
    rows: i64,
    cols: i64,
    ellBlockSize: i64,
    ellCols: i64,
    ellColInd: *const ::std::os::raw::c_void,
    ellValue: *const ::std::os::raw::c_void,
    ellIdxType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstBlockedEll {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            ellBlockSize,
                            ellCols,
                            ellColInd,
                            ellValue,
                            ellIdxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseBlockedEllGet(
    spMatDescr: cusparseSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    ellBlockSize: *mut i64,
    ellCols: *mut i64,
    ellColInd: *mut *mut ::std::os::raw::c_void,
    ellValue: *mut *mut ::std::os::raw::c_void,
    ellIdxType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseBlockedEllGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            ellBlockSize,
                            ellCols,
                            ellColInd,
                            ellValue,
                            ellIdxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstBlockedEllGet(
    spMatDescr: cusparseConstSpMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    ellBlockSize: *mut i64,
    ellCols: *mut i64,
    ellColInd: *mut *const ::std::os::raw::c_void,
    ellValue: *mut *const ::std::os::raw::c_void,
    ellIdxType: *mut cusparseIndexType_t,
    idxBase: *mut cusparseIndexBase_t,
    valueType: *mut cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstBlockedEllGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            ellBlockSize,
                            ellCols,
                            ellColInd,
                            ellValue,
                            ellIdxType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateSlicedEll(
    spMatDescr: *mut cusparseSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    sellValuesSize: i64,
    sliceSize: i64,
    sellSliceOffsets: *mut ::std::os::raw::c_void,
    sellColInd: *mut ::std::os::raw::c_void,
    sellValues: *mut ::std::os::raw::c_void,
    sellSliceOffsetsType: cusparseIndexType_t,
    sellColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateSlicedEll {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            sellValuesSize,
                            sliceSize,
                            sellSliceOffsets,
                            sellColInd,
                            sellValues,
                            sellSliceOffsetsType,
                            sellColIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstSlicedEll(
    spMatDescr: *mut cusparseConstSpMatDescr_t,
    rows: i64,
    cols: i64,
    nnz: i64,
    sellValuesSize: i64,
    sliceSize: i64,
    sellSliceOffsets: *const ::std::os::raw::c_void,
    sellColInd: *const ::std::os::raw::c_void,
    sellValues: *const ::std::os::raw::c_void,
    sellSliceOffsetsType: cusparseIndexType_t,
    sellColIndType: cusparseIndexType_t,
    idxBase: cusparseIndexBase_t,
    valueType: cudaDataType,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstSlicedEll {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            spMatDescr,
                            rows,
                            cols,
                            nnz,
                            sellValuesSize,
                            sliceSize,
                            sellSliceOffsets,
                            sellColInd,
                            sellValues,
                            sellSliceOffsetsType,
                            sellColIndType,
                            idxBase,
                            valueType,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateDnMat(
    dnMatDescr: *mut cusparseDnMatDescr_t,
    rows: i64,
    cols: i64,
    ld: i64,
    values: *mut ::std::os::raw::c_void,
    valueType: cudaDataType,
    order: cusparseOrder_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateDnMat {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(dnMatDescr, rows, cols, ld, values, valueType, order)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseCreateConstDnMat(
    dnMatDescr: *mut cusparseConstDnMatDescr_t,
    rows: i64,
    cols: i64,
    ld: i64,
    values: *const ::std::os::raw::c_void,
    valueType: cudaDataType,
    order: cusparseOrder_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseCreateConstDnMat {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(dnMatDescr, rows, cols, ld, values, valueType, order)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDestroyDnMat(
    dnMatDescr: cusparseConstDnMatDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDestroyDnMat {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnMatDescr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnMatGet(
    dnMatDescr: cusparseDnMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    ld: *mut i64,
    values: *mut *mut ::std::os::raw::c_void,
    type_: *mut cudaDataType,
    order: *mut cusparseOrder_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnMatGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(dnMatDescr, rows, cols, ld, values, type_, order)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstDnMatGet(
    dnMatDescr: cusparseConstDnMatDescr_t,
    rows: *mut i64,
    cols: *mut i64,
    ld: *mut i64,
    values: *mut *const ::std::os::raw::c_void,
    type_: *mut cudaDataType,
    order: *mut cusparseOrder_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstDnMatGet {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(dnMatDescr, rows, cols, ld, values, type_, order)
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnMatGetValues(
    dnMatDescr: cusparseDnMatDescr_t,
    values: *mut *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnMatGetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnMatDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseConstDnMatGetValues(
    dnMatDescr: cusparseConstDnMatDescr_t,
    values: *mut *const ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseConstDnMatGetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnMatDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnMatSetValues(
    dnMatDescr: cusparseDnMatDescr_t,
    values: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnMatSetValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnMatDescr, values) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnMatSetStridedBatch(
    dnMatDescr: cusparseDnMatDescr_t,
    batchCount: ::std::os::raw::c_int,
    batchStride: i64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnMatSetStridedBatch {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnMatDescr, batchCount, batchStride) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDnMatGetStridedBatch(
    dnMatDescr: cusparseConstDnMatDescr_t,
    batchCount: *mut ::std::os::raw::c_int,
    batchStride: *mut i64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDnMatGetStridedBatch {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dnMatDescr, batchCount, batchStride) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseAxpby(
    handle: cusparseHandle_t,
    alpha: *const ::std::os::raw::c_void,
    vecX: cusparseConstSpVecDescr_t,
    beta: *const ::std::os::raw::c_void,
    vecY: cusparseDnVecDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseAxpby {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, alpha, vecX, beta, vecY) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseGather(
    handle: cusparseHandle_t,
    vecY: cusparseConstDnVecDescr_t,
    vecX: cusparseSpVecDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseGather {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, vecY, vecX) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseScatter(
    handle: cusparseHandle_t,
    vecX: cusparseConstSpVecDescr_t,
    vecY: cusparseDnVecDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseScatter {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, vecX, vecY) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseRot(
    handle: cusparseHandle_t,
    c_coeff: *const ::std::os::raw::c_void,
    s_coeff: *const ::std::os::raw::c_void,
    vecX: cusparseSpVecDescr_t,
    vecY: cusparseDnVecDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseRot {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, c_coeff, s_coeff, vecX, vecY) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpVV_bufferSize(
    handle: cusparseHandle_t,
    opX: cusparseOperation_t,
    vecX: cusparseConstSpVecDescr_t,
    vecY: cusparseConstDnVecDescr_t,
    result: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpVV_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opX,
                            vecX,
                            vecY,
                            result,
                            computeType,
                            bufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpVV(
    handle: cusparseHandle_t,
    opX: cusparseOperation_t,
    vecX: cusparseConstSpVecDescr_t,
    vecY: cusparseConstDnVecDescr_t,
    result: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpVV {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opX,
                            vecX,
                            vecY,
                            result,
                            computeType,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSparseToDense_bufferSize(
    handle: cusparseHandle_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseDnMatDescr_t,
    alg: cusparseSparseToDenseAlg_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSparseToDense_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, matA, matB, alg, bufferSize) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSparseToDense(
    handle: cusparseHandle_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseDnMatDescr_t,
    alg: cusparseSparseToDenseAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSparseToDense {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, matA, matB, alg, externalBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDenseToSparse_bufferSize(
    handle: cusparseHandle_t,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseSpMatDescr_t,
    alg: cusparseDenseToSparseAlg_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDenseToSparse_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, matA, matB, alg, bufferSize) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDenseToSparse_analysis(
    handle: cusparseHandle_t,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseSpMatDescr_t,
    alg: cusparseDenseToSparseAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDenseToSparse_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, matA, matB, alg, externalBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseDenseToSparse_convert(
    handle: cusparseHandle_t,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseSpMatDescr_t,
    alg: cusparseDenseToSparseAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseDenseToSparse_convert {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, matA, matB, alg, externalBuffer) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMV(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    vecX: cusparseConstDnVecDescr_t,
    beta: *const ::std::os::raw::c_void,
    vecY: cusparseDnVecDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpMVAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMV {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            alpha,
                            matA,
                            vecX,
                            beta,
                            vecY,
                            computeType,
                            alg,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMV_bufferSize(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    vecX: cusparseConstDnVecDescr_t,
    beta: *const ::std::os::raw::c_void,
    vecY: cusparseDnVecDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpMVAlg_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMV_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            alpha,
                            matA,
                            vecX,
                            beta,
                            vecY,
                            computeType,
                            alg,
                            bufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMV_preprocess(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    vecX: cusparseConstDnVecDescr_t,
    beta: *const ::std::os::raw::c_void,
    vecY: cusparseDnVecDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpMVAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMV_preprocess {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            alpha,
                            matA,
                            vecX,
                            beta,
                            vecY,
                            computeType,
                            alg,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSV_createDescr(
    descr: *mut cusparseSpSVDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSV_createDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSV_destroyDescr(
    descr: cusparseSpSVDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSV_destroyDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSV_bufferSize(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    vecX: cusparseConstDnVecDescr_t,
    vecY: cusparseDnVecDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpSVAlg_t,
    spsvDescr: cusparseSpSVDescr_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSV_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            alpha,
                            matA,
                            vecX,
                            vecY,
                            computeType,
                            alg,
                            spsvDescr,
                            bufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSV_analysis(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    vecX: cusparseConstDnVecDescr_t,
    vecY: cusparseDnVecDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpSVAlg_t,
    spsvDescr: cusparseSpSVDescr_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSV_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            alpha,
                            matA,
                            vecX,
                            vecY,
                            computeType,
                            alg,
                            spsvDescr,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSV_solve(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    vecX: cusparseConstDnVecDescr_t,
    vecY: cusparseDnVecDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpSVAlg_t,
    spsvDescr: cusparseSpSVDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSV_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            alpha,
                            matA,
                            vecX,
                            vecY,
                            computeType,
                            alg,
                            spsvDescr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSV_updateMatrix(
    handle: cusparseHandle_t,
    spsvDescr: cusparseSpSVDescr_t,
    newValues: *mut ::std::os::raw::c_void,
    updatePart: cusparseSpSVUpdate_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSV_updateMatrix {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, spsvDescr, newValues, updatePart) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSM_createDescr(
    descr: *mut cusparseSpSMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSM_createDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSM_destroyDescr(
    descr: cusparseSpSMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSM_destroyDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSM_bufferSize(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    matC: cusparseDnMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpSMAlg_t,
    spsmDescr: cusparseSpSMDescr_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSM_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            matC,
                            computeType,
                            alg,
                            spsmDescr,
                            bufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSM_analysis(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    matC: cusparseDnMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpSMAlg_t,
    spsmDescr: cusparseSpSMDescr_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSM_analysis {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            matC,
                            computeType,
                            alg,
                            spsmDescr,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSM_solve(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    matC: cusparseDnMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpSMAlg_t,
    spsmDescr: cusparseSpSMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSM_solve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            matC,
                            computeType,
                            alg,
                            spsmDescr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpSM_updateMatrix(
    handle: cusparseHandle_t,
    spsmDescr: cusparseSpSMDescr_t,
    newValues: *mut ::std::os::raw::c_void,
    updatePart: cusparseSpSMUpdate_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpSM_updateMatrix {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, spsmDescr, newValues, updatePart) }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMM_bufferSize(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseDnMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpMMAlg_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMM_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            bufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMM_preprocess(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseDnMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpMMAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMM_preprocess {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMM(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseDnMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpMMAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMM {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMM_createDescr(
    descr: *mut cusparseSpGEMMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMM_createDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMM_destroyDescr(
    descr: cusparseSpGEMMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMM_destroyDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMM_workEstimation(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstSpMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpGEMMAlg_t,
    spgemmDescr: cusparseSpGEMMDescr_t,
    bufferSize1: *mut usize,
    externalBuffer1: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMM_workEstimation {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            spgemmDescr,
                            bufferSize1,
                            externalBuffer1,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMM_getNumProducts(
    spgemmDescr: cusparseSpGEMMDescr_t,
    num_prods: *mut i64,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMM_getNumProducts {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(spgemmDescr, num_prods) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMM_estimateMemory(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstSpMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpGEMMAlg_t,
    spgemmDescr: cusparseSpGEMMDescr_t,
    chunk_fraction: f32,
    bufferSize3: *mut usize,
    externalBuffer3: *mut ::std::os::raw::c_void,
    bufferSize2: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMM_estimateMemory {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            spgemmDescr,
                            chunk_fraction,
                            bufferSize3,
                            externalBuffer3,
                            bufferSize2,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMM_compute(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstSpMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpGEMMAlg_t,
    spgemmDescr: cusparseSpGEMMDescr_t,
    bufferSize2: *mut usize,
    externalBuffer2: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMM_compute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            spgemmDescr,
                            bufferSize2,
                            externalBuffer2,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMM_copy(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstSpMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpGEMMAlg_t,
    spgemmDescr: cusparseSpGEMMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMM_copy {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            spgemmDescr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMMreuse_workEstimation(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstSpMatDescr_t,
    matC: cusparseSpMatDescr_t,
    alg: cusparseSpGEMMAlg_t,
    spgemmDescr: cusparseSpGEMMDescr_t,
    bufferSize1: *mut usize,
    externalBuffer1: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMMreuse_workEstimation {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            matA,
                            matB,
                            matC,
                            alg,
                            spgemmDescr,
                            bufferSize1,
                            externalBuffer1,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMMreuse_nnz(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstSpMatDescr_t,
    matC: cusparseSpMatDescr_t,
    alg: cusparseSpGEMMAlg_t,
    spgemmDescr: cusparseSpGEMMDescr_t,
    bufferSize2: *mut usize,
    externalBuffer2: *mut ::std::os::raw::c_void,
    bufferSize3: *mut usize,
    externalBuffer3: *mut ::std::os::raw::c_void,
    bufferSize4: *mut usize,
    externalBuffer4: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMMreuse_nnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            matA,
                            matB,
                            matC,
                            alg,
                            spgemmDescr,
                            bufferSize2,
                            externalBuffer2,
                            bufferSize3,
                            externalBuffer3,
                            bufferSize4,
                            externalBuffer4,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMMreuse_copy(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstSpMatDescr_t,
    matC: cusparseSpMatDescr_t,
    alg: cusparseSpGEMMAlg_t,
    spgemmDescr: cusparseSpGEMMDescr_t,
    bufferSize5: *mut usize,
    externalBuffer5: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMMreuse_copy {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            matA,
                            matB,
                            matC,
                            alg,
                            spgemmDescr,
                            bufferSize5,
                            externalBuffer5,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEMMreuse_compute(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstSpMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpGEMMAlg_t,
    spgemmDescr: cusparseSpGEMMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEMMreuse_compute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            spgemmDescr,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEAM_createDescr(
    descr: *mut cusparseSpGEAMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEAM_createDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEAM_destroyDescr(
    descr: cusparseSpGEAMDescr_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEAM_destroyDescr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(descr) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEAM_bufferSize(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseSpMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matB: cusparseSpMatDescr_t,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpGEAMAlg_t,
    spgeamDescr: cusparseSpGEAMDescr_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEAM_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            beta,
                            matB,
                            matC,
                            computeType,
                            alg,
                            spgeamDescr,
                            bufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEAM_nnz(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseSpMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matB: cusparseSpMatDescr_t,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpGEAMAlg_t,
    spgeamDescr: cusparseSpGEAMDescr_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEAM_nnz {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            beta,
                            matB,
                            matC,
                            computeType,
                            alg,
                            spgeamDescr,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpGEAM(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseSpMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matB: cusparseSpMatDescr_t,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpGEAMAlg_t,
    spgeamDescr: cusparseSpGEAMDescr_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpGEAM {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            beta,
                            matB,
                            matC,
                            computeType,
                            alg,
                            spgeamDescr,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSDDMM_bufferSize(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSDDMMAlg_t,
    bufferSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSDDMM_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            bufferSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSDDMM_preprocess(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSDDMMAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSDDMM_preprocess {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSDDMM(
    handle: cusparseHandle_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    alpha: *const ::std::os::raw::c_void,
    matA: cusparseConstDnMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    beta: *const ::std::os::raw::c_void,
    matC: cusparseSpMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSDDMMAlg_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSDDMM {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            opA,
                            opB,
                            alpha,
                            matA,
                            matB,
                            beta,
                            matC,
                            computeType,
                            alg,
                            externalBuffer,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMMOp_createPlan(
    handle: cusparseHandle_t,
    plan: *mut cusparseSpMMOpPlan_t,
    opA: cusparseOperation_t,
    opB: cusparseOperation_t,
    matA: cusparseConstSpMatDescr_t,
    matB: cusparseConstDnMatDescr_t,
    matC: cusparseDnMatDescr_t,
    computeType: cudaDataType,
    alg: cusparseSpMMOpAlg_t,
    addOperationLtoirBuffer: *const ::std::os::raw::c_void,
    addOperationBufferSize: usize,
    mulOperationLtoirBuffer: *const ::std::os::raw::c_void,
    mulOperationBufferSize: usize,
    epilogueLtoirBuffer: *const ::std::os::raw::c_void,
    epilogueBufferSize: usize,
    SpMMWorkspaceSize: *mut usize,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMMOp_createPlan {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            plan,
                            opA,
                            opB,
                            matA,
                            matB,
                            matC,
                            computeType,
                            alg,
                            addOperationLtoirBuffer,
                            addOperationBufferSize,
                            mulOperationLtoirBuffer,
                            mulOperationBufferSize,
                            epilogueLtoirBuffer,
                            epilogueBufferSize,
                            SpMMWorkspaceSize,
                        )
                    }
                }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMMOp(
    plan: cusparseSpMMOpPlan_t,
    externalBuffer: *mut ::std::os::raw::c_void,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMMOp {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, externalBuffer) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusparseSpMMOp_destroyPlan(
    plan: cusparseSpMMOpPlan_t,
) -> cusparseStatus_t {
    match cusparse_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusparseSpMMOp_destroyPlan {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan) }
                Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusparseStatus_t_CUSPARSE_STATUS_NOT_INITIALIZED,
    }
}
