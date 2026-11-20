#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV_8u_AC4P4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV_8u_AC4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYUV_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYUV_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYUV_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYUV_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYUV_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYUV_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYUV_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYUV_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYUV_8u_AC4P4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYUV_8u_AC4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToRGB_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToRGB_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToRGB_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToRGB_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToRGB_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToRGB_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToRGB_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToRGB_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToRGBBatch_8u_C3R_Ctx(
    pSrcBatchList: *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToRGBBatch_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToRGBBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToRGBBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToRGBBatch_8u_C3R_Advanced_Ctx(
    pSrcBatchList: *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToRGBBatch_8u_C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToRGBBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToRGBBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToBGR_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToBGR_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToBGR_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToBGR_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToBGR_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToBGR_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToBGR_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToBGR_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToBGRBatch_8u_C3R_Ctx(
    pSrcBatchList: *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToBGRBatch_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToBGRBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToBGRBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToBGRBatch_8u_C3R_Advanced_Ctx(
    pSrcBatchList: *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToBGRBatch_8u_C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUVToBGRBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUVToBGRBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_8u_C3C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_8u_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_8u_C2C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_8u_C2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_8u_P3AC4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_8u_P3AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGBBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGBBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGBBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGBBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToBGRBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToBGRBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToBGRBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToBGRBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV420_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV420_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV420_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV420_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_8u_P3AC4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_8u_P3AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGBBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGBBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGBBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGBBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToRGB_8u_P2C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToRGB_8u_P2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToRGB_709HDTV_8u_P2C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToRGB_709HDTV_8u_P2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToRGB_709CSC_8u_P2C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToRGB_709CSC_8u_P2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV21ToRGB_8u_P2C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV21ToRGB_8u_P2C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYUV420_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYUV420_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToBGR_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToBGR_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToBGR_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToBGR_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToBGRBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToBGRBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToBGRBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToBGRBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToBGR_8u_P2C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToBGR_8u_P2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToBGR_709HDTV_8u_P2C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToBGR_709HDTV_8u_P2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToBGR_709CSC_8u_P2C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToBGR_709CSC_8u_P2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV21ToBGR_8u_P2C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV21ToBGR_8u_P2C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGB_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGB_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGB_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGB_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGB_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGB_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGB_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGB_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGB_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGB_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGBBatch_8u_C3R_Ctx(
    pSrcBatchList: *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGBBatch_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGBBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGBBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGBBatch_8u_C3R_Advanced_Ctx(
    pSrcBatchList: *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGBBatch_8u_C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToRGBBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToRGBBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToBGR_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToBGR_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToBGR_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToBGR_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToBGRBatch_8u_C3R_Ctx(
    pSrcBatchList: *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToBGRBatch_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToBGRBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToBGRBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToBGRBatch_8u_C3R_Advanced_Ctx(
    pSrcBatchList: *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToBGRBatch_8u_C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToBGRBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToBGRBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToBGR_709CSC_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToBGR_709CSC_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCrToBGR_709CSC_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCrToBGR_709CSC_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr422_8u_C3C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr422_8u_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr422_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr422_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr422_8u_P3C2R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr422_8u_P3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToRGB_8u_C2C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToRGB_8u_C2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToRGB_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToRGB_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToRGB_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToRGB_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToRGBBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToRGBBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToRGBBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToRGBBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCrCb422_8u_C3C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCrCb422_8u_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCrCb422_8u_P3C2R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCrCb422_8u_P3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb422ToRGB_8u_C2C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb422ToRGB_8u_C2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb422ToRGB_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb422ToRGB_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr422_8u_C3C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr422_8u_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr422_8u_AC4C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr422_8u_AC4C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr422_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr422_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr422_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr422_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToBGR_8u_C2C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToBGR_8u_C2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToBGR_8u_C2C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToBGR_8u_C2C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToBGR_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToBGR_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToBGRBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToBGRBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToBGRBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToBGRBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToCbYCr422_8u_C3C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToCbYCr422_8u_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToCbYCr422Gamma_8u_C3C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToCbYCr422Gamma_8u_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToRGB_8u_C2C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToRGB_8u_C2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToCbYCr422_8u_AC4C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToCbYCr422_8u_AC4C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToCbYCr422_709HDTV_8u_C3C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToCbYCr422_709HDTV_8u_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToCbYCr422_709HDTV_8u_AC4C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToCbYCr422_709HDTV_8u_AC4C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToBGR_8u_C2C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToBGR_8u_C2C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToBGR_709HDTV_8u_C2C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToBGR_709HDTV_8u_C2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToBGR_709HDTV_8u_C2C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToBGR_709HDTV_8u_C2C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr420_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr420_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToRGB_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToRGB_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToRGBBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToRGBBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToRGBBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToRGBBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCrCb420_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCrCb420_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb420ToRGB_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb420ToRGB_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr420_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr420_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr420_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr420_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr420_709CSC_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr420_709CSC_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr420_709CSC_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr420_709CSC_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr420_709HDTV_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr420_709HDTV_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCrCb420_709CSC_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCrCb420_709CSC_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCrCb420_709CSC_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCrCb420_709CSC_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToBGR_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToBGR_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToBGR_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToBGR_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToBGRBatch_8u_P3C3R_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToBGRBatch_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToBGRBatch_8u_P3C3R_Advanced_Ctx(
    pSrcBatchList: *const *const NppiImageDescriptor,
    pDstBatchList: *mut NppiImageDescriptor,
    nBatchSize: ::std::os::raw::c_int,
    oMaxSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToBGRBatch_8u_P3C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcBatchList,
                            pDstBatchList,
                            nBatchSize,
                            oMaxSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToBGR_709CSC_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToBGR_709CSC_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToBGR_709HDTV_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToBGR_709HDTV_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCrCb420_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCrCb420_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCrCb420_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCrCb420_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr411_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr411_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr411_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr411_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr411_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr411_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr411_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr411_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr_8u_AC4P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr_8u_AC4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr_8u_AC4P4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr_8u_AC4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToBGR_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToBGR_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToBGR_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToBGR_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToRGB_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToRGB_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToRGB_8u_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nAval: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToRGB_8u_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nAval,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToXYZ_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToXYZ_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToXYZ_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToXYZ_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiXYZToRGB_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiXYZToRGB_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiXYZToRGB_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiXYZToRGB_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToLUV_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToLUV_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToLUV_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToLUV_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUVToRGB_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUVToRGB_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUVToRGB_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUVToRGB_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToLab_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToLab_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLabToBGR_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLabToBGR_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCC_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCC_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCC_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCC_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCCToRGB_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCCToRGB_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCCToRGB_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCCToRGB_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCCKToCMYK_JPEG_601_8u_P4R_Ctx(
    pSrc: *mut *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCCKToCMYK_JPEG_601_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCMYKOrYCCKToRGB_JPEG_8u_P4P3R_Ctx(
    pSrc: *mut *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCMYKOrYCCKToRGB_JPEG_8u_P4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCMYKOrYCCKToRGB_JPEG_8u_P4C3R_Ctx(
    pSrc: *mut *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCMYKOrYCCKToRGB_JPEG_8u_P4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCMYKOrYCCKToBGR_JPEG_8u_P4P3R_Ctx(
    pSrc: *mut *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCMYKOrYCCKToBGR_JPEG_8u_P4P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCMYKOrYCCKToBGR_JPEG_8u_P4C3R_Ctx(
    pSrc: *mut *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCMYKOrYCCKToBGR_JPEG_8u_P4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToHLS_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToHLS_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToHLS_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToHLS_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHLSToRGB_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHLSToRGB_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHLSToRGB_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHLSToRGB_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToHLS_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToHLS_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToHLS_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToHLS_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToHLS_8u_AC4P4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToHLS_8u_AC4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToHLS_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToHLS_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToHLS_8u_AP4C4R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToHLS_8u_AP4C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToHLS_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToHLS_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToHLS_8u_AP4R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToHLS_8u_AP4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHLSToBGR_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHLSToBGR_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHLSToBGR_8u_AC4P4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHLSToBGR_8u_AC4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHLSToBGR_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHLSToBGR_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHLSToBGR_8u_AP4R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHLSToBGR_8u_AP4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHLSToBGR_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHLSToBGR_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHLSToBGR_8u_AP4C4R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHLSToBGR_8u_AP4C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToHSV_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToHSV_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToHSV_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToHSV_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHSVToRGB_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHSVToRGB_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiHSVToRGB_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiHSVToRGB_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr420_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr420_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr422_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr422_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr411_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr411_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr444_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr444_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr420_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr420_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr422_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr422_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr411_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr411_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr444_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr444_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToRGB_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToRGB_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToRGB_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToRGB_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToRGB_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToRGB_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr444ToRGB_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr444ToRGB_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToBGR_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToBGR_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToBGR_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToBGR_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToBGR_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToBGR_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr444ToBGR_JPEG_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr444ToBGR_JPEG_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr420_JPEG_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr420_JPEG_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr422_JPEG_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr422_JPEG_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr411_JPEG_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr411_JPEG_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYCbCr444_JPEG_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYCbCr444_JPEG_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr420_JPEG_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr420_JPEG_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr422_JPEG_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr422_JPEG_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr411_JPEG_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr411_JPEG_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiBGRToYCbCr444_JPEG_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiBGRToYCbCr444_JPEG_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToRGB_JPEG_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToRGB_JPEG_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToRGB_JPEG_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToRGB_JPEG_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToRGB_JPEG_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToRGB_JPEG_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr444ToRGB_JPEG_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr444ToRGB_JPEG_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToBGR_JPEG_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToBGR_JPEG_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToBGR_JPEG_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToBGR_JPEG_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToBGR_JPEG_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToBGR_JPEG_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr444ToBGR_JPEG_8u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr444ToBGR_JPEG_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToGray_8u_C3C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToGray_8u_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToGray_8u_AC4C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToGray_8u_AC4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToGray_16u_C3C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToGray_16u_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToGray_16u_AC4C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToGray_16u_AC4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToGray_16s_C3C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToGray_16s_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToGray_16s_AC4C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToGray_16s_AC4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToGray_32f_C3C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToGray_32f_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToGray_32f_AC4C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToGray_32f_AC4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_8u_C3C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_8u_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_8u_AC4C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_8u_AC4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_8u_C4C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_8u_C4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_16u_C3C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_16u_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_16u_AC4C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_16u_AC4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_16u_C4C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_16u_C4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_16s_C3C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_16s_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_16s_AC4C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_16s_AC4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_16s_C4C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_16s_C4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_32f_C3C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_32f_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_32f_AC4C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_32f_AC4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorToGray_32f_C4C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aCoeffs: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorToGray_32f_C4C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aCoeffs,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGradientColorToGray_8u_C3C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eNorm: NppiNorm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGradientColorToGray_8u_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eNorm,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGradientColorToGray_16u_C3C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eNorm: NppiNorm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGradientColorToGray_16u_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eNorm,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGradientColorToGray_16s_C3C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eNorm: NppiNorm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGradientColorToGray_16s_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eNorm,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGradientColorToGray_32f_C3C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eNorm: NppiNorm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGradientColorToGray_32f_C3C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eNorm,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCFAToRGB_8u_C1C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    eGrid: NppiBayerGridPosition,
    eInterpolation: NppiInterpolationMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCFAToRGB_8u_C1C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            eGrid,
                            eInterpolation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCFAToRGBA_8u_C1AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    eGrid: NppiBayerGridPosition,
    eInterpolation: NppiInterpolationMode,
    nAlpha: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCFAToRGBA_8u_C1AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            eGrid,
                            eInterpolation,
                            nAlpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCFAToRGB_16u_C1C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    eGrid: NppiBayerGridPosition,
    eInterpolation: NppiInterpolationMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCFAToRGB_16u_C1C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            eGrid,
                            eInterpolation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCFAToRGBA_16u_C1AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    eGrid: NppiBayerGridPosition,
    eInterpolation: NppiInterpolationMode,
    nAlpha: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCFAToRGBA_16u_C1AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            eGrid,
                            eInterpolation,
                            nAlpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCFAToRGB_32u_C1C3R_Ctx(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcROI: NppiRect,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    eGrid: NppiBayerGridPosition,
    eInterpolation: NppiInterpolationMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCFAToRGB_32u_C1C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            eGrid,
                            eInterpolation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCFAToRGBA_32u_C1AC4R_Ctx(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcROI: NppiRect,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    eGrid: NppiBayerGridPosition,
    eInterpolation: NppiInterpolationMode,
    nAlpha: Npp32u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCFAToRGBA_32u_C1AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            eGrid,
                            eInterpolation,
                            nAlpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToYCbCr411_8u_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToYCbCr411_8u_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToYCbCr411_8u_P2P3R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToYCbCr411_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422_8u_P3C2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422_8u_P3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCrCb422_8u_C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCrCb422_8u_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCrCb422_8u_P3C2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCrCb422_8u_P3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToCbYCr422_8u_C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToCbYCr422_8u_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToYCbCr411_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToYCbCr411_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCbCr420_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCbCr420_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCbCr420_8u_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCbCr420_8u_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCbCr420_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCbCr420_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCbCr420_8u_C2P2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCbCr420_8u_C2P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb420ToYCbCr422_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb420ToYCbCr422_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb420ToYCbCr422_8u_P3C2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb420ToYCbCr422_8u_P3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCrCb420_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCrCb420_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCbCr411_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCbCr411_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCbCr411_8u_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCbCr411_8u_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCbCr411_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCbCr411_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr422ToYCbCr411_8u_C2P2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr422ToYCbCr411_8u_C2P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb422ToYCbCr422_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb422ToYCbCr422_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb422ToYCbCr420_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb422ToYCbCr420_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb422ToYCbCr411_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb422ToYCbCr411_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToYCbCr422_8u_C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToYCbCr422_8u_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToYCbCr422_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToYCbCr422_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToYCbCr420_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToYCbCr420_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToYCbCr420_8u_C2P2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToYCbCr420_8u_C2P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCbYCr422ToYCrCb420_8u_C2P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCbYCr422ToYCrCb420_8u_C2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420_8u_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420_8u_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420_8u_P2P3R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToYCbCr422_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToYCbCr422_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToYCbCr422_8u_P2P3R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToYCbCr422_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToYCbCr422_8u_P2C2R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToYCbCr422_8u_P2C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToCbYCr422_8u_P2C2R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToCbYCr422_8u_P2C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr420ToYCrCb420_8u_P2P3R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr420ToYCrCb420_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb420ToCbYCr422_8u_P3C2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb420ToCbYCr422_8u_P3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb420ToYCbCr420_8u_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb420ToYCbCr420_8u_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCrCb420ToYCbCr411_8u_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCrCb420ToYCbCr411_8u_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411_8u_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411_8u_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411_8u_P2P3R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCbCr422_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCbCr422_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCbCr422_8u_P3C2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCbCr422_8u_P3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCbCr422_8u_P2P3R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCbCr422_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCbCr422_8u_P2C2R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCbCr422_8u_P2C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCrCb422_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCrCb422_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCrCb422_8u_P3C2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCrCb422_8u_P3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCbCr420_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCbCr420_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCbCr420_8u_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    rSrcStep: *mut ::std::os::raw::c_int,
    pDstY: *mut Npp8u,
    nDstYStep: ::std::os::raw::c_int,
    pDstCbCr: *mut Npp8u,
    nDstCbCrStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCbCr420_8u_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            rSrcStep,
                            pDstY,
                            nDstYStep,
                            pDstCbCr,
                            nDstCbCrStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCbCr420_8u_P2P3R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCbCr420_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYCbCr411ToYCrCb420_8u_P2P3R_Ctx(
    pSrcY: *const Npp8u,
    nSrcYStep: ::std::os::raw::c_int,
    pSrcCbCr: *const Npp8u,
    nSrcCbCrStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    rDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYCbCr411ToYCrCb420_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcY,
                            nSrcYStep,
                            pSrcCbCr,
                            nSrcCbCrStep,
                            pDst,
                            rDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToYUV420_8u_P2P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToYUV420_8u_P2P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaFwd_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaFwd_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaFwd_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaFwd_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaFwd_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaFwd_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaFwd_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaFwd_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaFwd_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaFwd_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaFwd_8u_IP3R_Ctx(
    pSrcDst: *const *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaFwd_8u_IP3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaInv_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaInv_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaInv_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaInv_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaInv_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaInv_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaInv_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaInv_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaInv_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaInv_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGammaInv_8u_IP3R_Ctx(
    pSrcDst: *const *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGammaInv_8u_IP3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompColorKey_8u_C1R_Ctx(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nColorKeyConst: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompColorKey_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            nSrc1Step,
                            pSrc2,
                            nSrc2Step,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nColorKeyConst,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompColorKey_8u_C3R_Ctx(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nColorKeyConst: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompColorKey_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            nSrc1Step,
                            pSrc2,
                            nSrc2Step,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nColorKeyConst,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompColorKey_8u_C4R_Ctx(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nColorKeyConst: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompColorKey_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            nSrc1Step,
                            pSrc2,
                            nSrc2Step,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nColorKeyConst,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiAlphaCompColorKey_8u_AC4R_Ctx(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    nAlpha1: Npp8u,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    nAlpha2: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nColorKeyConst: *mut Npp8u,
    nppAlphaOp: NppiAlphaOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiAlphaCompColorKey_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            nSrc1Step,
                            nAlpha1,
                            pSrc2,
                            nSrc2Step,
                            nAlpha2,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nColorKeyConst,
                            nppAlphaOp,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_C2IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_C2IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_C4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32fC_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    aConstants: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32fC_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            aConstants,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32fC_8u_C4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    aConstants: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32fC_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aTwist,
                            aConstants,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *const *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8u_IP3R_Ctx(
    pSrcDst: *const *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8u_IP3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_C1R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_C1IR_Ctx(
    pSrcDst: *mut Npp8s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_C2R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_C2IR_Ctx(
    pSrcDst: *mut Npp8s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_C2IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_C3R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_C3IR_Ctx(
    pSrcDst: *mut Npp8s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_C4R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_C4IR_Ctx(
    pSrcDst: *mut Npp8s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_AC4R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_AC4IR_Ctx(
    pSrcDst: *mut Npp8s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_P3R_Ctx(
    pSrc: *const *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *const *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_8s_IP3R_Ctx(
    pSrcDst: *const *mut Npp8s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_8s_IP3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_C2R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_C2IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_C2IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_P3R_Ctx(
    pSrc: *const *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *const *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16u_IP3R_Ctx(
    pSrcDst: *const *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16u_IP3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_C2R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_C2IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_C2IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_P3R_Ctx(
    pSrc: *const *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *const *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16s_IP3R_Ctx(
    pSrcDst: *const *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16s_IP3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16f_C1R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16f_C1IR_Ctx(
    pSrcDst: *mut Npp16f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16f_C2R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16f_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16f_C2IR_Ctx(
    pSrcDst: *mut Npp16f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16f_C2IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16f_C3R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16f_C3IR_Ctx(
    pSrcDst: *mut Npp16f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16f_C4R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32f_16f_C4IR_Ctx(
    pSrcDst: *mut Npp16f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32f_16f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32fC_16f_C4R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    aConstants: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32fC_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            aConstants,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist32fC_16f_C4IR_Ctx(
    pSrcDst: *mut Npp16f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    aConstants: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist32fC_16f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aTwist,
                            aConstants,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_C2R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_C2IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_C2IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_C4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32fC_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    aConstants: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32fC_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            aConstants,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32fC_C4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    aConstants: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32fC_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aTwist,
                            aConstants,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_P3R_Ctx(
    pSrc: *const *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *const *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwist_32f_IP3R_Ctx(
    pSrcDst: *const *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwist_32f_IP3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aTwist, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_8u_C1R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_8u_C1IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_8u_C3R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_8u_C3IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_8u_C4R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_8u_C4IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_8u_AC4R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_8u_AC4IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32fC_8u_C4R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32fC_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32fC_8u_C4IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32fC_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32f_C1R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32f_C1IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32f_C3R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32f_C3IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32f_C4R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32f_C4IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32f_AC4R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32f_AC4IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32fC_C4R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32fC_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch_32fC_C4IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch_32fC_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_16f_C1R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_16f_C1IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_16f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_16f_C3R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_16f_C3IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_16f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_16f_C4R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32f_16f_C4IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32f_16f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32fC_16f_C4R_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32fC_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiColorTwistBatch32fC_16f_C4IR_Ctx(
    nMin: Npp32f,
    nMax: Npp32f,
    oSizeROI: NppiSize,
    pBatchList: *mut NppiColorTwistBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiColorTwistBatch32fC_16f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMin,
                            nMax,
                            oSizeROI,
                            pBatchList,
                            nBatchSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_8u_C4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16u_C4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16s_C4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16s_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32f,
    pLevels: *const Npp32f,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32f,
    pLevels: *const Npp32f,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_32f_C4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_32f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_8u_C4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16u_C4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16s_C4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16s_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32f,
    pLevels: *const Npp32f,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32f,
    pLevels: *const Npp32f,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_32f_C4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_32f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Linear_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Linear_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_8u_C4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16u_C4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32s,
    pLevels: *const Npp32s,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16s_C4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16s_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32s,
    pLevels: *mut *const Npp32s,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32f,
    pLevels: *const Npp32f,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *const Npp32f,
    pLevels: *const Npp32f,
    nLevels: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_32f_C4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_32f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Cubic_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut *const Npp32f,
    pLevels: *mut *const Npp32f,
    nLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Cubic_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            nLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Trilinear_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut Npp32u,
    pLevels: *mut *mut Npp8u,
    aLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Trilinear_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            aLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Trilinear_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut Npp32u,
    pLevels: *mut *mut Npp8u,
    aLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Trilinear_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            aLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUT_Trilinear_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pValues: *mut Npp32u,
    pLevels: *mut *mut Npp8u,
    aLevels: *mut ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUT_Trilinear_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            pValues,
                            pLevels,
                            aLevels,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTable: *const Npp8u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTable,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_8u24u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTable: *const Npp8u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_8u24u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTable,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_8u32u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTable: *const Npp32u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_8u32u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTable,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTables: *mut *const Npp8u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTables,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTables: *mut *const Npp8u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTables,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTables: *mut *const Npp8u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTables,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTable: *const Npp16u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTable,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_16u8u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTable: *const Npp8u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_16u8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTable,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_16u24u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTable: *const Npp8u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_16u24u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTable,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_16u32u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTable: *const Npp32u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_16u32u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTable,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTables: *mut *const Npp16u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTables,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTables: *mut *const Npp16u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTables,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPalette_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTables: *mut *const Npp16u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPalette_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTables,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPaletteSwap_8u_C3A0C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    nAlphaValue: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTables: *mut *const Npp8u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPaletteSwap_8u_C3A0C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nAlphaValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTables,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiLUTPaletteSwap_16u_C3A0C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    nAlphaValue: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pTables: *mut *const Npp16u,
    nBitSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiLUTPaletteSwap_16u_C3A0C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nAlphaValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pTables,
                            nBitSize,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV420_8u_ColorTwist32f_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV420_8u_ColorTwist32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV420_16u_ColorTwist32f_P3R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV420_16u_ColorTwist32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV420_8u_ColorTwist32f_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV420_8u_ColorTwist32f_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV420_16u_ColorTwist32f_C3P3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV420_16u_ColorTwist32f_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_8u_ColorTwist32f_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_8u_ColorTwist32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_16u_ColorTwist32f_P3R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_16u_ColorTwist32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_8u_ColorTwist32f_C3C2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_8u_ColorTwist32f_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_16u_ColorTwist32f_C3C2R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_16u_ColorTwist32f_C3C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_8u_ColorTwist32f_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_8u_ColorTwist32f_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToYUV422_16u_ColorTwist32f_C3P3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToYUV422_16u_ColorTwist32f_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToNV12_8u_ColorTwist32f_C3P2R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToNV12_8u_ColorTwist32f_C3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToNV12_16u_ColorTwist32f_C3P2R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToNV12_16u_ColorTwist32f_C3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToNV12_8u_ColorTwist32f_P3P2R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToNV12_8u_ColorTwist32f_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBToNV12_16u_ColorTwist32f_P3P2R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBToNV12_16u_ColorTwist32f_P3P2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_8u_ColorTwist32f_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_8u_ColorTwist32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_16u_ColorTwist32f_P3R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_16u_ColorTwist32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_8u_ColorTwist32f_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_8u_ColorTwist32f_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_16u_ColorTwist32f_P3C3R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_16u_ColorTwist32f_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_8u_ColorTwist32f_P3C4R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_8u_ColorTwist32f_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_16u_ColorTwist32f_P3C4R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_16u_ColorTwist32f_P3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_8u_ColorTwist32f_P3AC4R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nAlpha: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_8u_ColorTwist32f_P3AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nAlpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV420ToRGB_16u_ColorTwist32f_P3AC4R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nAlpha: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV420ToRGB_16u_ColorTwist32f_P3AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nAlpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_8u_ColorTwist32f_C2C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_8u_ColorTwist32f_C2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_16u_ColorTwist32f_C2C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_16u_ColorTwist32f_C2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_8u_ColorTwist32f_P3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_8u_ColorTwist32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_16u_ColorTwist32f_P3R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    aDstStep: *mut ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_16u_ColorTwist32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            aDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_8u_ColorTwist32f_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_8u_ColorTwist32f_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_16u_ColorTwist32f_P3C3R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_16u_ColorTwist32f_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_8u_ColorTwist32f_P3AC4R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nAlpha: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_8u_ColorTwist32f_P3AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nAlpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiYUV422ToRGB_16u_ColorTwist32f_P3AC4R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nAlpha: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiYUV422ToRGB_16u_ColorTwist32f_P3AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nAlpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToRGB_8u_ColorTwist32f_P2C3R_Ctx(
    pSrc: *const *const Npp8u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToRGB_8u_ColorTwist32f_P2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiNV12ToRGB_16u_ColorTwist32f_P2C3R_Ctx(
    pSrc: *const *const Npp16u,
    aSrcStep: *mut ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aTwist: *const [Npp32f; 4usize],
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiNV12ToRGB_16u_ColorTwist32f_P2C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            aSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aTwist,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGB_8u_ToUYVP_10u_C3C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGB_8u_ToUYVP_10u_C3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGB_8u_ToUYVP_10u_P3C3R_Ctx(
    pSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGB_8u_ToUYVP_10u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGB_8u_ToUYVP_10u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGB_8u_ToUYVP_10u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBA_8u_ToUYVP_10u_AC4C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBA_8u_ToUYVP_10u_AC4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGB_16u_ToUYVP_10u_C3C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGB_16u_ToUYVP_10u_C3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGB_16u_ToUYVP_10u_P3C3R_Ctx(
    pSrc: *const *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGB_16u_ToUYVP_10u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRGBA_16u_ToUYVP_10u_AC4C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRGBA_16u_ToUYVP_10u_AC4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiUYVP_10u_ToRGB_8u_C3C3R_Ctx(
    pSrc: *const Npp8u,
    srcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    dstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiUYVP_10u_ToRGB_8u_C3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            srcStep,
                            oSrcOffset,
                            pDst,
                            dstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiUYVP_10u_ToRGB_8u_C3AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    alpha: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiUYVP_10u_ToRGB_8u_C3AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            alpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiUYVP_10u_ToRGB_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiUYVP_10u_ToRGB_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiUYVP_10u_ToRGB_16u_C3C3R_Ctx(
    pSrc: *const Npp8u,
    srcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    dstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiUYVP_10u_ToRGB_16u_C3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            srcStep,
                            oSrcOffset,
                            pDst,
                            dstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiUYVP_10u_ToRGB_16u_C3AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    alpha: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiUYVP_10u_ToRGB_16u_C3AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            alpha,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiUYVP_10u_ToRGB_16u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcOffset: NppiPoint,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eColorSpace: NppiColorSpace,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppicc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiUYVP_10u_ToRGB_16u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eColorSpace,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
