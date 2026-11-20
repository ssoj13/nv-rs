#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_C1R_Ctx(
    nValue: Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_C1R(
    nValue: Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_C2R_Ctx(
    aValue: *const Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_C2R(
    aValue: *const Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_C3R_Ctx(
    aValue: *const Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_C3R(
    aValue: *const Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_C4R_Ctx(
    aValue: *const Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_C4R(
    aValue: *const Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_AC4R_Ctx(
    aValue: *const Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8s_AC4R(
    aValue: *const Npp8s,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8s_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C1R_Ctx(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C1R(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C2R_Ctx(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C2R(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C3R_Ctx(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C3R(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C4R_Ctx(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C4R(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_AC4R_Ctx(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_AC4R(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C1R_Ctx(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C1R(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C2R_Ctx(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C2R(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C3R_Ctx(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C3R(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C4R_Ctx(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C4R(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_AC4R_Ctx(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_AC4R(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C1R_Ctx(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C1R(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C2R_Ctx(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C2R(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C3R_Ctx(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C3R(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C4R_Ctx(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C4R(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_AC4R_Ctx(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_AC4R(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_C1R_Ctx(
    oValue: Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(oValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_C1R(
    oValue: Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_C2R_Ctx(
    aValue: *const Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_C2R(
    aValue: *const Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_C3R_Ctx(
    aValue: *const Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_C3R(
    aValue: *const Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_C4R_Ctx(
    aValue: *const Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_C4R(
    aValue: *const Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_AC4R_Ctx(
    aValue: *const Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16sc_AC4R(
    aValue: *const Npp16sc,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16sc_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C1R_Ctx(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C1R(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C2R_Ctx(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C2R(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C3R_Ctx(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C3R(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C4R_Ctx(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C4R(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_AC4R_Ctx(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_AC4R(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_C1R_Ctx(
    nValue: Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_C1R(
    nValue: Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_C2R_Ctx(
    aValue: *const Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_C2R(
    aValue: *const Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_C3R_Ctx(
    aValue: *const Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_C3R(
    aValue: *const Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_C4R_Ctx(
    aValue: *const Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_C4R(
    aValue: *const Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_AC4R_Ctx(
    aValue: *const Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32u_AC4R(
    aValue: *const Npp32u,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32u_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_C1R_Ctx(
    oValue: Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(oValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_C1R(
    oValue: Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_C2R_Ctx(
    aValue: *const Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_C2R(
    aValue: *const Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_C3R_Ctx(
    aValue: *const Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_C3R(
    aValue: *const Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_C4R_Ctx(
    aValue: *const Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_C4R(
    aValue: *const Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_AC4R_Ctx(
    aValue: *const Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32sc_AC4R(
    aValue: *const Npp32sc,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32sc_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16f_C1R_Ctx(
    nValue: Npp32f,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16f_C1R(
    nValue: Npp32f,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16f_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16f_C2R_Ctx(
    aValues: *const Npp32f,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16f_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValues, pDst, nDstStep, oSizeROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16f_C2R(
    aValues: *const Npp32f,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16f_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValues, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16f_C3R_Ctx(
    aValues: *const Npp32f,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValues, pDst, nDstStep, oSizeROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16f_C3R(
    aValues: *const Npp32f,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16f_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValues, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16f_C4R_Ctx(
    aValues: *const Npp32f,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValues, pDst, nDstStep, oSizeROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16f_C4R(
    aValues: *const Npp32f,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16f_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValues, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C1R_Ctx(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C1R(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C2R_Ctx(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C2R(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C3R_Ctx(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C3R(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C4R_Ctx(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C4R(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_AC4R_Ctx(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_AC4R(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_C1R_Ctx(
    oValue: Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(oValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_C1R(
    oValue: Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_C1R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(oValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_C2R_Ctx(
    aValue: *const Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_C2R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_C2R(
    aValue: *const Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_C2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_C3R_Ctx(
    aValue: *const Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_C3R(
    aValue: *const Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_C3R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_C4R_Ctx(
    aValue: *const Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_C4R(
    aValue: *const Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_C4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_AC4R_Ctx(
    aValue: *const Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32fc_AC4R(
    aValue: *const Npp32fc,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32fc_AC4R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C1MR_Ctx(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C1MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C1MR(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C1MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C3MR_Ctx(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C3MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C3MR(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C3MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C4MR_Ctx(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C4MR(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_AC4MR_Ctx(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_AC4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_AC4MR(
    aValue: *const Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_AC4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C1MR_Ctx(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C1MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C1MR(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C1MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C3MR_Ctx(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C3MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C3MR(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C3MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C4MR_Ctx(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C4MR(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_AC4MR_Ctx(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_AC4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_AC4MR(
    aValue: *const Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_AC4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C1MR_Ctx(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C1MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C1MR(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C1MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C3MR_Ctx(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C3MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C3MR(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C3MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C4MR_Ctx(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C4MR(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_AC4MR_Ctx(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_AC4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_AC4MR(
    aValue: *const Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_AC4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C1MR_Ctx(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C1MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C1MR(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C1MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C3MR_Ctx(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C3MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C3MR(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C3MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C4MR_Ctx(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C4MR(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_AC4MR_Ctx(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_AC4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_AC4MR(
    aValue: *const Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_AC4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C1MR_Ctx(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C1MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            nValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C1MR(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C1MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C3MR_Ctx(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C3MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C3MR(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C3MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C4MR_Ctx(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C4MR(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_AC4MR_Ctx(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_AC4MR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aValue,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            pMask,
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_AC4MR(
    aValue: *const Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_AC4MR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(aValue, pDst, nDstStep, oSizeROI, pMask, nMaskStep)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C3CR_Ctx(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C3CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C3CR(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C3CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C4CR_Ctx(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C4CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_8u_C4CR(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_8u_C4CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C3CR_Ctx(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C3CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C3CR(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C3CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C4CR_Ctx(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C4CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16u_C4CR(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16u_C4CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C3CR_Ctx(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C3CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C3CR(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C3CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C4CR_Ctx(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C4CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_16s_C4CR(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_16s_C4CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C3CR_Ctx(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C3CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C3CR(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C3CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C4CR_Ctx(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C4CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32s_C4CR(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32s_C4CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C3CR_Ctx(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C3CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C3CR(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C3CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C4CR_Ctx(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C4CR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSet_32f_C4CR(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSet_32f_C4CR {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nDstStep, oSizeROI) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8s_C1R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8s_C1R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8s_C2R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_C2R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8s_C2R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_C2R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8s_C3R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8s_C3R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8s_C4R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8s_C4R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8s_AC4R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8s_AC4R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16sc_C1R_Ctx(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16sc_C1R(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16sc_C2R_Ctx(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_C2R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16sc_C2R(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_C2R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16sc_C3R_Ctx(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16sc_C3R(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16sc_C4R_Ctx(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16sc_C4R(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16sc_AC4R_Ctx(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16sc_AC4R(
    pSrc: *const Npp16sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16sc_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32sc_C1R_Ctx(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32sc_C1R(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32sc_C2R_Ctx(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_C2R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32sc_C2R(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_C2R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32sc_C3R_Ctx(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32sc_C3R(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32sc_C4R_Ctx(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32sc_C4R(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32sc_AC4R_Ctx(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32sc_AC4R(
    pSrc: *const Npp32sc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32sc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32sc_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16f_C1R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16f_C1R(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16f_C3R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16f_C3R(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16f_C4R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16f_C4R(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32fc_C1R_Ctx(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32fc_C1R(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32fc_C2R_Ctx(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_C2R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32fc_C2R(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_C2R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32fc_C3R_Ctx(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32fc_C3R(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32fc_C4R_Ctx(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32fc_C4R(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32fc_AC4R_Ctx(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32fc_AC4R(
    pSrc: *const Npp32fc,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32fc,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32fc_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C1MR_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C1MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C1MR(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C1MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C3MR_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C3MR(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C4MR_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C4MR(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_AC4MR_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_AC4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_AC4MR(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_AC4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C1MR_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C1MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C1MR(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C1MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C3MR_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C3MR(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C4MR_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C4MR(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_AC4MR_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_AC4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_AC4MR(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_AC4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C1MR_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C1MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C1MR(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C1MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C3MR_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C3MR(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C4MR_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C4MR(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_AC4MR_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_AC4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_AC4MR(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_AC4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C1MR_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C1MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C1MR(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C1MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C3MR_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C3MR(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C4MR_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C4MR(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_AC4MR_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_AC4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_AC4MR(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_AC4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C1MR_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C1MR(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C3MR_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C3MR(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C4MR_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C4MR(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_AC4MR_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_AC4MR_Ctx {
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
                            nMaskStep,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_AC4MR(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    pMask: *const Npp8u,
    nMaskStep: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_AC4MR {
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
                            nMaskStep,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C3CR_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C3CR(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C4CR_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C4CR(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C3CR_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C3CR(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C4CR_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C4CR(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C3CR_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C3CR(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C4CR_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C4CR(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C3CR_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C3CR(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C4CR_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C4CR(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C3CR_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C3CR(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C4CR_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4CR_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C4CR(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4CR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C3C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C3C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C4C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C4C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C3C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C3C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C4C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C4C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C3C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C3C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C4C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C4C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C3C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C3C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C4C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C4C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C2C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C2C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C2C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C2C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C3C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C3C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C4C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4C1R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C4C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C1C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C1C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C1C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C1C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C1C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_8u_C1C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C1C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C1C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C1C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C1C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C1C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16s_C1C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C1C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C1C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C1C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C1C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C1C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_16u_C1C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C1C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C1C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C1C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C1C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C1C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32s_C1C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C1C2R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1C2R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C1C2R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1C2R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C1C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1C3R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C1C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C1C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1C4R_Ctx {
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
pub unsafe extern "C" fn nppiCopy_32f_C1C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C3P3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_8u_C3P3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C3P3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_C4P4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_8u_C4P4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_C4P4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C3P3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_16s_C3P3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C3P3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_C4P4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_16s_C4P4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_C4P4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C3P3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_16u_C3P3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C3P3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_C4P4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_16u_C4P4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_C4P4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C3P3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_32s_C3P3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C3P3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_C4P4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_32s_C4P4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_C4P4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C3P3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3P3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_32f_C3P3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C3P3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_C4P4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4P4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            aDst,
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
pub unsafe extern "C" fn nppiCopy_32f_C4P4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    aDst: *const *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_C4P4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, aDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_P3C3R_Ctx(
    aSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_8u_P3C3R(
    aSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_P3C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_8u_P4C4R_Ctx(
    aSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_P4C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_8u_P4C4R(
    aSrc: *const *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_8u_P4C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_P3C3R_Ctx(
    aSrc: *const *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_16u_P3C3R(
    aSrc: *const *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_P3C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16u_P4C4R_Ctx(
    aSrc: *const *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_P4C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_16u_P4C4R(
    aSrc: *const *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16u_P4C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_P3C3R_Ctx(
    aSrc: *const *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_16s_P3C3R(
    aSrc: *const *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_P3C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_16s_P4C4R_Ctx(
    aSrc: *const *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_P4C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_16s_P4C4R(
    aSrc: *const *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_16s_P4C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_P3C3R_Ctx(
    aSrc: *const *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_32s_P3C3R(
    aSrc: *const *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_P3C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32s_P4C4R_Ctx(
    aSrc: *const *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_P4C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_32s_P4C4R(
    aSrc: *const *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32s_P4C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_P3C3R_Ctx(
    aSrc: *const *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_P3C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_32f_P3C3R(
    aSrc: *const *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_P3C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopy_32f_P4C4R_Ctx(
    aSrc: *const *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_P4C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            aSrc,
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
pub unsafe extern "C" fn nppiCopy_32f_P4C4R(
    aSrc: *const *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopy_32f_P4C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(aSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp8u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp8u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_8u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp8u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp8u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp16u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp16s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp16s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp32s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_32s_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp32s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32s_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32s_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32s_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nValue: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
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
pub unsafe extern "C" fn nppiCopyConstBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyConstBorder_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    aValue: *const Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyConstBorder_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            aValue,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_8u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32s_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32s_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32s_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32s_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyReplicateBorder_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyReplicateBorder_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_8u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32s_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32s_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32s_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32s_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopyWrapBorder_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    oSrcSizeROI: NppiSize,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nTopBorderHeight: ::std::os::raw::c_int,
    nLeftBorderWidth: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopyWrapBorder_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            oSrcSizeROI,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nTopBorderHeight,
                            nLeftBorderWidth,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_8u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32s_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32s_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32s_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32s_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nDx,
                            nDy,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiCopySubpix_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nDx: Npp32f,
    nDy: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiCopySubpix_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI, nDx, nDy)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u16u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u16u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u16u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u16u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u16u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u16u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u16u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u16u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u16s_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u16s_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u16s_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u16s_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u16s_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u16s_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u16s_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u16s_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u32s_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u32s_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u32s_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u32s_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u32s_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u32s_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u32s_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u32s_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u32f_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u32f_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u32f_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u32f_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u32f_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u32f_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u32f_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32f_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8u32f_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32s_C1R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32s_C1R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32s_C3R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32s_C3R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32s_C4R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32s_C4R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32s_AC4R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32s_AC4R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32f_C1R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32f_C1R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32f_C3R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32f_C3R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32f_C4R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32f_C4R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32f_AC4R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32f_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32f_AC4R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32s_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32s_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32s_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32s_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32s_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32s_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32s_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32s_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32f_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32f_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32f_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32f_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32f_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32f_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32f_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32f_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32f_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32f_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32f_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32f_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32f_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32f_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32f_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32f_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32f_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32f_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16f32f_C1R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16f32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16f32f_C1R(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16f32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16f32f_C3R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16f32f_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16f32f_C3R(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16f32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16f32f_C4R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16f32f_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16f32f_C4R(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16f32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16f32f_AC4R_Ctx(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16f32f_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16f32f_AC4R(
    pSrc: *const Npp16f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16f32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s8u_C1Rs_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s8u_C1Rs_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s8u_C1Rs(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s8u_C1Rs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s16u_C1Rs_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s16u_C1Rs_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s16u_C1Rs(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s16u_C1Rs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s16s_C1R_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s16s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s16s_C1R(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8s32u_C1Rs_Ctx(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32u_C1Rs_Ctx {
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
pub unsafe extern "C" fn nppiConvert_8s32u_C1Rs(
    pSrc: *const Npp8s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8s32u_C1Rs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s16u_C1Rs_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s16u_C1Rs_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s16u_C1Rs(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s16u_C1Rs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s32u_C1Rs_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32u_C1Rs_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s32u_C1Rs(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s32u_C1Rs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u32u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u32u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u32u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s32u_C1Rs_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s32u_C1Rs_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s32u_C1Rs(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s32u_C1Rs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s32f_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s32f_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u32f_C1R_Ctx(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u32f_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32u32f_C1R(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u8u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u8u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u8u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u8u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u8u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u8u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u8u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16u8u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s8u_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s8u_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s8u_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s8u_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s8u_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s8u_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s8u_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_16s8u_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s8u_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s8u_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s8u_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s8u_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s8u_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s8u_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s8u_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s8u_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s8s_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s8s_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s8s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s8s_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s8s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s8s_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s8s_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiConvert_32s8s_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s8s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u8s_C1RSfs_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u8s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_8u8s_C1RSfs(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_8u8s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u8s_C1RSfs_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u8s_C1RSfs(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u8s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s8s_C1RSfs_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16s8s_C1RSfs(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16s8s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u16s_C1RSfs_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u16s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_16u16s_C1RSfs(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_16u16s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u8u_C1RSfs_Ctx(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u8u_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u8u_C1RSfs(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u8u_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u8s_C1RSfs_Ctx(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u8s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u8s_C1RSfs(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u8s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u16u_C1RSfs_Ctx(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u16u_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u16u_C1RSfs(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u16u_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u16s_C1RSfs_Ctx(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u16s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u16s_C1RSfs(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u16s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u32s_C1RSfs_Ctx(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u32s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32u32s_C1RSfs(
    pSrc: *const Npp32u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32u32s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s16u_C1RSfs_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s16u_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s16u_C1RSfs(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s16u_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s16s_C1RSfs_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s16s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32s16s_C1RSfs(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32s16s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, eRoundMode)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_C1RSfs_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8u_C1RSfs(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8u_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_C1RSfs_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f8s_C1RSfs(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f8s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_C1RSfs_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16u_C1RSfs(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16u_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_C1RSfs_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f16s_C1RSfs(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f16s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f32u_C1RSfs_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f32u_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f32u_C1RSfs(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f32u_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f32s_C1RSfs_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f32s_C1RSfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiConvert_32f32s_C1RSfs(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiConvert_32f32s_C1RSfs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            eRoundMode,
                            nScaleFactor,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u16u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16u_C1R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u16u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u16u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16u_C3R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u16u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u16u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16u_C4R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u16u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u16u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16u_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u16u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u16s_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u16s_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u16s_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u16s_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u16s_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u16s_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u16s_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u16s_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32s_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32s_C1R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u32s_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32s_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32s_C3R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u32s_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32s_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32s_C4R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u32s_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32s_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32s_AC4R_Ctx {
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
pub unsafe extern "C" fn nppiScale_8u32s_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32f_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nMin,
                            nMax,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32f_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nMin, nMax)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32f_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nMin,
                            nMax,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32f_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nMin, nMax)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32f_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nMin,
                            nMax,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32f_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nMin, nMax)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32f_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nMin,
                            nMax,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_8u32f_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_8u32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nMin, nMax)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16u8u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16u8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16u8u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16u8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16u8u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16u8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16u8u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16u8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16u8u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16u8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16u8u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16u8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16u8u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16u8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16u8u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16u8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16s8u_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16s8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16s8u_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16s8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16s8u_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16s8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16s8u_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16s8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16s8u_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16s8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16s8u_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16s8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16s8u_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16s8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_16s8u_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_16s8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32s8u_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32s8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32s8u_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32s8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32s8u_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32s8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32s8u_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32s8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32s8u_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32s8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32s8u_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32s8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32s8u_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32s8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            hint,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32s8u_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    hint: NppHintAlgorithm,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32s8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, hint) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32f8u_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32f8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nMin,
                            nMax,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32f8u_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32f8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nMin, nMax)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32f8u_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32f8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nMin,
                            nMax,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32f8u_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32f8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nMin, nMax)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32f8u_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32f8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nMin,
                            nMax,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32f8u_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32f8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nMin, nMax)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32f8u_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32f8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            nMin,
                            nMax,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiScale_32f8u_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    nMin: Npp32f,
    nMax: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiScale_32f8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, nMin, nMax)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_8u_C1C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_8u_C1C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_8u_C1C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_8u_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_8u_C1C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_8u_C1C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_8u_C1C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_8u_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_8u_C1AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_8u_C1AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_8u_C1AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_8u_C1AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16u_C1C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16u_C1C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16u_C1C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16u_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16u_C1C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16u_C1C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16u_C1C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16u_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16u_C1AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16u_C1AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16u_C1AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16u_C1AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16s_C1C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16s_C1C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16s_C1C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16s_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16s_C1C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16s_C1C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16s_C1C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16s_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16s_C1AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16s_C1AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_16s_C1AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_16s_C1AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32s_C1C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32s_C1C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32s_C1C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32s_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32s_C1C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32s_C1C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32s_C1C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32s_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32s_C1AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32s_C1AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32s_C1AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32s_C1AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32f_C1C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32f_C1C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32f_C1C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32f_C1C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32f_C1C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32f_C1C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32f_C1C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32f_C1C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32f_C1AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32f_C1AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oDstSizeROI,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiDup_32f_C1AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oDstSizeROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiDup_32f_C1AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oDstSizeROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_8u_C1R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_8u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_8u_C1R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_8u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_8u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16u_C1R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16u_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16u_C1R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16u_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16s_C1R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16s_C1R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_16s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32s_C1R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32s_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32s_C1R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32s_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32s_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32s_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32f_C1R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32f_C1R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32f_C1R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32f_C1R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiTranspose_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSrcROI: NppiSize,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiTranspose_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSrcROI) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C3IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C3IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C4C3R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C4C3R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C4C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C4IR_Ctx(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C4IR(
    pSrcDst: *mut Npp8u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_C3C4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_8u_C3C4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp8u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_C3C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_8u_AC4R_Ctx(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_8u_AC4R(
    pSrc: *const Npp8u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_8u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C3IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C3IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C4C3R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C4C3R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C4C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C4IR_Ctx(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C4IR(
    pSrcDst: *mut Npp16u,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_C3C4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp16u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_16u_C3C4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp16u,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_C3C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_16u_AC4R_Ctx(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16u_AC4R(
    pSrc: *const Npp16u,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16u_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C3IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C3IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C4C3R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C4C3R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C4C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C4IR_Ctx(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C4IR(
    pSrcDst: *mut Npp16s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_C3C4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_16s_C3C4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp16s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_C3C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_16s_AC4R_Ctx(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_16s_AC4R(
    pSrc: *const Npp16s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_16s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C3IR_Ctx(
    pSrcDst: *mut Npp32s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C3IR(
    pSrcDst: *mut Npp32s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C4C3R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C4C3R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C4C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C4IR_Ctx(
    pSrcDst: *mut Npp32s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C4IR(
    pSrcDst: *mut Npp32s,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_C3C4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp32s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_32s_C3C4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp32s,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_C3C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_32s_AC4R_Ctx(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32s_AC4R(
    pSrc: *const Npp32s,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32s_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C3IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C3IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C3IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C3IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C4C3R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C4C3R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C4C3R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C4C3R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C4IR_Ctx(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C4IR_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrcDst,
                            nSrcDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C4IR(
    pSrcDst: *mut Npp32f,
    nSrcDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C4IR {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nSrcDstStep, oSizeROI, aDstOrder) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_C3C4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C3C4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_32f_C3C4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nValue: Npp32f,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_C3C4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
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
pub unsafe extern "C" fn nppiSwapChannels_32f_AC4R_Ctx(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_AC4R_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nSrcStep,
                            pDst,
                            nDstStep,
                            oSizeROI,
                            aDstOrder,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppiSwapChannels_32f_AC4R(
    pSrc: *const Npp32f,
    nSrcStep: ::std::os::raw::c_int,
    pDst: *mut Npp32f,
    nDstStep: ::std::os::raw::c_int,
    oSizeROI: NppiSize,
    aDstOrder: *const ::std::os::raw::c_int,
) -> NppStatus {
    match nppidei_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppiSwapChannels_32f_AC4R {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nSrcStep, pDst, nDstStep, oSizeROI, aDstOrder)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
