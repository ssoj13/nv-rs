#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgCreate(
    handle: *mut cusolverMgHandle_t,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgDestroy(
    handle: cusolverMgHandle_t,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgDeviceSelect(
    handle: cusolverMgHandle_t,
    nbDevices: ::std::os::raw::c_int,
    deviceId: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgDeviceSelect {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, nbDevices, deviceId) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgCreateDeviceGrid(
    grid: *mut cudaLibMgGrid_t,
    numRowDevices: i32,
    numColDevices: i32,
    deviceId: *const i32,
    mapping: cusolverMgGridMapping_t,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgCreateDeviceGrid {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(grid, numRowDevices, numColDevices, deviceId, mapping)
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgDestroyGrid(
    grid: cudaLibMgGrid_t,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgDestroyGrid {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(grid) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgCreateMatrixDesc(
    desc: *mut cudaLibMgMatrixDesc_t,
    numRows: i64,
    numCols: i64,
    rowBlockSize: i64,
    colBlockSize: i64,
    dataType: cudaDataType,
    grid: cudaLibMgGrid_t,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgCreateMatrixDesc {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            desc,
                            numRows,
                            numCols,
                            rowBlockSize,
                            colBlockSize,
                            dataType,
                            grid,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgDestroyMatrixDesc(
    desc: cudaLibMgMatrixDesc_t,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgDestroyMatrixDesc {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(desc) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgSyevd_bufferSize(
    handle: cusolverMgHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    N: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    W: *mut ::std::os::raw::c_void,
    dataTypeW: cudaDataType,
    computeType: cudaDataType,
    lwork: *mut i64,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgSyevd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            N,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            W,
                            dataTypeW,
                            computeType,
                            lwork,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgSyevd(
    handle: cusolverMgHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    N: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    W: *mut ::std::os::raw::c_void,
    dataTypeW: cudaDataType,
    computeType: cudaDataType,
    array_d_work: *mut *mut ::std::os::raw::c_void,
    lwork: i64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgSyevd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            N,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            W,
                            dataTypeW,
                            computeType,
                            array_d_work,
                            lwork,
                            info,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgGetrf_bufferSize(
    handle: cusolverMgHandle_t,
    M: ::std::os::raw::c_int,
    N: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    array_d_IPIV: *mut *mut ::std::os::raw::c_int,
    computeType: cudaDataType,
    lwork: *mut i64,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgGetrf_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            M,
                            N,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            array_d_IPIV,
                            computeType,
                            lwork,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgGetrf(
    handle: cusolverMgHandle_t,
    M: ::std::os::raw::c_int,
    N: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    array_d_IPIV: *mut *mut ::std::os::raw::c_int,
    computeType: cudaDataType,
    array_d_work: *mut *mut ::std::os::raw::c_void,
    lwork: i64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgGetrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            M,
                            N,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            array_d_IPIV,
                            computeType,
                            array_d_work,
                            lwork,
                            info,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgGetrs_bufferSize(
    handle: cusolverMgHandle_t,
    TRANS: cublasOperation_t,
    N: ::std::os::raw::c_int,
    NRHS: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    array_d_IPIV: *mut *mut ::std::os::raw::c_int,
    array_d_B: *mut *mut ::std::os::raw::c_void,
    IB: ::std::os::raw::c_int,
    JB: ::std::os::raw::c_int,
    descrB: cudaLibMgMatrixDesc_t,
    computeType: cudaDataType,
    lwork: *mut i64,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgGetrs_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            TRANS,
                            N,
                            NRHS,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            array_d_IPIV,
                            array_d_B,
                            IB,
                            JB,
                            descrB,
                            computeType,
                            lwork,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgGetrs(
    handle: cusolverMgHandle_t,
    TRANS: cublasOperation_t,
    N: ::std::os::raw::c_int,
    NRHS: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    array_d_IPIV: *mut *mut ::std::os::raw::c_int,
    array_d_B: *mut *mut ::std::os::raw::c_void,
    IB: ::std::os::raw::c_int,
    JB: ::std::os::raw::c_int,
    descrB: cudaLibMgMatrixDesc_t,
    computeType: cudaDataType,
    array_d_work: *mut *mut ::std::os::raw::c_void,
    lwork: i64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgGetrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            TRANS,
                            N,
                            NRHS,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            array_d_IPIV,
                            array_d_B,
                            IB,
                            JB,
                            descrB,
                            computeType,
                            array_d_work,
                            lwork,
                            info,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgPotrf_bufferSize(
    handle: cusolverMgHandle_t,
    uplo: cublasFillMode_t,
    N: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    computeType: cudaDataType,
    lwork: *mut i64,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgPotrf_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            N,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            computeType,
                            lwork,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgPotrf(
    handle: cusolverMgHandle_t,
    uplo: cublasFillMode_t,
    N: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    computeType: cudaDataType,
    array_d_work: *mut *mut ::std::os::raw::c_void,
    lwork: i64,
    h_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgPotrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            N,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            computeType,
                            array_d_work,
                            lwork,
                            h_info,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgPotrs_bufferSize(
    handle: cusolverMgHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    array_d_B: *mut *mut ::std::os::raw::c_void,
    IB: ::std::os::raw::c_int,
    JB: ::std::os::raw::c_int,
    descrB: cudaLibMgMatrixDesc_t,
    computeType: cudaDataType,
    lwork: *mut i64,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgPotrs_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            nrhs,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            array_d_B,
                            IB,
                            JB,
                            descrB,
                            computeType,
                            lwork,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgPotrs(
    handle: cusolverMgHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    array_d_B: *mut *mut ::std::os::raw::c_void,
    IB: ::std::os::raw::c_int,
    JB: ::std::os::raw::c_int,
    descrB: cudaLibMgMatrixDesc_t,
    computeType: cudaDataType,
    array_d_work: *mut *mut ::std::os::raw::c_void,
    lwork: i64,
    h_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgPotrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            nrhs,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            array_d_B,
                            IB,
                            JB,
                            descrB,
                            computeType,
                            array_d_work,
                            lwork,
                            h_info,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgPotri_bufferSize(
    handle: cusolverMgHandle_t,
    uplo: cublasFillMode_t,
    N: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    computeType: cudaDataType,
    lwork: *mut i64,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgPotri_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            N,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            computeType,
                            lwork,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverMgPotri(
    handle: cusolverMgHandle_t,
    uplo: cublasFillMode_t,
    N: ::std::os::raw::c_int,
    array_d_A: *mut *mut ::std::os::raw::c_void,
    IA: ::std::os::raw::c_int,
    JA: ::std::os::raw::c_int,
    descrA: cudaLibMgMatrixDesc_t,
    computeType: cudaDataType,
    array_d_work: *mut *mut ::std::os::raw::c_void,
    lwork: i64,
    h_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolvermg_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverMgPotri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            N,
                            array_d_A,
                            IA,
                            JA,
                            descrA,
                            computeType,
                            array_d_work,
                            lwork,
                            h_info,
                        )
                    }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
