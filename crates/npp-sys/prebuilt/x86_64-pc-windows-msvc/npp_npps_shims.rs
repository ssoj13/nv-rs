#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_8u(nSize: usize) -> *mut Npp8u {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_8u {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_8s(nSize: usize) -> *mut Npp8s {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_8s {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_16u(nSize: usize) -> *mut Npp16u {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_16u {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_16s(nSize: usize) -> *mut Npp16s {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_16s {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_16sc(nSize: usize) -> *mut Npp16sc {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_16sc {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_32u(nSize: usize) -> *mut Npp32u {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_32u {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_32s(nSize: usize) -> *mut Npp32s {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_32s {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_32sc(nSize: usize) -> *mut Npp32sc {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_32sc {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_32f(nSize: usize) -> *mut Npp32f {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_32f {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_32fc(nSize: usize) -> *mut Npp32fc {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_32fc {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_64s(nSize: usize) -> *mut Npp64s {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_64s {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_64sc(nSize: usize) -> *mut Npp64sc {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_64sc {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_64f(nSize: usize) -> *mut Npp64f {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_64f {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMalloc_64fc(nSize: usize) -> *mut Npp64fc {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMalloc_64fc {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nSize) }
                Err(__nv_error) => ::core::ptr::null_mut(),
            }
        }
        Err(__nv_error) => ::core::ptr::null_mut(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe fn nppsFree(
    pValues: *mut ::std::os::raw::c_void,
) -> ::core::result::Result<(), nv_loader::CallError> {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsFree {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pValues) };
                    ::core::result::Result::Ok(())
                }
                Err(__nv_error) => {
                    ::core::result::Result::Err(
                        nv_loader::CallError::symbol(
                            "npp",
                            stringify!(nppsFree),
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
pub unsafe extern "C" fn nppsSet_8u_Ctx(
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_8s_Ctx(
    nValue: Npp8s,
    pDst: *mut Npp8s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_8s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_16u_Ctx(
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_16u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_16s_Ctx(
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_16sc_Ctx(
    nValue: Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_16sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_32u_Ctx(
    nValue: Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_32u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_32s_Ctx(
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_32sc_Ctx(
    nValue: Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_32sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_32f_Ctx(
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_32fc_Ctx(
    nValue: Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_64s_Ctx(
    nValue: Npp64s,
    pDst: *mut Npp64s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_64s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_64sc_Ctx(
    nValue: Npp64sc,
    pDst: *mut Npp64sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_64f_Ctx(
    nValue: Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSet_64fc_Ctx(
    nValue: Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSet_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nValue, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_8u_Ctx(
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_16s_Ctx(
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_16sc_Ctx(
    pDst: *mut Npp16sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_16sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_32s_Ctx(
    pDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_32sc_Ctx(
    pDst: *mut Npp32sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_32sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_32f_Ctx(
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_32fc_Ctx(
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_64s_Ctx(
    pDst: *mut Npp64s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_64s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_64sc_Ctx(
    pDst: *mut Npp64sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_64f_Ctx(
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZero_64fc_Ctx(
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZero_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_8u_Ctx(
    pSrc: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_16s_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_32s_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_64s_Ctx(
    pSrc: *const Npp64s,
    pDst: *mut Npp64s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_64s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_16sc_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_16sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_32sc_Ctx(
    pSrc: *const Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_32sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_64sc_Ctx(
    pSrc: *const Npp64sc,
    pDst: *mut Npp64sc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCopy_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCopy_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_8s16s_Ctx(
    pSrc: *const Npp8s,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_8s16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_8s32f_Ctx(
    pSrc: *const Npp8s,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_8s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_8u32f_Ctx(
    pSrc: *const Npp8u,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_8u32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_16s8s_Sfs_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp8s,
    nLength: Npp32u,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_16s8s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_16s32s_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_16s32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_16s32f_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_16u32f_Ctx(
    pSrc: *const Npp16u,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_16u32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_32s16s_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32s16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_32s32f_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_32s64f_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32s64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_32f64f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32f64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_64s64f_Ctx(
    pSrc: *const Npp64s,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_64s64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_64f32f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_64f32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_16s32f_Sfs_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp32f,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_16s32f_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_16s64f_Sfs_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp64f,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_16s64f_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_32s16s_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32s16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_32s32f_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp32f,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32s32f_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_32s64f_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp64f,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32s64f_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsConvert_32f8s_Sfs_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp8s,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32f8s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_32f8u_Sfs_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp8u,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32f8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_32f16s_Sfs_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp16s,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32f16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_32f16u_Sfs_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp16u,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32f16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_32f32s_Sfs_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32s,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_32f32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_64s32s_Sfs_Ctx(
    pSrc: *const Npp64s,
    pDst: *mut Npp32s,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_64s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_64f16s_Sfs_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp16s,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_64f16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_64f32s_Sfs_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp32s,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_64f32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsConvert_64f64s_Sfs_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64s,
    nLength: usize,
    eRoundMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsConvert_64f64s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsThreshold_16s_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nRelOp, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_16s_I_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nRelOp, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_16sc_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nRelOp, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_16sc_I_Ctx(
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_16sc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nRelOp, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nRelOp, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nRelOp, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nRelOp, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_32fc_I_Ctx(
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nRelOp, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nRelOp, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nRelOp, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nRelOp, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_64fc_I_Ctx(
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nRelOp: NppCmpOp,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nRelOp, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_16s_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_16s_I_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_16sc_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_16sc_I_Ctx(
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_16sc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_32fc_I_Ctx(
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LT_64fc_I_Ctx(
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LT_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_16s_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_16s_I_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_16sc_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_16sc_I_Ctx(
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_16sc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_32fc_I_Ctx(
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GT_64fc_I_Ctx(
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GT_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_16s_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_16s_I_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_16sc_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nValue: Npp16sc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_16sc_I_Ctx(
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nValue: Npp16sc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_16sc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nValue: Npp32fc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_32fc_I_Ctx(
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nValue: Npp32fc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nValue: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nValue: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nValue: Npp64fc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_LTVal_64fc_I_Ctx(
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nValue: Npp64fc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_LTVal_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_16s_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_16s_I_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nLevel: Npp16s,
    nValue: Npp16s,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_16sc_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nValue: Npp16sc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_16sc_I_Ctx(
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nLevel: Npp16s,
    nValue: Npp16sc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_16sc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nLevel: Npp32f,
    nValue: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nValue: Npp32fc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_32fc_I_Ctx(
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nLevel: Npp32f,
    nValue: Npp32fc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nValue: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nLevel: Npp64f,
    nValue: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nValue: Npp64fc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nLevel, nValue, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsThreshold_GTVal_64fc_I_Ctx(
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nLevel: Npp64f,
    nValue: Npp64fc,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsThreshold_GTVal_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nLevel, nValue, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_8u_ISfs_Ctx(
    nValue: Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_8u_Sfs_Ctx(
    pSrc: *const Npp8u,
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAddC_16u_ISfs_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_16u_Sfs_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAddC_16s_ISfs_Ctx(
    nValue: Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAddC_16sc_ISfs_Ctx(
    nValue: Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    nValue: Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAddC_32s_ISfs_Ctx(
    nValue: Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAddC_32sc_ISfs_Ctx(
    nValue: Npp32sc,
    pSrcDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_32sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_32sc_Sfs_Ctx(
    pSrc: *const Npp32sc,
    nValue: Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAddC_32f_I_Ctx(
    nValue: Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_32f_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_32fc_I_Ctx(
    nValue: Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_32fc_Ctx(
    pSrc: *const Npp32fc,
    nValue: Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_64f_I_Ctx(
    nValue: Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_64f_Ctx(
    pSrc: *const Npp64f,
    nValue: Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_64fc_I_Ctx(
    nValue: Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddC_64fc_Ctx(
    pSrc: *const Npp64fc,
    nValue: Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddC_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddProductC_32f_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddProductC_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_8u_ISfs_Ctx(
    nValue: Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_8u_Sfs_Ctx(
    pSrc: *const Npp8u,
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMulC_16u_ISfs_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_16u_Sfs_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMulC_16s_ISfs_Ctx(
    nValue: Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMulC_16sc_ISfs_Ctx(
    nValue: Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    nValue: Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMulC_32s_ISfs_Ctx(
    nValue: Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMulC_32sc_ISfs_Ctx(
    nValue: Npp32sc,
    pSrcDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_32sc_Sfs_Ctx(
    pSrc: *const Npp32sc,
    nValue: Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMulC_32f_I_Ctx(
    nValue: Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_32f_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_Low_32f16s_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_Low_32f16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_32f16s_Sfs_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32f16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMulC_32fc_I_Ctx(
    nValue: Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_32fc_Ctx(
    pSrc: *const Npp32fc,
    nValue: Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_64f_I_Ctx(
    nValue: Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_64f_Ctx(
    pSrc: *const Npp64f,
    nValue: Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_64f64s_ISfs_Ctx(
    nValue: Npp64f,
    pDst: *mut Npp64s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_64f64s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_64fc_I_Ctx(
    nValue: Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMulC_64fc_Ctx(
    pSrc: *const Npp64fc,
    nValue: Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMulC_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_8u_ISfs_Ctx(
    nValue: Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_8u_Sfs_Ctx(
    pSrc: *const Npp8u,
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubC_16u_ISfs_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_16u_Sfs_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubC_16s_ISfs_Ctx(
    nValue: Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubC_16sc_ISfs_Ctx(
    nValue: Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    nValue: Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubC_32s_ISfs_Ctx(
    nValue: Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubC_32sc_ISfs_Ctx(
    nValue: Npp32sc,
    pSrcDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_32sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_32sc_Sfs_Ctx(
    pSrc: *const Npp32sc,
    nValue: Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubC_32f_I_Ctx(
    nValue: Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_32f_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_32fc_I_Ctx(
    nValue: Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_32fc_Ctx(
    pSrc: *const Npp32fc,
    nValue: Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_64f_I_Ctx(
    nValue: Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_64f_Ctx(
    pSrc: *const Npp64f,
    nValue: Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_64fc_I_Ctx(
    nValue: Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubC_64fc_Ctx(
    pSrc: *const Npp64fc,
    nValue: Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubC_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_8u_ISfs_Ctx(
    nValue: Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_8u_Sfs_Ctx(
    pSrc: *const Npp8u,
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubCRev_16u_ISfs_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_16u_Sfs_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubCRev_16s_ISfs_Ctx(
    nValue: Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubCRev_16sc_ISfs_Ctx(
    nValue: Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    nValue: Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubCRev_32s_ISfs_Ctx(
    nValue: Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    nValue: Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubCRev_32sc_ISfs_Ctx(
    nValue: Npp32sc,
    pSrcDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_32sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_32sc_Sfs_Ctx(
    pSrc: *const Npp32sc,
    nValue: Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSubCRev_32f_I_Ctx(
    nValue: Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_32f_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_32fc_I_Ctx(
    nValue: Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_32fc_Ctx(
    pSrc: *const Npp32fc,
    nValue: Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_64f_I_Ctx(
    nValue: Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_64f_Ctx(
    pSrc: *const Npp64f,
    nValue: Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_64fc_I_Ctx(
    nValue: Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSubCRev_64fc_Ctx(
    pSrc: *const Npp64fc,
    nValue: Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSubCRev_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_8u_ISfs_Ctx(
    nValue: Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_8u_Sfs_Ctx(
    pSrc: *const Npp8u,
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDivC_16u_ISfs_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_16u_Sfs_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDivC_16s_ISfs_Ctx(
    nValue: Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nValue: Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDivC_16sc_ISfs_Ctx(
    nValue: Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(nValue, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    nValue: Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nValue,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDivC_32f_I_Ctx(
    nValue: Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_32f_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_32fc_I_Ctx(
    nValue: Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_32fc_Ctx(
    pSrc: *const Npp32fc,
    nValue: Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_64f_I_Ctx(
    nValue: Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_64f_Ctx(
    pSrc: *const Npp64f,
    nValue: Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_64fc_I_Ctx(
    nValue: Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivC_64fc_Ctx(
    pSrc: *const Npp64fc,
    nValue: Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivC_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivCRev_16u_I_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivCRev_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivCRev_16u_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivCRev_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivCRev_32f_I_Ctx(
    nValue: Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivCRev_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDivCRev_32f_Ctx(
    pSrc: *const Npp32f,
    nValue: Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDivCRev_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_16s_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_16u_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_32u_Ctx(
    pSrc1: *const Npp32u,
    pSrc2: *const Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_8u16u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_8u16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_16s32f_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_8u_Sfs_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAdd_16u_Sfs_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAdd_16s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAdd_32s_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAdd_64s_Sfs_Ctx(
    pSrc1: *const Npp64s,
    pSrc2: *const Npp64s,
    pDst: *mut Npp64s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_64s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAdd_16sc_Sfs_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAdd_32sc_Sfs_Ctx(
    pSrc1: *const Npp32sc,
    pSrc2: *const Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAdd_16s_I_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_32f_I_Ctx(
    pSrc: *const Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_64f_I_Ctx(
    pSrc: *const Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_32fc_I_Ctx(
    pSrc: *const Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_64fc_I_Ctx(
    pSrc: *const Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_16s32s_I_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16s32s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_8u_ISfs_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_16u_ISfs_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_16s_ISfs_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_32s_ISfs_Ctx(
    pSrc: *const Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_16sc_ISfs_Ctx(
    pSrc: *const Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAdd_32sc_ISfs_Ctx(
    pSrc: *const Npp32sc,
    pSrcDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAdd_32sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddProduct_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddProduct_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddProduct_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddProduct_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddProduct_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddProduct_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddProduct_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddProduct_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAddProduct_16s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddProduct_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAddProduct_32s_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddProduct_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsAddProduct_16s32s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAddProduct_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_16s_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_8u16u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_8u16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_16s32f_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32f32fc_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32f32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_8u_Sfs_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_16u_Sfs_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_16s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_32s_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_16sc_Sfs_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_32sc_Sfs_Ctx(
    pSrc1: *const Npp32sc,
    pSrc2: *const Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_16u16s_Sfs_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16u16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_16s32s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_32s32sc_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32s32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_Low_32s_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_Low_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsMul_16s_I_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32f_I_Ctx(
    pSrc: *const Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_64f_I_Ctx(
    pSrc: *const Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32fc_I_Ctx(
    pSrc: *const Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_64fc_I_Ctx(
    pSrc: *const Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32f32fc_I_Ctx(
    pSrc: *const Npp32f,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32f32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_8u_ISfs_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_16u_ISfs_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_16s_ISfs_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32s_ISfs_Ctx(
    pSrc: *const Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_16sc_ISfs_Ctx(
    pSrc: *const Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32sc_ISfs_Ctx(
    pSrc: *const Npp32sc,
    pSrcDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMul_32s32sc_ISfs_Ctx(
    pSrc: *const Npp32s,
    pSrcDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMul_32s32sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_16s_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_16s32f_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_8u_Sfs_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSub_16u_Sfs_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSub_16s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSub_32s_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSub_16sc_Sfs_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSub_32sc_Sfs_Ctx(
    pSrc1: *const Npp32sc,
    pSrc2: *const Npp32sc,
    pDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsSub_16s_I_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_32f_I_Ctx(
    pSrc: *const Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_64f_I_Ctx(
    pSrc: *const Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_32fc_I_Ctx(
    pSrc: *const Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_64fc_I_Ctx(
    pSrc: *const Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_8u_ISfs_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_16u_ISfs_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_16s_ISfs_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_32s_ISfs_Ctx(
    pSrc: *const Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_16sc_ISfs_Ctx(
    pSrc: *const Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSub_32sc_ISfs_Ctx(
    pSrc: *const Npp32sc,
    pSrcDst: *mut Npp32sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSub_32sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_8u_Sfs_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDiv_16u_Sfs_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDiv_16s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDiv_32s_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDiv_16sc_Sfs_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDiv_32s16s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp32s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_32s16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
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
pub unsafe extern "C" fn nppsDiv_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_8u_ISfs_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_16u_ISfs_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_16s_ISfs_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_16sc_ISfs_Ctx(
    pSrc: *const Npp16sc,
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_32s_ISfs_Ctx(
    pSrc: *const Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pSrcDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_32f_I_Ctx(
    pSrc: *const Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_64f_I_Ctx(
    pSrc: *const Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_32fc_I_Ctx(
    pSrc: *const Npp32fc,
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_32fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_64fc_I_Ctx(
    pSrc: *const Npp64fc,
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_64fc_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDiv_Round_8u_Sfs_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nRndMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_Round_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
                            nRndMode,
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
pub unsafe extern "C" fn nppsDiv_Round_16u_Sfs_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nRndMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_Round_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
                            nRndMode,
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
pub unsafe extern "C" fn nppsDiv_Round_16s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nRndMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_Round_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            pDst,
                            nLength,
                            nRndMode,
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
pub unsafe extern "C" fn nppsDiv_Round_8u_ISfs_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nRndMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_Round_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pSrcDst,
                            nLength,
                            nRndMode,
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
pub unsafe extern "C" fn nppsDiv_Round_16u_ISfs_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nRndMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_Round_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pSrcDst,
                            nLength,
                            nRndMode,
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
pub unsafe extern "C" fn nppsDiv_Round_16s_ISfs_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nRndMode: NppRoundMode,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDiv_Round_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pSrcDst,
                            nLength,
                            nRndMode,
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
pub unsafe extern "C" fn nppsAbs_16s_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAbs_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAbs_32s_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAbs_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAbs_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAbs_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAbs_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAbs_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAbs_16s_I_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAbs_16s_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAbs_32s_I_Ctx(
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAbs_32s_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAbs_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAbs_32f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAbs_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAbs_64f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_32f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_64f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_32fc_I_Ctx(
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_32fc_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_64fc_I_Ctx(
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_64fc_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_8u_Sfs_Ctx(
    pSrc: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_16u_Sfs_Ctx(
    pSrc: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_8u_ISfs_Ctx(
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_16u_ISfs_Ctx(
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_16s_ISfs_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqr_16sc_ISfs_Ctx(
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqr_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_32f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_64f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_32fc_I_Ctx(
    pSrcDst: *mut Npp32fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_32fc_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_64fc_I_Ctx(
    pSrcDst: *mut Npp64fc,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_64fc_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_8u_Sfs_Ctx(
    pSrc: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_8u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_16u_Sfs_Ctx(
    pSrc: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_16u_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_64s_Sfs_Ctx(
    pSrc: *const Npp64s,
    pDst: *mut Npp64s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_64s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_32s16s_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_32s16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_64s16s_Sfs_Ctx(
    pSrc: *const Npp64s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_64s16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_8u_ISfs_Ctx(
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_8u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_16u_ISfs_Ctx(
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_16u_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_16s_ISfs_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_16sc_ISfs_Ctx(
    pSrcDst: *mut Npp16sc,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_16sc_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSqrt_64s_ISfs_Ctx(
    pSrcDst: *mut Npp64s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSqrt_64s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCubrt_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCubrt_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCubrt_32s16s_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCubrt_32s16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_32f64f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_32f64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_32f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_64f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_64s_Sfs_Ctx(
    pSrc: *const Npp64s,
    pDst: *mut Npp64s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_64s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_16s_ISfs_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_32s_ISfs_Ctx(
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsExp_64s_ISfs_Ctx(
    pSrcDst: *mut Npp64s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsExp_64s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_64f32f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_64f32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_32f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_64f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_32s16s_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_32s16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_16s_ISfs_Ctx(
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_16s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLn_32s_ISfs_Ctx(
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLn_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn npps10Log10_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.npps10Log10_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, nScaleFactor, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn npps10Log10_32s_ISfs_Ctx(
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.npps10Log10_32s_ISfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nScaleFactor, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumLnGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumLnGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumLn_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pDst: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumLn_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pDst, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumLnGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumLnGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumLn_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumLn_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pDst, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumLnGetBufferSize_32f64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumLnGetBufferSize_32f64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumLn_32f64f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumLn_32f64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pDst, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumLnGetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumLnGetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumLn_16s32f_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pDst: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumLn_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pDst, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsArctan_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsArctan_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsArctan_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsArctan_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsArctan_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsArctan_32f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsArctan_64f_I_Ctx(
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsArctan_64f_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormalize_32f_Ctx(
    pSrc: *const Npp32f,
    pDst: *mut Npp32f,
    nLength: usize,
    vSub: Npp32f,
    vDiv: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormalize_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, vSub, vDiv, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormalize_32fc_Ctx(
    pSrc: *const Npp32fc,
    pDst: *mut Npp32fc,
    nLength: usize,
    vSub: Npp32fc,
    vDiv: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormalize_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, vSub, vDiv, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormalize_64f_Ctx(
    pSrc: *const Npp64f,
    pDst: *mut Npp64f,
    nLength: usize,
    vSub: Npp64f,
    vDiv: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormalize_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, vSub, vDiv, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormalize_64fc_Ctx(
    pSrc: *const Npp64fc,
    pDst: *mut Npp64fc,
    nLength: usize,
    vSub: Npp64fc,
    vDiv: Npp64f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormalize_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pDst, nLength, vSub, vDiv, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormalize_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    pDst: *mut Npp16s,
    nLength: usize,
    vSub: Npp16s,
    vDiv: ::std::os::raw::c_int,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormalize_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
                            vSub,
                            vDiv,
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
pub unsafe extern "C" fn nppsNormalize_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    pDst: *mut Npp16sc,
    nLength: usize,
    vSub: Npp16sc,
    vDiv: ::std::os::raw::c_int,
    nScaleFactor: ::std::os::raw::c_int,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormalize_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            pDst,
                            nLength,
                            vSub,
                            vDiv,
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
pub unsafe extern "C" fn nppsCauchy_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nParam: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCauchy_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nParam, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCauchyD_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nParam: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCauchyD_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrcDst, nLength, nParam, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCauchyDD2_32f_I_Ctx(
    pSrcDst: *mut Npp32f,
    pD2FVal: *mut Npp32f,
    nLength: usize,
    nParam: Npp32f,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCauchyDD2_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrcDst, pD2FVal, nLength, nParam, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAndC_8u_Ctx(
    pSrc: *const Npp8u,
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAndC_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAndC_16u_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAndC_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAndC_32u_Ctx(
    pSrc: *const Npp32u,
    nValue: Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAndC_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAndC_8u_I_Ctx(
    nValue: Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAndC_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAndC_16u_I_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAndC_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAndC_32u_I_Ctx(
    nValue: Npp32u,
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAndC_32u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAnd_8u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAnd_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAnd_16u_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAnd_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAnd_32u_Ctx(
    pSrc1: *const Npp32u,
    pSrc2: *const Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAnd_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAnd_8u_I_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAnd_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAnd_16u_I_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAnd_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAnd_32u_I_Ctx(
    pSrc: *const Npp32u,
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAnd_32u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOrC_8u_Ctx(
    pSrc: *const Npp8u,
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOrC_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOrC_16u_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOrC_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOrC_32u_Ctx(
    pSrc: *const Npp32u,
    nValue: Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOrC_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOrC_8u_I_Ctx(
    nValue: Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOrC_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOrC_16u_I_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOrC_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOrC_32u_I_Ctx(
    nValue: Npp32u,
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOrC_32u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOr_8u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOr_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOr_16u_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOr_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOr_32u_Ctx(
    pSrc1: *const Npp32u,
    pSrc2: *const Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOr_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOr_8u_I_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOr_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOr_16u_I_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOr_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsOr_32u_I_Ctx(
    pSrc: *const Npp32u,
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsOr_32u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXorC_8u_Ctx(
    pSrc: *const Npp8u,
    nValue: Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXorC_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXorC_16u_Ctx(
    pSrc: *const Npp16u,
    nValue: Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXorC_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXorC_32u_Ctx(
    pSrc: *const Npp32u,
    nValue: Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXorC_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXorC_8u_I_Ctx(
    nValue: Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXorC_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXorC_16u_I_Ctx(
    nValue: Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXorC_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXorC_32u_I_Ctx(
    nValue: Npp32u,
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXorC_32u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXor_8u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXor_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXor_16u_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXor_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXor_32u_Ctx(
    pSrc1: *const Npp32u,
    pSrc2: *const Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXor_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc1, pSrc2, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXor_8u_I_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXor_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXor_16u_I_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXor_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsXor_32u_I_Ctx(
    pSrc: *const Npp32u,
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsXor_32u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNot_8u_Ctx(
    pSrc: *const Npp8u,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNot_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNot_16u_Ctx(
    pSrc: *const Npp16u,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNot_16u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNot_32u_Ctx(
    pSrc: *const Npp32u,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNot_32u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrc, pDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNot_8u_I_Ctx(
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNot_8u_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNot_16u_I_Ctx(
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNot_16u_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNot_32u_I_Ctx(
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNot_32u_I_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pSrcDst, nLength, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_8u_Ctx(
    pSrc: *const Npp8u,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_16u_Ctx(
    pSrc: *const Npp16u,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_16s_Ctx(
    pSrc: *const Npp16s,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_32u_Ctx(
    pSrc: *const Npp32u,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_32s_Ctx(
    pSrc: *const Npp32s,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_8u_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_16u_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_16s_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_32u_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_32u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsLShiftC_32s_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsLShiftC_32s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_8u_Ctx(
    pSrc: *const Npp8u,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_16u_Ctx(
    pSrc: *const Npp16u,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_16s_Ctx(
    pSrc: *const Npp16s,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_32u_Ctx(
    pSrc: *const Npp32u,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_32s_Ctx(
    pSrc: *const Npp32s,
    nValue: ::std::os::raw::c_int,
    pDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, nValue, pDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_8u_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_16u_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_16s_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_32u_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp32u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_32u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsRShiftC_32s_I_Ctx(
    nValue: ::std::os::raw::c_int,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsRShiftC_32s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(nValue, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinEvery_8u_I_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinEvery_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinEvery_16u_I_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinEvery_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinEvery_16s_I_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinEvery_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinEvery_32s_I_Ctx(
    pSrc: *const Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinEvery_32s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinEvery_32f_I_Ctx(
    pSrc: *const Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinEvery_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinEvery_64f_I_Ctx(
    pSrc: *const Npp64f,
    pSrcDst: *mut Npp64f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinEvery_64f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxEvery_8u_I_Ctx(
    pSrc: *const Npp8u,
    pSrcDst: *mut Npp8u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxEvery_8u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxEvery_16u_I_Ctx(
    pSrc: *const Npp16u,
    pSrcDst: *mut Npp16u,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxEvery_16u_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxEvery_16s_I_Ctx(
    pSrc: *const Npp16s,
    pSrcDst: *mut Npp16s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxEvery_16s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxEvery_32s_I_Ctx(
    pSrc: *const Npp32s,
    pSrcDst: *mut Npp32s,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxEvery_32s_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxEvery_32f_I_Ctx(
    pSrc: *const Npp32f,
    pSrcDst: *mut Npp32f,
    nLength: usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxEvery_32f_I_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(pSrc, pSrcDst, nLength, nppStreamCtx) }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_16s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_16s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_16sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_16sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_16sc32sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_16sc32sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSumGetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSumGetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pSum: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pSum, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_32fc_Ctx(
    pSrc: *const Npp32fc,
    nLength: usize,
    pSum: *mut Npp32fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pSum, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pSum: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pSum, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_64fc_Ctx(
    pSrc: *const Npp64fc,
    nLength: usize,
    pSum: *mut Npp64fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pSum, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pSum: *mut Npp16s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pSum,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pSum: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pSum,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    nLength: usize,
    pSum: *mut Npp16sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pSum,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_16sc32sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    nLength: usize,
    pSum: *mut Npp32sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_16sc32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pSum,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsSum_16s32s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pSum: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsSum_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pSum,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMax_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMax: *mut Npp16s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMax_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMax, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMax_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMax: *mut Npp32s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMax_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMax, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMax_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pMax: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMax_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMax, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMax_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pMax: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMax_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMax, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxIndxGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxIndxGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxIndxGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxIndxGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxIndxGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxIndxGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxIndxGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxIndxGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxIndx_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMax: *mut Npp16s,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxIndx_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMax,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxIndx_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMax: *mut Npp32s,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxIndx_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMax,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxIndx_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pMax: *mut Npp32f,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxIndx_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMax,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxIndx_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pMax: *mut Npp64f,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxIndx_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMax,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxAbsGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxAbsGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxAbsGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxAbsGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxAbs_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMaxAbs: *mut Npp16s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxAbs_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMaxAbs, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxAbs_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMaxAbs: *mut Npp32s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxAbs_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMaxAbs, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxAbsIndxGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxAbsIndxGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxAbsIndxGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxAbsIndxGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxAbsIndx_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMaxAbs: *mut Npp16s,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxAbsIndx_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMaxAbs,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaxAbsIndx_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMaxAbs: *mut Npp32s,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaxAbsIndx_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMaxAbs,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMin_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMin: *mut Npp16s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMin_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMin, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMin_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMin: *mut Npp32s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMin_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMin, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMin_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pMin: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMin_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMin, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMin_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pMin: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMin_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMin, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinIndxGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinIndxGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinIndxGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinIndxGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinIndxGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinIndxGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinIndxGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinIndxGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinIndx_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMin: *mut Npp16s,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinIndx_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinIndx_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMin: *mut Npp32s,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinIndx_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinIndx_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pMin: *mut Npp32f,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinIndx_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinIndx_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pMin: *mut Npp64f,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinIndx_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinAbsGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinAbsGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinAbsGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinAbsGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinAbs_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMinAbs: *mut Npp16s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinAbs_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMinAbs, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinAbs_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMinAbs: *mut Npp32s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinAbs_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMinAbs, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinAbsIndxGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinAbsIndxGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinAbsIndxGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinAbsIndxGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinAbsIndx_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMinAbs: *mut Npp16s,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinAbsIndx_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMinAbs,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinAbsIndx_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMinAbs: *mut Npp32s,
    pIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinAbsIndx_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMinAbs,
                            pIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanGetBufferSize_32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanGetBufferSize_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanGetBufferSize_64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanGetBufferSize_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanGetBufferSize_16s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanGetBufferSize_16s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanGetBufferSize_32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanGetBufferSize_32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanGetBufferSize_16sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanGetBufferSize_16sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMean_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pMean: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMean_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMean, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMean_32fc_Ctx(
    pSrc: *const Npp32fc,
    nLength: usize,
    pMean: *mut Npp32fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMean_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMean, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMean_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pMean: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMean_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMean, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMean_64fc_Ctx(
    pSrc: *const Npp64fc,
    nLength: usize,
    pMean: *mut Npp64fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMean_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pMean, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMean_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMean: *mut Npp16s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMean_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMean,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMean_32s_Sfs_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMean: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMean_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMean,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMean_16sc_Sfs_Ctx(
    pSrc: *const Npp16sc,
    nLength: usize,
    pMean: *mut Npp16sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMean_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMean,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsStdDevGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsStdDevGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsStdDevGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsStdDevGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsStdDevGetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsStdDevGetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsStdDevGetBufferSize_16s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsStdDevGetBufferSize_16s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsStdDev_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pStdDev: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsStdDev_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pStdDev, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsStdDev_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pStdDev: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsStdDev_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pStdDev, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsStdDev_16s32s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pStdDev: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsStdDev_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pStdDev,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsStdDev_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pStdDev: *mut Npp16s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsStdDev_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pStdDev,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanStdDevGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanStdDevGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanStdDevGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanStdDevGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanStdDevGetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanStdDevGetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanStdDevGetBufferSize_16s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanStdDevGetBufferSize_16s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanStdDev_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pMean: *mut Npp32f,
    pStdDev: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanStdDev_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMean,
                            pStdDev,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanStdDev_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pMean: *mut Npp64f,
    pStdDev: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanStdDev_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMean,
                            pStdDev,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanStdDev_16s32s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMean: *mut Npp32s,
    pStdDev: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanStdDev_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMean,
                            pStdDev,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMeanStdDev_16s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMean: *mut Npp16s,
    pStdDev: *mut Npp16s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMeanStdDev_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMean,
                            pStdDev,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxGetBufferSize_8u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxGetBufferSize_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxGetBufferSize_16u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxGetBufferSize_16u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxGetBufferSize_32u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxGetBufferSize_32u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMax_8u_Ctx(
    pSrc: *const Npp8u,
    nLength: usize,
    pMin: *mut Npp8u,
    pMax: *mut Npp8u,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMax_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMax,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMax_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMin: *mut Npp16s,
    pMax: *mut Npp16s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMax_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMax,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMax_16u_Ctx(
    pSrc: *const Npp16u,
    nLength: usize,
    pMin: *mut Npp16u,
    pMax: *mut Npp16u,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMax_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMax,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMax_32u_Ctx(
    pSrc: *const Npp32u,
    nLength: usize,
    pMin: *mut Npp32u,
    pMax: *mut Npp32u,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMax_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMax,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMax_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMin: *mut Npp32s,
    pMax: *mut Npp32s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMax_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMax,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMax_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pMin: *mut Npp32f,
    pMax: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMax_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMax,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMax_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pMin: *mut Npp64f,
    pMax: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMax_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMax,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndxGetBufferSize_8u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndxGetBufferSize_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndxGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndxGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndxGetBufferSize_16u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndxGetBufferSize_16u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndxGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndxGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndxGetBufferSize_32u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndxGetBufferSize_32u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndxGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndxGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndxGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndxGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndx_8u_Ctx(
    pSrc: *const Npp8u,
    nLength: usize,
    pMin: *mut Npp8u,
    pMinIndx: *mut ::std::os::raw::c_int,
    pMax: *mut Npp8u,
    pMaxIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndx_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMinIndx,
                            pMax,
                            pMaxIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndx_16s_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pMin: *mut Npp16s,
    pMinIndx: *mut ::std::os::raw::c_int,
    pMax: *mut Npp16s,
    pMaxIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndx_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMinIndx,
                            pMax,
                            pMaxIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndx_16u_Ctx(
    pSrc: *const Npp16u,
    nLength: usize,
    pMin: *mut Npp16u,
    pMinIndx: *mut ::std::os::raw::c_int,
    pMax: *mut Npp16u,
    pMaxIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndx_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMinIndx,
                            pMax,
                            pMaxIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndx_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pMin: *mut Npp32s,
    pMinIndx: *mut ::std::os::raw::c_int,
    pMax: *mut Npp32s,
    pMaxIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndx_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMinIndx,
                            pMax,
                            pMaxIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndx_32u_Ctx(
    pSrc: *const Npp32u,
    nLength: usize,
    pMin: *mut Npp32u,
    pMinIndx: *mut ::std::os::raw::c_int,
    pMax: *mut Npp32u,
    pMaxIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndx_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMinIndx,
                            pMax,
                            pMaxIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndx_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pMin: *mut Npp32f,
    pMinIndx: *mut ::std::os::raw::c_int,
    pMax: *mut Npp32f,
    pMaxIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndx_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMinIndx,
                            pMax,
                            pMaxIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMinMaxIndx_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pMin: *mut Npp64f,
    pMinIndx: *mut ::std::os::raw::c_int,
    pMax: *mut Npp64f,
    pMaxIndx: *mut ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMinMaxIndx_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pMin,
                            pMinIndx,
                            pMax,
                            pMaxIndx,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormInfGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormInfGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_Inf_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_Inf_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormInfGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormInfGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_Inf_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_Inf_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormInfGetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormInfGetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_Inf_16s32f_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_Inf_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormInfGetBufferSize_32fc32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormInfGetBufferSize_32fc32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_Inf_32fc32f_Ctx(
    pSrc: *const Npp32fc,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_Inf_32fc32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormInfGetBufferSize_64fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormInfGetBufferSize_64fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_Inf_64fc64f_Ctx(
    pSrc: *const Npp64fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_Inf_64fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormInfGetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormInfGetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_Inf_16s32s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_Inf_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL1GetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL1GetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L1_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L1_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL1GetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL1GetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L1_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L1_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL1GetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL1GetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L1_16s32f_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L1_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL1GetBufferSize_32fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL1GetBufferSize_32fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L1_32fc64f_Ctx(
    pSrc: *const Npp32fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L1_32fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL1GetBufferSize_64fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL1GetBufferSize_64fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L1_64fc64f_Ctx(
    pSrc: *const Npp64fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L1_64fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL1GetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL1GetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L1_16s32s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L1_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL1GetBufferSize_16s64s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL1GetBufferSize_16s64s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L1_16s64s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp64s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L1_16s64s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL2GetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL2GetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L2_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L2_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL2GetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL2GetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L2_64f_Ctx(
    pSrc: *const Npp64f,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L2_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL2GetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL2GetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L2_16s32f_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L2_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL2GetBufferSize_32fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL2GetBufferSize_32fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L2_32fc64f_Ctx(
    pSrc: *const Npp32fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L2_32fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL2GetBufferSize_64fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL2GetBufferSize_64fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L2_64fc64f_Ctx(
    pSrc: *const Npp64fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L2_64fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, nLength, pNorm, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL2GetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL2GetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L2_16s32s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L2_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormL2SqrGetBufferSize_16s64s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormL2SqrGetBufferSize_16s64s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNorm_L2Sqr_16s64s_Sfs_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp64s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNorm_L2Sqr_16s64s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffInfGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffInfGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_Inf_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_Inf_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffInfGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffInfGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_Inf_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_Inf_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffInfGetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffInfGetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_Inf_16s32f_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_Inf_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffInfGetBufferSize_32fc32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffInfGetBufferSize_32fc32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_Inf_32fc32f_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_Inf_32fc32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffInfGetBufferSize_64fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffInfGetBufferSize_64fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_Inf_64fc64f_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_Inf_64fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffInfGetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffInfGetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_Inf_16s32s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_Inf_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL1GetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL1GetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L1_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L1_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL1GetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL1GetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L1_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L1_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL1GetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL1GetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L1_16s32f_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L1_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL1GetBufferSize_32fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL1GetBufferSize_32fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L1_32fc64f_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L1_32fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL1GetBufferSize_64fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL1GetBufferSize_64fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L1_64fc64f_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L1_64fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL1GetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL1GetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L1_16s32s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L1_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL1GetBufferSize_16s64s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL1GetBufferSize_16s64s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L1_16s64s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp64s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L1_16s64s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL2GetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL2GetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L2_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L2_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL2GetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL2GetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L2_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L2_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL2GetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL2GetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L2_16s32f_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L2_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL2GetBufferSize_32fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL2GetBufferSize_32fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L2_32fc64f_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L2_32fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL2GetBufferSize_64fc64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL2GetBufferSize_64fc64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L2_64fc64f_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pNorm: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L2_64fc64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL2GetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL2GetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L2_16s32s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L2_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiffL2SqrGetBufferSize_16s64s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiffL2SqrGetBufferSize_16s64s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsNormDiff_L2Sqr_16s64s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pNorm: *mut Npp64s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsNormDiff_L2Sqr_16s64s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pNorm,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pDp: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pDp: *mut Npp32fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32f32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32f32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32f32fc_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pDp: *mut Npp32fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32f32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32f64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32f64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32f64f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pDp: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32f64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32fc64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32fc64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32fc64fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pDp: *mut Npp64fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32fc64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32f32fc64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32f32fc64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32f32fc64fc_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pDp: *mut Npp64fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32f32fc64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    nLength: usize,
    pDp: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pDp: *mut Npp64fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_64f64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_64f64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_64f64fc_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pDp: *mut Npp64fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_64f64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s64s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s64s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s64s_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pDp: *mut Npp64s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s64s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16sc64sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16sc64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16sc64sc_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDp: *mut Npp64sc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16sc64sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s16sc64sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s16sc64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s16sc64sc_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDp: *mut Npp64sc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s16sc64sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s32f_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pDp: *mut Npp32f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16sc32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16sc32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16sc32fc_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDp: *mut Npp32fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16sc32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s16sc32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s16sc32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s16sc32fc_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDp: *mut Npp32fc,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s16sc32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pDp: *mut Npp16s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16sc_Sfs_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDp: *mut Npp16sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32s_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    nLength: usize,
    pDp: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32sc_Sfs_Ctx(
    pSrc1: *const Npp32sc,
    pSrc2: *const Npp32sc,
    nLength: usize,
    pDp: *mut Npp32sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s32s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pDp: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s16sc32sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s16sc32sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s16sc32sc_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDp: *mut Npp32sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s16sc32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s32s32s_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s32s32s_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s32s32s_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp32s,
    nLength: usize,
    pDp: *mut Npp32s,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s32s32s_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16s16sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16s16sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16s16sc_Sfs_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDp: *mut Npp16sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16s16sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_16sc32sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_16sc32sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_16sc32sc_Sfs_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDp: *mut Npp32sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_16sc32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProdGetBufferSize_32s32sc_Sfs_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProdGetBufferSize_32s32sc_Sfs_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsDotProd_32s32sc_Sfs_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32sc,
    nLength: usize,
    pDp: *mut Npp32sc,
    nScaleFactor: ::std::os::raw::c_int,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsDotProd_32s32sc_Sfs_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDp,
                            nScaleFactor,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCountInRangeGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCountInRangeGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsCountInRange_32s_Ctx(
    pSrc: *const Npp32s,
    nLength: usize,
    pCounts: *mut ::std::os::raw::c_int,
    nLowerBound: Npp32s,
    nUpperBound: Npp32s,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsCountInRange_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pCounts,
                            nLowerBound,
                            nUpperBound,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZeroCrossingGetBufferSize_16s32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZeroCrossingGetBufferSize_16s32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZeroCrossing_16s32f_Ctx(
    pSrc: *const Npp16s,
    nLength: usize,
    pValZC: *mut Npp32f,
    tZCType: NppsZCType,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZeroCrossing_16s32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pValZC,
                            tZCType,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZeroCrossingGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZeroCrossingGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsZeroCrossing_32f_Ctx(
    pSrc: *const Npp32f,
    nLength: usize,
    pValZC: *mut Npp32f,
    tZCType: NppsZCType,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsZeroCrossing_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc,
                            nLength,
                            pValZC,
                            tZCType,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_8u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_8s_Ctx(
    pSrc1: *const Npp8s,
    pSrc2: *const Npp8s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_8s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_16u_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_16s_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_16sc_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_32u_Ctx(
    pSrc1: *const Npp32u,
    pSrc2: *const Npp32u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_32s_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_32sc_Ctx(
    pSrc1: *const Npp32sc,
    pSrc2: *const Npp32sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_32sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_64s_Ctx(
    pSrc1: *const Npp64s,
    pSrc2: *const Npp64s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_64s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_64sc_Ctx(
    pSrc1: *const Npp64sc,
    pSrc2: *const Npp64sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_64sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumError_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumError_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_8u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_8s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_8s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_16u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_16u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_16sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_16sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_32u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_32u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_32sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_32sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_64s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_64s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_64sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumErrorGetBufferSize_64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumErrorGetBufferSize_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_8u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_8s_Ctx(
    pSrc1: *const Npp8s,
    pSrc2: *const Npp8s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_8s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_16u_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_16s_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_16sc_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_32u_Ctx(
    pSrc1: *const Npp32u,
    pSrc2: *const Npp32u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_32s_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_32sc_Ctx(
    pSrc1: *const Npp32sc,
    pSrc2: *const Npp32sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_32sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_64s_Ctx(
    pSrc1: *const Npp64s,
    pSrc2: *const Npp64s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_64s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_64sc_Ctx(
    pSrc1: *const Npp64sc,
    pSrc2: *const Npp64sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_64sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageError_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageError_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_8u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_8s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_8s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_16u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_16u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_16sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_16sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_32u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_32u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_32sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_32sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_64s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_64s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_64sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageErrorGetBufferSize_64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageErrorGetBufferSize_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_8u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_8s_Ctx(
    pSrc1: *const Npp8s,
    pSrc2: *const Npp8s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_8s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_16u_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_16s_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_16sc_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_32u_Ctx(
    pSrc1: *const Npp32u,
    pSrc2: *const Npp32u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_32s_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_32sc_Ctx(
    pSrc1: *const Npp32sc,
    pSrc2: *const Npp32sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_32sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_64s_Ctx(
    pSrc1: *const Npp64s,
    pSrc2: *const Npp64s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_64s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_64sc_Ctx(
    pSrc1: *const Npp64sc,
    pSrc2: *const Npp64sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_64sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeError_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeError_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_8u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_8s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_8s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_16u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_16u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_16sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_16sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_32u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_32u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_32sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_32sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_64s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_64s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_64sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsMaximumRelativeErrorGetBufferSize_64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsMaximumRelativeErrorGetBufferSize_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_8u_Ctx(
    pSrc1: *const Npp8u,
    pSrc2: *const Npp8u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_8u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_8s_Ctx(
    pSrc1: *const Npp8s,
    pSrc2: *const Npp8s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_8s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_16u_Ctx(
    pSrc1: *const Npp16u,
    pSrc2: *const Npp16u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_16u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_16s_Ctx(
    pSrc1: *const Npp16s,
    pSrc2: *const Npp16s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_16s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_16sc_Ctx(
    pSrc1: *const Npp16sc,
    pSrc2: *const Npp16sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_16sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_32u_Ctx(
    pSrc1: *const Npp32u,
    pSrc2: *const Npp32u,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_32u_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_32s_Ctx(
    pSrc1: *const Npp32s,
    pSrc2: *const Npp32s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_32sc_Ctx(
    pSrc1: *const Npp32sc,
    pSrc2: *const Npp32sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_32sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_64s_Ctx(
    pSrc1: *const Npp64s,
    pSrc2: *const Npp64s,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_64s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_64sc_Ctx(
    pSrc1: *const Npp64sc,
    pSrc2: *const Npp64sc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_64sc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_32f_Ctx(
    pSrc1: *const Npp32f,
    pSrc2: *const Npp32f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_32f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_32fc_Ctx(
    pSrc1: *const Npp32fc,
    pSrc2: *const Npp32fc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_32fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_64f_Ctx(
    pSrc1: *const Npp64f,
    pSrc2: *const Npp64f,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_64f_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeError_64fc_Ctx(
    pSrc1: *const Npp64fc,
    pSrc2: *const Npp64fc,
    nLength: usize,
    pDst: *mut Npp64f,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeError_64fc_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            pSrc1,
                            pSrc2,
                            nLength,
                            pDst,
                            pDeviceBuffer,
                            nppStreamCtx,
                        )
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_8u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_8u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_8s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_8s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_16u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_16u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_16s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_16s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_16sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_16sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_32u_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_32u_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_32s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_32s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_32sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_32sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_64s_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_64s_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_64sc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_64sc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_32f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_32f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_32fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_32fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_64f_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_64f_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsAverageRelativeErrorGetBufferSize_64fc_Ctx(
    nLength: usize,
    hpBufferSize: *mut usize,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsAverageRelativeErrorGetBufferSize_64fc_Ctx {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize, nppStreamCtx) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsIntegralGetBufferSize_32s(
    nLength: usize,
    hpBufferSize: *mut usize,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsIntegralGetBufferSize_32s {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nLength, hpBufferSize) }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppsIntegral_32s_Ctx(
    pSrc: *const Npp32s,
    pDst: *mut Npp32s,
    nLength: usize,
    pDeviceBuffer: *mut Npp8u,
    nppStreamCtx: NppStreamContext,
) -> NppStatus {
    match npps_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppsIntegral_32s_Ctx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(pSrc, pDst, nLength, pDeviceBuffer, nppStreamCtx)
                    }
                }
                Err(__nv_error) => NppStatus_NPP_NOT_IMPLEMENTED_ERROR,
            }
        }
        Err(__nv_error) => NppStatus_NPP_ERROR,
    }
}
