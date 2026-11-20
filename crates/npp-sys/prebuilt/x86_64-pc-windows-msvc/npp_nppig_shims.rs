#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGetResizeRect(
    oSrcROI: NppiRect,
    pDstRect: *mut NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetResizeRect {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSrcROI,
                            pDstRect,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
                            eInterpolation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiResizeSqrPixel_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_8u_P4R_Ctx(
    pSrc: *const *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16u_P3R_Ctx(
    pSrc: *const *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16u_P4R_Ctx(
    pSrc: *const *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16s_P3R_Ctx(
    pSrc: *const *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_16s_P4R_Ctx(
    pSrc: *const *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_16s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_32f_P3R_Ctx(
    pSrc: *const *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_32f_P4R_Ctx(
    pSrc: *const *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_64f_C1R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_64f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_64f_C3R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_64f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_64f_C4R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_64f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_64f_AC4R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_64f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_64f_P3R_Ctx(
    pSrc: *const *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_64f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeSqrPixel_64f_P4R_Ctx(
    pSrc: *const *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    nXShift: f64,
    nYShift: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_64f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            nXShift,
                            nYShift,
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
pub unsafe extern "C" fn nppiResizeAdvancedGetBufferHostSize_8u_C1R(
    oSrcROI: NppiSize,
    oDstROI: NppiSize,
    hpBufferSize: *mut ::std::os::raw::c_int,
    eInterpolationMode: ::std::os::raw::c_int,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeAdvancedGetBufferHostSize_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSrcROI, oDstROI, hpBufferSize, eInterpolationMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiResizeSqrPixel_8u_C1R_Advanced_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nXFactor: f64,
    nYFactor: f64,
    pBuffer: *mut Npp8u,
    eInterpolationMode: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeSqrPixel_8u_C1R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nXFactor,
                            nYFactor,
                            pBuffer,
                            eInterpolationMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGetResizeTiledSourceOffset(
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    pNewSrcRectOffset: *mut NppiPoint,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetResizeTiledSourceOffset {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(oSrcRectROI, oDstRectROI, pNewSrcRectOffset) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiResize_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_8u_P3R_Ctx(
    pSrc: *mut *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_8u_P4R_Ctx(
    pSrc: *mut *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16u_P3R_Ctx(
    pSrc: *mut *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16u_P4R_Ctx(
    pSrc: *mut *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16s_P3R_Ctx(
    pSrc: *mut *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16s_P4R_Ctx(
    pSrc: *mut *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16f_C1R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16f_C3R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_16f_C4R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_32f_P3R_Ctx(
    pSrc: *mut *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResize_32f_P4R_Ctx(
    pSrc: *mut *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResize_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSize,
                            oSrcRectROI,
                            pDst,
                            nDstStep,
                            oDstSize,
                            oDstRectROI,
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
pub unsafe extern "C" fn nppiResizeBatch_8u_C1R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oSmallestDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiResizeBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oSmallestDstSize,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiResizeBatch_8u_C3R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oSmallestDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiResizeBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oSmallestDstSize,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiResizeBatch_8u_C4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oSmallestDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiResizeBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oSmallestDstSize,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiResizeBatch_8u_AC4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oSmallestDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiResizeBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oSmallestDstSize,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiResizeBatch_32f_C1R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oSmallestDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiResizeBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oSmallestDstSize,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiResizeBatch_32f_C3R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oSmallestDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiResizeBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oSmallestDstSize,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiResizeBatch_32f_C4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oSmallestDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiResizeBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oSmallestDstSize,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiResizeBatch_32f_AC4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oSmallestDstSize: NppiSize,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiResizeBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oSmallestDstSize,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiResizeBatch_8u_C1R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_8u_C1R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_8u_C3R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_8u_C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_8u_C4R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_8u_C4R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_8u_AC4R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_8u_AC4R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_16f_C1R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_16f_C1R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_16f_C3R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_16f_C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_16f_C4R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_16f_C4R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_32f_C1R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_32f_C1R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_32f_C3R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_32f_C3R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_32f_C4R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_32f_C4R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiResizeBatch_32f_AC4R_Advanced_Ctx(
    nMaxWidth: ::std::os::raw::c_int,
    nMaxHeight: ::std::os::raw::c_int,
    pBatchSrc: *mut NppiImageDescriptor,
    pBatchDst: *mut NppiImageDescriptor,
    pBatchROI: *mut NppiResizeBatchROI_Advanced,
    nBatchSize: ::std::os::raw::c_uint,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiResizeBatch_32f_AC4R_Advanced_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nMaxWidth,
                            nMaxHeight,
                            pBatchSrc,
                            pBatchDst,
                            pBatchROI,
                            nBatchSize,
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
pub unsafe extern "C" fn nppiRemap_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_8u_P3R_Ctx(
    pSrc: *const *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_8u_P4R_Ctx(
    pSrc: *const *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16u_P3R_Ctx(
    pSrc: *const *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16u_P4R_Ctx(
    pSrc: *const *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16s_P3R_Ctx(
    pSrc: *const *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_16s_P4R_Ctx(
    pSrc: *const *const Npp16s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_16s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_32f_P3R_Ctx(
    pSrc: *const *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_32f_P4R_Ctx(
    pSrc: *const *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp32f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp32f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_64f_C1R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp64f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp64f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_64f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_64f_C3R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp64f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp64f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_64f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_64f_C4R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp64f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp64f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_64f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_64f_AC4R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp64f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp64f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_64f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_64f_P3R_Ctx(
    pSrc: *const *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp64f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp64f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_64f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiRemap_64f_P4R_Ctx(
    pSrc: *const *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pXMap: *const Npp64f,
    nXMapStep: ::std::os::raw::c_int,
    pYMap: *const Npp64f,
    nYMapStep: ::std::os::raw::c_int,
    pDst: *mut *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRemap_64f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pXMap,
                            nXMapStep,
                            pYMap,
                            nYMapStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
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
pub unsafe extern "C" fn nppiGetRotateQuad(
    oSrcROI: NppiRect,
    aQuad: *mut [f64; 2usize],
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetRotateQuad {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(oSrcROI, aQuad, nAngle, nShiftX, nShiftY) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGetRotateBound(
    oSrcROI: NppiRect,
    aBoundingBox: *mut [f64; 2usize],
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetRotateBound {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSrcROI, aBoundingBox, nAngle, nShiftX, nShiftY)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiRotate_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiRotate_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    nAngle: f64,
    nShiftX: f64,
    nShiftY: f64,
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiRotate_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            nAngle,
                            nShiftX,
                            nShiftY,
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
pub unsafe extern "C" fn nppiMirror_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_8u_C4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16u_C4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16s_C4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16s_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32s_C1IR_Ctx(
    pSrcDst: *mut Npp32s,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32s_C3IR_Ctx(
    pSrcDst: *mut Npp32s,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32s_C4IR_Ctx(
    pSrcDst: *mut Npp32s,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32s_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32s_AC4IR_Ctx(
    pSrcDst: *mut Npp32s,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32f_C4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oROI,
                            flip,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirror_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oROI: NppiSize,
    flip: NppiAxis,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirror_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oROI, flip, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirrorBatch_32f_C1R_Ctx(
    oSizeROI: NppiSize,
    flip: NppiAxis,
    pBatchList: *mut NppiMirrorBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirrorBatch_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSizeROI, flip, pBatchList, nBatchSize, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirrorBatch_32f_C1IR_Ctx(
    oSizeROI: NppiSize,
    flip: NppiAxis,
    pBatchList: *mut NppiMirrorBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirrorBatch_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSizeROI, flip, pBatchList, nBatchSize, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirrorBatch_32f_C3R_Ctx(
    oSizeROI: NppiSize,
    flip: NppiAxis,
    pBatchList: *mut NppiMirrorBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirrorBatch_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSizeROI, flip, pBatchList, nBatchSize, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirrorBatch_32f_C3IR_Ctx(
    oSizeROI: NppiSize,
    flip: NppiAxis,
    pBatchList: *mut NppiMirrorBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirrorBatch_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSizeROI, flip, pBatchList, nBatchSize, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirrorBatch_32f_C4R_Ctx(
    oSizeROI: NppiSize,
    flip: NppiAxis,
    pBatchList: *mut NppiMirrorBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirrorBatch_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSizeROI, flip, pBatchList, nBatchSize, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirrorBatch_32f_C4IR_Ctx(
    oSizeROI: NppiSize,
    flip: NppiAxis,
    pBatchList: *mut NppiMirrorBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirrorBatch_32f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSizeROI, flip, pBatchList, nBatchSize, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirrorBatch_32f_AC4R_Ctx(
    oSizeROI: NppiSize,
    flip: NppiAxis,
    pBatchList: *mut NppiMirrorBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirrorBatch_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSizeROI, flip, pBatchList, nBatchSize, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiMirrorBatch_32f_AC4IR_Ctx(
    oSizeROI: NppiSize,
    flip: NppiAxis,
    pBatchList: *mut NppiMirrorBatchCXR,
    nBatchSize: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiMirrorBatch_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(oSizeROI, flip, pBatchList, nBatchSize, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGetAffineTransform(
    oSrcROI: NppiRect,
    aQuad: *const [f64; 2usize],
    aCoeffs: *mut [f64; 3usize],
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetAffineTransform {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSrcROI, aQuad, aCoeffs) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGetAffineQuad(
    oSrcROI: NppiRect,
    aQuad: *mut [f64; 2usize],
    aCoeffs: *const [f64; 3usize],
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetAffineQuad {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSrcROI, aQuad, aCoeffs) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGetAffineBound(
    oSrcROI: NppiRect,
    aBound: *mut [f64; 2usize],
    aCoeffs: *const [f64; 3usize],
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetAffineBound {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSrcROI, aBound, aCoeffs) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiWarpAffine_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_8u_P3R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_8u_P4R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16u_P3R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16u_P4R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32s_P3R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32s_P4R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16f_C1R_Ctx(
    pSrc: *const Npp16f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16f_C3R_Ctx(
    pSrc: *const Npp16f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_16f_C4R_Ctx(
    pSrc: *const Npp16f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32f_P3R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_32f_P4R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_64f_C1R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_64f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_64f_C3R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_64f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_64f_C4R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_64f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_64f_AC4R_Ctx(
    pSrc: *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_64f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_64f_P3R_Ctx(
    aSrc: *mut *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aDst: *mut *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_64f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffine_64f_P4R_Ctx(
    aSrc: *mut *const Npp64f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aDst: *mut *mut Npp64f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffine_64f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBatchInit_Ctx(
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatchInit_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pBatchList, nBatchSize, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiWarpAffineBatch_8u_C1R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_8u_C3R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_8u_C4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_8u_AC4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_16f_C1R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_16f_C3R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_16f_C4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_32f_C1R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_32f_C3R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_32f_C4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBatch_32f_AC4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpAffineBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBatch_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpAffineBack_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_8u_P3R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_8u_P4R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_16u_P3R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_16u_P4R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32s_P3R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32s_P4R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32f_P3R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineBack_32f_P4R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineBack_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_8u_P3R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_8u_P4R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_16u_P3R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_16u_P4R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32s_P3R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32s_P4R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32f_P3R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpAffineQuad_32f_P4R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpAffineQuad_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiGetPerspectiveTransform(
    oSrcROI: NppiRect,
    quad: *const [f64; 2usize],
    aCoeffs: *mut [f64; 3usize],
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetPerspectiveTransform {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSrcROI, quad, aCoeffs) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGetPerspectiveQuad(
    oSrcROI: NppiRect,
    quad: *mut [f64; 2usize],
    aCoeffs: *const [f64; 3usize],
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetPerspectiveQuad {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSrcROI, quad, aCoeffs) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiGetPerspectiveBound(
    oSrcROI: NppiRect,
    bound: *mut [f64; 2usize],
    aCoeffs: *const [f64; 3usize],
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiGetPerspectiveBound {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oSrcROI, bound, aCoeffs) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiWarpPerspective_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_8u_P3R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_8u_P4R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16u_P3R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16u_P4R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32s_P3R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32s_P4R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16f_C1R_Ctx(
    pSrc: *const Npp16f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16f_C3R_Ctx(
    pSrc: *const Npp16f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_16f_C4R_Ctx(
    pSrc: *const Npp16f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32f_P3R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspective_32f_P4R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspective_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatchInit_Ctx(
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatchInit_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pBatchList, nBatchSize, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_8u_C1R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_8u_C3R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_8u_C4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_8u_AC4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_16f_C1R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_16f_C3R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_16f_C4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_32f_C1R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_32f_C3R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_32f_C4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBatch_32f_AC4R_Ctx(
    oSmallestSrcSize: NppiSize,
    oSrcRectROI: NppiRect,
    oDstRectROI: NppiRect,
    eInterpolation: ::std::os::raw::c_int,
    pBatchList: *mut NppiWarpPerspectiveBatchCXR,
    nBatchSize: ::std::os::raw::c_uint,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBatch_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            oSmallestSrcSize,
                            oSrcRectROI,
                            oDstRectROI,
                            eInterpolation,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_8u_P3R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_8u_P4R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_16u_P3R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_16u_P4R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32s_P3R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32s_P4R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32f_P3R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveBack_32f_P4R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aCoeffs: *const [f64; 3usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveBack_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aCoeffs,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_8u_P3R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_8u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_8u_P4R_Ctx(
    pSrc: *mut *const Npp8u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_8u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_16u_P3R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_16u_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_16u_P4R_Ctx(
    pSrc: *mut *const Npp16u,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_16u_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32s_P3R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32s_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32s_P4R_Ctx(
    pSrc: *mut *const Npp32s,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32s_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32f_P3R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32f_P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
pub unsafe extern "C" fn nppiWarpPerspectiveQuad_32f_P4R_Ctx(
    pSrc: *mut *const Npp32f,
    oSrcSize: NppiSize,
    nSrcStep: ::std::os::raw::c_int,
    oSrcROI: NppiRect,
    aSrcQuad: *const [f64; 2usize],
    pDst: *mut *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstROI: NppiRect,
    aDstQuad: *const [f64; 2usize],
    eInterpolation: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppig_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiWarpPerspectiveQuad_32f_P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            oSrcSize,
                            nSrcStep,
                            oSrcROI,
                            aSrcQuad,
                            pDst,
                            nDstStep,
                            oDstROI,
                            aDstQuad,
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
