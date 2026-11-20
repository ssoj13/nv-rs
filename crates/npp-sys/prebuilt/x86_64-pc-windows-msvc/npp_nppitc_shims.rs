#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_C1IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_C1IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_C1IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_C1IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_C3IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_C3IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_C3IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_C3IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_8u_AC4IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_8u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16u_AC4IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_16s_AC4IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_16s_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_32f_AC4IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_32f_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nThreshold)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_C1IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nThreshold)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_C1IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nThreshold)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_C1IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nThreshold)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_C1IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_C3IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_C3IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_C3IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_C3IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_8u_AC4IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_8u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16u_AC4IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_16s_AC4IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_16s_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GT_32f_AC4IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GT_32f_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nThreshold)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_C1IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nThreshold)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_C1IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nThreshold)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_C1IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nThreshold)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_C1IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_C3IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_C3IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_C3IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_C3IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_8u_AC4IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_8u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16u_AC4IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_16s_AC4IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_16s_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, rThresholds)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LT_32f_AC4IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LT_32f_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_C1IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_C1IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_C1IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_C1IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_C3IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_C3IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_C3IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_C3IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_8u_AC4IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_8u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16u_AC4IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_16s_AC4IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_16s_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_Val_32f_AC4IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_Val_32f_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_C1IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold, nValue)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_C1IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold, nValue)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_C1IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold, nValue)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_C1IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold, nValue)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_C3IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_C3IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_C3IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_C3IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_8u_AC4IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_8u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16u_AC4IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_16s_AC4IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_16s_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_GTVal_32f_AC4IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_GTVal_32f_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiFusedAbsDiff_Threshold_GTVal_Ctx(
    eSrcDstType: NppDataType,
    eSrcDstChannels: NppiChannels,
    pSrc1: *const ::std::os::raw::c_void,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const ::std::os::raw::c_void,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut ::std::os::raw::c_void,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pThreshold: *const ::std::os::raw::c_void,
    pvalue: *const ::std::os::raw::c_void,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiFusedAbsDiff_Threshold_GTVal_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            eSrcDstType,
                            eSrcDstChannels,
                            pSrc1,
                            nSrc1Step,
                            pSrc2,
                            nSrc2Step,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pThreshold,
                            pvalue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiFusedAbsDiff_Threshold_GTVal_I_Ctx(
    eSrcDstType: NppDataType,
    eSrcDstChannels: NppiChannels,
    pSrcDst: *mut ::std::os::raw::c_void,
    nSrcDstStep: ::std::os::raw::c_int,
    pSrc2: *const ::std::os::raw::c_void,
    nSrc2Step: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pThreshold: *const ::std::os::raw::c_void,
    pvalue: *const ::std::os::raw::c_void,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiFusedAbsDiff_Threshold_GTVal_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            eSrcDstType,
                            eSrcDstChannels,
                            pSrcDst,
                            nSrcDstStep,
                            pSrc2,
                            nSrc2Step,
                            oSizeROI,
                            pThreshold,
                            pvalue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_C1IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp8u,
    nValue: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold, nValue)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_C1IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16u,
    nValue: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold, nValue)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_C1IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp16s,
    nValue: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold, nValue)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThreshold,
                            nValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_C1IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThreshold: Npp32f,
    nValue: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, nThreshold, nValue)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_C3IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_C3IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_C3IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_C3IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_8u_AC4IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp8u,
    rValues: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_8u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16u_AC4IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16u,
    rValues: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_16s_AC4IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp16s,
    rValues: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_16s_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholds,
                            rValues,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTVal_32f_AC4IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholds: *const Npp32f,
    rValues: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTVal_32f_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, rThresholds, rValues)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp8u,
    nValueLT: Npp8u,
    nThresholdGT: Npp8u,
    nValueGT: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp8u,
    nValueLT: Npp8u,
    nThresholdGT: Npp8u,
    nValueGT: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_C1IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp8u,
    nValueLT: Npp8u,
    nThresholdGT: Npp8u,
    nValueGT: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_C1IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp8u,
    nValueLT: Npp8u,
    nThresholdGT: Npp8u,
    nValueGT: Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp16u,
    nValueLT: Npp16u,
    nThresholdGT: Npp16u,
    nValueGT: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp16u,
    nValueLT: Npp16u,
    nThresholdGT: Npp16u,
    nValueGT: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_C1IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp16u,
    nValueLT: Npp16u,
    nThresholdGT: Npp16u,
    nValueGT: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_C1IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp16u,
    nValueLT: Npp16u,
    nThresholdGT: Npp16u,
    nValueGT: Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp16s,
    nValueLT: Npp16s,
    nThresholdGT: Npp16s,
    nValueGT: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp16s,
    nValueLT: Npp16s,
    nThresholdGT: Npp16s,
    nValueGT: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_C1IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp16s,
    nValueLT: Npp16s,
    nThresholdGT: Npp16s,
    nValueGT: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_C1IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp16s,
    nValueLT: Npp16s,
    nThresholdGT: Npp16s,
    nValueGT: Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp32f,
    nValueLT: Npp32f,
    nThresholdGT: Npp32f,
    nValueGT: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp32f,
    nValueLT: Npp32f,
    nThresholdGT: Npp32f,
    nValueGT: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_C1IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp32f,
    nValueLT: Npp32f,
    nThresholdGT: Npp32f,
    nValueGT: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_C1IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_C1IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nThresholdLT: Npp32f,
    nValueLT: Npp32f,
    nThresholdGT: Npp32f,
    nValueGT: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_C1IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            nThresholdLT,
                            nValueLT,
                            nThresholdGT,
                            nValueGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp8u,
    rValuesLT: *const Npp8u,
    rThresholdsGT: *const Npp8u,
    rValuesGT: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp8u,
    rValuesLT: *const Npp8u,
    rThresholdsGT: *const Npp8u,
    rValuesGT: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp8u,
    rValuesLT: *const Npp8u,
    rThresholdsGT: *const Npp8u,
    rValuesGT: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_C3IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp8u,
    rValuesLT: *const Npp8u,
    rThresholdsGT: *const Npp8u,
    rValuesGT: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16u,
    rValuesLT: *const Npp16u,
    rThresholdsGT: *const Npp16u,
    rValuesGT: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16u,
    rValuesLT: *const Npp16u,
    rThresholdsGT: *const Npp16u,
    rValuesGT: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16u,
    rValuesLT: *const Npp16u,
    rThresholdsGT: *const Npp16u,
    rValuesGT: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_C3IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16u,
    rValuesLT: *const Npp16u,
    rThresholdsGT: *const Npp16u,
    rValuesGT: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16s,
    rValuesLT: *const Npp16s,
    rThresholdsGT: *const Npp16s,
    rValuesGT: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16s,
    rValuesLT: *const Npp16s,
    rThresholdsGT: *const Npp16s,
    rValuesGT: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16s,
    rValuesLT: *const Npp16s,
    rThresholdsGT: *const Npp16s,
    rValuesGT: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_C3IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16s,
    rValuesLT: *const Npp16s,
    rThresholdsGT: *const Npp16s,
    rValuesGT: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp32f,
    rValuesLT: *const Npp32f,
    rThresholdsGT: *const Npp32f,
    rValuesGT: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp32f,
    rValuesLT: *const Npp32f,
    rThresholdsGT: *const Npp32f,
    rValuesGT: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp32f,
    rValuesLT: *const Npp32f,
    rThresholdsGT: *const Npp32f,
    rValuesGT: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_C3IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp32f,
    rValuesLT: *const Npp32f,
    rThresholdsGT: *const Npp32f,
    rValuesGT: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp8u,
    rValuesLT: *const Npp8u,
    rThresholdsGT: *const Npp8u,
    rValuesGT: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp8u,
    rValuesLT: *const Npp8u,
    rThresholdsGT: *const Npp8u,
    rValuesGT: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_AC4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp8u,
    rValuesLT: *const Npp8u,
    rThresholdsGT: *const Npp8u,
    rValuesGT: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_8u_AC4IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp8u,
    rValuesLT: *const Npp8u,
    rThresholdsGT: *const Npp8u,
    rValuesGT: *const Npp8u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_8u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16u,
    rValuesLT: *const Npp16u,
    rThresholdsGT: *const Npp16u,
    rValuesGT: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16u,
    rValuesLT: *const Npp16u,
    rThresholdsGT: *const Npp16u,
    rValuesGT: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_AC4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16u,
    rValuesLT: *const Npp16u,
    rThresholdsGT: *const Npp16u,
    rValuesGT: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16u_AC4IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16u,
    rValuesLT: *const Npp16u,
    rThresholdsGT: *const Npp16u,
    rValuesGT: *const Npp16u,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16u_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16s,
    rValuesLT: *const Npp16s,
    rThresholdsGT: *const Npp16s,
    rValuesGT: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16s,
    rValuesLT: *const Npp16s,
    rThresholdsGT: *const Npp16s,
    rValuesGT: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_AC4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16s,
    rValuesLT: *const Npp16s,
    rThresholdsGT: *const Npp16s,
    rValuesGT: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_16s_AC4IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp16s,
    rValuesLT: *const Npp16s,
    rThresholdsGT: *const Npp16s,
    rValuesGT: *const Npp16s,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_16s_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp32f,
    rValuesLT: *const Npp32f,
    rThresholdsGT: *const Npp32f,
    rValuesGT: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp32f,
    rValuesLT: *const Npp32f,
    rThresholdsGT: *const Npp32f,
    rValuesGT: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_AC4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp32f,
    rValuesLT: *const Npp32f,
    rThresholdsGT: *const Npp32f,
    rValuesGT: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_AC4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiThreshold_LTValGTVal_32f_AC4IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    rThresholdsLT: *const Npp32f,
    rValuesLT: *const Npp32f,
    rThresholdsGT: *const Npp32f,
    rValuesGT: *const Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiThreshold_LTValGTVal_32f_AC4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            rThresholdsLT,
                            rValuesLT,
                            rThresholdsGT,
                            rValuesGT,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_8u_C1R_Ctx(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_8u_C1R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_8u_C1R(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_8u_C1R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_8u_C3R_Ctx(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_8u_C3R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_8u_C3R(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_8u_C3R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_8u_C4R_Ctx(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_8u_C4R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_8u_C4R(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_8u_C4R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_8u_AC4R_Ctx(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_8u_AC4R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_8u_AC4R(
    pSrc1: *const Npp8u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp8u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_8u_AC4R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16u_C1R_Ctx(
    pSrc1: *const Npp16u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16u_C1R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16u_C1R(
    pSrc1: *const Npp16u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16u_C1R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16u_C3R_Ctx(
    pSrc1: *const Npp16u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16u_C3R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16u_C3R(
    pSrc1: *const Npp16u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16u_C3R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16u_C4R_Ctx(
    pSrc1: *const Npp16u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16u_C4R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16u_C4R(
    pSrc1: *const Npp16u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16u_C4R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16u_AC4R_Ctx(
    pSrc1: *const Npp16u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16u_AC4R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16u_AC4R(
    pSrc1: *const Npp16u,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16u,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16u_AC4R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16s_C1R_Ctx(
    pSrc1: *const Npp16s,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16s,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16s_C1R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16s_C1R(
    pSrc1: *const Npp16s,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16s,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16s_C1R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16s_C3R_Ctx(
    pSrc1: *const Npp16s,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16s,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16s_C3R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16s_C3R(
    pSrc1: *const Npp16s,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16s,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16s_C3R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16s_C4R_Ctx(
    pSrc1: *const Npp16s,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16s,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16s_C4R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16s_C4R(
    pSrc1: *const Npp16s,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16s,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16s_C4R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16s_AC4R_Ctx(
    pSrc1: *const Npp16s,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16s,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16s_AC4R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_16s_AC4R(
    pSrc1: *const Npp16s,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp16s,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_16s_AC4R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_32f_C1R_Ctx(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_32f_C1R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_32f_C1R(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_32f_C1R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_32f_C3R_Ctx(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_32f_C3R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_32f_C3R(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_32f_C3R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_32f_C4R_Ctx(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_32f_C4R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_32f_C4R(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_32f_C4R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_32f_AC4R_Ctx(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_32f_AC4R_Ctx {
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
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompare_32f_AC4R(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompare_32f_AC4R {
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
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_8u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp16u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp16u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp16s,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp16s,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16s,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16s,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16s,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16s,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16s,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp16s,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareC_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eComparisonOperation: NppCmpOp,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareC_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eComparisonOperation,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEps_32f_C1R_Ctx(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEps_32f_C1R_Ctx {
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
                            nEpsilon,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEps_32f_C1R(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEps_32f_C1R {
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
                            nEpsilon,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEps_32f_C3R_Ctx(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEps_32f_C3R_Ctx {
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
                            nEpsilon,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEps_32f_C3R(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEps_32f_C3R {
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
                            nEpsilon,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEps_32f_C4R_Ctx(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEps_32f_C4R_Ctx {
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
                            nEpsilon,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEps_32f_C4R(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEps_32f_C4R {
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
                            nEpsilon,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEps_32f_AC4R_Ctx(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEps_32f_AC4R_Ctx {
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
                            nEpsilon,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEps_32f_AC4R(
    pSrc1: *const Npp32f,
    nSrc1Step: ::std::os::raw::c_int,
    pSrc2: *const Npp32f,
    nSrc2Step: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEps_32f_AC4R {
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
                            nEpsilon,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEpsC_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEpsC_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nEpsilon,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEpsC_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    nConstant: Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEpsC_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            nConstant,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nEpsilon,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEpsC_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEpsC_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nEpsilon,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEpsC_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEpsC_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nEpsilon,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEpsC_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEpsC_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nEpsilon,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEpsC_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEpsC_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nEpsilon,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEpsC_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEpsC_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nEpsilon,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCompareEqualEpsC_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pConstants: *const Npp32f,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nEpsilon: Npp32f,
) -> NppStatus {
    match nppitc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCompareEqualEpsC_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pConstants,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nEpsilon,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
