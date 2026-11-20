#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidGetDecoderCaps(pdc: *mut CUVIDDECODECAPS) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidGetDecoderCaps {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pdc) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidCreateDecoder(
    phDecoder: *mut CUvideodecoder,
    pdci: *mut CUVIDDECODECREATEINFO,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidCreateDecoder {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(phDecoder, pdci) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidDestroyDecoder(hDecoder: CUvideodecoder) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidDestroyDecoder {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(hDecoder) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidRegisterDecodeSurfaces(
    hDecoder: CUvideodecoder,
    pDecSurfInfo: *mut CUVIDREGISTERDECODESURFACESINFO,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidRegisterDecodeSurfaces {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(hDecoder, pDecSurfInfo) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidDecodePictureAsync(
    hDecoder: CUvideodecoder,
    pPicParams: *mut CUVIDPICPARAMS,
    strm: CUstream,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidDecodePictureAsync {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(hDecoder, pPicParams, strm) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidDecodePicture(
    hDecoder: CUvideodecoder,
    pPicParams: *mut CUVIDPICPARAMS,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidDecodePicture {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(hDecoder, pPicParams) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidGetDecodeStatus(
    hDecoder: CUvideodecoder,
    nPicIdx: ::std::os::raw::c_int,
    pDecodeStatus: *mut CUVIDGETDECODESTATUS,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidGetDecodeStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(hDecoder, nPicIdx, pDecodeStatus) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidReconfigureDecoder(
    hDecoder: CUvideodecoder,
    pDecReconfigParams: *mut CUVIDRECONFIGUREDECODERINFO,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidReconfigureDecoder {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(hDecoder, pDecReconfigParams) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidMapVideoFrame64(
    hDecoder: CUvideodecoder,
    nPicIdx: ::std::os::raw::c_int,
    pDevPtr: *mut ::std::os::raw::c_ulonglong,
    pPitch: *mut ::std::os::raw::c_uint,
    pVPP: *mut CUVIDPROCPARAMS,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidMapVideoFrame64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(hDecoder, nPicIdx, pDevPtr, pPitch, pVPP) }
                }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidUnmapVideoFrame64(
    hDecoder: CUvideodecoder,
    DevPtr: ::std::os::raw::c_ulonglong,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidUnmapVideoFrame64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(hDecoder, DevPtr) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidCtxLockCreate(
    pLock: *mut CUvideoctxlock,
    ctx: CUcontext,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidCtxLockCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pLock, ctx) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidCtxLockDestroy(lck: CUvideoctxlock) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidCtxLockDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(lck) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidCtxLock(
    lck: CUvideoctxlock,
    reserved_flags: ::std::os::raw::c_uint,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidCtxLock {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(lck, reserved_flags) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidCtxUnlock(
    lck: CUvideoctxlock,
    reserved_flags: ::std::os::raw::c_uint,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidCtxUnlock {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(lck, reserved_flags) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidCreateVideoSource(
    pObj: *mut CUvideosource,
    pszFileName: *const ::std::os::raw::c_char,
    pParams: *mut CUVIDSOURCEPARAMS,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidCreateVideoSource {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pObj, pszFileName, pParams) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidCreateVideoSourceW(
    pObj: *mut CUvideosource,
    pwszFileName: *const wchar_t,
    pParams: *mut CUVIDSOURCEPARAMS,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidCreateVideoSourceW {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pObj, pwszFileName, pParams) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidDestroyVideoSource(obj: CUvideosource) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidDestroyVideoSource {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(obj) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidSetVideoSourceState(
    obj: CUvideosource,
    state: cudaVideoState,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidSetVideoSourceState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(obj, state) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn cuvidGetVideoSourceState(
    obj: CUvideosource,
) -> ::core::result::Result<cudaVideoState, nv_loader::CallError> {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidGetVideoSourceState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => ::core::result::Result::Ok(unsafe { (*__nv_fn)(obj) }),
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "nvcuvid",
                            stringify!(cuvidGetVideoSourceState),
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
pub unsafe extern "C" fn cuvidGetSourceVideoFormat(
    obj: CUvideosource,
    pvidfmt: *mut CUVIDEOFORMAT,
    flags: ::std::os::raw::c_uint,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidGetSourceVideoFormat {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(obj, pvidfmt, flags) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidGetSourceAudioFormat(
    obj: CUvideosource,
    paudfmt: *mut CUAUDIOFORMAT,
    flags: ::std::os::raw::c_uint,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidGetSourceAudioFormat {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(obj, paudfmt, flags) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidCreateVideoParser(
    pObj: *mut CUvideoparser,
    pParams: *mut CUVIDPARSERPARAMS,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidCreateVideoParser {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pObj, pParams) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidParseVideoData(
    obj: CUvideoparser,
    pPacket: *mut CUVIDSOURCEDATAPACKET,
) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidParseVideoData {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(obj, pPacket) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuvidDestroyVideoParser(obj: CUvideoparser) -> CUresult {
    match nvcuvid_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuvidDestroyVideoParser {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(obj) }
                Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_FOUND,
            }
        }
        Err(__nv_error) => cuda_bindings::cudaError_enum_CUDA_ERROR_NOT_INITIALIZED,
    }
}
