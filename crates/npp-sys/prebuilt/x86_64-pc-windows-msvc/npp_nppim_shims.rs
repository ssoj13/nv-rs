#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilateBorder_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilateBorder_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGrayDilateBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp32s,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGrayDilateBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGrayDilateBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp32f,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGrayDilateBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_8u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_8u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_8u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_8u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_16u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_16u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_16u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_16u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_32f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_32f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_32f_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3_64f_C1R_Ctx(
    pSrc: *const Npp64f,
    nSrcStep: Npp32s,
    pDst: *mut Npp64f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3_64f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiDilate3x3Border_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDilate3x3Border_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDilate3x3Border_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErodeBorder_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErodeBorder_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGrayErodeBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp32s,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGrayErodeBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGrayErodeBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    pMask: *const Npp32f,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGrayErodeBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_8u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_8u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_8u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_8u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_16u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_16u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_16u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_16u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_32f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_32f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_32f_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3_64f_C1R_Ctx(
    pSrc: *const Npp64f,
    nSrcStep: Npp32s,
    pDst: *mut Npp64f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3_64f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiErode3x3Border_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: Npp32s,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: Npp32s,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiErode3x3Border_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiErode3x3Border_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGetBufferSize_8u_C1R(
    oSizeROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGetBufferSize_8u_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSizeROI, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGetBufferSize_8u_C3R(
    oSizeROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGetBufferSize_8u_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSizeROI, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGetBufferSize_8u_C4R(
    oSizeROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGetBufferSize_8u_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSizeROI, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGetBufferSize_16u_C1R(
    oSizeROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGetBufferSize_16u_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSizeROI, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGetBufferSize_16s_C1R(
    oSizeROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGetBufferSize_16s_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSizeROI, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGetBufferSize_32f_C1R(
    oSizeROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGetBufferSize_32f_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSizeROI, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGetBufferSize_32f_C3R(
    oSizeROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGetBufferSize_32f_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSizeROI, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGetBufferSize_32f_C4R(
    oSizeROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGetBufferSize_32f_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSizeROI, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphCloseBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphCloseBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphCloseBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphCloseBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphCloseBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphCloseBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphCloseBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphCloseBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphCloseBorder_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphCloseBorder_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphCloseBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphCloseBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphCloseBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphCloseBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphCloseBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphCloseBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphOpenBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphOpenBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphOpenBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphOpenBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphOpenBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphOpenBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphOpenBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphOpenBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphOpenBorder_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphOpenBorder_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphOpenBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphOpenBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphOpenBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphOpenBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphOpenBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphOpenBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphTopHatBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphTopHatBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphTopHatBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphTopHatBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphTopHatBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphTopHatBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphTopHatBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphTopHatBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphTopHatBorder_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphTopHatBorder_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphTopHatBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphTopHatBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphTopHatBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphTopHatBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphTopHatBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphTopHatBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphBlackHatBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphBlackHatBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphBlackHatBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphBlackHatBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphBlackHatBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphBlackHatBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphBlackHatBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphBlackHatBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphBlackHatBorder_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphBlackHatBorder_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphBlackHatBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphBlackHatBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphBlackHatBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphBlackHatBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphBlackHatBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphBlackHatBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGradientBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGradientBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGradientBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGradientBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGradientBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGradientBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGradientBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGradientBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGradientBorder_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGradientBorder_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGradientBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGradientBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGradientBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGradientBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMorphGradientBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcOffset: NppiPoint,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    oMaskSize: NppiSize,
    oAnchor: NppiPoint,
    pBuffer: *mut Npp8u,
    eBorderType: NppiBorderType,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppim_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMorphGradientBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcOffset,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            oMaskSize,
                            oAnchor,
                            pBuffer,
                            eBorderType,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
