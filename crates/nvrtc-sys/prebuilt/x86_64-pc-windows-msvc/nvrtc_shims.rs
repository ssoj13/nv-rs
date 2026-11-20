#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetErrorString(
    result: nvrtcResult,
) -> *const ::std::os::raw::c_char {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetErrorString {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(result) }
                Err(__nv_error) => ::core::ptr::null(),
            }
        }
        Err(__nv_error) => ::core::ptr::null(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcVersion(
    major: *mut ::std::os::raw::c_int,
    minor: *mut ::std::os::raw::c_int,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(major, minor) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetNumSupportedArchs(
    numArchs: *mut ::std::os::raw::c_int,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetNumSupportedArchs {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(numArchs) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetSupportedArchs(
    supportedArchs: *mut ::std::os::raw::c_int,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetSupportedArchs {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(supportedArchs) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcCreateProgram(
    prog: *mut nvrtcProgram,
    src: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    numHeaders: ::std::os::raw::c_int,
    headers: *const *const ::std::os::raw::c_char,
    includeNames: *const *const ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcCreateProgram {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(prog, src, name, numHeaders, headers, includeNames)
                    }
                }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcDestroyProgram(prog: *mut nvrtcProgram) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcDestroyProgram {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcCompileProgram(
    prog: nvrtcProgram,
    numOptions: ::std::os::raw::c_int,
    options: *const *const ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcCompileProgram {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, numOptions, options) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetPTXSize(
    prog: nvrtcProgram,
    ptxSizeRet: *mut usize,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetPTXSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, ptxSizeRet) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetPTX(
    prog: nvrtcProgram,
    ptx: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetPTX {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, ptx) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetCUBINSize(
    prog: nvrtcProgram,
    cubinSizeRet: *mut usize,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetCUBINSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, cubinSizeRet) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetCUBIN(
    prog: nvrtcProgram,
    cubin: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetCUBIN {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, cubin) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetLTOIRSize(
    prog: nvrtcProgram,
    LTOIRSizeRet: *mut usize,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetLTOIRSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, LTOIRSizeRet) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetLTOIR(
    prog: nvrtcProgram,
    LTOIR: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetLTOIR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, LTOIR) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetOptiXIRSize(
    prog: nvrtcProgram,
    optixirSizeRet: *mut usize,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetOptiXIRSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, optixirSizeRet) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetOptiXIR(
    prog: nvrtcProgram,
    optixir: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetOptiXIR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, optixir) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetProgramLogSize(
    prog: nvrtcProgram,
    logSizeRet: *mut usize,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetProgramLogSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, logSizeRet) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetProgramLog(
    prog: nvrtcProgram,
    log: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetProgramLog {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, log) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcAddNameExpression(
    prog: nvrtcProgram,
    name_expression: *const ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcAddNameExpression {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, name_expression) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetLoweredName(
    prog: nvrtcProgram,
    name_expression: *const ::std::os::raw::c_char,
    lowered_name: *mut *const ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetLoweredName {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, name_expression, lowered_name) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetPCHHeapSize(ret: *mut usize) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetPCHHeapSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(ret) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcSetPCHHeapSize(size: usize) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcSetPCHHeapSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(size) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetPCHCreateStatus(prog: nvrtcProgram) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetPCHCreateStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetPCHHeapSizeRequired(
    prog: nvrtcProgram,
    size: *mut usize,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetPCHHeapSizeRequired {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, size) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcSetFlowCallback(
    prog: nvrtcProgram,
    callback: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    payload: *mut ::std::os::raw::c_void,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcSetFlowCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, callback, payload) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetTileIRSize(
    prog: nvrtcProgram,
    TileIRSizeRet: *mut usize,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetTileIRSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, TileIRSizeRet) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetTileIR(
    prog: nvrtcProgram,
    TileIR: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetTileIR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(prog, TileIR) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcInstallBundledHeaders(
    installPath: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_uint,
    errorLog: *mut *const ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcInstallBundledHeaders {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(installPath, flags, errorLog) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcGetBundledHeadersInfo(
    info: *mut nvrtcBundledHeadersInfo,
    errorLog: *mut *const ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcGetBundledHeadersInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info, errorLog) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvrtcRemoveBundledHeaders(
    installPath: *const ::std::os::raw::c_char,
    errorLog: *mut *const ::std::os::raw::c_char,
) -> nvrtcResult {
    match nvrtc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvrtcRemoveBundledHeaders {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(installPath, errorLog) }
                Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
            }
        }
        Err(__nv_error) => nvrtcResult_NVRTC_ERROR_INTERNAL_ERROR,
    }
}
