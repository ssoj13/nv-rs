#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCreate_v2(handle: *mut cublasHandle_t) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCreate_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDestroy_v2(handle: cublasHandle_t) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDestroy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetVersion_v2(
    handle: cublasHandle_t,
    version: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetVersion_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, version) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetProperty {
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
pub unsafe fn cublasGetCudartVersion() -> ::core::result::Result<
    usize,
    nv_loader::CallError,
> {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetCudartVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)() }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cublas",
                            stringify!(cublasGetCudartVersion),
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
pub unsafe extern "C" fn cublasSetWorkspace_v2(
    handle: cublasHandle_t,
    workspace: *mut ::std::os::raw::c_void,
    workspaceSizeInBytes: usize,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetWorkspace_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, workspace, workspaceSizeInBytes) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetStream_v2(
    handle: cublasHandle_t,
    streamId: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetStream_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetStream_v2(
    handle: cublasHandle_t,
    streamId: *mut cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetStream_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetPointerMode_v2(
    handle: cublasHandle_t,
    mode: *mut cublasPointerMode_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetPointerMode_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetPointerMode_v2(
    handle: cublasHandle_t,
    mode: cublasPointerMode_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetPointerMode_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetAtomicsMode(
    handle: cublasHandle_t,
    mode: *mut cublasAtomicsMode_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetAtomicsMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetAtomicsMode(
    handle: cublasHandle_t,
    mode: cublasAtomicsMode_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetAtomicsMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetMathMode(
    handle: cublasHandle_t,
    mode: *mut cublasMath_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetMathMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetMathMode(
    handle: cublasHandle_t,
    mode: cublasMath_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetMathMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetSmCountTarget(
    handle: cublasHandle_t,
    smCountTarget: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetSmCountTarget {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, smCountTarget) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetSmCountTarget(
    handle: cublasHandle_t,
    smCountTarget: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetSmCountTarget {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, smCountTarget) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetEmulationStrategy(
    handle: cublasHandle_t,
    emulationStrategy: *mut cublasEmulationStrategy_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetEmulationStrategy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, emulationStrategy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetEmulationStrategy(
    handle: cublasHandle_t,
    emulationStrategy: cublasEmulationStrategy_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetEmulationStrategy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, emulationStrategy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetEmulationSpecialValuesSupport(
    handle: cublasHandle_t,
    mask: *mut cudaEmulationSpecialValuesSupport,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetEmulationSpecialValuesSupport {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mask) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetEmulationSpecialValuesSupport(
    handle: cublasHandle_t,
    mask: cudaEmulationSpecialValuesSupport,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetEmulationSpecialValuesSupport {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mask) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetFixedPointEmulationMantissaControl(
    handle: cublasHandle_t,
    mantissaControl: *mut cudaEmulationMantissaControl,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetFixedPointEmulationMantissaControl {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaControl) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetFixedPointEmulationMantissaControl(
    handle: cublasHandle_t,
    mantissaControl: cudaEmulationMantissaControl,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetFixedPointEmulationMantissaControl {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaControl) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetFixedPointEmulationMaxMantissaBitCount(
    handle: cublasHandle_t,
    maxMantissaBitCount: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetFixedPointEmulationMaxMantissaBitCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, maxMantissaBitCount) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetFixedPointEmulationMaxMantissaBitCount(
    handle: cublasHandle_t,
    maxMantissaBitCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetFixedPointEmulationMaxMantissaBitCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, maxMantissaBitCount) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetFixedPointEmulationMantissaBitOffset(
    handle: cublasHandle_t,
    mantissaBitOffset: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetFixedPointEmulationMantissaBitOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaBitOffset) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetFixedPointEmulationMantissaBitOffset(
    handle: cublasHandle_t,
    mantissaBitOffset: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetFixedPointEmulationMantissaBitOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaBitOffset) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetFixedPointEmulationMantissaBitCountPointer(
    handle: cublasHandle_t,
    mantissaBitCount: *mut *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetFixedPointEmulationMantissaBitCountPointer {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaBitCount) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetFixedPointEmulationMantissaBitCountPointer(
    handle: cublasHandle_t,
    mantissaBitCount: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetFixedPointEmulationMantissaBitCountPointer {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaBitCount) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetStatusName(
    status: cublasStatus_t,
) -> *const ::std::os::raw::c_char {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetStatusName {
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
pub unsafe extern "C" fn cublasGetStatusString(
    status: cublasStatus_t,
) -> *const ::std::os::raw::c_char {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetStatusString {
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
pub unsafe extern "C" fn cublasLoggerConfigure(
    logIsOn: ::std::os::raw::c_int,
    logToStdOut: ::std::os::raw::c_int,
    logToStdErr: ::std::os::raw::c_int,
    logFileName: *const ::std::os::raw::c_char,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasLoggerConfigure {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(logIsOn, logToStdOut, logToStdErr, logFileName) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetLoggerCallback(
    userCallback: cublasLogCallback,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetLoggerCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(userCallback) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetLoggerCallback(
    userCallback: *mut cublasLogCallback,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetLoggerCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(userCallback) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetVector(
    n: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    incx: ::std::os::raw::c_int,
    devicePtr: *mut ::std::os::raw::c_void,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetVector {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(n, elemSize, x, incx, devicePtr, incy) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetVector_64(
    n: i64,
    elemSize: i64,
    x: *const ::std::os::raw::c_void,
    incx: i64,
    devicePtr: *mut ::std::os::raw::c_void,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetVector_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(n, elemSize, x, incx, devicePtr, incy) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetVector(
    n: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetVector {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(n, elemSize, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetVector_64(
    n: i64,
    elemSize: i64,
    x: *const ::std::os::raw::c_void,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetVector_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(n, elemSize, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetMatrix(
    rows: ::std::os::raw::c_int,
    cols: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    A: *const ::std::os::raw::c_void,
    lda: ::std::os::raw::c_int,
    B: *mut ::std::os::raw::c_void,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetMatrix {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rows, cols, elemSize, A, lda, B, ldb) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetMatrix_64(
    rows: i64,
    cols: i64,
    elemSize: i64,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetMatrix_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rows, cols, elemSize, A, lda, B, ldb) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetMatrix(
    rows: ::std::os::raw::c_int,
    cols: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    A: *const ::std::os::raw::c_void,
    lda: ::std::os::raw::c_int,
    B: *mut ::std::os::raw::c_void,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetMatrix {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rows, cols, elemSize, A, lda, B, ldb) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetMatrix_64(
    rows: i64,
    cols: i64,
    elemSize: i64,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetMatrix_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rows, cols, elemSize, A, lda, B, ldb) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetVectorAsync(
    n: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    hostPtr: *const ::std::os::raw::c_void,
    incx: ::std::os::raw::c_int,
    devicePtr: *mut ::std::os::raw::c_void,
    incy: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetVectorAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(n, elemSize, hostPtr, incx, devicePtr, incy, stream)
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
pub unsafe extern "C" fn cublasSetVectorAsync_64(
    n: i64,
    elemSize: i64,
    hostPtr: *const ::std::os::raw::c_void,
    incx: i64,
    devicePtr: *mut ::std::os::raw::c_void,
    incy: i64,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetVectorAsync_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(n, elemSize, hostPtr, incx, devicePtr, incy, stream)
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
pub unsafe extern "C" fn cublasGetVectorAsync(
    n: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    devicePtr: *const ::std::os::raw::c_void,
    incx: ::std::os::raw::c_int,
    hostPtr: *mut ::std::os::raw::c_void,
    incy: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetVectorAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(n, elemSize, devicePtr, incx, hostPtr, incy, stream)
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
pub unsafe extern "C" fn cublasGetVectorAsync_64(
    n: i64,
    elemSize: i64,
    devicePtr: *const ::std::os::raw::c_void,
    incx: i64,
    hostPtr: *mut ::std::os::raw::c_void,
    incy: i64,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetVectorAsync_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(n, elemSize, devicePtr, incx, hostPtr, incy, stream)
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
pub unsafe extern "C" fn cublasSetMatrixAsync(
    rows: ::std::os::raw::c_int,
    cols: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    A: *const ::std::os::raw::c_void,
    lda: ::std::os::raw::c_int,
    B: *mut ::std::os::raw::c_void,
    ldb: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetMatrixAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rows, cols, elemSize, A, lda, B, ldb, stream) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSetMatrixAsync_64(
    rows: i64,
    cols: i64,
    elemSize: i64,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSetMatrixAsync_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rows, cols, elemSize, A, lda, B, ldb, stream) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetMatrixAsync(
    rows: ::std::os::raw::c_int,
    cols: ::std::os::raw::c_int,
    elemSize: ::std::os::raw::c_int,
    A: *const ::std::os::raw::c_void,
    lda: ::std::os::raw::c_int,
    B: *mut ::std::os::raw::c_void,
    ldb: ::std::os::raw::c_int,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetMatrixAsync {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rows, cols, elemSize, A, lda, B, ldb, stream) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasGetMatrixAsync_64(
    rows: i64,
    cols: i64,
    elemSize: i64,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    stream: cudaStream_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGetMatrixAsync_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(rows, cols, elemSize, A, lda, B, ldb, stream) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn cublasXerbla(
    srName: *const ::std::os::raw::c_char,
    info: ::std::os::raw::c_int,
) -> ::core::result::Result<(), nv_loader::CallError> {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasXerbla {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(srName, info) };
                    ::core::result::Result::Ok(())
                }
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "cublas",
                            stringify!(cublasXerbla),
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
pub unsafe extern "C" fn cublasNrm2Ex(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasNrm2Ex {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            result,
                            resultType,
                            executionType,
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
pub unsafe extern "C" fn cublasNrm2Ex_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasNrm2Ex_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            result,
                            resultType,
                            executionType,
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
pub unsafe extern "C" fn cublasSnrm2_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSnrm2_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSnrm2_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f32,
    incx: i64,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSnrm2_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDnrm2_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDnrm2_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDnrm2_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f64,
    incx: i64,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDnrm2_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasScnrm2_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasScnrm2_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasScnrm2_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuComplex,
    incx: i64,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasScnrm2_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDznrm2_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDznrm2_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDznrm2_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDznrm2_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDotEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *const ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDotEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            result,
                            resultType,
                            executionType,
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
pub unsafe extern "C" fn cublasDotEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *const ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDotEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            result,
                            resultType,
                            executionType,
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
pub unsafe extern "C" fn cublasDotcEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *const ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDotcEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            result,
                            resultType,
                            executionType,
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
pub unsafe extern "C" fn cublasDotcEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *const ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDotcEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            result,
                            resultType,
                            executionType,
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
pub unsafe extern "C" fn cublasSdot_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    y: *const f32,
    incy: ::std::os::raw::c_int,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSdot_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSdot_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f32,
    incx: i64,
    y: *const f32,
    incy: i64,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSdot_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDdot_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    y: *const f64,
    incy: ::std::os::raw::c_int,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDdot_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDdot_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f64,
    incx: i64,
    y: *const f64,
    incy: i64,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDdot_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCdotu_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    result: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCdotu_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCdotu_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuComplex,
    incx: i64,
    y: *const cuComplex,
    incy: i64,
    result: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCdotu_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCdotc_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    result: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCdotc_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCdotc_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuComplex,
    incx: i64,
    y: *const cuComplex,
    incy: i64,
    result: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCdotc_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdotu_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    result: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdotu_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdotu_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *const cuDoubleComplex,
    incy: i64,
    result: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdotu_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdotc_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    result: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdotc_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdotc_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *const cuDoubleComplex,
    incy: i64,
    result: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdotc_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasScalEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    alphaType: cudaDataType,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasScalEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            alpha,
                            alphaType,
                            x,
                            xType,
                            incx,
                            executionType,
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
pub unsafe extern "C" fn cublasScalEx_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const ::std::os::raw::c_void,
    alphaType: cudaDataType,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    executionType: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasScalEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            alpha,
                            alphaType,
                            x,
                            xType,
                            incx,
                            executionType,
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
pub unsafe extern "C" fn cublasSscal_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSscal_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSscal_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const f32,
    x: *mut f32,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSscal_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDscal_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDscal_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDscal_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const f64,
    x: *mut f64,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDscal_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCscal_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCscal_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCscal_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const cuComplex,
    x: *mut cuComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCscal_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCsscal_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsscal_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCsscal_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const f32,
    x: *mut cuComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsscal_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZscal_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZscal_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZscal_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZscal_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdscal_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdscal_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdscal_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const f64,
    x: *mut cuDoubleComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdscal_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasAxpyEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    alphaType: cudaDataType,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasAxpyEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            alpha,
                            alphaType,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            executiontype,
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
pub unsafe extern "C" fn cublasAxpyEx_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const ::std::os::raw::c_void,
    alphaType: cudaDataType,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasAxpyEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            alpha,
                            alphaType,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            executiontype,
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
pub unsafe extern "C" fn cublasSaxpy_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSaxpy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSaxpy_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const f32,
    x: *const f32,
    incx: i64,
    y: *mut f32,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSaxpy_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDaxpy_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDaxpy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDaxpy_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const f64,
    x: *const f64,
    incx: i64,
    y: *mut f64,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDaxpy_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCaxpy_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCaxpy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCaxpy_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: i64,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCaxpy_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZaxpy_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZaxpy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZaxpy_v2_64(
    handle: cublasHandle_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZaxpy_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, alpha, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCopyEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCopyEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, x, xType, incx, y, yType, incy) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCopyEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCopyEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, x, xType, incx, y, yType, incy) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasScopy_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasScopy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasScopy_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f32,
    incx: i64,
    y: *mut f32,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasScopy_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDcopy_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDcopy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDcopy_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f64,
    incx: i64,
    y: *mut f64,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDcopy_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCcopy_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCcopy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCcopy_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuComplex,
    incx: i64,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCcopy_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZcopy_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZcopy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZcopy_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZcopy_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSswap_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSswap_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSswap_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut f32,
    incx: i64,
    y: *mut f32,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSswap_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDswap_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDswap_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDswap_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut f64,
    incx: i64,
    y: *mut f64,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDswap_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCswap_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCswap_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCswap_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut cuComplex,
    incx: i64,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCswap_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZswap_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZswap_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZswap_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut cuDoubleComplex,
    incx: i64,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZswap_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSwapEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSwapEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, x, xType, incx, y, yType, incy) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSwapEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSwapEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, x, xType, incx, y, yType, incy) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIsamax_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIsamax_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIsamax_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f32,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIsamax_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIdamax_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIdamax_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIdamax_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f64,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIdamax_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIcamax_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIcamax_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIcamax_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuComplex,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIcamax_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIzamax_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIzamax_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIzamax_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIzamax_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIamaxEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIamaxEx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, xType, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIamaxEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIamaxEx_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, xType, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIsamin_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIsamin_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIsamin_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f32,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIsamin_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIdamin_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIdamin_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIdamin_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f64,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIdamin_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIcamin_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIcamin_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIcamin_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuComplex,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIcamin_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIzamin_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIzamin_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIzamin_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIzamin_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIaminEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIaminEx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, xType, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasIaminEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    result: *mut i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasIaminEx_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, xType, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasAsumEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasAsumEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            result,
                            resultType,
                            executiontype,
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
pub unsafe extern "C" fn cublasAsumEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    result: *mut ::std::os::raw::c_void,
    resultType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasAsumEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            result,
                            resultType,
                            executiontype,
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
pub unsafe extern "C" fn cublasSasum_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSasum_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSasum_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f32,
    incx: i64,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSasum_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDasum_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDasum_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDasum_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const f64,
    incx: i64,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDasum_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasScasum_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasScasum_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasScasum_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuComplex,
    incx: i64,
    result: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasScasum_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDzasum_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDzasum_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDzasum_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    result: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDzasum_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, result) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSrot_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
    c: *const f32,
    s: *const f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSrot_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSrot_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut f32,
    incx: i64,
    y: *mut f32,
    incy: i64,
    c: *const f32,
    s: *const f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSrot_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDrot_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
    c: *const f64,
    s: *const f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDrot_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDrot_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut f64,
    incx: i64,
    y: *mut f64,
    incy: i64,
    c: *const f64,
    s: *const f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDrot_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCrot_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
    c: *const f32,
    s: *const cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCrot_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCrot_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut cuComplex,
    incx: i64,
    y: *mut cuComplex,
    incy: i64,
    c: *const f32,
    s: *const cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCrot_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCsrot_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
    c: *const f32,
    s: *const f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsrot_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCsrot_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut cuComplex,
    incx: i64,
    y: *mut cuComplex,
    incy: i64,
    c: *const f32,
    s: *const f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsrot_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZrot_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    c: *const f64,
    s: *const cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZrot_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZrot_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut cuDoubleComplex,
    incx: i64,
    y: *mut cuDoubleComplex,
    incy: i64,
    c: *const f64,
    s: *const cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZrot_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdrot_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    c: *const f64,
    s: *const f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdrot_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdrot_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut cuDoubleComplex,
    incx: i64,
    y: *mut cuDoubleComplex,
    incy: i64,
    c: *const f64,
    s: *const f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdrot_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasRotEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    c: *const ::std::os::raw::c_void,
    s: *const ::std::os::raw::c_void,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasRotEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            c,
                            s,
                            csType,
                            executiontype,
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
pub unsafe extern "C" fn cublasRotEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    c: *const ::std::os::raw::c_void,
    s: *const ::std::os::raw::c_void,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasRotEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            c,
                            s,
                            csType,
                            executiontype,
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
pub unsafe extern "C" fn cublasSrotg_v2(
    handle: cublasHandle_t,
    a: *mut f32,
    b: *mut f32,
    c: *mut f32,
    s: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSrotg_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, a, b, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDrotg_v2(
    handle: cublasHandle_t,
    a: *mut f64,
    b: *mut f64,
    c: *mut f64,
    s: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDrotg_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, a, b, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCrotg_v2(
    handle: cublasHandle_t,
    a: *mut cuComplex,
    b: *mut cuComplex,
    c: *mut f32,
    s: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCrotg_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, a, b, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZrotg_v2(
    handle: cublasHandle_t,
    a: *mut cuDoubleComplex,
    b: *mut cuDoubleComplex,
    c: *mut f64,
    s: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZrotg_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, a, b, c, s) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasRotgEx(
    handle: cublasHandle_t,
    a: *mut ::std::os::raw::c_void,
    b: *mut ::std::os::raw::c_void,
    abType: cudaDataType,
    c: *mut ::std::os::raw::c_void,
    s: *mut ::std::os::raw::c_void,
    csType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasRotgEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, a, b, abType, c, s, csType, executiontype)
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
pub unsafe extern "C" fn cublasSrotm_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
    param: *const f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSrotm_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, param) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSrotm_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut f32,
    incx: i64,
    y: *mut f32,
    incy: i64,
    param: *const f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSrotm_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, param) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDrotm_v2(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
    param: *const f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDrotm_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, param) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDrotm_v2_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut f64,
    incx: i64,
    y: *mut f64,
    incy: i64,
    param: *const f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDrotm_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, x, incx, y, incy, param) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasRotmEx(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: ::std::os::raw::c_int,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: ::std::os::raw::c_int,
    param: *const ::std::os::raw::c_void,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasRotmEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            param,
                            paramType,
                            executiontype,
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
pub unsafe extern "C" fn cublasRotmEx_64(
    handle: cublasHandle_t,
    n: i64,
    x: *mut ::std::os::raw::c_void,
    xType: cudaDataType,
    incx: i64,
    y: *mut ::std::os::raw::c_void,
    yType: cudaDataType,
    incy: i64,
    param: *const ::std::os::raw::c_void,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasRotmEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            x,
                            xType,
                            incx,
                            y,
                            yType,
                            incy,
                            param,
                            paramType,
                            executiontype,
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
pub unsafe extern "C" fn cublasSrotmg_v2(
    handle: cublasHandle_t,
    d1: *mut f32,
    d2: *mut f32,
    x1: *mut f32,
    y1: *const f32,
    param: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSrotmg_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, d1, d2, x1, y1, param) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDrotmg_v2(
    handle: cublasHandle_t,
    d1: *mut f64,
    d2: *mut f64,
    x1: *mut f64,
    y1: *const f64,
    param: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDrotmg_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, d1, d2, x1, y1, param) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasRotmgEx(
    handle: cublasHandle_t,
    d1: *mut ::std::os::raw::c_void,
    d1Type: cudaDataType,
    d2: *mut ::std::os::raw::c_void,
    d2Type: cudaDataType,
    x1: *mut ::std::os::raw::c_void,
    x1Type: cudaDataType,
    y1: *const ::std::os::raw::c_void,
    y1Type: cudaDataType,
    param: *mut ::std::os::raw::c_void,
    paramType: cudaDataType,
    executiontype: cudaDataType,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasRotmgEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            d1,
                            d1Type,
                            d2,
                            d2Type,
                            x1,
                            x1Type,
                            y1,
                            y1Type,
                            param,
                            paramType,
                            executiontype,
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
pub unsafe extern "C" fn cublasSgemv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasSgemv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    x: *const f32,
    incx: i64,
    beta: *const f32,
    y: *mut f32,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasDgemv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasDgemv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    x: *const f64,
    incx: i64,
    beta: *const f64,
    y: *mut f64,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasCgemv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasCgemv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    x: *const cuComplex,
    incx: i64,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZgemv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZgemv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasSgbmv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kl: ::std::os::raw::c_int,
    ku: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            kl,
                            ku,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasSgbmv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    x: *const f32,
    incx: i64,
    beta: *const f32,
    y: *mut f32,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            kl,
                            ku,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasDgbmv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kl: ::std::os::raw::c_int,
    ku: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            kl,
                            ku,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasDgbmv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    x: *const f64,
    incx: i64,
    beta: *const f64,
    y: *mut f64,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            kl,
                            ku,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasCgbmv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kl: ::std::os::raw::c_int,
    ku: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            kl,
                            ku,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasCgbmv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    x: *const cuComplex,
    incx: i64,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            kl,
                            ku,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZgbmv_v2(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    kl: ::std::os::raw::c_int,
    ku: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            kl,
                            ku,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZgbmv_v2_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    kl: i64,
    ku: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            kl,
                            ku,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasStrmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStrmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    A: *const f32,
    lda: i64,
    x: *mut f32,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtrmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtrmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    A: *const f64,
    lda: i64,
    x: *mut f64,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtrmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtrmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    A: *const cuComplex,
    lda: i64,
    x: *mut cuComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtrmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtrmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *mut cuDoubleComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasStbmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    k: i64,
    A: *const f32,
    lda: i64,
    x: *mut f32,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasDtbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasDtbmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    k: i64,
    A: *const f64,
    lda: i64,
    x: *mut f64,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasCtbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasCtbmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    k: i64,
    A: *const cuComplex,
    lda: i64,
    x: *mut cuComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasZtbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasZtbmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    k: i64,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *mut cuDoubleComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasStpmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    AP: *const f32,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStpmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStpmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    AP: *const f32,
    x: *mut f32,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStpmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtpmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    AP: *const f64,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtpmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtpmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    AP: *const f64,
    x: *mut f64,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtpmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtpmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    AP: *const cuComplex,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtpmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtpmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    AP: *const cuComplex,
    x: *mut cuComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtpmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtpmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    AP: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtpmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtpmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    AP: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtpmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStrsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStrsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    A: *const f32,
    lda: i64,
    x: *mut f32,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtrsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtrsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    A: *const f64,
    lda: i64,
    x: *mut f64,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtrsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtrsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    A: *const cuComplex,
    lda: i64,
    x: *mut cuComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtrsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtrsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *mut cuDoubleComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, A, lda, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStpsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    AP: *const f32,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStpsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStpsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    AP: *const f32,
    x: *mut f32,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStpsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtpsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    AP: *const f64,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtpsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtpsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    AP: *const f64,
    x: *mut f64,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtpsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtpsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    AP: *const cuComplex,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtpsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtpsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    AP: *const cuComplex,
    x: *mut cuComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtpsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtpsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    AP: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtpsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtpsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    AP: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtpsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, trans, diag, n, AP, x, incx) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStbsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *mut f32,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStbsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasStbsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    k: i64,
    A: *const f32,
    lda: i64,
    x: *mut f32,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStbsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasDtbsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *mut f64,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtbsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasDtbsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    k: i64,
    A: *const f64,
    lda: i64,
    x: *mut f64,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtbsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasCtbsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtbsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasCtbsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    k: i64,
    A: *const cuComplex,
    lda: i64,
    x: *mut cuComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtbsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasZtbsv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    incx: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtbsv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasZtbsv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    n: i64,
    k: i64,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *mut cuDoubleComplex,
    incx: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtbsv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, trans, diag, n, k, A, lda, x, incx)
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
pub unsafe extern "C" fn cublasSsymv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsymv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasSsymv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    x: *const f32,
    incx: i64,
    beta: *const f32,
    y: *mut f32,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsymv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasDsymv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsymv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasDsymv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    x: *const f64,
    incx: i64,
    beta: *const f64,
    y: *mut f64,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsymv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasCsymv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsymv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasCsymv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    x: *const cuComplex,
    incx: i64,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsymv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZsymv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsymv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZsymv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsymv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasChemv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChemv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasChemv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    x: *const cuComplex,
    incx: i64,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChemv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZhemv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhemv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZhemv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhemv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasSsbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasSsbmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    x: *const f32,
    incx: i64,
    beta: *const f32,
    y: *mut f32,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasDsbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasDsbmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    x: *const f64,
    incx: i64,
    beta: *const f64,
    y: *mut f64,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasChbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasChbmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    x: *const cuComplex,
    incx: i64,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZhbmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhbmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasZhbmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhbmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            x,
                            incx,
                            beta,
                            y,
                            incy,
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
pub unsafe extern "C" fn cublasSspmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    AP: *const f32,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSspmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, AP, x, incx, beta, y, incy)
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
pub unsafe extern "C" fn cublasSspmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f32,
    AP: *const f32,
    x: *const f32,
    incx: i64,
    beta: *const f32,
    y: *mut f32,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSspmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, AP, x, incx, beta, y, incy)
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
pub unsafe extern "C" fn cublasDspmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    AP: *const f64,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDspmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, AP, x, incx, beta, y, incy)
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
pub unsafe extern "C" fn cublasDspmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f64,
    AP: *const f64,
    x: *const f64,
    incx: i64,
    beta: *const f64,
    y: *mut f64,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDspmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, AP, x, incx, beta, y, incy)
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
pub unsafe extern "C" fn cublasChpmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    AP: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChpmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, AP, x, incx, beta, y, incy)
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
pub unsafe extern "C" fn cublasChpmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuComplex,
    AP: *const cuComplex,
    x: *const cuComplex,
    incx: i64,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChpmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, AP, x, incx, beta, y, incy)
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
pub unsafe extern "C" fn cublasZhpmv_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    AP: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhpmv_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, AP, x, incx, beta, y, incy)
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
pub unsafe extern "C" fn cublasZhpmv_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    AP: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhpmv_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, AP, x, incx, beta, y, incy)
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
pub unsafe extern "C" fn cublasSger_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    y: *const f32,
    incy: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSger_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSger_v2_64(
    handle: cublasHandle_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    x: *const f32,
    incx: i64,
    y: *const f32,
    incy: i64,
    A: *mut f32,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSger_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDger_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    y: *const f64,
    incy: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDger_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDger_v2_64(
    handle: cublasHandle_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    x: *const f64,
    incx: i64,
    y: *const f64,
    incy: i64,
    A: *mut f64,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDger_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCgeru_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgeru_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCgeru_v2_64(
    handle: cublasHandle_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: i64,
    y: *const cuComplex,
    incy: i64,
    A: *mut cuComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgeru_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCgerc_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgerc_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCgerc_v2_64(
    handle: cublasHandle_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: i64,
    y: *const cuComplex,
    incy: i64,
    A: *mut cuComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgerc_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZgeru_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgeru_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZgeru_v2_64(
    handle: cublasHandle_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *const cuDoubleComplex,
    incy: i64,
    A: *mut cuDoubleComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgeru_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZgerc_v2(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgerc_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZgerc_v2_64(
    handle: cublasHandle_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *const cuDoubleComplex,
    incy: i64,
    A: *mut cuDoubleComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgerc_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, m, n, alpha, x, incx, y, incy, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSsyr_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyr_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSsyr_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f32,
    x: *const f32,
    incx: i64,
    A: *mut f32,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyr_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDsyr_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyr_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDsyr_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f64,
    x: *const f64,
    incx: i64,
    A: *mut f64,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyr_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCsyr_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyr_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCsyr_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: i64,
    A: *mut cuComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyr_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZsyr_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyr_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZsyr_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: i64,
    A: *mut cuDoubleComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyr_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCher_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCher_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCher_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f32,
    x: *const cuComplex,
    incx: i64,
    A: *mut cuComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCher_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZher_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZher_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZher_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f64,
    x: *const cuDoubleComplex,
    incx: i64,
    A: *mut cuDoubleComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZher_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, A, lda) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSspr_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    AP: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSspr_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSspr_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f32,
    x: *const f32,
    incx: i64,
    AP: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSspr_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDspr_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    AP: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDspr_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDspr_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f64,
    x: *const f64,
    incx: i64,
    AP: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDspr_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasChpr_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    AP: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChpr_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasChpr_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f32,
    x: *const cuComplex,
    incx: i64,
    AP: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChpr_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZhpr_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    AP: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhpr_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZhpr_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f64,
    x: *const cuDoubleComplex,
    incx: i64,
    AP: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhpr_v2_64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSsyr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    y: *const f32,
    incy: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyr2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasSsyr2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f32,
    x: *const f32,
    incx: i64,
    y: *const f32,
    incy: i64,
    A: *mut f32,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyr2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasDsyr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    y: *const f64,
    incy: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyr2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasDsyr2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f64,
    x: *const f64,
    incx: i64,
    y: *const f64,
    incy: i64,
    A: *mut f64,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyr2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasCsyr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyr2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasCsyr2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: i64,
    y: *const cuComplex,
    incy: i64,
    A: *mut cuComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyr2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasZsyr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyr2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasZsyr2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *const cuDoubleComplex,
    incy: i64,
    A: *mut cuDoubleComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyr2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasCher2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCher2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasCher2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: i64,
    y: *const cuComplex,
    incy: i64,
    A: *mut cuComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCher2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasZher2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZher2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasZher2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *const cuDoubleComplex,
    incy: i64,
    A: *mut cuDoubleComplex,
    lda: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZher2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, A, lda)
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
pub unsafe extern "C" fn cublasSspr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    y: *const f32,
    incy: ::std::os::raw::c_int,
    AP: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSspr2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, AP) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSspr2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f32,
    x: *const f32,
    incx: i64,
    y: *const f32,
    incy: i64,
    AP: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSspr2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, AP) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDspr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    y: *const f64,
    incy: ::std::os::raw::c_int,
    AP: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDspr2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, AP) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDspr2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const f64,
    x: *const f64,
    incx: i64,
    y: *const f64,
    incy: i64,
    AP: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDspr2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, AP) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasChpr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuComplex,
    incy: ::std::os::raw::c_int,
    AP: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChpr2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, AP) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasChpr2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuComplex,
    x: *const cuComplex,
    incx: i64,
    y: *const cuComplex,
    incy: i64,
    AP: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChpr2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, AP) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZhpr2_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    y: *const cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    AP: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhpr2_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, AP) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZhpr2_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    alpha: *const cuDoubleComplex,
    x: *const cuDoubleComplex,
    incx: i64,
    y: *const cuDoubleComplex,
    incy: i64,
    AP: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhpr2_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, alpha, x, incx, y, incy, AP) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSgemvBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    Aarray: *const *const f32,
    lda: ::std::os::raw::c_int,
    xarray: *const *const f32,
    incx: ::std::os::raw::c_int,
    beta: *const f32,
    yarray: *const *mut f32,
    incy: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            Aarray,
                            lda,
                            xarray,
                            incx,
                            beta,
                            yarray,
                            incy,
                            batchCount,
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
pub unsafe extern "C" fn cublasSgemvBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    Aarray: *const *const f32,
    lda: i64,
    xarray: *const *const f32,
    incx: i64,
    beta: *const f32,
    yarray: *const *mut f32,
    incy: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemvBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            Aarray,
                            lda,
                            xarray,
                            incx,
                            beta,
                            yarray,
                            incy,
                            batchCount,
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
pub unsafe extern "C" fn cublasDgemvBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    Aarray: *const *const f64,
    lda: ::std::os::raw::c_int,
    xarray: *const *const f64,
    incx: ::std::os::raw::c_int,
    beta: *const f64,
    yarray: *const *mut f64,
    incy: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            Aarray,
                            lda,
                            xarray,
                            incx,
                            beta,
                            yarray,
                            incy,
                            batchCount,
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
pub unsafe extern "C" fn cublasDgemvBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    Aarray: *const *const f64,
    lda: i64,
    xarray: *const *const f64,
    incx: i64,
    beta: *const f64,
    yarray: *const *mut f64,
    incy: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemvBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            Aarray,
                            lda,
                            xarray,
                            incx,
                            beta,
                            yarray,
                            incy,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemvBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    xarray: *const *const cuComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuComplex,
    yarray: *const *mut cuComplex,
    incy: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            Aarray,
                            lda,
                            xarray,
                            incx,
                            beta,
                            yarray,
                            incy,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemvBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: i64,
    xarray: *const *const cuComplex,
    incx: i64,
    beta: *const cuComplex,
    yarray: *const *mut cuComplex,
    incy: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemvBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            Aarray,
                            lda,
                            xarray,
                            incx,
                            beta,
                            yarray,
                            incy,
                            batchCount,
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
pub unsafe extern "C" fn cublasZgemvBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    Aarray: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    xarray: *const *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    yarray: *const *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            Aarray,
                            lda,
                            xarray,
                            incx,
                            beta,
                            yarray,
                            incy,
                            batchCount,
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
pub unsafe extern "C" fn cublasZgemvBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    Aarray: *const *const cuDoubleComplex,
    lda: i64,
    xarray: *const *const cuDoubleComplex,
    incx: i64,
    beta: *const cuDoubleComplex,
    yarray: *const *mut cuDoubleComplex,
    incy: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemvBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            Aarray,
                            lda,
                            xarray,
                            incx,
                            beta,
                            yarray,
                            incy,
                            batchCount,
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
pub unsafe extern "C" fn cublasSgemvStridedBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    stridex: ::std::os::raw::c_longlong,
    beta: *const f32,
    y: *mut f32,
    incy: ::std::os::raw::c_int,
    stridey: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemvStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            strideA,
                            x,
                            incx,
                            stridex,
                            beta,
                            y,
                            incy,
                            stridey,
                            batchCount,
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
pub unsafe extern "C" fn cublasSgemvStridedBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    x: *const f32,
    incx: i64,
    stridex: ::std::os::raw::c_longlong,
    beta: *const f32,
    y: *mut f32,
    incy: i64,
    stridey: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemvStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            strideA,
                            x,
                            incx,
                            stridex,
                            beta,
                            y,
                            incy,
                            stridey,
                            batchCount,
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
pub unsafe extern "C" fn cublasDgemvStridedBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    stridex: ::std::os::raw::c_longlong,
    beta: *const f64,
    y: *mut f64,
    incy: ::std::os::raw::c_int,
    stridey: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemvStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            strideA,
                            x,
                            incx,
                            stridex,
                            beta,
                            y,
                            incy,
                            stridey,
                            batchCount,
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
pub unsafe extern "C" fn cublasDgemvStridedBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    x: *const f64,
    incx: i64,
    stridex: ::std::os::raw::c_longlong,
    beta: *const f64,
    y: *mut f64,
    incy: i64,
    stridey: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemvStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            strideA,
                            x,
                            incx,
                            stridex,
                            beta,
                            y,
                            incy,
                            stridey,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemvStridedBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    stridex: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: ::std::os::raw::c_int,
    stridey: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemvStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            strideA,
                            x,
                            incx,
                            stridex,
                            beta,
                            y,
                            incy,
                            stridey,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemvStridedBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    x: *const cuComplex,
    incx: i64,
    stridex: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    y: *mut cuComplex,
    incy: i64,
    stridey: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemvStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            strideA,
                            x,
                            incx,
                            stridex,
                            beta,
                            y,
                            incy,
                            stridey,
                            batchCount,
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
pub unsafe extern "C" fn cublasZgemvStridedBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    stridex: ::std::os::raw::c_longlong,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: ::std::os::raw::c_int,
    stridey: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemvStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            strideA,
                            x,
                            incx,
                            stridex,
                            beta,
                            y,
                            incy,
                            stridey,
                            batchCount,
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
pub unsafe extern "C" fn cublasZgemvStridedBatched_64(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    x: *const cuDoubleComplex,
    incx: i64,
    stridex: ::std::os::raw::c_longlong,
    beta: *const cuDoubleComplex,
    y: *mut cuDoubleComplex,
    incy: i64,
    stridey: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemvStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            strideA,
                            x,
                            incx,
                            stridex,
                            beta,
                            y,
                            incy,
                            stridey,
                            batchCount,
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
pub unsafe extern "C" fn cublasSgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSgemm_v2_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    B: *const f32,
    ldb: i64,
    beta: *const f32,
    C: *mut f32,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDgemm_v2_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    B: *const f64,
    ldb: i64,
    beta: *const f64,
    C: *mut f64,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCgemm_v2_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCgemm3m(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm3m {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCgemm3m_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm3m_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCgemm3mEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm3mEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            B,
                            Btype,
                            ldb,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCgemm3mEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm3mEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            B,
                            Btype,
                            ldb,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasZgemm_v2(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZgemm_v2_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZgemm3m(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemm3m {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZgemm3m_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemm3m_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSgemmEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemmEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            B,
                            Btype,
                            ldb,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasSgemmEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemmEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            B,
                            Btype,
                            ldb,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasGemmEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const ::std::os::raw::c_void,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGemmEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            B,
                            Btype,
                            ldb,
                            beta,
                            C,
                            Ctype,
                            ldc,
                            computeType,
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
pub unsafe extern "C" fn cublasGemmEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const ::std::os::raw::c_void,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGemmEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            B,
                            Btype,
                            ldb,
                            beta,
                            C,
                            Ctype,
                            ldc,
                            computeType,
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
pub unsafe extern "C" fn cublasCgemmEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemmEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            B,
                            Btype,
                            ldb,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCgemmEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemmEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            B,
                            Btype,
                            ldb,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasSsyrk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyrk_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSsyrk_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    beta: *const f32,
    C: *mut f32,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyrk_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDsyrk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyrk_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDsyrk_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    beta: *const f64,
    C: *mut f64,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyrk_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyrk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyrk_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyrk_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyrk_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZsyrk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyrk_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZsyrk_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyrk_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyrkEx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyrkEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyrkEx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyrkEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyrk3mEx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyrk3mEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyrk3mEx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    beta: *const cuComplex,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyrk3mEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCherk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCherk_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCherk_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const cuComplex,
    lda: i64,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCherk_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZherk_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZherk_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZherk_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f64,
    A: *const cuDoubleComplex,
    lda: i64,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZherk_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCherkEx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCherkEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCherkEx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCherkEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCherk3mEx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCherk3mEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasCherk3mEx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    beta: *const f32,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCherk3mEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            beta,
                            C,
                            Ctype,
                            ldc,
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
pub unsafe extern "C" fn cublasSsyr2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyr2k_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSsyr2k_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    B: *const f32,
    ldb: i64,
    beta: *const f32,
    C: *mut f32,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyr2k_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDsyr2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyr2k_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDsyr2k_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    B: *const f64,
    ldb: i64,
    beta: *const f64,
    C: *mut f64,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyr2k_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyr2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyr2k_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyr2k_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyr2k_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZsyr2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyr2k_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZsyr2k_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyr2k_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCher2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCher2k_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCher2k_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCher2k_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZher2k_v2(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZher2k_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZher2k_v2_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZher2k_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSsyrkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyrkx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSsyrkx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    B: *const f32,
    ldb: i64,
    beta: *const f32,
    C: *mut f32,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsyrkx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDsyrkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyrkx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDsyrkx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    B: *const f64,
    ldb: i64,
    beta: *const f64,
    C: *mut f64,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsyrkx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyrkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyrkx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsyrkx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsyrkx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZsyrkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyrkx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZsyrkx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsyrkx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCherkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCherkx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCherkx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const f32,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCherkx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZherkx(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZherkx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZherkx_64(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const f64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZherkx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            trans,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSsymm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsymm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSsymm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    B: *const f32,
    ldb: i64,
    beta: *const f32,
    C: *mut f32,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSsymm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDsymm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsymm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDsymm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    B: *const f64,
    ldb: i64,
    beta: *const f64,
    C: *mut f64,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDsymm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsymm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsymm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCsymm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCsymm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZsymm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsymm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZsymm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZsymm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasChemm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChemm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasChemm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasChemm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZhemm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhemm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZhemm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZhemm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            beta,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasStrsm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrsm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cublasStrsm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    B: *mut f32,
    ldb: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrsm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cublasDtrsm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrsm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cublasDtrsm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    B: *mut f64,
    ldb: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrsm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cublasCtrsm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrsm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cublasCtrsm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *mut cuComplex,
    ldb: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrsm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cublasZtrsm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrsm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cublasZtrsm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *mut cuDoubleComplex,
    ldb: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrsm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cublasStrmm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrmm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasStrmm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    B: *const f32,
    ldb: i64,
    C: *mut f32,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrmm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDtrmm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrmm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDtrmm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    B: *const f64,
    ldb: i64,
    C: *mut f64,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrmm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCtrmm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrmm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCtrmm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    B: *const cuComplex,
    ldb: i64,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrmm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZtrmm_v2(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrmm_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZtrmm_v2_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    B: *const cuDoubleComplex,
    ldb: i64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrmm_v2_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSgemmBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    Aarray: *const *const f32,
    lda: ::std::os::raw::c_int,
    Barray: *const *const f32,
    ldb: ::std::os::raw::c_int,
    beta: *const f32,
    Carray: *const *mut f32,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemmBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasSgemmBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f32,
    Aarray: *const *const f32,
    lda: i64,
    Barray: *const *const f32,
    ldb: i64,
    beta: *const f32,
    Carray: *const *mut f32,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemmBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasDgemmBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    Aarray: *const *const f64,
    lda: ::std::os::raw::c_int,
    Barray: *const *const f64,
    ldb: ::std::os::raw::c_int,
    beta: *const f64,
    Carray: *const *mut f64,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemmBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasDgemmBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f64,
    Aarray: *const *const f64,
    lda: i64,
    Barray: *const *const f64,
    ldb: i64,
    beta: *const f64,
    Carray: *const *mut f64,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemmBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemmBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    Barray: *const *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    Carray: *const *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemmBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemmBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: i64,
    Barray: *const *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    Carray: *const *mut cuComplex,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemmBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemm3mBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    Barray: *const *const cuComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuComplex,
    Carray: *const *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm3mBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemm3mBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    Aarray: *const *const cuComplex,
    lda: i64,
    Barray: *const *const cuComplex,
    ldb: i64,
    beta: *const cuComplex,
    Carray: *const *mut cuComplex,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm3mBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasZgemmBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    Aarray: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Barray: *const *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    Carray: *const *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemmBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasZgemmBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    Aarray: *const *const cuDoubleComplex,
    lda: i64,
    Barray: *const *const cuDoubleComplex,
    ldb: i64,
    beta: *const cuDoubleComplex,
    Carray: *const *mut cuDoubleComplex,
    ldc: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemmBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            lda,
                            Barray,
                            ldb,
                            beta,
                            Carray,
                            ldc,
                            batchCount,
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
pub unsafe extern "C" fn cublasSgemmStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemmStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasSgemmStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const f32,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const f32,
    C: *mut f32,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemmStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasDgemmStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemmStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasDgemmStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const f64,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const f64,
    C: *mut f64,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemmStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemmStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemmStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemmStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuComplex,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemmStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemm3mStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm3mStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasCgemm3mStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuComplex,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuComplex,
    C: *mut cuComplex,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgemm3mStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasZgemmStridedBatched(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemmStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasZgemmStridedBatched_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const cuDoubleComplex,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgemmStridedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            lda,
                            strideA,
                            B,
                            ldb,
                            strideB,
                            beta,
                            C,
                            ldc,
                            strideC,
                            batchCount,
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
pub unsafe extern "C" fn cublasGemmBatchedEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    Aarray: *const *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    Barray: *const *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    beta: *const ::std::os::raw::c_void,
    Carray: *const *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGemmBatchedEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            Atype,
                            lda,
                            Barray,
                            Btype,
                            ldb,
                            beta,
                            Carray,
                            Ctype,
                            ldc,
                            batchCount,
                            computeType,
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
pub unsafe extern "C" fn cublasGemmBatchedEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const ::std::os::raw::c_void,
    Aarray: *const *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    Barray: *const *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    beta: *const ::std::os::raw::c_void,
    Carray: *const *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
    batchCount: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGemmBatchedEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            Aarray,
                            Atype,
                            lda,
                            Barray,
                            Btype,
                            ldb,
                            beta,
                            Carray,
                            Ctype,
                            ldc,
                            batchCount,
                            computeType,
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
pub unsafe extern "C" fn cublasGemmStridedBatchedEx(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: ::std::os::raw::c_int,
    strideB: ::std::os::raw::c_longlong,
    beta: *const ::std::os::raw::c_void,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: ::std::os::raw::c_int,
    strideC: ::std::os::raw::c_longlong,
    batchCount: ::std::os::raw::c_int,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGemmStridedBatchedEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            strideA,
                            B,
                            Btype,
                            ldb,
                            strideB,
                            beta,
                            C,
                            Ctype,
                            ldc,
                            strideC,
                            batchCount,
                            computeType,
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
pub unsafe extern "C" fn cublasGemmStridedBatchedEx_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    k: i64,
    alpha: *const ::std::os::raw::c_void,
    A: *const ::std::os::raw::c_void,
    Atype: cudaDataType,
    lda: i64,
    strideA: ::std::os::raw::c_longlong,
    B: *const ::std::os::raw::c_void,
    Btype: cudaDataType,
    ldb: i64,
    strideB: ::std::os::raw::c_longlong,
    beta: *const ::std::os::raw::c_void,
    C: *mut ::std::os::raw::c_void,
    Ctype: cudaDataType,
    ldc: i64,
    strideC: ::std::os::raw::c_longlong,
    batchCount: i64,
    computeType: cublasComputeType_t,
    algo: cublasGemmAlgo_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGemmStridedBatchedEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            k,
                            alpha,
                            A,
                            Atype,
                            lda,
                            strideA,
                            B,
                            Btype,
                            ldb,
                            strideB,
                            beta,
                            C,
                            Ctype,
                            ldc,
                            strideC,
                            batchCount,
                            computeType,
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
pub unsafe extern "C" fn cublasSgemmGroupedBatched(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const ::std::os::raw::c_int,
    n_array: *const ::std::os::raw::c_int,
    k_array: *const ::std::os::raw::c_int,
    alpha_array: *const f32,
    Aarray: *const *const f32,
    lda_array: *const ::std::os::raw::c_int,
    Barray: *const *const f32,
    ldb_array: *const ::std::os::raw::c_int,
    beta_array: *const f32,
    Carray: *const *mut f32,
    ldc_array: *const ::std::os::raw::c_int,
    group_count: ::std::os::raw::c_int,
    group_size: *const ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemmGroupedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa_array,
                            transb_array,
                            m_array,
                            n_array,
                            k_array,
                            alpha_array,
                            Aarray,
                            lda_array,
                            Barray,
                            ldb_array,
                            beta_array,
                            Carray,
                            ldc_array,
                            group_count,
                            group_size,
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
pub unsafe extern "C" fn cublasSgemmGroupedBatched_64(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const i64,
    n_array: *const i64,
    k_array: *const i64,
    alpha_array: *const f32,
    Aarray: *const *const f32,
    lda_array: *const i64,
    Barray: *const *const f32,
    ldb_array: *const i64,
    beta_array: *const f32,
    Carray: *const *mut f32,
    ldc_array: *const i64,
    group_count: i64,
    group_size: *const i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgemmGroupedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa_array,
                            transb_array,
                            m_array,
                            n_array,
                            k_array,
                            alpha_array,
                            Aarray,
                            lda_array,
                            Barray,
                            ldb_array,
                            beta_array,
                            Carray,
                            ldc_array,
                            group_count,
                            group_size,
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
pub unsafe extern "C" fn cublasDgemmGroupedBatched(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const ::std::os::raw::c_int,
    n_array: *const ::std::os::raw::c_int,
    k_array: *const ::std::os::raw::c_int,
    alpha_array: *const f64,
    Aarray: *const *const f64,
    lda_array: *const ::std::os::raw::c_int,
    Barray: *const *const f64,
    ldb_array: *const ::std::os::raw::c_int,
    beta_array: *const f64,
    Carray: *const *mut f64,
    ldc_array: *const ::std::os::raw::c_int,
    group_count: ::std::os::raw::c_int,
    group_size: *const ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemmGroupedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa_array,
                            transb_array,
                            m_array,
                            n_array,
                            k_array,
                            alpha_array,
                            Aarray,
                            lda_array,
                            Barray,
                            ldb_array,
                            beta_array,
                            Carray,
                            ldc_array,
                            group_count,
                            group_size,
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
pub unsafe extern "C" fn cublasDgemmGroupedBatched_64(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const i64,
    n_array: *const i64,
    k_array: *const i64,
    alpha_array: *const f64,
    Aarray: *const *const f64,
    lda_array: *const i64,
    Barray: *const *const f64,
    ldb_array: *const i64,
    beta_array: *const f64,
    Carray: *const *mut f64,
    ldc_array: *const i64,
    group_count: i64,
    group_size: *const i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgemmGroupedBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa_array,
                            transb_array,
                            m_array,
                            n_array,
                            k_array,
                            alpha_array,
                            Aarray,
                            lda_array,
                            Barray,
                            ldb_array,
                            beta_array,
                            Carray,
                            ldc_array,
                            group_count,
                            group_size,
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
pub unsafe extern "C" fn cublasGemmGroupedBatchedEx(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const ::std::os::raw::c_int,
    n_array: *const ::std::os::raw::c_int,
    k_array: *const ::std::os::raw::c_int,
    alpha_array: *const ::std::os::raw::c_void,
    Aarray: *const *const ::std::os::raw::c_void,
    Atype: cudaDataType_t,
    lda_array: *const ::std::os::raw::c_int,
    Barray: *const *const ::std::os::raw::c_void,
    Btype: cudaDataType_t,
    ldb_array: *const ::std::os::raw::c_int,
    beta_array: *const ::std::os::raw::c_void,
    Carray: *const *mut ::std::os::raw::c_void,
    Ctype: cudaDataType_t,
    ldc_array: *const ::std::os::raw::c_int,
    group_count: ::std::os::raw::c_int,
    group_size: *const ::std::os::raw::c_int,
    computeType: cublasComputeType_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGemmGroupedBatchedEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa_array,
                            transb_array,
                            m_array,
                            n_array,
                            k_array,
                            alpha_array,
                            Aarray,
                            Atype,
                            lda_array,
                            Barray,
                            Btype,
                            ldb_array,
                            beta_array,
                            Carray,
                            Ctype,
                            ldc_array,
                            group_count,
                            group_size,
                            computeType,
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
pub unsafe extern "C" fn cublasGemmGroupedBatchedEx_64(
    handle: cublasHandle_t,
    transa_array: *const cublasOperation_t,
    transb_array: *const cublasOperation_t,
    m_array: *const i64,
    n_array: *const i64,
    k_array: *const i64,
    alpha_array: *const ::std::os::raw::c_void,
    Aarray: *const *const ::std::os::raw::c_void,
    Atype: cudaDataType_t,
    lda_array: *const i64,
    Barray: *const *const ::std::os::raw::c_void,
    Btype: cudaDataType_t,
    ldb_array: *const i64,
    beta_array: *const ::std::os::raw::c_void,
    Carray: *const *mut ::std::os::raw::c_void,
    Ctype: cudaDataType_t,
    ldc_array: *const i64,
    group_count: i64,
    group_size: *const i64,
    computeType: cublasComputeType_t,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasGemmGroupedBatchedEx_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa_array,
                            transb_array,
                            m_array,
                            n_array,
                            k_array,
                            alpha_array,
                            Aarray,
                            Atype,
                            lda_array,
                            Barray,
                            Btype,
                            ldb_array,
                            beta_array,
                            Carray,
                            Ctype,
                            ldc_array,
                            group_count,
                            group_size,
                            computeType,
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
pub unsafe extern "C" fn cublasSgeam(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    beta: *const f32,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgeam {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            beta,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasSgeam_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const f32,
    lda: i64,
    beta: *const f32,
    B: *const f32,
    ldb: i64,
    C: *mut f32,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgeam_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            beta,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDgeam(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    beta: *const f64,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgeam {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            beta,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasDgeam_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const f64,
    lda: i64,
    beta: *const f64,
    B: *const f64,
    ldb: i64,
    C: *mut f64,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgeam_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            beta,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCgeam(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    beta: *const cuComplex,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgeam {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            beta,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasCgeam_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const cuComplex,
    lda: i64,
    beta: *const cuComplex,
    B: *const cuComplex,
    ldb: i64,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgeam_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            beta,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZgeam(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    beta: *const cuDoubleComplex,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgeam {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            beta,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasZgeam_64(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const cuDoubleComplex,
    lda: i64,
    beta: *const cuDoubleComplex,
    B: *const cuDoubleComplex,
    ldb: i64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgeam_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            beta,
                            B,
                            ldb,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cublasStrsmBatched(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f32,
    A: *const *const f32,
    lda: ::std::os::raw::c_int,
    B: *const *mut f32,
    ldb: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrsmBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            batchCount,
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
pub unsafe extern "C" fn cublasStrsmBatched_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const f32,
    A: *const *const f32,
    lda: i64,
    B: *const *mut f32,
    ldb: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrsmBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            batchCount,
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
pub unsafe extern "C" fn cublasDtrsmBatched(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const f64,
    A: *const *const f64,
    lda: ::std::os::raw::c_int,
    B: *const *mut f64,
    ldb: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrsmBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            batchCount,
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
pub unsafe extern "C" fn cublasDtrsmBatched_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const f64,
    A: *const *const f64,
    lda: i64,
    B: *const *mut f64,
    ldb: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrsmBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            batchCount,
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
pub unsafe extern "C" fn cublasCtrsmBatched(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuComplex,
    A: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrsmBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            batchCount,
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
pub unsafe extern "C" fn cublasCtrsmBatched_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuComplex,
    A: *const *const cuComplex,
    lda: i64,
    B: *const *mut cuComplex,
    ldb: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrsmBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            batchCount,
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
pub unsafe extern "C" fn cublasZtrsmBatched(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    alpha: *const cuDoubleComplex,
    A: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    batchCount: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrsmBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            batchCount,
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
pub unsafe extern "C" fn cublasZtrsmBatched_64(
    handle: cublasHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    diag: cublasDiagType_t,
    m: i64,
    n: i64,
    alpha: *const cuDoubleComplex,
    A: *const *const cuDoubleComplex,
    lda: i64,
    B: *const *mut cuDoubleComplex,
    ldb: i64,
    batchCount: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrsmBatched_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            diag,
                            m,
                            n,
                            alpha,
                            A,
                            lda,
                            B,
                            ldb,
                            batchCount,
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
pub unsafe extern "C" fn cublasSdgmm(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    x: *const f32,
    incx: ::std::os::raw::c_int,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSdgmm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, mode, m, n, A, lda, x, incx, C, ldc) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSdgmm_64(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: i64,
    n: i64,
    A: *const f32,
    lda: i64,
    x: *const f32,
    incx: i64,
    C: *mut f32,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSdgmm_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, mode, m, n, A, lda, x, incx, C, ldc) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDdgmm(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    x: *const f64,
    incx: ::std::os::raw::c_int,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDdgmm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, mode, m, n, A, lda, x, incx, C, ldc) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDdgmm_64(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: i64,
    n: i64,
    A: *const f64,
    lda: i64,
    x: *const f64,
    incx: i64,
    C: *mut f64,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDdgmm_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, mode, m, n, A, lda, x, incx, C, ldc) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCdgmm(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuComplex,
    incx: ::std::os::raw::c_int,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCdgmm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, mode, m, n, A, lda, x, incx, C, ldc) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCdgmm_64(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: i64,
    n: i64,
    A: *const cuComplex,
    lda: i64,
    x: *const cuComplex,
    incx: i64,
    C: *mut cuComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCdgmm_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, mode, m, n, A, lda, x, incx, C, ldc) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdgmm(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    x: *const cuDoubleComplex,
    incx: ::std::os::raw::c_int,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdgmm {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, mode, m, n, A, lda, x, incx, C, ldc) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZdgmm_64(
    handle: cublasHandle_t,
    mode: cublasSideMode_t,
    m: i64,
    n: i64,
    A: *const cuDoubleComplex,
    lda: i64,
    x: *const cuDoubleComplex,
    incx: i64,
    C: *mut cuDoubleComplex,
    ldc: i64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZdgmm_64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, mode, m, n, A, lda, x, incx, C, ldc) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSmatinvBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const f32,
    lda: ::std::os::raw::c_int,
    Ainv: *const *mut f32,
    lda_inv: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSmatinvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, n, A, lda, Ainv, lda_inv, info, batchSize)
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
pub unsafe extern "C" fn cublasDmatinvBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const f64,
    lda: ::std::os::raw::c_int,
    Ainv: *const *mut f64,
    lda_inv: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDmatinvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, n, A, lda, Ainv, lda_inv, info, batchSize)
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
pub unsafe extern "C" fn cublasCmatinvBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    Ainv: *const *mut cuComplex,
    lda_inv: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCmatinvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, n, A, lda, Ainv, lda_inv, info, batchSize)
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
pub unsafe extern "C" fn cublasZmatinvBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Ainv: *const *mut cuDoubleComplex,
    lda_inv: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZmatinvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, n, A, lda, Ainv, lda_inv, info, batchSize)
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
pub unsafe extern "C" fn cublasSgeqrfBatched(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Aarray: *const *mut f32,
    lda: ::std::os::raw::c_int,
    TauArray: *const *mut f32,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgeqrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, Aarray, lda, TauArray, info, batchSize)
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
pub unsafe extern "C" fn cublasDgeqrfBatched(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Aarray: *const *mut f64,
    lda: ::std::os::raw::c_int,
    TauArray: *const *mut f64,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgeqrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, Aarray, lda, TauArray, info, batchSize)
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
pub unsafe extern "C" fn cublasCgeqrfBatched(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Aarray: *const *mut cuComplex,
    lda: ::std::os::raw::c_int,
    TauArray: *const *mut cuComplex,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgeqrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, Aarray, lda, TauArray, info, batchSize)
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
pub unsafe extern "C" fn cublasZgeqrfBatched(
    handle: cublasHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Aarray: *const *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    TauArray: *const *mut cuDoubleComplex,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgeqrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, Aarray, lda, TauArray, info, batchSize)
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
pub unsafe extern "C" fn cublasSgelsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *mut f32,
    lda: ::std::os::raw::c_int,
    Carray: *const *mut f32,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    devInfoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgelsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            nrhs,
                            Aarray,
                            lda,
                            Carray,
                            ldc,
                            info,
                            devInfoArray,
                            batchSize,
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
pub unsafe extern "C" fn cublasDgelsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *mut f64,
    lda: ::std::os::raw::c_int,
    Carray: *const *mut f64,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    devInfoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgelsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            nrhs,
                            Aarray,
                            lda,
                            Carray,
                            ldc,
                            info,
                            devInfoArray,
                            batchSize,
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
pub unsafe extern "C" fn cublasCgelsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *mut cuComplex,
    lda: ::std::os::raw::c_int,
    Carray: *const *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    devInfoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgelsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            nrhs,
                            Aarray,
                            lda,
                            Carray,
                            ldc,
                            info,
                            devInfoArray,
                            batchSize,
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
pub unsafe extern "C" fn cublasZgelsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Carray: *const *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    devInfoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgelsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            m,
                            n,
                            nrhs,
                            Aarray,
                            lda,
                            Carray,
                            ldc,
                            info,
                            devInfoArray,
                            batchSize,
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
pub unsafe extern "C" fn cublasStpttr(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    AP: *const f32,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStpttr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, AP, A, lda) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtpttr(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    AP: *const f64,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtpttr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, AP, A, lda) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtpttr(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    AP: *const cuComplex,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtpttr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, AP, A, lda) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtpttr(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    AP: *const cuDoubleComplex,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtpttr {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, AP, A, lda) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasStrttp(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    AP: *mut f32,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasStrttp {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDtrttp(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    AP: *mut f64,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDtrttp {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCtrttp(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    AP: *mut cuComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCtrttp {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZtrttp(
    handle: cublasHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    AP: *mut cuDoubleComplex,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZtrttp {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, AP) }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSgetrfBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *mut f32,
    lda: ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgetrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, P, info, batchSize) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDgetrfBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *mut f64,
    lda: ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgetrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, P, info, batchSize) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCgetrfBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *mut cuComplex,
    lda: ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgetrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, P, info, batchSize) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZgetrfBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgetrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, P, info, batchSize) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSgetriBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const f32,
    lda: ::std::os::raw::c_int,
    P: *const ::std::os::raw::c_int,
    C: *const *mut f32,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgetriBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, P, C, ldc, info, batchSize) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasDgetriBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const f64,
    lda: ::std::os::raw::c_int,
    P: *const ::std::os::raw::c_int,
    C: *const *mut f64,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgetriBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, P, C, ldc, info, batchSize) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasCgetriBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    P: *const ::std::os::raw::c_int,
    C: *const *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgetriBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, P, C, ldc, info, batchSize) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasZgetriBatched(
    handle: cublasHandle_t,
    n: ::std::os::raw::c_int,
    A: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    P: *const ::std::os::raw::c_int,
    C: *const *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgetriBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, P, C, ldc, info, batchSize) }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cublasSgetrsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *const f32,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    Barray: *const *mut f32,
    ldb: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasSgetrsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            n,
                            nrhs,
                            Aarray,
                            lda,
                            devIpiv,
                            Barray,
                            ldb,
                            info,
                            batchSize,
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
pub unsafe extern "C" fn cublasDgetrsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *const f64,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    Barray: *const *mut f64,
    ldb: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasDgetrsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            n,
                            nrhs,
                            Aarray,
                            lda,
                            devIpiv,
                            Barray,
                            ldb,
                            info,
                            batchSize,
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
pub unsafe extern "C" fn cublasCgetrsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *const cuComplex,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    Barray: *const *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasCgetrsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            n,
                            nrhs,
                            Aarray,
                            lda,
                            devIpiv,
                            Barray,
                            ldb,
                            info,
                            batchSize,
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
pub unsafe extern "C" fn cublasZgetrsBatched(
    handle: cublasHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Aarray: *const *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    Barray: *const *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasZgetrsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            n,
                            nrhs,
                            Aarray,
                            lda,
                            devIpiv,
                            Barray,
                            ldb,
                            info,
                            batchSize,
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
pub unsafe extern "C" fn cublasUint8gemmBias(
    handle: cublasHandle_t,
    transa: cublasOperation_t,
    transb: cublasOperation_t,
    transc: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const ::std::os::raw::c_uchar,
    A_bias: ::std::os::raw::c_int,
    lda: ::std::os::raw::c_int,
    B: *const ::std::os::raw::c_uchar,
    B_bias: ::std::os::raw::c_int,
    ldb: ::std::os::raw::c_int,
    C: *mut ::std::os::raw::c_uchar,
    C_bias: ::std::os::raw::c_int,
    ldc: ::std::os::raw::c_int,
    C_mult: ::std::os::raw::c_int,
    C_shift: ::std::os::raw::c_int,
) -> cublasStatus_t {
    match cublas_api() {
        Ok(__nv_api) => {
            match &__nv_api.cublasUint8gemmBias {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            transa,
                            transb,
                            transc,
                            m,
                            n,
                            k,
                            A,
                            A_bias,
                            lda,
                            B,
                            B_bias,
                            ldb,
                            C,
                            C_bias,
                            ldc,
                            C_mult,
                            C_shift,
                        )
                    }
                }
                Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cublasStatus_t_CUBLAS_STATUS_NOT_INITIALIZED,
    }
}
