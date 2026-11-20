#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverGetProperty(
    type_: libraryPropertyType,
    value: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverGetProperty {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_, value) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverGetVersion(
    version: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverGetVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(version) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCreate(
    handle: *mut cusolverDnHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCreate {
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
pub unsafe extern "C" fn cusolverDnDestroy(
    handle: cusolverDnHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDestroy {
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
pub unsafe extern "C" fn cusolverDnSetStream(
    handle: cusolverDnHandle_t,
    streamId: cudaStream_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnGetStream(
    handle: cusolverDnHandle_t,
    streamId: *mut cudaStream_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnGetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSetDeterministicMode(
    handle: cusolverDnHandle_t,
    mode: cusolverDeterministicMode_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetDeterministicMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnGetDeterministicMode(
    handle: cusolverDnHandle_t,
    mode: *mut cusolverDeterministicMode_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnGetDeterministicMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSetMathMode(
    handle: cusolverDnHandle_t,
    mode: cusolverMathMode_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetMathMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnGetMathMode(
    handle: cusolverDnHandle_t,
    mode: *mut cusolverMathMode_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnGetMathMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mode) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSetEmulationStrategy(
    handle: cusolverDnHandle_t,
    strategy: cudaEmulationStrategy_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetEmulationStrategy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, strategy) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnGetEmulationStrategy(
    handle: cusolverDnHandle_t,
    strategy: *mut cudaEmulationStrategy_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnGetEmulationStrategy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, strategy) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSetFixedPointEmulationMantissaControl(
    handle: cusolverDnHandle_t,
    control: cudaEmulationMantissaControl_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetFixedPointEmulationMantissaControl {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, control) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnGetFixedPointEmulationMantissaControl(
    handle: cusolverDnHandle_t,
    control: *mut cudaEmulationMantissaControl_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnGetFixedPointEmulationMantissaControl {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, control) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSetFixedPointEmulationMaxMantissaBitCount(
    handle: cusolverDnHandle_t,
    mantissaBitCount: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetFixedPointEmulationMaxMantissaBitCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaBitCount) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnGetFixedPointEmulationMaxMantissaBitCount(
    handle: cusolverDnHandle_t,
    mantissaBitCount: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnGetFixedPointEmulationMaxMantissaBitCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaBitCount) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSetFixedPointEmulationMantissaBitOffset(
    handle: cusolverDnHandle_t,
    mantissaBitOffset: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetFixedPointEmulationMantissaBitOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaBitOffset) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnGetFixedPointEmulationMantissaBitOffset(
    handle: cusolverDnHandle_t,
    mantissaBitOffset: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnGetFixedPointEmulationMantissaBitOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mantissaBitOffset) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSetEmulationSpecialValuesSupport(
    handle: cusolverDnHandle_t,
    mask: cudaEmulationSpecialValuesSupport_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetEmulationSpecialValuesSupport {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mask) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnGetEmulationSpecialValuesSupport(
    handle: cusolverDnHandle_t,
    mask: *mut cudaEmulationSpecialValuesSupport_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnGetEmulationSpecialValuesSupport {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, mask) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsCreate(
    params_ptr: *mut cusolverDnIRSParams_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params_ptr) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsDestroy(
    params: cusolverDnIRSParams_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsSetRefinementSolver(
    params: cusolverDnIRSParams_t,
    refinement_solver: cusolverIRSRefinement_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsSetRefinementSolver {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, refinement_solver) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsSetSolverMainPrecision(
    params: cusolverDnIRSParams_t,
    solver_main_precision: cusolverPrecType_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsSetSolverMainPrecision {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, solver_main_precision) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsSetSolverLowestPrecision(
    params: cusolverDnIRSParams_t,
    solver_lowest_precision: cusolverPrecType_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsSetSolverLowestPrecision {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, solver_lowest_precision) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsSetSolverPrecisions(
    params: cusolverDnIRSParams_t,
    solver_main_precision: cusolverPrecType_t,
    solver_lowest_precision: cusolverPrecType_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsSetSolverPrecisions {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            params,
                            solver_main_precision,
                            solver_lowest_precision,
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
pub unsafe extern "C" fn cusolverDnIRSParamsSetTol(
    params: cusolverDnIRSParams_t,
    val: f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsSetTol {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, val) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsSetTolInner(
    params: cusolverDnIRSParams_t,
    val: f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsSetTolInner {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, val) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsSetMaxIters(
    params: cusolverDnIRSParams_t,
    maxiters: cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsSetMaxIters {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, maxiters) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsSetMaxItersInner(
    params: cusolverDnIRSParams_t,
    maxiters_inner: cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsSetMaxItersInner {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, maxiters_inner) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsGetMaxIters(
    params: cusolverDnIRSParams_t,
    maxiters: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsGetMaxIters {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, maxiters) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsEnableFallback(
    params: cusolverDnIRSParams_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsEnableFallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSParamsDisableFallback(
    params: cusolverDnIRSParams_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSParamsDisableFallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSInfosDestroy(
    infos: cusolverDnIRSInfos_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSInfosDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(infos) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSInfosCreate(
    infos_ptr: *mut cusolverDnIRSInfos_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSInfosCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(infos_ptr) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSInfosGetNiters(
    infos: cusolverDnIRSInfos_t,
    niters: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSInfosGetNiters {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(infos, niters) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSInfosGetOuterNiters(
    infos: cusolverDnIRSInfos_t,
    outer_niters: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSInfosGetOuterNiters {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(infos, outer_niters) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSInfosRequestResidual(
    infos: cusolverDnIRSInfos_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSInfosRequestResidual {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(infos) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSInfosGetResidualHistory(
    infos: cusolverDnIRSInfos_t,
    residual_history: *mut *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSInfosGetResidualHistory {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(infos, residual_history) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSInfosGetMaxIters(
    infos: cusolverDnIRSInfos_t,
    maxiters: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSInfosGetMaxIters {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(infos, maxiters) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZZgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZZgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZCgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZCgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZKgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZKgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZEgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZEgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZYgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZYgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCCgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCCgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCEgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCEgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCKgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCKgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCYgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCYgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDDgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDDgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDSgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDSgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDHgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDHgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDBgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDBgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDXgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDXgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnSSgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSSgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnSHgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSHgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnSBgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSBgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnSXgesv(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSXgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZZgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZZgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZCgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZCgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZKgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZKgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZEgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZEgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZYgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZYgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnCCgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCCgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnCKgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCKgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnCEgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCEgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnCYgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCYgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDDgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDDgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDSgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDSgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDHgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDHgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDBgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDBgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDXgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDXgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnSSgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSSgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnSHgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSHgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnSBgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSBgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnSXgesv_bufferSize(
    handle: cusolverDnHandle_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dipiv: *mut cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSXgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dipiv,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZZgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZZgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZCgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZCgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZKgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZKgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZEgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZEgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZYgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZYgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCCgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCCgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCKgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCKgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCEgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCEgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCYgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCYgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDDgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDDgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDSgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDSgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDHgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDHgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDBgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDBgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnDXgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDXgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnSSgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSSgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnSHgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSHgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnSBgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSBgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnSXgels(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    iter: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSXgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            iter,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnZZgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZZgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZCgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZCgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZKgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZKgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZEgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZEgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnZYgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuDoubleComplex,
    ldda: cusolver_int_t,
    dB: *mut cuDoubleComplex,
    lddb: cusolver_int_t,
    dX: *mut cuDoubleComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZYgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnCCgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCCgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnCKgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCKgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnCEgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCEgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnCYgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut cuComplex,
    ldda: cusolver_int_t,
    dB: *mut cuComplex,
    lddb: cusolver_int_t,
    dX: *mut cuComplex,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCYgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDDgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDDgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDSgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDSgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDHgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDHgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDBgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDBgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnDXgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f64,
    ldda: cusolver_int_t,
    dB: *mut f64,
    lddb: cusolver_int_t,
    dX: *mut f64,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDXgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnSSgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSSgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnSHgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSHgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnSBgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSBgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnSXgels_bufferSize(
    handle: cusolverDnHandle_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut f32,
    ldda: cusolver_int_t,
    dB: *mut f32,
    lddb: cusolver_int_t,
    dX: *mut f32,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSXgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
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
pub unsafe extern "C" fn cusolverDnIRSXgesv(
    handle: cusolverDnHandle_t,
    gesv_irs_params: cusolverDnIRSParams_t,
    gesv_irs_infos: cusolverDnIRSInfos_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut ::std::os::raw::c_void,
    ldda: cusolver_int_t,
    dB: *mut ::std::os::raw::c_void,
    lddb: cusolver_int_t,
    dX: *mut ::std::os::raw::c_void,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    niters: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSXgesv {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            gesv_irs_params,
                            gesv_irs_infos,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            niters,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnIRSXgesv_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnIRSParams_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSXgesv_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, params, n, nrhs, lwork_bytes) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnIRSXgels(
    handle: cusolverDnHandle_t,
    gels_irs_params: cusolverDnIRSParams_t,
    gels_irs_infos: cusolverDnIRSInfos_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    dA: *mut ::std::os::raw::c_void,
    ldda: cusolver_int_t,
    dB: *mut ::std::os::raw::c_void,
    lddb: cusolver_int_t,
    dX: *mut ::std::os::raw::c_void,
    lddx: cusolver_int_t,
    dWorkspace: *mut ::std::os::raw::c_void,
    lwork_bytes: usize,
    niters: *mut cusolver_int_t,
    d_info: *mut cusolver_int_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSXgels {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            gels_irs_params,
                            gels_irs_infos,
                            m,
                            n,
                            nrhs,
                            dA,
                            ldda,
                            dB,
                            lddb,
                            dX,
                            lddx,
                            dWorkspace,
                            lwork_bytes,
                            niters,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnIRSXgels_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnIRSParams_t,
    m: cusolver_int_t,
    n: cusolver_int_t,
    nrhs: cusolver_int_t,
    lwork_bytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnIRSXgels_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, params, m, n, nrhs, lwork_bytes) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSpotrf_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSpotrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDpotrf_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDpotrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCpotrf_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCpotrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZpotrf_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZpotrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSpotrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    Workspace: *mut f32,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSpotrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, Workspace, Lwork, devInfo)
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
pub unsafe extern "C" fn cusolverDnDpotrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    Workspace: *mut f64,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDpotrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, Workspace, Lwork, devInfo)
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
pub unsafe extern "C" fn cusolverDnCpotrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    Workspace: *mut cuComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCpotrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, Workspace, Lwork, devInfo)
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
pub unsafe extern "C" fn cusolverDnZpotrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Workspace: *mut cuDoubleComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZpotrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, Workspace, Lwork, devInfo)
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
pub unsafe extern "C" fn cusolverDnSpotrs(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSpotrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, nrhs, A, lda, B, ldb, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDpotrs(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDpotrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, nrhs, A, lda, B, ldb, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCpotrs(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCpotrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, nrhs, A, lda, B, ldb, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZpotrs(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZpotrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, nrhs, A, lda, B, ldb, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSpotrfBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    Aarray: *mut *mut f32,
    lda: ::std::os::raw::c_int,
    infoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSpotrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, Aarray, lda, infoArray, batchSize)
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
pub unsafe extern "C" fn cusolverDnDpotrfBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    Aarray: *mut *mut f64,
    lda: ::std::os::raw::c_int,
    infoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDpotrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, Aarray, lda, infoArray, batchSize)
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
pub unsafe extern "C" fn cusolverDnCpotrfBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    Aarray: *mut *mut cuComplex,
    lda: ::std::os::raw::c_int,
    infoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCpotrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, Aarray, lda, infoArray, batchSize)
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
pub unsafe extern "C" fn cusolverDnZpotrfBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    Aarray: *mut *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    infoArray: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZpotrfBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, Aarray, lda, infoArray, batchSize)
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
pub unsafe extern "C" fn cusolverDnSpotrsBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *mut *mut f32,
    lda: ::std::os::raw::c_int,
    B: *mut *mut f32,
    ldb: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSpotrsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            nrhs,
                            A,
                            lda,
                            B,
                            ldb,
                            d_info,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnDpotrsBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *mut *mut f64,
    lda: ::std::os::raw::c_int,
    B: *mut *mut f64,
    ldb: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDpotrsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            nrhs,
                            A,
                            lda,
                            B,
                            ldb,
                            d_info,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnCpotrsBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *mut *mut cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCpotrsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            nrhs,
                            A,
                            lda,
                            B,
                            ldb,
                            d_info,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnZpotrsBatched(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *mut *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZpotrsBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            nrhs,
                            A,
                            lda,
                            B,
                            ldb,
                            d_info,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnSpotri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSpotri_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDpotri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDpotri_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCpotri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCpotri_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZpotri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZpotri_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSpotri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSpotri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, work, lwork, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDpotri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDpotri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, work, lwork, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCpotri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCpotri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, work, lwork, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZpotri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZpotri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, work, lwork, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXtrtri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    diag: cublasDiagType_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXtrtri_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            diag,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXtrtri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    diag: cublasDiagType_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXtrtri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            diag,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnSlauum_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSlauum_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDlauum_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDlauum_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnClauum_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnClauum_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZlauum_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZlauum_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSlauum(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSlauum {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, work, lwork, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDlauum(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDlauum {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, work, lwork, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnClauum(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnClauum {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, work, lwork, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZlauum(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZlauum {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, work, lwork, devInfo) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSgetrf_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgetrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, A, lda, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDgetrf_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgetrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, A, lda, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCgetrf_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgetrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, A, lda, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZgetrf_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgetrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, A, lda, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSgetrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    Workspace: *mut f32,
    devIpiv: *mut ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgetrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, A, lda, Workspace, devIpiv, devInfo)
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
pub unsafe extern "C" fn cusolverDnDgetrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    Workspace: *mut f64,
    devIpiv: *mut ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgetrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, A, lda, Workspace, devIpiv, devInfo)
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
pub unsafe extern "C" fn cusolverDnCgetrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    Workspace: *mut cuComplex,
    devIpiv: *mut ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgetrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, A, lda, Workspace, devIpiv, devInfo)
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
pub unsafe extern "C" fn cusolverDnZgetrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    Workspace: *mut cuDoubleComplex,
    devIpiv: *mut ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgetrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, A, lda, Workspace, devIpiv, devInfo)
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
pub unsafe extern "C" fn cusolverDnSlaswp(
    handle: cusolverDnHandle_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    k1: ::std::os::raw::c_int,
    k2: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    incx: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSlaswp {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, k1, k2, devIpiv, incx) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDlaswp(
    handle: cusolverDnHandle_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    k1: ::std::os::raw::c_int,
    k2: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    incx: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDlaswp {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, k1, k2, devIpiv, incx) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnClaswp(
    handle: cusolverDnHandle_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    k1: ::std::os::raw::c_int,
    k2: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    incx: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnClaswp {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, k1, k2, devIpiv, incx) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZlaswp(
    handle: cusolverDnHandle_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    k1: ::std::os::raw::c_int,
    k2: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    incx: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZlaswp {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, n, A, lda, k1, k2, devIpiv, incx) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSgetrs(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgetrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            n,
                            nrhs,
                            A,
                            lda,
                            devIpiv,
                            B,
                            ldb,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnDgetrs(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgetrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            n,
                            nrhs,
                            A,
                            lda,
                            devIpiv,
                            B,
                            ldb,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnCgetrs(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgetrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            n,
                            nrhs,
                            A,
                            lda,
                            devIpiv,
                            B,
                            ldb,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnZgetrs(
    handle: cusolverDnHandle_t,
    trans: cublasOperation_t,
    n: ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    devIpiv: *const ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgetrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            trans,
                            n,
                            nrhs,
                            A,
                            lda,
                            devIpiv,
                            B,
                            ldb,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnSgeqrf_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgeqrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDgeqrf_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgeqrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCgeqrf_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgeqrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZgeqrf_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgeqrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSgeqrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    TAU: *mut f32,
    Workspace: *mut f32,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgeqrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, A, lda, TAU, Workspace, Lwork, devInfo)
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
pub unsafe extern "C" fn cusolverDnDgeqrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    TAU: *mut f64,
    Workspace: *mut f64,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgeqrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, A, lda, TAU, Workspace, Lwork, devInfo)
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
pub unsafe extern "C" fn cusolverDnCgeqrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    TAU: *mut cuComplex,
    Workspace: *mut cuComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgeqrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, A, lda, TAU, Workspace, Lwork, devInfo)
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
pub unsafe extern "C" fn cusolverDnZgeqrf(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    TAU: *mut cuDoubleComplex,
    Workspace: *mut cuDoubleComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgeqrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, A, lda, TAU, Workspace, Lwork, devInfo)
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
pub unsafe extern "C" fn cusolverDnSorgqr_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSorgqr_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, k, A, lda, tau, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDorgqr_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDorgqr_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, k, A, lda, tau, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCungqr_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCungqr_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, k, A, lda, tau, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZungqr_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZungqr_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, k, A, lda, tau, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSorgqr(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSorgqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, k, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnDorgqr(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDorgqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, k, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnCungqr(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCungqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, k, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnZungqr(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZungqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, m, n, k, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnSormqr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    C: *const f32,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSormqr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            trans,
                            m,
                            n,
                            k,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cusolverDnDormqr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    C: *const f64,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDormqr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            trans,
                            m,
                            n,
                            k,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cusolverDnCunmqr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    C: *const cuComplex,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCunmqr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            trans,
                            m,
                            n,
                            k,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cusolverDnZunmqr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    C: *const cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZunmqr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            trans,
                            m,
                            n,
                            k,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cusolverDnSormqr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSormqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            trans,
                            m,
                            n,
                            k,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
                            work,
                            lwork,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnDormqr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDormqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            trans,
                            m,
                            n,
                            k,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
                            work,
                            lwork,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnCunmqr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCunmqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            trans,
                            m,
                            n,
                            k,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
                            work,
                            lwork,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnZunmqr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZunmqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            trans,
                            m,
                            n,
                            k,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
                            work,
                            lwork,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnSsytrf_bufferSize(
    handle: cusolverDnHandle_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsytrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDsytrf_bufferSize(
    handle: cusolverDnHandle_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsytrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCsytrf_bufferSize(
    handle: cusolverDnHandle_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCsytrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZsytrf_bufferSize(
    handle: cusolverDnHandle_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZsytrf_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, n, A, lda, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSsytrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    ipiv: *mut ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsytrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, ipiv, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnDsytrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    ipiv: *mut ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsytrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, ipiv, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnCsytrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *mut ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCsytrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, ipiv, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnZsytrf(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *mut ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZsytrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, ipiv, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnXsytrs_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    ipiv: *const i64,
    dataTypeB: cudaDataType,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsytrs_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            nrhs,
                            dataTypeA,
                            A,
                            lda,
                            ipiv,
                            dataTypeB,
                            B,
                            ldb,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXsytrs(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    ipiv: *const i64,
    dataTypeB: cudaDataType,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsytrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            uplo,
                            n,
                            nrhs,
                            dataTypeA,
                            A,
                            lda,
                            ipiv,
                            dataTypeB,
                            B,
                            ldb,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnSsytri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsytri_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, ipiv, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDsytri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsytri_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, ipiv, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCsytri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCsytri_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, ipiv, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZsytri_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZsytri_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, ipiv, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSsytri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsytri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, ipiv, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnDsytri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsytri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, ipiv, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnCsytri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCsytri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, ipiv, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnZsytri(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    ipiv: *const ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZsytri {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, ipiv, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnSgebrd_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgebrd_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDgebrd_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgebrd_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCgebrd_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgebrd_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZgebrd_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    Lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgebrd_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, Lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSgebrd(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    D: *mut f32,
    E: *mut f32,
    TAUQ: *mut f32,
    TAUP: *mut f32,
    Work: *mut f32,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgebrd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            D,
                            E,
                            TAUQ,
                            TAUP,
                            Work,
                            Lwork,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnDgebrd(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    D: *mut f64,
    E: *mut f64,
    TAUQ: *mut f64,
    TAUP: *mut f64,
    Work: *mut f64,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgebrd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            D,
                            E,
                            TAUQ,
                            TAUP,
                            Work,
                            Lwork,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnCgebrd(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    D: *mut f32,
    E: *mut f32,
    TAUQ: *mut cuComplex,
    TAUP: *mut cuComplex,
    Work: *mut cuComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgebrd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            D,
                            E,
                            TAUQ,
                            TAUP,
                            Work,
                            Lwork,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnZgebrd(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    D: *mut f64,
    E: *mut f64,
    TAUQ: *mut cuDoubleComplex,
    TAUP: *mut cuDoubleComplex,
    Work: *mut cuDoubleComplex,
    Lwork: ::std::os::raw::c_int,
    devInfo: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgebrd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            A,
                            lda,
                            D,
                            E,
                            TAUQ,
                            TAUP,
                            Work,
                            Lwork,
                            devInfo,
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
pub unsafe extern "C" fn cusolverDnSorgbr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSorgbr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, side, m, n, k, A, lda, tau, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDorgbr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDorgbr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, side, m, n, k, A, lda, tau, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCungbr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCungbr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, side, m, n, k, A, lda, tau, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZungbr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZungbr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, side, m, n, k, A, lda, tau, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSorgbr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSorgbr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, side, m, n, k, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnDorgbr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDorgbr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, side, m, n, k, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnCungbr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCungbr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, side, m, n, k, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnZungbr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    k: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZungbr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, side, m, n, k, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnSsytrd_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    d: *const f32,
    e: *const f32,
    tau: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsytrd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, d, e, tau, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDsytrd_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    d: *const f64,
    e: *const f64,
    tau: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsytrd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, d, e, tau, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnChetrd_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    d: *const f32,
    e: *const f32,
    tau: *const cuComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnChetrd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, d, e, tau, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZhetrd_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    d: *const f64,
    e: *const f64,
    tau: *const cuDoubleComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZhetrd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, uplo, n, A, lda, d, e, tau, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSsytrd(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    d: *mut f32,
    e: *mut f32,
    tau: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsytrd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, d, e, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnDsytrd(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    d: *mut f64,
    e: *mut f64,
    tau: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsytrd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, d, e, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnChetrd(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    d: *mut f32,
    e: *mut f32,
    tau: *mut cuComplex,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnChetrd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, d, e, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnZhetrd(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    d: *mut f64,
    e: *mut f64,
    tau: *mut cuDoubleComplex,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZhetrd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, d, e, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnSorgtr_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSorgtr_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, tau, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDorgtr_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDorgtr_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, tau, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCungtr_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCungtr_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, tau, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZungtr_bufferSize(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZungtr_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, uplo, n, A, lda, tau, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSorgtr(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSorgtr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnDorgtr(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDorgtr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnCungtr(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCungtr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnZungtr(
    handle: cusolverDnHandle_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZungtr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, uplo, n, A, lda, tau, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnSormtr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    tau: *const f32,
    C: *const f32,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSormtr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            m,
                            n,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cusolverDnDormtr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    tau: *const f64,
    C: *const f64,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDormtr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            m,
                            n,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cusolverDnCunmtr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuComplex,
    C: *const cuComplex,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCunmtr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            m,
                            n,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cusolverDnZunmtr_bufferSize(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *const cuDoubleComplex,
    C: *const cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZunmtr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            m,
                            n,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
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
pub unsafe extern "C" fn cusolverDnSormtr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    tau: *mut f32,
    C: *mut f32,
    ldc: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSormtr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            m,
                            n,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
                            work,
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
pub unsafe extern "C" fn cusolverDnDormtr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    tau: *mut f64,
    C: *mut f64,
    ldc: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDormtr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            m,
                            n,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
                            work,
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
pub unsafe extern "C" fn cusolverDnCunmtr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    tau: *mut cuComplex,
    C: *mut cuComplex,
    ldc: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCunmtr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            m,
                            n,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
                            work,
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
pub unsafe extern "C" fn cusolverDnZunmtr(
    handle: cusolverDnHandle_t,
    side: cublasSideMode_t,
    uplo: cublasFillMode_t,
    trans: cublasOperation_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    tau: *mut cuDoubleComplex,
    C: *mut cuDoubleComplex,
    ldc: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZunmtr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            side,
                            uplo,
                            trans,
                            m,
                            n,
                            A,
                            lda,
                            tau,
                            C,
                            ldc,
                            work,
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
pub unsafe extern "C" fn cusolverDnSgesvd_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgesvd_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDgesvd_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgesvd_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCgesvd_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgesvd_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZgesvd_bufferSize(
    handle: cusolverDnHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgesvd_bufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, m, n, lwork) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSgesvd(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut f32,
    ldu: ::std::os::raw::c_int,
    VT: *mut f32,
    ldvt: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    rwork: *mut f32,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgesvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobu,
                            jobvt,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            VT,
                            ldvt,
                            work,
                            lwork,
                            rwork,
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
pub unsafe extern "C" fn cusolverDnDgesvd(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut f64,
    ldu: ::std::os::raw::c_int,
    VT: *mut f64,
    ldvt: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    rwork: *mut f64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgesvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobu,
                            jobvt,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            VT,
                            ldvt,
                            work,
                            lwork,
                            rwork,
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
pub unsafe extern "C" fn cusolverDnCgesvd(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut cuComplex,
    ldu: ::std::os::raw::c_int,
    VT: *mut cuComplex,
    ldvt: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    rwork: *mut f32,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgesvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobu,
                            jobvt,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            VT,
                            ldvt,
                            work,
                            lwork,
                            rwork,
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
pub unsafe extern "C" fn cusolverDnZgesvd(
    handle: cusolverDnHandle_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    VT: *mut cuDoubleComplex,
    ldvt: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    rwork: *mut f64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgesvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobu,
                            jobvt,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            VT,
                            ldvt,
                            work,
                            lwork,
                            rwork,
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
pub unsafe extern "C" fn cusolverDnSsyevd_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsyevd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDsyevd_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsyevd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnCheevd_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCheevd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnZheevd_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZheevd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, lwork) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSsyevd(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsyevd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnDsyevd(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsyevd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnCheevd(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCheevd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnZheevd(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZheevd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, work, lwork, info)
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
pub unsafe extern "C" fn cusolverDnSsyevdx_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsyevdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
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
pub unsafe extern "C" fn cusolverDnDsyevdx_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsyevdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
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
pub unsafe extern "C" fn cusolverDnCheevdx_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCheevdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
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
pub unsafe extern "C" fn cusolverDnZheevdx_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZheevdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
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
pub unsafe extern "C" fn cusolverDnSsyevdx(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsyevdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnDsyevdx(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsyevdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnCheevdx(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCheevdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnZheevdx(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZheevdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnSsygvdx_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsygvdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
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
pub unsafe extern "C" fn cusolverDnDsygvdx_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsygvdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
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
pub unsafe extern "C" fn cusolverDnChegvdx_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnChegvdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
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
pub unsafe extern "C" fn cusolverDnZhegvdx_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZhegvdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
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
pub unsafe extern "C" fn cusolverDnSsygvdx(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsygvdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnDsygvdx(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsygvdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnChegvdx(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    vl: f32,
    vu: f32,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnChegvdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnZhegvdx(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    vl: f64,
    vu: f64,
    il: ::std::os::raw::c_int,
    iu: ::std::os::raw::c_int,
    meig: *mut ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZhegvdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnSsygvd_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsygvd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
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
pub unsafe extern "C" fn cusolverDnDsygvd_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsygvd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
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
pub unsafe extern "C" fn cusolverDnChegvd_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnChegvd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
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
pub unsafe extern "C" fn cusolverDnZhegvd_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZhegvd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
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
pub unsafe extern "C" fn cusolverDnSsygvd(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsygvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnDsygvd(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsygvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnChegvd(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnChegvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnZhegvd(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZhegvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            work,
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
pub unsafe extern "C" fn cusolverDnXsygvd_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: *const ::std::os::raw::c_void,
    ldb: i64,
    dataTypeW: cudaDataType,
    d_W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsygvd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            itype,
                            jobz,
                            uplo,
                            n,
                            dataTypeA,
                            d_A,
                            lda,
                            dataTypeB,
                            d_B,
                            ldb,
                            dataTypeW,
                            d_W,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXsygvd(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: *mut ::std::os::raw::c_void,
    ldb: i64,
    dataTypeW: cudaDataType,
    d_W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsygvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            itype,
                            jobz,
                            uplo,
                            n,
                            dataTypeA,
                            d_A,
                            lda,
                            dataTypeB,
                            d_B,
                            ldb,
                            dataTypeW,
                            d_W,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnXsygvdx_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: *const ::std::os::raw::c_void,
    ldb: i64,
    vl: *mut ::std::os::raw::c_void,
    vu: *mut ::std::os::raw::c_void,
    il: i64,
    iu: i64,
    meig: *mut i64,
    dataTypeW: cudaDataType,
    d_W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsygvdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            itype,
                            jobz,
                            uplo,
                            n,
                            dataTypeA,
                            d_A,
                            lda,
                            dataTypeB,
                            d_B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            dataTypeW,
                            d_W,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXsygvdx(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    d_A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    d_B: *mut ::std::os::raw::c_void,
    ldb: i64,
    vl: *mut ::std::os::raw::c_void,
    vu: *mut ::std::os::raw::c_void,
    il: i64,
    iu: i64,
    meig: *mut i64,
    dataTypeW: cudaDataType,
    d_W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsygvdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            itype,
                            jobz,
                            range,
                            uplo,
                            n,
                            dataTypeA,
                            d_A,
                            lda,
                            dataTypeB,
                            d_B,
                            ldb,
                            vl,
                            vu,
                            il,
                            iu,
                            meig,
                            dataTypeW,
                            d_W,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnCreateSyevjInfo(
    info: *mut syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCreateSyevjInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDestroySyevjInfo(
    info: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDestroySyevjInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXsyevjSetTolerance(
    info: syevjInfo_t,
    tolerance: f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevjSetTolerance {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info, tolerance) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXsyevjSetMaxSweeps(
    info: syevjInfo_t,
    max_sweeps: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevjSetMaxSweeps {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info, max_sweeps) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXsyevjSetSortEig(
    info: syevjInfo_t,
    sort_eig: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevjSetSortEig {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info, sort_eig) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXsyevjGetResidual(
    handle: cusolverDnHandle_t,
    info: syevjInfo_t,
    residual: *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevjGetResidual {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, residual) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXsyevjGetSweeps(
    handle: cusolverDnHandle_t,
    info: syevjInfo_t,
    executed_sweeps: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevjGetSweeps {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, executed_sweeps) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSsyevjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsyevjBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            lwork,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnDsyevjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsyevjBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            lwork,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnCheevjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCheevjBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            lwork,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnZheevjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZheevjBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            lwork,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnSsyevjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsyevjBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            work,
                            lwork,
                            info,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnDsyevjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsyevjBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            work,
                            lwork,
                            info,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnCheevjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCheevjBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            work,
                            lwork,
                            info,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnZheevjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZheevjBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            work,
                            lwork,
                            info,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnSsyevj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsyevj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, lwork, params)
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
pub unsafe extern "C" fn cusolverDnDsyevj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsyevj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, lwork, params)
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
pub unsafe extern "C" fn cusolverDnCheevj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCheevj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, lwork, params)
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
pub unsafe extern "C" fn cusolverDnZheevj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZheevj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, jobz, uplo, n, A, lda, W, lwork, params)
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
pub unsafe extern "C" fn cusolverDnSsyevj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsyevj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnDsyevj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsyevj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnCheevj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCheevj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnZheevj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZheevj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            W,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnSsygvj_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    B: *const f32,
    ldb: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsygvj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            lwork,
                            params,
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
pub unsafe extern "C" fn cusolverDnDsygvj_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    B: *const f64,
    ldb: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsygvj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            lwork,
                            params,
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
pub unsafe extern "C" fn cusolverDnChegvj_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuComplex,
    ldb: ::std::os::raw::c_int,
    W: *const f32,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnChegvj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            lwork,
                            params,
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
pub unsafe extern "C" fn cusolverDnZhegvj_bufferSize(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *const cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    W: *const f64,
    lwork: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZhegvj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            lwork,
                            params,
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
pub unsafe extern "C" fn cusolverDnSsygvj(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    B: *mut f32,
    ldb: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSsygvj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnDsygvj(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    B: *mut f64,
    ldb: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDsygvj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnChegvj(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuComplex,
    ldb: ::std::os::raw::c_int,
    W: *mut f32,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnChegvj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnZhegvj(
    handle: cusolverDnHandle_t,
    itype: cusolverEigType_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    B: *mut cuDoubleComplex,
    ldb: ::std::os::raw::c_int,
    W: *mut f64,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: syevjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZhegvj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            itype,
                            jobz,
                            uplo,
                            n,
                            A,
                            lda,
                            B,
                            ldb,
                            W,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnCreateGesvdjInfo(
    info: *mut gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCreateGesvdjInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDestroyGesvdjInfo(
    info: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDestroyGesvdjInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXgesvdjSetTolerance(
    info: gesvdjInfo_t,
    tolerance: f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdjSetTolerance {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info, tolerance) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXgesvdjSetMaxSweeps(
    info: gesvdjInfo_t,
    max_sweeps: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdjSetMaxSweeps {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info, max_sweeps) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXgesvdjSetSortEig(
    info: gesvdjInfo_t,
    sort_svd: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdjSetSortEig {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info, sort_svd) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXgesvdjGetResidual(
    handle: cusolverDnHandle_t,
    info: gesvdjInfo_t,
    residual: *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdjGetResidual {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, residual) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXgesvdjGetSweeps(
    handle: cusolverDnHandle_t,
    info: gesvdjInfo_t,
    executed_sweeps: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdjGetSweeps {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, info, executed_sweeps) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSgesvdjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    S: *const f32,
    U: *const f32,
    ldu: ::std::os::raw::c_int,
    V: *const f32,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgesvdjBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            lwork,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnDgesvdjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    S: *const f64,
    U: *const f64,
    ldu: ::std::os::raw::c_int,
    V: *const f64,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgesvdjBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            lwork,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnCgesvdjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    S: *const f32,
    U: *const cuComplex,
    ldu: ::std::os::raw::c_int,
    V: *const cuComplex,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgesvdjBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            lwork,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnZgesvdjBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *const f64,
    U: *const cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    V: *const cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgesvdjBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            lwork,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnSgesvdjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut f32,
    ldu: ::std::os::raw::c_int,
    V: *mut f32,
    ldv: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgesvdjBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            work,
                            lwork,
                            info,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnDgesvdjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut f64,
    ldu: ::std::os::raw::c_int,
    V: *mut f64,
    ldv: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgesvdjBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            work,
                            lwork,
                            info,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnCgesvdjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut cuComplex,
    ldu: ::std::os::raw::c_int,
    V: *mut cuComplex,
    ldv: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgesvdjBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            work,
                            lwork,
                            info,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnZgesvdjBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    V: *mut cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgesvdjBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            work,
                            lwork,
                            info,
                            params,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnSgesvdj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f32,
    lda: ::std::os::raw::c_int,
    S: *const f32,
    U: *const f32,
    ldu: ::std::os::raw::c_int,
    V: *const f32,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgesvdj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            econ,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            lwork,
                            params,
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
pub unsafe extern "C" fn cusolverDnDgesvdj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const f64,
    lda: ::std::os::raw::c_int,
    S: *const f64,
    U: *const f64,
    ldu: ::std::os::raw::c_int,
    V: *const f64,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgesvdj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            econ,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            lwork,
                            params,
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
pub unsafe extern "C" fn cusolverDnCgesvdj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    S: *const f32,
    U: *const cuComplex,
    ldu: ::std::os::raw::c_int,
    V: *const cuComplex,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgesvdj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            econ,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            lwork,
                            params,
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
pub unsafe extern "C" fn cusolverDnZgesvdj_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *const f64,
    U: *const cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    V: *const cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    lwork: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgesvdj_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            econ,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            lwork,
                            params,
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
pub unsafe extern "C" fn cusolverDnSgesvdj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f32,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut f32,
    ldu: ::std::os::raw::c_int,
    V: *mut f32,
    ldv: ::std::os::raw::c_int,
    work: *mut f32,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgesvdj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            econ,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnDgesvdj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut f64,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut f64,
    ldu: ::std::os::raw::c_int,
    V: *mut f64,
    ldv: ::std::os::raw::c_int,
    work: *mut f64,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgesvdj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            econ,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnCgesvdj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f32,
    U: *mut cuComplex,
    ldu: ::std::os::raw::c_int,
    V: *mut cuComplex,
    ldv: ::std::os::raw::c_int,
    work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgesvdj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            econ,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnZgesvdj(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    A: *mut cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    S: *mut f64,
    U: *mut cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    V: *mut cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    info: *mut ::std::os::raw::c_int,
    params: gesvdjInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgesvdj {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            econ,
                            m,
                            n,
                            A,
                            lda,
                            S,
                            U,
                            ldu,
                            V,
                            ldv,
                            work,
                            lwork,
                            info,
                            params,
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
pub unsafe extern "C" fn cusolverDnSgesvdaStridedBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const f32,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *const f32,
    strideS: ::std::os::raw::c_longlong,
    d_U: *const f32,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *const f32,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    lwork: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgesvdaStridedBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            rank,
                            m,
                            n,
                            d_A,
                            lda,
                            strideA,
                            d_S,
                            strideS,
                            d_U,
                            ldu,
                            strideU,
                            d_V,
                            ldv,
                            strideV,
                            lwork,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnDgesvdaStridedBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const f64,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *const f64,
    strideS: ::std::os::raw::c_longlong,
    d_U: *const f64,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *const f64,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    lwork: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgesvdaStridedBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            rank,
                            m,
                            n,
                            d_A,
                            lda,
                            strideA,
                            d_S,
                            strideS,
                            d_U,
                            ldu,
                            strideU,
                            d_V,
                            ldv,
                            strideV,
                            lwork,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnCgesvdaStridedBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *const f32,
    strideS: ::std::os::raw::c_longlong,
    d_U: *const cuComplex,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *const cuComplex,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    lwork: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgesvdaStridedBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            rank,
                            m,
                            n,
                            d_A,
                            lda,
                            strideA,
                            d_S,
                            strideS,
                            d_U,
                            ldu,
                            strideU,
                            d_V,
                            ldv,
                            strideV,
                            lwork,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnZgesvdaStridedBatched_bufferSize(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *const f64,
    strideS: ::std::os::raw::c_longlong,
    d_U: *const cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *const cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    lwork: *mut ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgesvdaStridedBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            rank,
                            m,
                            n,
                            d_A,
                            lda,
                            strideA,
                            d_S,
                            strideS,
                            d_U,
                            ldu,
                            strideU,
                            d_V,
                            ldv,
                            strideV,
                            lwork,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnSgesvdaStridedBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const f32,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *mut f32,
    strideS: ::std::os::raw::c_longlong,
    d_U: *mut f32,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *mut f32,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    d_work: *mut f32,
    lwork: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    h_R_nrmF: *mut f64,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSgesvdaStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            rank,
                            m,
                            n,
                            d_A,
                            lda,
                            strideA,
                            d_S,
                            strideS,
                            d_U,
                            ldu,
                            strideU,
                            d_V,
                            ldv,
                            strideV,
                            d_work,
                            lwork,
                            d_info,
                            h_R_nrmF,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnDgesvdaStridedBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const f64,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *mut f64,
    strideS: ::std::os::raw::c_longlong,
    d_U: *mut f64,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *mut f64,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    d_work: *mut f64,
    lwork: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    h_R_nrmF: *mut f64,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDgesvdaStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            rank,
                            m,
                            n,
                            d_A,
                            lda,
                            strideA,
                            d_S,
                            strideS,
                            d_U,
                            ldu,
                            strideU,
                            d_V,
                            ldv,
                            strideV,
                            d_work,
                            lwork,
                            d_info,
                            h_R_nrmF,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnCgesvdaStridedBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const cuComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *mut f32,
    strideS: ::std::os::raw::c_longlong,
    d_U: *mut cuComplex,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *mut cuComplex,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    d_work: *mut cuComplex,
    lwork: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    h_R_nrmF: *mut f64,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCgesvdaStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            rank,
                            m,
                            n,
                            d_A,
                            lda,
                            strideA,
                            d_S,
                            strideS,
                            d_U,
                            ldu,
                            strideU,
                            d_V,
                            ldv,
                            strideV,
                            d_work,
                            lwork,
                            d_info,
                            h_R_nrmF,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnZgesvdaStridedBatched(
    handle: cusolverDnHandle_t,
    jobz: cusolverEigMode_t,
    rank: ::std::os::raw::c_int,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    d_A: *const cuDoubleComplex,
    lda: ::std::os::raw::c_int,
    strideA: ::std::os::raw::c_longlong,
    d_S: *mut f64,
    strideS: ::std::os::raw::c_longlong,
    d_U: *mut cuDoubleComplex,
    ldu: ::std::os::raw::c_int,
    strideU: ::std::os::raw::c_longlong,
    d_V: *mut cuDoubleComplex,
    ldv: ::std::os::raw::c_int,
    strideV: ::std::os::raw::c_longlong,
    d_work: *mut cuDoubleComplex,
    lwork: ::std::os::raw::c_int,
    d_info: *mut ::std::os::raw::c_int,
    h_R_nrmF: *mut f64,
    batchSize: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnZgesvdaStridedBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            jobz,
                            rank,
                            m,
                            n,
                            d_A,
                            lda,
                            strideA,
                            d_S,
                            strideS,
                            d_U,
                            ldu,
                            strideU,
                            d_V,
                            ldv,
                            strideV,
                            d_work,
                            lwork,
                            d_info,
                            h_R_nrmF,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnCreateParams(
    params: *mut cusolverDnParams_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnCreateParams {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnDestroyParams(
    params: cusolverDnParams_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnDestroyParams {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnSetAdvOptions(
    params: cusolverDnParams_t,
    function: cusolverDnFunction_t,
    algo: cusolverAlgMode_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnSetAdvOptions {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(params, function, algo) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXpotrf_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXpotrf_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            uplo,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXpotrf(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXpotrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            uplo,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXpotrs(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeB: cudaDataType,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXpotrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            uplo,
                            n,
                            nrhs,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeB,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cusolverDnXgeqrf_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeTau: cudaDataType,
    tau: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgeqrf_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            m,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeTau,
                            tau,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgeqrf(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeTau: cudaDataType,
    tau: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgeqrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            m,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeTau,
                            tau,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgetrf_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgetrf_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            m,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgetrf(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    ipiv: *mut i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgetrf {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            m,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            ipiv,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgetrs(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    trans: cublasOperation_t,
    n: i64,
    nrhs: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    ipiv: *const i64,
    dataTypeB: cudaDataType,
    B: *mut ::std::os::raw::c_void,
    ldb: i64,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgetrs {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            trans,
                            n,
                            nrhs,
                            dataTypeA,
                            A,
                            lda,
                            ipiv,
                            dataTypeB,
                            B,
                            ldb,
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
pub unsafe extern "C" fn cusolverDnXsyevd_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobz,
                            uplo,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeW,
                            W,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXsyevd(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobz,
                            uplo,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeW,
                            W,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXstedc_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    compz: cusolverEigComp_t,
    n: i64,
    dataTypeDE: cudaDataType,
    D: *const ::std::os::raw::c_void,
    E: *const ::std::os::raw::c_void,
    dataTypeZ: cudaDataType,
    Z: *const ::std::os::raw::c_void,
    ldz: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXstedc_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            compz,
                            n,
                            dataTypeDE,
                            D,
                            E,
                            dataTypeZ,
                            Z,
                            ldz,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXstedc(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    compz: cusolverEigComp_t,
    n: i64,
    dataTypeDE: cudaDataType,
    D: *mut ::std::os::raw::c_void,
    E: *mut ::std::os::raw::c_void,
    dataTypeZ: cudaDataType,
    Z: *mut ::std::os::raw::c_void,
    ldz: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXstedc {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            compz,
                            n,
                            dataTypeDE,
                            D,
                            E,
                            dataTypeZ,
                            Z,
                            ldz,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXsyevBatched_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
    batchSize: i64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevBatched_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobz,
                            uplo,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeW,
                            W,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnXsyevBatched(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
    batchSize: i64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobz,
                            uplo,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeW,
                            W,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
                            info,
                            batchSize,
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
pub unsafe extern "C" fn cusolverDnXsyevdx_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    vl: *mut ::std::os::raw::c_void,
    vu: *mut ::std::os::raw::c_void,
    il: i64,
    iu: i64,
    h_meig: *mut i64,
    dataTypeW: cudaDataType,
    W: *const ::std::os::raw::c_void,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevdx_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobz,
                            range,
                            uplo,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            h_meig,
                            dataTypeW,
                            W,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXsyevdx(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    range: cusolverEigRange_t,
    uplo: cublasFillMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    vl: *mut ::std::os::raw::c_void,
    vu: *mut ::std::os::raw::c_void,
    il: i64,
    iu: i64,
    meig64: *mut i64,
    dataTypeW: cudaDataType,
    W: *mut ::std::os::raw::c_void,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXsyevdx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobz,
                            range,
                            uplo,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            vl,
                            vu,
                            il,
                            iu,
                            meig64,
                            dataTypeW,
                            W,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgeev_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobvl: cusolverEigMode_t,
    jobvr: cusolverEigMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *const ::std::os::raw::c_void,
    dataTypeVL: cudaDataType,
    VL: *const ::std::os::raw::c_void,
    ldvl: i64,
    dataTypeVR: cudaDataType,
    VR: *const ::std::os::raw::c_void,
    ldvr: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgeev_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobvl,
                            jobvr,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeW,
                            W,
                            dataTypeVL,
                            VL,
                            ldvl,
                            dataTypeVR,
                            VR,
                            ldvr,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgeev(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobvl: cusolverEigMode_t,
    jobvr: cusolverEigMode_t,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeW: cudaDataType,
    W: *mut ::std::os::raw::c_void,
    dataTypeVL: cudaDataType,
    VL: *mut ::std::os::raw::c_void,
    ldvl: i64,
    dataTypeVR: cudaDataType,
    VR: *mut ::std::os::raw::c_void,
    ldvr: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgeev {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobvl,
                            jobvr,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeW,
                            W,
                            dataTypeVL,
                            VL,
                            ldvl,
                            dataTypeVR,
                            VR,
                            ldvr,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgesvd_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeS: cudaDataType,
    S: *const ::std::os::raw::c_void,
    dataTypeU: cudaDataType,
    U: *const ::std::os::raw::c_void,
    ldu: i64,
    dataTypeVT: cudaDataType,
    VT: *const ::std::os::raw::c_void,
    ldvt: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvd_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobu,
                            jobvt,
                            m,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeS,
                            S,
                            dataTypeU,
                            U,
                            ldu,
                            dataTypeVT,
                            VT,
                            ldvt,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgesvd(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobvt: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeS: cudaDataType,
    S: *mut ::std::os::raw::c_void,
    dataTypeU: cudaDataType,
    U: *mut ::std::os::raw::c_void,
    ldu: i64,
    dataTypeVT: cudaDataType,
    VT: *mut ::std::os::raw::c_void,
    ldvt: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvd {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobu,
                            jobvt,
                            m,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeS,
                            S,
                            dataTypeU,
                            U,
                            ldu,
                            dataTypeVT,
                            VT,
                            ldvt,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgesvdp_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeS: cudaDataType,
    S: *const ::std::os::raw::c_void,
    dataTypeU: cudaDataType,
    U: *const ::std::os::raw::c_void,
    ldu: i64,
    dataTypeV: cudaDataType,
    V: *const ::std::os::raw::c_void,
    ldv: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdp_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobz,
                            econ,
                            m,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeS,
                            S,
                            dataTypeU,
                            U,
                            ldu,
                            dataTypeV,
                            V,
                            ldv,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgesvdp(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobz: cusolverEigMode_t,
    econ: ::std::os::raw::c_int,
    m: i64,
    n: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeS: cudaDataType,
    S: *mut ::std::os::raw::c_void,
    dataTypeU: cudaDataType,
    U: *mut ::std::os::raw::c_void,
    ldu: i64,
    dataTypeV: cudaDataType,
    V: *mut ::std::os::raw::c_void,
    ldv: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_info: *mut ::std::os::raw::c_int,
    h_err_sigma: *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdp {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobz,
                            econ,
                            m,
                            n,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeS,
                            S,
                            dataTypeU,
                            U,
                            ldu,
                            dataTypeV,
                            V,
                            ldv,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
                            d_info,
                            h_err_sigma,
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
pub unsafe extern "C" fn cusolverDnXgesvdr_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobv: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    k: i64,
    p: i64,
    niters: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeSrand: cudaDataType,
    Srand: *const ::std::os::raw::c_void,
    dataTypeUrand: cudaDataType,
    Urand: *const ::std::os::raw::c_void,
    ldUrand: i64,
    dataTypeVrand: cudaDataType,
    Vrand: *const ::std::os::raw::c_void,
    ldVrand: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdr_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobu,
                            jobv,
                            m,
                            n,
                            k,
                            p,
                            niters,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeSrand,
                            Srand,
                            dataTypeUrand,
                            Urand,
                            ldUrand,
                            dataTypeVrand,
                            Vrand,
                            ldVrand,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXgesvdr(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    jobu: ::std::os::raw::c_schar,
    jobv: ::std::os::raw::c_schar,
    m: i64,
    n: i64,
    k: i64,
    p: i64,
    niters: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeSrand: cudaDataType,
    Srand: *mut ::std::os::raw::c_void,
    dataTypeUrand: cudaDataType,
    Urand: *mut ::std::os::raw::c_void,
    ldUrand: i64,
    dataTypeVrand: cudaDataType,
    Vrand: *mut ::std::os::raw::c_void,
    ldVrand: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXgesvdr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            jobu,
                            jobv,
                            m,
                            n,
                            k,
                            p,
                            niters,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeSrand,
                            Srand,
                            dataTypeUrand,
                            Urand,
                            ldUrand,
                            dataTypeVrand,
                            Vrand,
                            ldVrand,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
                            d_info,
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
pub unsafe extern "C" fn cusolverDnXlarft_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    direct: cusolverDirectMode_t,
    storev: cusolverStorevMode_t,
    n: i64,
    k: i64,
    dataTypeV: cudaDataType,
    V: *const ::std::os::raw::c_void,
    ldv: i64,
    dataTypeTau: cudaDataType,
    tau: *const ::std::os::raw::c_void,
    dataTypeT: cudaDataType,
    T: *mut ::std::os::raw::c_void,
    ldt: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXlarft_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            direct,
                            storev,
                            n,
                            k,
                            dataTypeV,
                            V,
                            ldv,
                            dataTypeTau,
                            tau,
                            dataTypeT,
                            T,
                            ldt,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXlarft(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    direct: cusolverDirectMode_t,
    storev: cusolverStorevMode_t,
    n: i64,
    k: i64,
    dataTypeV: cudaDataType,
    V: *const ::std::os::raw::c_void,
    ldv: i64,
    dataTypeTau: cudaDataType,
    tau: *const ::std::os::raw::c_void,
    dataTypeT: cudaDataType,
    T: *mut ::std::os::raw::c_void,
    ldt: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXlarft {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            direct,
                            storev,
                            n,
                            k,
                            dataTypeV,
                            V,
                            ldv,
                            dataTypeTau,
                            tau,
                            dataTypeT,
                            T,
                            ldt,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnLoggerSetCallback(
    callback: cusolverDnLoggerCallback_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnLoggerSetCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(callback) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnLoggerSetFile(file: *mut FILE) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnLoggerSetFile {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(file) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnLoggerOpenFile(
    logFile: *const ::std::os::raw::c_char,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnLoggerOpenFile {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(logFile) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnLoggerSetLevel(
    level: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnLoggerSetLevel {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(level) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnLoggerSetMask(
    mask: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnLoggerSetMask {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(mask) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnLoggerForceDisable() -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnLoggerForceDisable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverDnXpolar_bufferSize(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    M: i64,
    N: i64,
    dataTypeA: cudaDataType,
    A: *const ::std::os::raw::c_void,
    lda: i64,
    dataTypeH: cudaDataType,
    H: *const ::std::os::raw::c_void,
    ldh: i64,
    computeType: cudaDataType,
    workspaceInBytesOnDevice: *mut usize,
    workspaceInBytesOnHost: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXpolar_bufferSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            uplo,
                            M,
                            N,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeH,
                            H,
                            ldh,
                            computeType,
                            workspaceInBytesOnDevice,
                            workspaceInBytesOnHost,
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
pub unsafe extern "C" fn cusolverDnXpolar(
    handle: cusolverDnHandle_t,
    params: cusolverDnParams_t,
    uplo: cublasFillMode_t,
    M: i64,
    N: i64,
    dataTypeA: cudaDataType,
    A: *mut ::std::os::raw::c_void,
    lda: i64,
    dataTypeH: cudaDataType,
    H: *mut ::std::os::raw::c_void,
    ldh: i64,
    computeType: cudaDataType,
    bufferOnDevice: *mut ::std::os::raw::c_void,
    workspaceInBytesOnDevice: usize,
    bufferOnHost: *mut ::std::os::raw::c_void,
    workspaceInBytesOnHost: usize,
    d_res_nrm: *mut f64,
    d_A_nrmF: *mut f64,
    d_rcond: *mut f64,
    d_info: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverDnXpolar {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            params,
                            uplo,
                            M,
                            N,
                            dataTypeA,
                            A,
                            lda,
                            dataTypeH,
                            H,
                            ldh,
                            computeType,
                            bufferOnDevice,
                            workspaceInBytesOnDevice,
                            bufferOnHost,
                            workspaceInBytesOnHost,
                            d_res_nrm,
                            d_A_nrmF,
                            d_rcond,
                            d_info,
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
pub unsafe extern "C" fn cusolverSpCreate(
    handle: *mut cusolverSpHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCreate {
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
pub unsafe extern "C" fn cusolverSpDestroy(
    handle: cusolverSpHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDestroy {
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
pub unsafe extern "C" fn cusolverSpSetStream(
    handle: cusolverSpHandle_t,
    streamId: cudaStream_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpSetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverSpGetStream(
    handle: cusolverSpHandle_t,
    streamId: *mut cudaStream_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpGetStream {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, streamId) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverSpXcsrissymHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrEndPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    issym: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpXcsrissymHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnzA,
                            descrA,
                            csrRowPtrA,
                            csrEndPtrA,
                            csrColIndA,
                            issym,
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
pub unsafe extern "C" fn cusolverSpScsrlsvluHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrlsvluHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnzA,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpDcsrlsvluHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrlsvluHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnzA,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpCcsrlsvluHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrlsvluHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnzA,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpZcsrlsvluHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrlsvluHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnzA,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpScsrlsvqr(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrlsvqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpDcsrlsvqr(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrlsvqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpCcsrlsvqr(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrlsvqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpZcsrlsvqr(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrlsvqr {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpScsrlsvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrlsvqrHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpDcsrlsvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrlsvqrHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpCcsrlsvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrlsvqrHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpZcsrlsvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrlsvqrHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpScsrlsvcholHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrlsvcholHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpDcsrlsvcholHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrlsvcholHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpCcsrlsvcholHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrlsvcholHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpZcsrlsvcholHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrlsvcholHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpScsrlsvchol(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut f32,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrlsvchol {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpDcsrlsvchol(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut f64,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrlsvchol {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpCcsrlsvchol(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    reorder: ::std::os::raw::c_int,
    x: *mut cuComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrlsvchol {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpZcsrlsvchol(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    reorder: ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    singularity: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrlsvchol {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            b,
                            tol,
                            reorder,
                            x,
                            singularity,
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
pub unsafe extern "C" fn cusolverSpScsrlsqvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f32,
    tol: f32,
    rankA: *mut ::std::os::raw::c_int,
    x: *mut f32,
    p: *mut ::std::os::raw::c_int,
    min_norm: *mut f32,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrlsqvqrHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            rankA,
                            x,
                            p,
                            min_norm,
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
pub unsafe extern "C" fn cusolverSpDcsrlsqvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f64,
    tol: f64,
    rankA: *mut ::std::os::raw::c_int,
    x: *mut f64,
    p: *mut ::std::os::raw::c_int,
    min_norm: *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrlsqvqrHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            rankA,
                            x,
                            p,
                            min_norm,
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
pub unsafe extern "C" fn cusolverSpCcsrlsqvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    tol: f32,
    rankA: *mut ::std::os::raw::c_int,
    x: *mut cuComplex,
    p: *mut ::std::os::raw::c_int,
    min_norm: *mut f32,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrlsqvqrHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            rankA,
                            x,
                            p,
                            min_norm,
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
pub unsafe extern "C" fn cusolverSpZcsrlsqvqrHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    tol: f64,
    rankA: *mut ::std::os::raw::c_int,
    x: *mut cuDoubleComplex,
    p: *mut ::std::os::raw::c_int,
    min_norm: *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrlsqvqrHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            tol,
                            rankA,
                            x,
                            p,
                            min_norm,
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
pub unsafe extern "C" fn cusolverSpScsreigvsiHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: f32,
    x0: *const f32,
    maxite: ::std::os::raw::c_int,
    tol: f32,
    mu: *mut f32,
    x: *mut f32,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsreigvsiHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            mu0,
                            x0,
                            maxite,
                            tol,
                            mu,
                            x,
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
pub unsafe extern "C" fn cusolverSpDcsreigvsiHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: f64,
    x0: *const f64,
    maxite: ::std::os::raw::c_int,
    tol: f64,
    mu: *mut f64,
    x: *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsreigvsiHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            mu0,
                            x0,
                            maxite,
                            tol,
                            mu,
                            x,
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
pub unsafe extern "C" fn cusolverSpCcsreigvsiHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: cuComplex,
    x0: *const cuComplex,
    maxite: ::std::os::raw::c_int,
    tol: f32,
    mu: *mut cuComplex,
    x: *mut cuComplex,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsreigvsiHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            mu0,
                            x0,
                            maxite,
                            tol,
                            mu,
                            x,
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
pub unsafe extern "C" fn cusolverSpZcsreigvsiHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: cuDoubleComplex,
    x0: *const cuDoubleComplex,
    maxite: ::std::os::raw::c_int,
    tol: f64,
    mu: *mut cuDoubleComplex,
    x: *mut cuDoubleComplex,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsreigvsiHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            mu0,
                            x0,
                            maxite,
                            tol,
                            mu,
                            x,
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
pub unsafe extern "C" fn cusolverSpScsreigvsi(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: f32,
    x0: *const f32,
    maxite: ::std::os::raw::c_int,
    eps: f32,
    mu: *mut f32,
    x: *mut f32,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsreigvsi {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            mu0,
                            x0,
                            maxite,
                            eps,
                            mu,
                            x,
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
pub unsafe extern "C" fn cusolverSpDcsreigvsi(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: f64,
    x0: *const f64,
    maxite: ::std::os::raw::c_int,
    eps: f64,
    mu: *mut f64,
    x: *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsreigvsi {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            mu0,
                            x0,
                            maxite,
                            eps,
                            mu,
                            x,
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
pub unsafe extern "C" fn cusolverSpCcsreigvsi(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: cuComplex,
    x0: *const cuComplex,
    maxite: ::std::os::raw::c_int,
    eps: f32,
    mu: *mut cuComplex,
    x: *mut cuComplex,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsreigvsi {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            mu0,
                            x0,
                            maxite,
                            eps,
                            mu,
                            x,
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
pub unsafe extern "C" fn cusolverSpZcsreigvsi(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    mu0: cuDoubleComplex,
    x0: *const cuDoubleComplex,
    maxite: ::std::os::raw::c_int,
    eps: f64,
    mu: *mut cuDoubleComplex,
    x: *mut cuDoubleComplex,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsreigvsi {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            mu0,
                            x0,
                            maxite,
                            eps,
                            mu,
                            x,
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
pub unsafe extern "C" fn cusolverSpScsreigsHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    left_bottom_corner: cuComplex,
    right_upper_corner: cuComplex,
    num_eigs: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsreigsHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            left_bottom_corner,
                            right_upper_corner,
                            num_eigs,
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
pub unsafe extern "C" fn cusolverSpDcsreigsHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    left_bottom_corner: cuDoubleComplex,
    right_upper_corner: cuDoubleComplex,
    num_eigs: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsreigsHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            left_bottom_corner,
                            right_upper_corner,
                            num_eigs,
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
pub unsafe extern "C" fn cusolverSpCcsreigsHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    left_bottom_corner: cuComplex,
    right_upper_corner: cuComplex,
    num_eigs: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsreigsHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            left_bottom_corner,
                            right_upper_corner,
                            num_eigs,
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
pub unsafe extern "C" fn cusolverSpZcsreigsHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    left_bottom_corner: cuDoubleComplex,
    right_upper_corner: cuDoubleComplex,
    num_eigs: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsreigsHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            left_bottom_corner,
                            right_upper_corner,
                            num_eigs,
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
pub unsafe extern "C" fn cusolverSpXcsrsymrcmHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    p: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpXcsrsymrcmHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, n, nnzA, descrA, csrRowPtrA, csrColIndA, p)
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
pub unsafe extern "C" fn cusolverSpXcsrsymmdqHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    p: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpXcsrsymmdqHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, n, nnzA, descrA, csrRowPtrA, csrColIndA, p)
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
pub unsafe extern "C" fn cusolverSpXcsrsymamdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    p: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpXcsrsymamdHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(handle, n, nnzA, descrA, csrRowPtrA, csrColIndA, p)
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
pub unsafe extern "C" fn cusolverSpXcsrmetisndHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    options: *const i64,
    p: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpXcsrmetisndHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnzA,
                            descrA,
                            csrRowPtrA,
                            csrColIndA,
                            options,
                            p,
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
pub unsafe extern "C" fn cusolverSpScsrzfdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    numnz: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrzfdHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            P,
                            numnz,
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
pub unsafe extern "C" fn cusolverSpDcsrzfdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    numnz: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrzfdHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            P,
                            numnz,
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
pub unsafe extern "C" fn cusolverSpCcsrzfdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    numnz: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrzfdHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            P,
                            numnz,
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
pub unsafe extern "C" fn cusolverSpZcsrzfdHost(
    handle: cusolverSpHandle_t,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    P: *mut ::std::os::raw::c_int,
    numnz: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrzfdHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            P,
                            numnz,
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
pub unsafe extern "C" fn cusolverSpXcsrperm_bufferSizeHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    p: *const ::std::os::raw::c_int,
    q: *const ::std::os::raw::c_int,
    bufferSizeInBytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpXcsrperm_bufferSizeHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrRowPtrA,
                            csrColIndA,
                            p,
                            q,
                            bufferSizeInBytes,
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
pub unsafe extern "C" fn cusolverSpXcsrpermHost(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *mut ::std::os::raw::c_int,
    csrColIndA: *mut ::std::os::raw::c_int,
    p: *const ::std::os::raw::c_int,
    q: *const ::std::os::raw::c_int,
    map: *mut ::std::os::raw::c_int,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpXcsrpermHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrRowPtrA,
                            csrColIndA,
                            p,
                            q,
                            map,
                            pBuffer,
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
pub unsafe extern "C" fn cusolverSpCreateCsrqrInfo(
    info: *mut csrqrInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCreateCsrqrInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverSpDestroyCsrqrInfo(
    info: csrqrInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDestroyCsrqrInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(info) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverSpXcsrqrAnalysisBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    info: csrqrInfo_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpXcsrqrAnalysisBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnzA,
                            descrA,
                            csrRowPtrA,
                            csrColIndA,
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
pub unsafe extern "C" fn cusolverSpScsrqrBufferInfoBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f32,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    internalDataInBytes: *mut usize,
    workspaceInBytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrqrBufferInfoBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            batchSize,
                            info,
                            internalDataInBytes,
                            workspaceInBytes,
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
pub unsafe extern "C" fn cusolverSpDcsrqrBufferInfoBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const f64,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    internalDataInBytes: *mut usize,
    workspaceInBytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrqrBufferInfoBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            batchSize,
                            info,
                            internalDataInBytes,
                            workspaceInBytes,
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
pub unsafe extern "C" fn cusolverSpCcsrqrBufferInfoBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    internalDataInBytes: *mut usize,
    workspaceInBytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrqrBufferInfoBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            batchSize,
                            info,
                            internalDataInBytes,
                            workspaceInBytes,
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
pub unsafe extern "C" fn cusolverSpZcsrqrBufferInfoBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrVal: *const cuDoubleComplex,
    csrRowPtr: *const ::std::os::raw::c_int,
    csrColInd: *const ::std::os::raw::c_int,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    internalDataInBytes: *mut usize,
    workspaceInBytes: *mut usize,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrqrBufferInfoBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrVal,
                            csrRowPtr,
                            csrColInd,
                            batchSize,
                            info,
                            internalDataInBytes,
                            workspaceInBytes,
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
pub unsafe extern "C" fn cusolverSpScsrqrsvBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f32,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f32,
    x: *mut f32,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpScsrqrsvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            x,
                            batchSize,
                            info,
                            pBuffer,
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
pub unsafe extern "C" fn cusolverSpDcsrqrsvBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const f64,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const f64,
    x: *mut f64,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpDcsrqrsvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            x,
                            batchSize,
                            info,
                            pBuffer,
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
pub unsafe extern "C" fn cusolverSpCcsrqrsvBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuComplex,
    x: *mut cuComplex,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpCcsrqrsvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            x,
                            batchSize,
                            info,
                            pBuffer,
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
pub unsafe extern "C" fn cusolverSpZcsrqrsvBatched(
    handle: cusolverSpHandle_t,
    m: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnz: ::std::os::raw::c_int,
    descrA: cusparseMatDescr_t,
    csrValA: *const cuDoubleComplex,
    csrRowPtrA: *const ::std::os::raw::c_int,
    csrColIndA: *const ::std::os::raw::c_int,
    b: *const cuDoubleComplex,
    x: *mut cuDoubleComplex,
    batchSize: ::std::os::raw::c_int,
    info: csrqrInfo_t,
    pBuffer: *mut ::std::os::raw::c_void,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverSpZcsrqrsvBatched {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            m,
                            n,
                            nnz,
                            descrA,
                            csrValA,
                            csrRowPtrA,
                            csrColIndA,
                            b,
                            x,
                            batchSize,
                            info,
                            pBuffer,
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
pub unsafe extern "C" fn cusolverRfCreate(
    handle: *mut cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfCreate {
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
pub unsafe extern "C" fn cusolverRfDestroy(
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfDestroy {
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
pub unsafe extern "C" fn cusolverRfGetMatrixFormat(
    handle: cusolverRfHandle_t,
    format: *mut cusolverRfMatrixFormat_t,
    diag: *mut cusolverRfUnitDiagonal_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfGetMatrixFormat {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, format, diag) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfSetMatrixFormat(
    handle: cusolverRfHandle_t,
    format: cusolverRfMatrixFormat_t,
    diag: cusolverRfUnitDiagonal_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfSetMatrixFormat {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, format, diag) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfSetNumericProperties(
    handle: cusolverRfHandle_t,
    zero: f64,
    boost: f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfSetNumericProperties {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, zero, boost) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfGetNumericProperties(
    handle: cusolverRfHandle_t,
    zero: *mut f64,
    boost: *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfGetNumericProperties {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, zero, boost) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfGetNumericBoostReport(
    handle: cusolverRfHandle_t,
    report: *mut cusolverRfNumericBoostReport_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfGetNumericBoostReport {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, report) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfSetAlgs(
    handle: cusolverRfHandle_t,
    factAlg: cusolverRfFactorization_t,
    solveAlg: cusolverRfTriangularSolve_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfSetAlgs {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, factAlg, solveAlg) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfGetAlgs(
    handle: cusolverRfHandle_t,
    factAlg: *mut cusolverRfFactorization_t,
    solveAlg: *mut cusolverRfTriangularSolve_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfGetAlgs {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, factAlg, solveAlg) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfGetResetValuesFastMode(
    handle: cusolverRfHandle_t,
    fastMode: *mut cusolverRfResetValuesFastMode_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfGetResetValuesFastMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, fastMode) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfSetResetValuesFastMode(
    handle: cusolverRfHandle_t,
    fastMode: cusolverRfResetValuesFastMode_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfSetResetValuesFastMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, fastMode) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfSetupHost(
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    h_csrRowPtrA: *mut ::std::os::raw::c_int,
    h_csrColIndA: *mut ::std::os::raw::c_int,
    h_csrValA: *mut f64,
    nnzL: ::std::os::raw::c_int,
    h_csrRowPtrL: *mut ::std::os::raw::c_int,
    h_csrColIndL: *mut ::std::os::raw::c_int,
    h_csrValL: *mut f64,
    nnzU: ::std::os::raw::c_int,
    h_csrRowPtrU: *mut ::std::os::raw::c_int,
    h_csrColIndU: *mut ::std::os::raw::c_int,
    h_csrValU: *mut f64,
    h_P: *mut ::std::os::raw::c_int,
    h_Q: *mut ::std::os::raw::c_int,
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfSetupHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            n,
                            nnzA,
                            h_csrRowPtrA,
                            h_csrColIndA,
                            h_csrValA,
                            nnzL,
                            h_csrRowPtrL,
                            h_csrColIndL,
                            h_csrValL,
                            nnzU,
                            h_csrRowPtrU,
                            h_csrColIndU,
                            h_csrValU,
                            h_P,
                            h_Q,
                            handle,
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
pub unsafe extern "C" fn cusolverRfSetupDevice(
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    csrRowPtrA: *mut ::std::os::raw::c_int,
    csrColIndA: *mut ::std::os::raw::c_int,
    csrValA: *mut f64,
    nnzL: ::std::os::raw::c_int,
    csrRowPtrL: *mut ::std::os::raw::c_int,
    csrColIndL: *mut ::std::os::raw::c_int,
    csrValL: *mut f64,
    nnzU: ::std::os::raw::c_int,
    csrRowPtrU: *mut ::std::os::raw::c_int,
    csrColIndU: *mut ::std::os::raw::c_int,
    csrValU: *mut f64,
    P: *mut ::std::os::raw::c_int,
    Q: *mut ::std::os::raw::c_int,
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfSetupDevice {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            n,
                            nnzA,
                            csrRowPtrA,
                            csrColIndA,
                            csrValA,
                            nnzL,
                            csrRowPtrL,
                            csrColIndL,
                            csrValL,
                            nnzU,
                            csrRowPtrU,
                            csrColIndU,
                            csrValU,
                            P,
                            Q,
                            handle,
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
pub unsafe extern "C" fn cusolverRfResetValues(
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    csrRowPtrA: *mut ::std::os::raw::c_int,
    csrColIndA: *mut ::std::os::raw::c_int,
    csrValA: *mut f64,
    P: *mut ::std::os::raw::c_int,
    Q: *mut ::std::os::raw::c_int,
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfResetValues {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            n,
                            nnzA,
                            csrRowPtrA,
                            csrColIndA,
                            csrValA,
                            P,
                            Q,
                            handle,
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
pub unsafe extern "C" fn cusolverRfAnalyze(
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfAnalyze {
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
pub unsafe extern "C" fn cusolverRfRefactor(
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfRefactor {
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
pub unsafe extern "C" fn cusolverRfAccessBundledFactorsDevice(
    handle: cusolverRfHandle_t,
    nnzM: *mut ::std::os::raw::c_int,
    Mp: *mut *mut ::std::os::raw::c_int,
    Mi: *mut *mut ::std::os::raw::c_int,
    Mx: *mut *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfAccessBundledFactorsDevice {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, nnzM, Mp, Mi, Mx) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfExtractBundledFactorsHost(
    handle: cusolverRfHandle_t,
    h_nnzM: *mut ::std::os::raw::c_int,
    h_Mp: *mut *mut ::std::os::raw::c_int,
    h_Mi: *mut *mut ::std::os::raw::c_int,
    h_Mx: *mut *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfExtractBundledFactorsHost {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, h_nnzM, h_Mp, h_Mi, h_Mx) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfExtractSplitFactorsHost(
    handle: cusolverRfHandle_t,
    h_nnzL: *mut ::std::os::raw::c_int,
    h_csrRowPtrL: *mut *mut ::std::os::raw::c_int,
    h_csrColIndL: *mut *mut ::std::os::raw::c_int,
    h_csrValL: *mut *mut f64,
    h_nnzU: *mut ::std::os::raw::c_int,
    h_csrRowPtrU: *mut *mut ::std::os::raw::c_int,
    h_csrColIndU: *mut *mut ::std::os::raw::c_int,
    h_csrValU: *mut *mut f64,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfExtractSplitFactorsHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            handle,
                            h_nnzL,
                            h_csrRowPtrL,
                            h_csrColIndL,
                            h_csrValL,
                            h_nnzU,
                            h_csrRowPtrU,
                            h_csrColIndU,
                            h_csrValU,
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
pub unsafe extern "C" fn cusolverRfSolve(
    handle: cusolverRfHandle_t,
    P: *mut ::std::os::raw::c_int,
    Q: *mut ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Temp: *mut f64,
    ldt: ::std::os::raw::c_int,
    XF: *mut f64,
    ldxf: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfSolve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, P, Q, nrhs, Temp, ldt, XF, ldxf) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfBatchSetupHost(
    batchSize: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    h_csrRowPtrA: *mut ::std::os::raw::c_int,
    h_csrColIndA: *mut ::std::os::raw::c_int,
    h_csrValA_array: *mut *mut f64,
    nnzL: ::std::os::raw::c_int,
    h_csrRowPtrL: *mut ::std::os::raw::c_int,
    h_csrColIndL: *mut ::std::os::raw::c_int,
    h_csrValL: *mut f64,
    nnzU: ::std::os::raw::c_int,
    h_csrRowPtrU: *mut ::std::os::raw::c_int,
    h_csrColIndU: *mut ::std::os::raw::c_int,
    h_csrValU: *mut f64,
    h_P: *mut ::std::os::raw::c_int,
    h_Q: *mut ::std::os::raw::c_int,
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfBatchSetupHost {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            batchSize,
                            n,
                            nnzA,
                            h_csrRowPtrA,
                            h_csrColIndA,
                            h_csrValA_array,
                            nnzL,
                            h_csrRowPtrL,
                            h_csrColIndL,
                            h_csrValL,
                            nnzU,
                            h_csrRowPtrU,
                            h_csrColIndU,
                            h_csrValU,
                            h_P,
                            h_Q,
                            handle,
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
pub unsafe extern "C" fn cusolverRfBatchResetValues(
    batchSize: ::std::os::raw::c_int,
    n: ::std::os::raw::c_int,
    nnzA: ::std::os::raw::c_int,
    csrRowPtrA: *mut ::std::os::raw::c_int,
    csrColIndA: *mut ::std::os::raw::c_int,
    csrValA_array: *mut *mut f64,
    P: *mut ::std::os::raw::c_int,
    Q: *mut ::std::os::raw::c_int,
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfBatchResetValues {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            batchSize,
                            n,
                            nnzA,
                            csrRowPtrA,
                            csrColIndA,
                            csrValA_array,
                            P,
                            Q,
                            handle,
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
pub unsafe extern "C" fn cusolverRfBatchAnalyze(
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfBatchAnalyze {
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
pub unsafe extern "C" fn cusolverRfBatchRefactor(
    handle: cusolverRfHandle_t,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfBatchRefactor {
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
pub unsafe extern "C" fn cusolverRfBatchSolve(
    handle: cusolverRfHandle_t,
    P: *mut ::std::os::raw::c_int,
    Q: *mut ::std::os::raw::c_int,
    nrhs: ::std::os::raw::c_int,
    Temp: *mut f64,
    ldt: ::std::os::raw::c_int,
    XF_array: *mut *mut f64,
    ldxf: ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfBatchSolve {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(handle, P, Q, nrhs, Temp, ldt, XF_array, ldxf) }
                }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cusolverRfBatchZeroPivot(
    handle: cusolverRfHandle_t,
    position: *mut ::std::os::raw::c_int,
) -> cusolverStatus_t {
    match cusolver_api() {
        Ok(__nv_api) => {
            match &__nv_api.cusolverRfBatchZeroPivot {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(handle, position) }
                Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => cusolverStatus_t_CUSOLVER_STATUS_NOT_INITIALIZED,
    }
}
