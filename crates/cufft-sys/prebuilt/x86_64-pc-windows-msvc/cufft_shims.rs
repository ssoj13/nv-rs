#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftPlan1d(
    plan: *mut cufftHandle,
    nx: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftPlan1d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, nx, type_, batch) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftPlan2d(
    plan: *mut cufftHandle,
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    type_: cufftType,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftPlan2d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, nx, ny, type_) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftPlan3d(
    plan: *mut cufftHandle,
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    nz: ::std::os::raw::c_int,
    type_: cufftType,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftPlan3d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, nx, ny, nz, type_) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftPlanMany(
    plan: *mut cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_int,
    inembed: *mut ::std::os::raw::c_int,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    onembed: *mut ::std::os::raw::c_int,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftPlanMany {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            plan,
                            rank,
                            n,
                            inembed,
                            istride,
                            idist,
                            onembed,
                            ostride,
                            odist,
                            type_,
                            batch,
                        )
                    }
                }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftMakePlan1d(
    plan: cufftHandle,
    nx: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftMakePlan1d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, nx, type_, batch, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftMakePlan2d(
    plan: cufftHandle,
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftMakePlan2d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, nx, ny, type_, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftMakePlan3d(
    plan: cufftHandle,
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    nz: ::std::os::raw::c_int,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftMakePlan3d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, nx, ny, nz, type_, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftMakePlanMany(
    plan: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_int,
    inembed: *mut ::std::os::raw::c_int,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    onembed: *mut ::std::os::raw::c_int,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftMakePlanMany {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            plan,
                            rank,
                            n,
                            inembed,
                            istride,
                            idist,
                            onembed,
                            ostride,
                            odist,
                            type_,
                            batch,
                            workSize,
                        )
                    }
                }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftMakePlanMany64(
    plan: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_longlong,
    inembed: *mut ::std::os::raw::c_longlong,
    istride: ::std::os::raw::c_longlong,
    idist: ::std::os::raw::c_longlong,
    onembed: *mut ::std::os::raw::c_longlong,
    ostride: ::std::os::raw::c_longlong,
    odist: ::std::os::raw::c_longlong,
    type_: cufftType,
    batch: ::std::os::raw::c_longlong,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftMakePlanMany64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            plan,
                            rank,
                            n,
                            inembed,
                            istride,
                            idist,
                            onembed,
                            ostride,
                            odist,
                            type_,
                            batch,
                            workSize,
                        )
                    }
                }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetSizeMany64(
    plan: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_longlong,
    inembed: *mut ::std::os::raw::c_longlong,
    istride: ::std::os::raw::c_longlong,
    idist: ::std::os::raw::c_longlong,
    onembed: *mut ::std::os::raw::c_longlong,
    ostride: ::std::os::raw::c_longlong,
    odist: ::std::os::raw::c_longlong,
    type_: cufftType,
    batch: ::std::os::raw::c_longlong,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetSizeMany64 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            plan,
                            rank,
                            n,
                            inembed,
                            istride,
                            idist,
                            onembed,
                            ostride,
                            odist,
                            type_,
                            batch,
                            workSize,
                        )
                    }
                }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftEstimate1d(
    nx: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftEstimate1d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nx, type_, batch, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftEstimate2d(
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftEstimate2d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nx, ny, type_, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftEstimate3d(
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    nz: ::std::os::raw::c_int,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftEstimate3d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nx, ny, nz, type_, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftEstimateMany(
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_int,
    inembed: *mut ::std::os::raw::c_int,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    onembed: *mut ::std::os::raw::c_int,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftEstimateMany {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            rank,
                            n,
                            inembed,
                            istride,
                            idist,
                            onembed,
                            ostride,
                            odist,
                            type_,
                            batch,
                            workSize,
                        )
                    }
                }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftCreate(handle: *mut cufftHandle) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetSize1d(
    handle: cufftHandle,
    nx: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetSize1d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, nx, type_, batch, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetSize2d(
    handle: cufftHandle,
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetSize2d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, nx, ny, type_, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetSize3d(
    handle: cufftHandle,
    nx: ::std::os::raw::c_int,
    ny: ::std::os::raw::c_int,
    nz: ::std::os::raw::c_int,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetSize3d {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, nx, ny, nz, type_, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetSizeMany(
    handle: cufftHandle,
    rank: ::std::os::raw::c_int,
    n: *mut ::std::os::raw::c_int,
    inembed: *mut ::std::os::raw::c_int,
    istride: ::std::os::raw::c_int,
    idist: ::std::os::raw::c_int,
    onembed: *mut ::std::os::raw::c_int,
    ostride: ::std::os::raw::c_int,
    odist: ::std::os::raw::c_int,
    type_: cufftType,
    batch: ::std::os::raw::c_int,
    workArea: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetSizeMany {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            rank,
                            n,
                            inembed,
                            istride,
                            idist,
                            onembed,
                            ostride,
                            odist,
                            type_,
                            batch,
                            workArea,
                        )
                    }
                }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetSize(
    handle: cufftHandle,
    workSize: *mut usize,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, workSize) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftSetWorkArea(
    plan: cufftHandle,
    workArea: *mut ::std::os::raw::c_void,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftSetWorkArea {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, workArea) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftSetAutoAllocation(
    plan: cufftHandle,
    autoAllocate: ::std::os::raw::c_int,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftSetAutoAllocation {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, autoAllocate) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftExecC2C(
    plan: cufftHandle,
    idata: *mut cufftComplex,
    odata: *mut cufftComplex,
    direction: ::std::os::raw::c_int,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftExecC2C {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, idata, odata, direction) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftExecR2C(
    plan: cufftHandle,
    idata: *mut cufftReal,
    odata: *mut cufftComplex,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftExecR2C {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, idata, odata) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftExecC2R(
    plan: cufftHandle,
    idata: *mut cufftComplex,
    odata: *mut cufftReal,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftExecC2R {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, idata, odata) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftExecZ2Z(
    plan: cufftHandle,
    idata: *mut cufftDoubleComplex,
    odata: *mut cufftDoubleComplex,
    direction: ::std::os::raw::c_int,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftExecZ2Z {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, idata, odata, direction) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftExecD2Z(
    plan: cufftHandle,
    idata: *mut cufftDoubleReal,
    odata: *mut cufftDoubleComplex,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftExecD2Z {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, idata, odata) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftExecZ2D(
    plan: cufftHandle,
    idata: *mut cufftDoubleComplex,
    odata: *mut cufftDoubleReal,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftExecZ2D {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, idata, odata) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftSetStream(
    plan: cufftHandle,
    stream: cudaStream_t,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftSetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, stream) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftDestroy(plan: cufftHandle) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetVersion(
    version: *mut ::std::os::raw::c_int,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(version) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetProperty {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_, value) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftSetPlanPropertyInt64(
    plan: cufftHandle,
    property: cufftProperty,
    inputValueInt: ::std::os::raw::c_longlong,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftSetPlanPropertyInt64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, property, inputValueInt) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftGetPlanPropertyInt64(
    plan: cufftHandle,
    property: cufftProperty,
    returnPtrValue: *mut ::std::os::raw::c_longlong,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftGetPlanPropertyInt64 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, property, returnPtrValue) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cufftResetPlanProperty(
    plan: cufftHandle,
    property: cufftProperty,
) -> cufftResult {
    match cufft_api() {
        Ok(__nv_api) => {
            match &__nv_api.cufftResetPlanProperty {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(plan, property) }
                Err(__nv_error) => cufftResult_t_CUFFT_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cufftResult_t_CUFFT_SETUP_FAILED,
    }
}
