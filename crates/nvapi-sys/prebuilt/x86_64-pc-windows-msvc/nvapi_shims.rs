#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetDriverAndBranchVersion(
    pDriverVersion: *mut NvU32,
    szBuildBranchString: *mut ::std::os::raw::c_char,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(690399917u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pDriverVersion: *mut NvU32,
                            szBuildBranchString: *mut ::std::os::raw::c_char,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pDriverVersion, szBuildBranchString) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetMemoryInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pMemoryInfo: *mut NV_DISPLAY_DRIVER_MEMORY_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(133804904u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pMemoryInfo: *mut NV_DISPLAY_DRIVER_MEMORY_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pMemoryInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetMemoryInfoEx(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pMemoryInfo: *mut NV_GPU_MEMORY_INFO_EX,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3227096216u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pMemoryInfo: *mut NV_GPU_MEMORY_INFO_EX,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pMemoryInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_EnumPhysicalGPUs(
    nvGPUHandle: *mut NvPhysicalGpuHandle,
    pGpuCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3853292063u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            nvGPUHandle: *mut NvPhysicalGpuHandle,
                            pGpuCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(nvGPUHandle, pGpuCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetGDIPrimaryDisplayId(
    displayId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(513641009u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(displayId: *mut NvU32) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_GetDisplayViewportsByResolution(
    displayId: NvU32,
    srcWidth: NvU32,
    srcHeight: NvU32,
    viewports: *mut NV_RECT,
    bezelCorrected: *mut NvU8,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3698182355u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            srcWidth: NvU32,
                            srcHeight: NvU32,
                            viewports: *mut NV_RECT,
                            bezelCorrected: *mut NvU8,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(displayId, srcWidth, srcHeight, viewports, bezelCorrected)
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_Enable() -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(597443909u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn() -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn() }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_Disable() -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(784665643u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn() -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn() }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_IsEnabled(
    pIsStereoEnabled: *mut NvU8,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(881850593u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(pIsStereoEnabled: *mut NvU8) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pIsStereoEnabled) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_DestroyHandle(
    stereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(974467380u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(stereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_Activate(
    stereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4137790824u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(stereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_Deactivate(
    stereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(761847446u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(stereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_IsActivated(
    stereoHandle: StereoHandle,
    pIsStereoOn: *mut NvU8,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(531676208u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            stereoHandle: StereoHandle,
                            pIsStereoOn: *mut NvU8,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle, pIsStereoOn) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_GetSeparation(
    stereoHandle: StereoHandle,
    pSeparationPercentage: *mut f32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1159668020u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            stereoHandle: StereoHandle,
                            pSeparationPercentage: *mut f32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle, pSeparationPercentage) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetSeparation(
    stereoHandle: StereoHandle,
    newSeparationPercentage: f32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1543937955u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            stereoHandle: StereoHandle,
                            newSeparationPercentage: f32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle, newSeparationPercentage) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_GetConvergence(
    stereoHandle: StereoHandle,
    pConvergence: *mut f32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1253050676u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            stereoHandle: StereoHandle,
                            pConvergence: *mut f32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle, pConvergence) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetConvergence(
    stereoHandle: StereoHandle,
    newConvergence: f32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1037481291u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            stereoHandle: StereoHandle,
                            newConvergence: f32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle, newConvergence) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetActiveEye(
    hStereoHandle: StereoHandle,
    StereoEye: NV_STEREO_ACTIVE_EYE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2532223480u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hStereoHandle: StereoHandle,
                            StereoEye: NV_STEREO_ACTIVE_EYE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle, StereoEye) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetDriverMode(
    mode: NV_STEREO_DRIVER_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1586433004u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(mode: NV_STEREO_DRIVER_MODE) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(mode) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_GetEyeSeparation(
    hStereoHandle: StereoHandle,
    pSeparation: *mut f32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3462738215u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hStereoHandle: StereoHandle,
                            pSeparation: *mut f32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle, pSeparation) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_IsWindowedModeSupported(
    bSupported: *mut NvU8,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1086909790u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(bSupported: *mut NvU8) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(bSupported) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetSurfaceCreationMode(
    hStereoHandle: StereoHandle,
    creationMode: NVAPI_STEREO_SURFACECREATEMODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4124900538u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hStereoHandle: StereoHandle,
                            creationMode: NVAPI_STEREO_SURFACECREATEMODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle, creationMode) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_GetSurfaceCreationMode(
    hStereoHandle: StereoHandle,
    pCreationMode: *mut NVAPI_STEREO_SURFACECREATEMODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(921814838u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hStereoHandle: StereoHandle,
                            pCreationMode: *mut NVAPI_STEREO_SURFACECREATEMODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle, pCreationMode) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_Debug_WasLastDrawStereoized(
    hStereoHandle: StereoHandle,
    pWasStereoized: *mut NvU8,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3980662469u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hStereoHandle: StereoHandle,
                            pWasStereoized: *mut NvU8,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle, pWasStereoized) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetDefaultProfile(
    szProfileName: *const ::std::os::raw::c_char,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1156639953u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            szProfileName: *const ::std::os::raw::c_char,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(szProfileName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_GetDefaultProfile(
    cbSizeIn: NvU32,
    szProfileName: *mut ::std::os::raw::c_char,
    pcbSizeOut: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1649287618u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            cbSizeIn: NvU32,
                            szProfileName: *mut ::std::os::raw::c_char,
                            pcbSizeOut: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(cbSizeIn, szProfileName, pcbSizeOut) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Initialize() -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(22079528u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn() -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn() }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Unload() -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3526090110u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn() -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn() }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetErrorMessage(
    nr: NvAPI_Status,
    szDesc: *mut ::std::os::raw::c_char,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1814889612u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            nr: NvAPI_Status,
                            szDesc: *mut ::std::os::raw::c_char,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(nr, szDesc) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetInterfaceVersionString(
    szDesc: *mut ::std::os::raw::c_char,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(17121189u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            szDesc: *mut ::std::os::raw::c_char,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(szDesc) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetEDID(
    hPhysicalGpu: NvPhysicalGpuHandle,
    displayOutputId: NvU32,
    pEDID: *mut NV_EDID,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(936586857u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            displayOutputId: NvU32,
                            pEDID: *mut NV_EDID,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, displayOutputId, pEDID) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SetView(
    hNvDisplay: NvDisplayHandle,
    pTargetInfo: *mut NV_VIEW_TARGET_INFO,
    targetView: NV_TARGET_VIEW_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(156751798u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            pTargetInfo: *mut NV_VIEW_TARGET_INFO,
                            targetView: NV_TARGET_VIEW_MODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, pTargetInfo, targetView) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SetViewEx(
    hNvDisplay: NvDisplayHandle,
    pPathInfo: *mut NV_DISPLAY_PATH_INFO,
    displayView: NV_TARGET_VIEW_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(112762472u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            pPathInfo: *mut NV_DISPLAY_PATH_INFO,
                            displayView: NV_TARGET_VIEW_MODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, pPathInfo, displayView) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetDisplayDriverVersion(
    hNvDisplay: NvDisplayHandle,
    pVersion: *mut NV_DISPLAY_DRIVER_VERSION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4182877393u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            pVersion: *mut NV_DISPLAY_DRIVER_VERSION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, pVersion) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_OGL_ExpertModeSet(
    expertDetailLevel: NvU32,
    expertReportMask: NvU32,
    expertOutputMask: NvU32,
    expertCallback: NVAPI_OGLEXPERT_CALLBACK,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(939913082u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            expertDetailLevel: NvU32,
                            expertReportMask: NvU32,
                            expertOutputMask: NvU32,
                            expertCallback: NVAPI_OGLEXPERT_CALLBACK,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(
                        expertDetailLevel,
                        expertReportMask,
                        expertOutputMask,
                        expertCallback,
                    )
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_OGL_ExpertModeGet(
    pExpertDetailLevel: *mut NvU32,
    pExpertReportMask: *mut NvU32,
    pExpertOutputMask: *mut NvU32,
    pExpertCallback: *mut NVAPI_OGLEXPERT_CALLBACK,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(585995542u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pExpertDetailLevel: *mut NvU32,
                            pExpertReportMask: *mut NvU32,
                            pExpertOutputMask: *mut NvU32,
                            pExpertCallback: *mut NVAPI_OGLEXPERT_CALLBACK,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(
                        pExpertDetailLevel,
                        pExpertReportMask,
                        pExpertOutputMask,
                        pExpertCallback,
                    )
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_OGL_ExpertModeDefaultsSet(
    expertDetailLevel: NvU32,
    expertReportMask: NvU32,
    expertOutputMask: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3027920254u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            expertDetailLevel: NvU32,
                            expertReportMask: NvU32,
                            expertOutputMask: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(expertDetailLevel, expertReportMask, expertOutputMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_OGL_ExpertModeDefaultsGet(
    pExpertDetailLevel: *mut NvU32,
    pExpertReportMask: *mut NvU32,
    pExpertOutputMask: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2928811794u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pExpertDetailLevel: *mut NvU32,
                            pExpertReportMask: *mut NvU32,
                            pExpertOutputMask: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(pExpertDetailLevel, pExpertReportMask, pExpertOutputMask)
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_EnumTCCPhysicalGPUs(
    nvGPUHandle: *mut NvPhysicalGpuHandle,
    pGpuCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3650292487u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            nvGPUHandle: *mut NvPhysicalGpuHandle,
                            pGpuCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(nvGPUHandle, pGpuCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_EnumLogicalGPUs(
    nvGPUHandle: *mut NvLogicalGpuHandle,
    pGpuCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1219750489u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            nvGPUHandle: *mut NvLogicalGpuHandle,
                            pGpuCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(nvGPUHandle, pGpuCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetPhysicalGPUsFromDisplay(
    hNvDisp: NvDisplayHandle,
    nvGPUHandle: *mut NvPhysicalGpuHandle,
    pGpuCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(888116486u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisp: NvDisplayHandle,
                            nvGPUHandle: *mut NvPhysicalGpuHandle,
                            pGpuCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisp, nvGPUHandle, pGpuCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetPhysicalGPUFromUnAttachedDisplay(
    hNvUnAttachedDisp: NvUnAttachedDisplayHandle,
    pPhysicalGpu: *mut NvPhysicalGpuHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1343810913u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvUnAttachedDisp: NvUnAttachedDisplayHandle,
                            pPhysicalGpu: *mut NvPhysicalGpuHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvUnAttachedDisp, pPhysicalGpu) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetLogicalGPUFromDisplay(
    hNvDisp: NvDisplayHandle,
    pLogicalGPU: *mut NvLogicalGpuHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3994251471u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisp: NvDisplayHandle,
                            pLogicalGPU: *mut NvLogicalGpuHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisp, pLogicalGPU) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetLogicalGPUFromPhysicalGPU(
    hPhysicalGPU: NvPhysicalGpuHandle,
    pLogicalGPU: *mut NvLogicalGpuHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2916484305u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGPU: NvPhysicalGpuHandle,
                            pLogicalGPU: *mut NvLogicalGpuHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGPU, pLogicalGPU) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetPhysicalGPUsFromLogicalGPU(
    hLogicalGPU: NvLogicalGpuHandle,
    hPhysicalGPU: *mut NvPhysicalGpuHandle,
    pGpuCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2929982002u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hLogicalGPU: NvLogicalGpuHandle,
                            hPhysicalGPU: *mut NvPhysicalGpuHandle,
                            pGpuCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hLogicalGPU, hPhysicalGPU, pGpuCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetPhysicalGPUFromGPUID(
    gpuId: NvU32,
    pPhysicalGPU: *mut NvPhysicalGpuHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1400941850u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            gpuId: NvU32,
                            pPhysicalGPU: *mut NvPhysicalGpuHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(gpuId, pPhysicalGPU) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetGPUIDfromPhysicalGPU(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pGpuId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1697901118u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pGpuId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pGpuId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetShaderSubPipeCount(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(199325987u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetGpuCoreCount(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3338824327u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetAllOutputs(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pOutputsMask: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2102742926u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pOutputsMask: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pOutputsMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetConnectedOutputs(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pOutputsMask: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(389070793u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pOutputsMask: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pOutputsMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetConnectedSLIOutputs(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pOutputsMask: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(109108745u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pOutputsMask: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pOutputsMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetConnectedDisplayIds(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pDisplayIds: *mut NV_GPU_DISPLAYIDS,
    pDisplayIdCount: *mut NvU32,
    flags: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(7920546u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pDisplayIds: *mut NV_GPU_DISPLAYIDS,
                            pDisplayIdCount: *mut NvU32,
                            flags: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pDisplayIds, pDisplayIdCount, flags) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetAllDisplayIds(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pDisplayIds: *mut NV_GPU_DISPLAYIDS,
    pDisplayIdCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2018644130u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pDisplayIds: *mut NV_GPU_DISPLAYIDS,
                            pDisplayIdCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pDisplayIds, pDisplayIdCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetConnectedOutputsWithLidState(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pOutputsMask: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3482103609u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pOutputsMask: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pOutputsMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetConnectedSLIOutputsWithLidState(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pOutputsMask: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2516860103u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pOutputsMask: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pOutputsMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetSystemType(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pSystemType: *mut NV_SYSTEM_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3131752396u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pSystemType: *mut NV_SYSTEM_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pSystemType) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetActiveOutputs(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pOutputsMask: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3823672175u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pOutputsMask: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pOutputsMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_SetEDID(
    hPhysicalGpu: NvPhysicalGpuHandle,
    displayOutputId: NvU32,
    pEDID: *mut NV_EDID,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3896337494u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            displayOutputId: NvU32,
                            pEDID: *mut NV_EDID,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, displayOutputId, pEDID) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetOutputType(
    hPhysicalGpu: NvPhysicalGpuHandle,
    outputId: NvU32,
    pOutputType: *mut NV_GPU_OUTPUT_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1084556772u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            outputId: NvU32,
                            pOutputType: *mut NV_GPU_OUTPUT_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, outputId, pOutputType) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ValidateOutputCombination(
    hPhysicalGpu: NvPhysicalGpuHandle,
    outputsMask: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(885637844u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            outputsMask: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, outputsMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetFullName(
    hPhysicalGpu: NvPhysicalGpuHandle,
    szName: *mut ::std::os::raw::c_char,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3471740575u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            szName: *mut ::std::os::raw::c_char,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, szName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetPCIIdentifiers(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pDeviceId: *mut NvU32,
    pSubSystemId: *mut NvU32,
    pRevisionId: *mut NvU32,
    pExtDeviceId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(769635950u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pDeviceId: *mut NvU32,
                            pSubSystemId: *mut NvU32,
                            pRevisionId: *mut NvU32,
                            pExtDeviceId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(
                        hPhysicalGpu,
                        pDeviceId,
                        pSubSystemId,
                        pRevisionId,
                        pExtDeviceId,
                    )
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetGPUType(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pGpuType: *mut NV_GPU_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3275468465u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pGpuType: *mut NV_GPU_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pGpuType) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetBusType(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pBusType: *mut NV_GPU_BUS_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(464619300u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pBusType: *mut NV_GPU_BUS_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pBusType) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetBusId(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pBusId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(467712229u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pBusId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pBusId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetBusSlotId(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pBusSlotId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(705312015u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pBusSlotId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pBusSlotId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetIRQ(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pIRQ: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3832632343u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pIRQ: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pIRQ) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetVbiosRevision(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pBiosRevision: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2898516490u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pBiosRevision: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pBiosRevision) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetVbiosOEMRevision(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pBiosRevision: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(759429937u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pBiosRevision: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pBiosRevision) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetVbiosVersionString(
    hPhysicalGpu: NvPhysicalGpuHandle,
    szBiosRevision: *mut ::std::os::raw::c_char,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2774662525u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            szBiosRevision: *mut ::std::os::raw::c_char,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, szBiosRevision) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetAGPAperture(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pSize: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1845766036u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pSize: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pSize) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetCurrentAGPRate(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pRate: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3343459744u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pRate: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pRate) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetCurrentPCIEDownstreamWidth(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pWidth: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3494429617u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pWidth: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pWidth) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetPhysicalFrameBufferSize(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pSize: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1190914819u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pSize: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pSize) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetVirtualFrameBufferSize(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pSize: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1510258244u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pSize: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pSize) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetQuadroStatus(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pStatus: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3811768903u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pStatus: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pStatus) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetBoardInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pBoardInfo: *mut NV_BOARD_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(584402211u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pBoardInfo: *mut NV_BOARD_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pBoardInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetRamBusWidth(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pBusWidth: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2037761409u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pBusWidth: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pBusWidth) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetArchInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pGpuArchInfo: *mut NV_GPU_ARCH_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3626392868u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pGpuArchInfo: *mut NV_GPU_ARCH_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pGpuArchInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_I2CRead(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pI2cInfo: *mut NV_I2C_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(803082949u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pI2cInfo: *mut NV_I2C_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pI2cInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_I2CWrite(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pI2cInfo: *mut NV_I2C_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3893553927u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pI2cInfo: *mut NV_I2C_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pI2cInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_WorkstationFeatureSetup(
    hPhysicalGpu: NvPhysicalGpuHandle,
    featureEnableMask: NvU32,
    featureDisableMask: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1813987300u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            featureEnableMask: NvU32,
                            featureDisableMask: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, featureEnableMask, featureDisableMask) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_WorkstationFeatureQuery(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pConfiguredFeatureMask: *mut NvU32,
    pConsistentFeatureMask: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4536287u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pConfiguredFeatureMask: *mut NvU32,
                            pConsistentFeatureMask: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(hPhysicalGpu, pConfiguredFeatureMask, pConsistentFeatureMask)
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetHDCPSupportStatus(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pGetHDCPSupportStatus: *mut NV_GPU_GET_HDCP_SUPPORT_STATUS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4035571445u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pGetHDCPSupportStatus: *mut NV_GPU_GET_HDCP_SUPPORT_STATUS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pGetHDCPSupportStatus) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_CudaEnumComputeCapableGpus(
    pComputeTopo: *mut NV_COMPUTE_GPU_TOPOLOGY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1468451950u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pComputeTopo: *mut NV_COMPUTE_GPU_TOPOLOGY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pComputeTopo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetTachReading(
    hPhysicalGPU: NvPhysicalGpuHandle,
    pValue: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1600160533u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGPU: NvPhysicalGpuHandle,
                            pValue: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGPU, pValue) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetECCStatusInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pECCStatusInfo: *mut NV_GPU_ECC_STATUS_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3390954227u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pECCStatusInfo: *mut NV_GPU_ECC_STATUS_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pECCStatusInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetECCErrorInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pECCErrorInfo: *mut NV_GPU_ECC_ERROR_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3340731814u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pECCErrorInfo: *mut NV_GPU_ECC_ERROR_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pECCErrorInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ResetECCErrorInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    bResetCurrent: NvU8,
    bResetAggregate: NvU8,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3224300576u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            bResetCurrent: NvU8,
                            bResetAggregate: NvU8,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, bResetCurrent, bResetAggregate) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetECCConfigurationInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pECCConfigurationInfo: *mut NV_GPU_ECC_CONFIGURATION_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2007471859u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pECCConfigurationInfo: *mut NV_GPU_ECC_CONFIGURATION_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pECCConfigurationInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_SetECCConfiguration(
    hPhysicalGpu: NvPhysicalGpuHandle,
    bEnable: NvU8,
    bEnableImmediately: NvU8,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(485898713u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            bEnable: NvU8,
                            bEnableImmediately: NvU8,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, bEnable, bEnableImmediately) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_QueryWorkstationFeatureSupport(
    physicalGpu: NvPhysicalGpuHandle,
    gpuWorkstationFeature: NV_GPU_WORKSTATION_FEATURE_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2159127481u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            physicalGpu: NvPhysicalGpuHandle,
                            gpuWorkstationFeature: NV_GPU_WORKSTATION_FEATURE_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(physicalGpu, gpuWorkstationFeature) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_SetScanoutIntensity(
    displayId: NvU32,
    scanoutIntensityData: *mut NV_SCANOUT_INTENSITY_DATA,
    pbSticky: *mut ::std::os::raw::c_int,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2775865252u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            scanoutIntensityData: *mut NV_SCANOUT_INTENSITY_DATA,
                            pbSticky: *mut ::std::os::raw::c_int,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, scanoutIntensityData, pbSticky) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetScanoutIntensityState(
    displayId: NvU32,
    scanoutIntensityStateData: *mut NV_SCANOUT_INTENSITY_STATE_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3894208566u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            scanoutIntensityStateData: *mut NV_SCANOUT_INTENSITY_STATE_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, scanoutIntensityStateData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_SetScanoutWarping(
    displayId: NvU32,
    scanoutWarpingData: *mut NV_SCANOUT_WARPING_DATA,
    piMaxNumVertices: *mut ::std::os::raw::c_int,
    pbSticky: *mut ::std::os::raw::c_int,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3008080719u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            scanoutWarpingData: *mut NV_SCANOUT_WARPING_DATA,
                            piMaxNumVertices: *mut ::std::os::raw::c_int,
                            pbSticky: *mut ::std::os::raw::c_int,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(displayId, scanoutWarpingData, piMaxNumVertices, pbSticky)
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetScanoutWarpingState(
    displayId: NvU32,
    scanoutWarpingStateData: *mut NV_SCANOUT_WARPING_STATE_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1867789743u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            scanoutWarpingStateData: *mut NV_SCANOUT_WARPING_STATE_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, scanoutWarpingStateData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_SetScanoutCompositionParameter(
    displayId: NvU32,
    parameter: NV_GPU_SCANOUT_COMPOSITION_PARAMETER,
    parameterValue: NV_GPU_SCANOUT_COMPOSITION_PARAMETER_VALUE,
    pContainer: *mut f32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4170720381u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            parameter: NV_GPU_SCANOUT_COMPOSITION_PARAMETER,
                            parameterValue: NV_GPU_SCANOUT_COMPOSITION_PARAMETER_VALUE,
                            pContainer: *mut f32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, parameter, parameterValue, pContainer) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetScanoutCompositionParameter(
    displayId: NvU32,
    parameter: NV_GPU_SCANOUT_COMPOSITION_PARAMETER,
    parameterData: *mut NV_GPU_SCANOUT_COMPOSITION_PARAMETER_VALUE,
    pContainer: *mut f32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1493062118u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            parameter: NV_GPU_SCANOUT_COMPOSITION_PARAMETER,
                            parameterData: *mut NV_GPU_SCANOUT_COMPOSITION_PARAMETER_VALUE,
                            pContainer: *mut f32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, parameter, parameterData, pContainer) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetScanoutConfiguration(
    displayId: NvU32,
    desktopRect: *mut NvSBox,
    scanoutRect: *mut NvSBox,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1788828515u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            desktopRect: *mut NvSBox,
                            scanoutRect: *mut NvSBox,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, desktopRect, scanoutRect) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetScanoutConfigurationEx(
    displayId: NvU32,
    pScanoutInformation: *mut NV_SCANOUT_INFORMATION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3806455536u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pScanoutInformation: *mut NV_SCANOUT_INFORMATION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pScanoutInformation) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetAdapterIdFromPhysicalGpu(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pOSAdapterId: *mut ::std::os::raw::c_void,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(267419614u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pOSAdapterId: *mut ::std::os::raw::c_void,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pOSAdapterId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetVirtualizationInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pVirtualizationInfo: *mut NV_GPU_VIRTUALIZATION_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1155539625u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pVirtualizationInfo: *mut NV_GPU_VIRTUALIZATION_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pVirtualizationInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetLogicalGpuInfo(
    hLogicalGpu: NvLogicalGpuHandle,
    pLogicalGpuData: *mut NV_LOGICAL_GPU_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2217412206u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hLogicalGpu: NvLogicalGpuHandle,
                            pLogicalGpuData: *mut NV_LOGICAL_GPU_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hLogicalGpu, pLogicalGpuData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetLicensableFeatures(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pLicensableFeatures: *mut NV_LICENSABLE_FEATURES,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1069913770u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pLicensableFeatures: *mut NV_LICENSABLE_FEATURES,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pLicensableFeatures) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetEncoderStatistics(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pEncoderStatistics: *mut NV_ENCODER_STATISTICS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4037652203u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pEncoderStatistics: *mut NV_ENCODER_STATISTICS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pEncoderStatistics) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetEncoderSessionsInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pEncoderSessionsInfo: *mut NV_ENCODER_SESSIONS_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3634834661u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pEncoderSessionsInfo: *mut NV_ENCODER_SESSIONS_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pEncoderSessionsInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetGPUInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pGpuInfo: *mut NV_GPU_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2949754924u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pGpuInfo: *mut NV_GPU_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pGpuInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetVRReadyData(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pGpuVrReadyData: *mut NV_GPU_VR_READY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2178296261u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pGpuVrReadyData: *mut NV_GPU_VR_READY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pGpuVrReadyData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetGspFeatures(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pGspInfo: *mut NV_GPU_GSP_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1478247313u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pGspInfo: *mut NV_GPU_GSP_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pGspInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetUUID(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pGpuUuid: *mut NV_GPU_UUID,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3700778813u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pGpuUuid: *mut NV_GPU_UUID,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pGpuUuid) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_NVLINK_GetCaps(
    hPhysicalGpu: NvPhysicalGpuHandle,
    capsParams: *mut NVLINK_GET_CAPS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3203469725u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            capsParams: *mut NVLINK_GET_CAPS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, capsParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_NVLINK_GetCapsEx(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pCapsEx: *mut NVLINK_GET_CAPS_EX,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2928381443u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pCapsEx: *mut NVLINK_GET_CAPS_EX,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pCapsEx) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_NVLINK_GetStatus(
    hPhysicalGpu: NvPhysicalGpuHandle,
    statusParams: *mut NVLINK_GET_STATUS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3341433059u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            statusParams: *mut NVLINK_GET_STATUS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, statusParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_NVLINK_GetStatusEx(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pStatusEx: *mut NVLINK_GET_STATUS_EX,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2495630716u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pStatusEx: *mut NVLINK_GET_STATUS_EX,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pStatusEx) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetPerfDecreaseInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pPerfDecrInfo: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2139047424u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pPerfDecrInfo: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pPerfDecrInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetPstatesInfoEx(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pPerfPstatesInfo: *mut NV_GPU_PERF_PSTATES_INFO,
    inputFlags: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2218525270u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pPerfPstatesInfo: *mut NV_GPU_PERF_PSTATES_INFO,
                            inputFlags: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pPerfPstatesInfo, inputFlags) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetPstates20(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pPstatesInfo: *mut NV_GPU_PERF_PSTATES20_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1878528531u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pPstatesInfo: *mut NV_GPU_PERF_PSTATES20_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pPstatesInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetCurrentPstate(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pCurrentPstate: *mut NV_GPU_PERF_PSTATE_ID,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2457707766u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pCurrentPstate: *mut NV_GPU_PERF_PSTATE_ID,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pCurrentPstate) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetDynamicPstatesInfoEx(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pDynamicPstatesInfoEx: *mut NV_GPU_DYNAMIC_PSTATES_INFO_EX,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1625215725u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pDynamicPstatesInfoEx: *mut NV_GPU_DYNAMIC_PSTATES_INFO_EX,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pDynamicPstatesInfoEx) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetThermalSettings(
    hPhysicalGpu: NvPhysicalGpuHandle,
    sensorIndex: NvU32,
    pThermalSettings: *mut NV_GPU_THERMAL_SETTINGS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3814984278u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            sensorIndex: NvU32,
                            pThermalSettings: *mut NV_GPU_THERMAL_SETTINGS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, sensorIndex, pThermalSettings) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetAllClockFrequencies(
    hPhysicalGPU: NvPhysicalGpuHandle,
    pClkFreqs: *mut NV_GPU_CLOCK_FREQUENCIES,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3702920899u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGPU: NvPhysicalGpuHandle,
                            pClkFreqs: *mut NV_GPU_CLOCK_FREQUENCIES,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGPU, pClkFreqs) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetOverclockStatus(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pOverclockStatus: PNV_GPU_OVERCLOCK_STATUS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1243571986u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pOverclockStatus: PNV_GPU_OVERCLOCK_STATUS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pOverclockStatus) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_QueryIlluminationSupport(
    pIlluminationSupportInfo: *mut NV_GPU_QUERY_ILLUMINATION_SUPPORT_PARM,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2787760689u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pIlluminationSupportInfo: *mut NV_GPU_QUERY_ILLUMINATION_SUPPORT_PARM,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pIlluminationSupportInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_GetIllumination(
    pIlluminationInfo: *mut NV_GPU_GET_ILLUMINATION_PARM,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2585498469u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pIlluminationInfo: *mut NV_GPU_GET_ILLUMINATION_PARM,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pIlluminationInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_SetIllumination(
    pIlluminationInfo: *mut NV_GPU_SET_ILLUMINATION_PARM,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(39100807u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pIlluminationInfo: *mut NV_GPU_SET_ILLUMINATION_PARM,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pIlluminationInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ClientIllumDevicesGetInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pIllumDevicesInfo: *mut NV_GPU_CLIENT_ILLUM_DEVICE_INFO_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3557822040u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pIllumDevicesInfo: *mut NV_GPU_CLIENT_ILLUM_DEVICE_INFO_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pIllumDevicesInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ClientIllumDevicesGetControl(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pClientIllumDevicesControl: *mut NV_GPU_CLIENT_ILLUM_DEVICE_CONTROL_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1941970264u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pClientIllumDevicesControl: *mut NV_GPU_CLIENT_ILLUM_DEVICE_CONTROL_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pClientIllumDevicesControl) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ClientIllumDevicesSetControl(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pClientIllumDevicesControl: *mut NV_GPU_CLIENT_ILLUM_DEVICE_CONTROL_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1459768418u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pClientIllumDevicesControl: *mut NV_GPU_CLIENT_ILLUM_DEVICE_CONTROL_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pClientIllumDevicesControl) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ClientIllumZonesGetInfo(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pIllumZonesInfo: *mut NV_GPU_CLIENT_ILLUM_ZONE_INFO_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1266754587u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pIllumZonesInfo: *mut NV_GPU_CLIENT_ILLUM_ZONE_INFO_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pIllumZonesInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ClientIllumZonesGetControl(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pIllumZonesControl: *mut NV_GPU_CLIENT_ILLUM_ZONE_CONTROL_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1035949924u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pIllumZonesControl: *mut NV_GPU_CLIENT_ILLUM_ZONE_CONTROL_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pIllumZonesControl) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ClientIllumZonesSetControl(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pIllumZonesControl: *mut NV_GPU_CLIENT_ILLUM_ZONE_CONTROL_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(427624030u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pIllumZonesControl: *mut NV_GPU_CLIENT_ILLUM_ZONE_CONTROL_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pIllumZonesControl) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Event_RegisterCallback(
    eventCallback: PNV_EVENT_REGISTER_CALLBACK,
    phClient: *mut NvEventHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3873172073u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            eventCallback: PNV_EVENT_REGISTER_CALLBACK,
                            phClient: *mut NvEventHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(eventCallback, phClient) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Event_UnregisterCallback(
    hClient: NvEventHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3726613317u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(hClient: NvEventHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hClient) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_EnumNvidiaDisplayHandle(
    thisEnum: NvU32,
    pNvDispHandle: *mut NvDisplayHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2596131853u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            thisEnum: NvU32,
                            pNvDispHandle: *mut NvDisplayHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(thisEnum, pNvDispHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_EnumNvidiaUnAttachedDisplayHandle(
    thisEnum: NvU32,
    pNvUnAttachedDispHandle: *mut NvUnAttachedDisplayHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(551457376u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            thisEnum: NvU32,
                            pNvUnAttachedDispHandle: *mut NvUnAttachedDisplayHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(thisEnum, pNvUnAttachedDispHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_CreateDisplayFromUnAttachedDisplay(
    hNvUnAttachedDisp: NvUnAttachedDisplayHandle,
    pNvDisplay: *mut NvDisplayHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1677293982u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvUnAttachedDisp: NvUnAttachedDisplayHandle,
                            pNvDisplay: *mut NvDisplayHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvUnAttachedDisp, pNvDisplay) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetAssociatedNvidiaDisplayHandle(
    szDisplayName: *const ::std::os::raw::c_char,
    pNvDispHandle: *mut NvDisplayHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(901943604u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            szDisplayName: *const ::std::os::raw::c_char,
                            pNvDispHandle: *mut NvDisplayHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(szDisplayName, pNvDispHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetAssociatedUnAttachedNvidiaDisplayHandle(
    szDisplayName: *const ::std::os::raw::c_char,
    pNvUnAttachedDispHandle: *mut NvUnAttachedDisplayHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2802123698u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            szDisplayName: *const ::std::os::raw::c_char,
                            pNvUnAttachedDispHandle: *mut NvUnAttachedDisplayHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(szDisplayName, pNvUnAttachedDispHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetAssociatedNvidiaDisplayName(
    NvDispHandle: NvDisplayHandle,
    szDisplayName: *mut ::std::os::raw::c_char,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(581405445u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            NvDispHandle: NvDisplayHandle,
                            szDisplayName: *mut ::std::os::raw::c_char,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(NvDispHandle, szDisplayName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetUnAttachedAssociatedDisplayName(
    hNvUnAttachedDisp: NvUnAttachedDisplayHandle,
    szDisplayName: *mut ::std::os::raw::c_char,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1216927632u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvUnAttachedDisp: NvUnAttachedDisplayHandle,
                            szDisplayName: *mut ::std::os::raw::c_char,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvUnAttachedDisp, szDisplayName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_EnableHWCursor(
    hNvDisplay: NvDisplayHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(677581965u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(hNvDisplay: NvDisplayHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DisableHWCursor(
    hNvDisplay: NvDisplayHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2870358167u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(hNvDisplay: NvDisplayHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetVBlankCounter(
    hNvDisplay: NvDisplayHandle,
    pCounter: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1739971413u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            pCounter: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, pCounter) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SetRefreshRateOverride(
    hNvDisplay: NvDisplayHandle,
    outputsMask: NvU32,
    refreshRate: f32,
    bSetDeferred: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(814918706u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            outputsMask: NvU32,
                            refreshRate: f32,
                            bSetDeferred: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, outputsMask, refreshRate, bSetDeferred) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetAssociatedDisplayOutputId(
    hNvDisplay: NvDisplayHandle,
    pOutputId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3650458494u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            pOutputId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, pOutputId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetDisplayPortInfo(
    hNvDisplay: NvDisplayHandle,
    outputId: NvU32,
    pInfo: *mut NV_DISPLAY_PORT_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3327128423u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            outputId: NvU32,
                            pInfo: *mut NV_DISPLAY_PORT_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, outputId, pInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SetDisplayPort(
    hNvDisplay: NvDisplayHandle,
    outputId: NvU32,
    pCfg: *mut NV_DISPLAY_PORT_CONFIG,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4195608154u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            outputId: NvU32,
                            pCfg: *mut NV_DISPLAY_PORT_CONFIG,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, outputId, pCfg) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetHDMISupportInfo(
    hNvDisplay: NvDisplayHandle,
    outputId: NvU32,
    pInfo: *mut NV_HDMI_SUPPORT_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1793158851u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            outputId: NvU32,
                            pInfo: *mut NV_HDMI_SUPPORT_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, outputId, pInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_InfoFrameControl(
    displayId: NvU32,
    pInfoframeData: *mut NV_INFOFRAME_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1617407807u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pInfoframeData: *mut NV_INFOFRAME_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pInfoframeData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_ColorControl(
    displayId: NvU32,
    pColorData: *mut NV_COLOR_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2465847309u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pColorData: *mut NV_COLOR_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pColorData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_GetHdrCapabilities(
    displayId: NvU32,
    pHdrCapabilities: *mut NV_HDR_CAPABILITIES,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2230495455u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pHdrCapabilities: *mut NV_HDR_CAPABILITIES,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pHdrCapabilities) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_HdrColorControl(
    displayId: NvU32,
    pHdrColorData: *mut NV_HDR_COLOR_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(891134500u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pHdrColorData: *mut NV_HDR_COLOR_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pHdrColorData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_SetSourceColorSpace(
    displayId: NvU32,
    colorSpaceType: NV_COLORSPACE_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1195076783u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            colorSpaceType: NV_COLORSPACE_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, colorSpaceType) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_GetSourceColorSpace(
    displayId: NvU32,
    pColorSpaceType: *mut NV_COLORSPACE_TYPE,
    sourcePID: NvU64,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3471689819u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pColorSpaceType: *mut NV_COLORSPACE_TYPE,
                            sourcePID: NvU64,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pColorSpaceType, sourcePID) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_SetSourceHdrMetadata(
    displayId: NvU32,
    pMetadata: *mut NV_HDR_METADATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2422126139u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pMetadata: *mut NV_HDR_METADATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pMetadata) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_GetSourceHdrMetadata(
    displayId: NvU32,
    pMetadata: *mut NV_HDR_METADATA,
    sourcePID: NvU64,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(222253786u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pMetadata: *mut NV_HDR_METADATA,
                            sourcePID: NvU64,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pMetadata, sourcePID) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_SetOutputMode(
    displayId: NvU32,
    pDisplayMode: *mut NV_DISPLAY_OUTPUT_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2565301786u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pDisplayMode: *mut NV_DISPLAY_OUTPUT_MODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pDisplayMode) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_GetOutputMode(
    displayId: NvU32,
    pDisplayMode: *mut NV_DISPLAY_OUTPUT_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2180962445u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pDisplayMode: *mut NV_DISPLAY_OUTPUT_MODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pDisplayMode) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_SetHdrToneMapping(
    displayId: NvU32,
    hdrTonemapping: NV_HDR_TONEMAPPING_METHOD,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3714949986u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            hdrTonemapping: NV_HDR_TONEMAPPING_METHOD,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, hdrTonemapping) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_GetHdrToneMapping(
    displayId: NvU32,
    pHdrTonemapping: *mut NV_HDR_TONEMAPPING_METHOD,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4224937585u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pHdrTonemapping: *mut NV_HDR_TONEMAPPING_METHOD,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pHdrTonemapping) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_GetColorimetry(
    displayId: NvU32,
    pColorimetry: *mut NV_DISPLAY_COLORIMETRY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(11805101u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pColorimetry: *mut NV_DISPLAY_COLORIMETRY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pColorimetry) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetTiming(
    displayId: NvU32,
    timingInput: *mut NV_TIMING_INPUT,
    pTiming: *mut NV_TIMING,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(391210985u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            timingInput: *mut NV_TIMING_INPUT,
                            pTiming: *mut NV_TIMING,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, timingInput, pTiming) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetMonitorCapabilities(
    displayId: NvU32,
    pMonitorCapabilities: *mut NV_MONITOR_CAPABILITIES,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(990234593u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pMonitorCapabilities: *mut NV_MONITOR_CAPABILITIES,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pMonitorCapabilities) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetMonitorColorCapabilities(
    displayId: NvU32,
    pMonitorColorCapabilities: *mut NV_MONITOR_COLOR_CAPS,
    pColorCapsCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1793380277u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pMonitorColorCapabilities: *mut NV_MONITOR_COLOR_CAPS,
                            pColorCapsCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pMonitorColorCapabilities, pColorCapsCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_EnumCustomDisplay(
    displayId: NvU32,
    index: NvU32,
    pCustDisp: *mut NV_CUSTOM_DISPLAY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2718379353u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            index: NvU32,
                            pCustDisp: *mut NV_CUSTOM_DISPLAY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, index, pCustDisp) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_TryCustomDisplay(
    pDisplayIds: *mut NvU32,
    count: NvU32,
    pCustDisp: *mut NV_CUSTOM_DISPLAY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(528332336u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pDisplayIds: *mut NvU32,
                            count: NvU32,
                            pCustDisp: *mut NV_CUSTOM_DISPLAY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pDisplayIds, count, pCustDisp) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_DeleteCustomDisplay(
    pDisplayIds: *mut NvU32,
    count: NvU32,
    pCustDisp: *mut NV_CUSTOM_DISPLAY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1429101467u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pDisplayIds: *mut NvU32,
                            count: NvU32,
                            pCustDisp: *mut NV_CUSTOM_DISPLAY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pDisplayIds, count, pCustDisp) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_SaveCustomDisplay(
    pDisplayIds: *mut NvU32,
    count: NvU32,
    isThisOutputIdOnly: NvU32,
    isThisMonitorIdOnly: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1233660022u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pDisplayIds: *mut NvU32,
                            count: NvU32,
                            isThisOutputIdOnly: NvU32,
                            isThisMonitorIdOnly: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(pDisplayIds, count, isThisOutputIdOnly, isThisMonitorIdOnly)
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_RevertCustomDisplayTrial(
    pDisplayIds: *mut NvU32,
    count: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3418177776u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pDisplayIds: *mut NvU32,
                            count: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pDisplayIds, count) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetView(
    hNvDisplay: NvDisplayHandle,
    pTargets: *mut NV_VIEW_TARGET_INFO,
    pTargetMaskCount: *mut NvU32,
    pTargetView: *mut NV_TARGET_VIEW_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3602488713u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            pTargets: *mut NV_VIEW_TARGET_INFO,
                            pTargetMaskCount: *mut NvU32,
                            pTargetView: *mut NV_TARGET_VIEW_MODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, pTargets, pTargetMaskCount, pTargetView) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetViewEx(
    hNvDisplay: NvDisplayHandle,
    pPathInfo: *mut NV_DISPLAY_PATH_INFO,
    pPathCount: *mut NvU32,
    pTargetViewMode: *mut NV_TARGET_VIEW_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3686533876u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            pPathInfo: *mut NV_DISPLAY_PATH_INFO,
                            pPathCount: *mut NvU32,
                            pTargetViewMode: *mut NV_TARGET_VIEW_MODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, pPathInfo, pPathCount, pTargetViewMode) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetSupportedViews(
    hNvDisplay: NvDisplayHandle,
    pTargetViews: *mut NV_TARGET_VIEW_MODE,
    pViewCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1727758272u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvDisplay: NvDisplayHandle,
                            pTargetViews: *mut NV_TARGET_VIEW_MODE,
                            pViewCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvDisplay, pTargetViews, pViewCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetDisplayIdByDisplayName(
    displayName: *const ::std::os::raw::c_char,
    displayId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2923786640u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayName: *const ::std::os::raw::c_char,
                            displayId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayName, displayId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetDisplayConfig(
    pathInfoCount: *mut NvU32,
    pathInfo: *mut NV_DISPLAYCONFIG_PATH_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(296471800u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pathInfoCount: *mut NvU32,
                            pathInfo: *mut NV_DISPLAYCONFIG_PATH_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pathInfoCount, pathInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_SetDisplayConfig(
    pathInfoCount: NvU32,
    pathInfo: *mut NV_DISPLAYCONFIG_PATH_INFO,
    flags: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1569519838u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pathInfoCount: NvU32,
                            pathInfo: *mut NV_DISPLAYCONFIG_PATH_INFO,
                            flags: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pathInfoCount, pathInfo, flags) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetEdidData(
    displayId: NvU32,
    pEdidParams: *mut NV_EDID_DATA,
    pFlag: *mut NV_EDID_FLAG,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1131212150u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pEdidParams: *mut NV_EDID_DATA,
                            pFlag: *mut NV_EDID_FLAG,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pEdidParams, pFlag) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetAdaptiveSyncData(
    displayId: NvU32,
    pAdaptiveSyncData: *mut NV_GET_ADAPTIVE_SYNC_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3074236137u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pAdaptiveSyncData: *mut NV_GET_ADAPTIVE_SYNC_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pAdaptiveSyncData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_SetAdaptiveSyncData(
    displayId: NvU32,
    pAdaptiveSyncData: *mut NV_SET_ADAPTIVE_SYNC_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1055635997u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pAdaptiveSyncData: *mut NV_SET_ADAPTIVE_SYNC_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pAdaptiveSyncData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetVirtualRefreshRateData(
    displayId: NvU32,
    pVirtualRefreshRateData: *mut NV_GET_VIRTUAL_REFRESH_RATE_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2348827290u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pVirtualRefreshRateData: *mut NV_GET_VIRTUAL_REFRESH_RATE_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pVirtualRefreshRateData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_SetVirtualRefreshRateData(
    displayId: NvU32,
    pVirtualRefreshRateData: *mut NV_SET_VIRTUAL_REFRESH_RATE_DATA,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1522263715u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pVirtualRefreshRateData: *mut NV_SET_VIRTUAL_REFRESH_RATE_DATA,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pVirtualRefreshRateData) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_SetPreferredStereoDisplay(
    pPreferredStereoDisplay: *mut NV_SET_PREFERRED_STEREO_DISPLAY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3385909855u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pPreferredStereoDisplay: *mut NV_SET_PREFERRED_STEREO_DISPLAY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pPreferredStereoDisplay) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DISP_GetPreferredStereoDisplay(
    pPreferredStereoDisplay: *mut NV_GET_PREFERRED_STEREO_DISPLAY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(527124070u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pPreferredStereoDisplay: *mut NV_GET_PREFERRED_STEREO_DISPLAY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pPreferredStereoDisplay) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Disp_GetVRRInfo(
    displayId: NvU32,
    pVrrInfo: *mut NV_GET_VRR_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3750746711u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            pVrrInfo: *mut NV_GET_VRR_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, pVrrInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_GetSupportedTopoInfo(
    pSupportedTopoInfo: *mut NV_MOSAIC_SUPPORTED_TOPO_INFO,
    type_: NV_MOSAIC_TOPO_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4256578689u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pSupportedTopoInfo: *mut NV_MOSAIC_SUPPORTED_TOPO_INFO,
                            type_: NV_MOSAIC_TOPO_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pSupportedTopoInfo, type_) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_GetTopoGroup(
    pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF,
    pTopoGroup: *mut NV_MOSAIC_TOPO_GROUP,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3414767645u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF,
                            pTopoGroup: *mut NV_MOSAIC_TOPO_GROUP,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pTopoBrief, pTopoGroup) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_GetOverlapLimits(
    pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF,
    pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING,
    pMinOverlapX: *mut NvS32,
    pMaxOverlapX: *mut NvS32,
    pMinOverlapY: *mut NvS32,
    pMaxOverlapY: *mut NvS32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2560001520u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF,
                            pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING,
                            pMinOverlapX: *mut NvS32,
                            pMaxOverlapX: *mut NvS32,
                            pMinOverlapY: *mut NvS32,
                            pMaxOverlapY: *mut NvS32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(
                        pTopoBrief,
                        pDisplaySetting,
                        pMinOverlapX,
                        pMaxOverlapX,
                        pMinOverlapY,
                        pMaxOverlapY,
                    )
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_SetCurrentTopo(
    pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF,
    pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING,
    overlapX: NvS32,
    overlapY: NvS32,
    enable: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2605983793u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF,
                            pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING,
                            overlapX: NvS32,
                            overlapY: NvS32,
                            enable: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(pTopoBrief, pDisplaySetting, overlapX, overlapY, enable)
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_GetCurrentTopo(
    pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF,
    pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING,
    pOverlapX: *mut NvS32,
    pOverlapY: *mut NvS32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3962737742u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pTopoBrief: *mut NV_MOSAIC_TOPO_BRIEF,
                            pDisplaySetting: *mut NV_MOSAIC_DISPLAY_SETTING,
                            pOverlapX: *mut NvS32,
                            pOverlapY: *mut NvS32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pTopoBrief, pDisplaySetting, pOverlapX, pOverlapY) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_EnableCurrentTopo(enable: NvU32) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1595582060u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(enable: NvU32) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(enable) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_SetDisplayGrids(
    pGridTopologies: *mut NV_MOSAIC_GRID_TOPO,
    gridCount: NvU32,
    setTopoFlags: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1301650057u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pGridTopologies: *mut NV_MOSAIC_GRID_TOPO,
                            gridCount: NvU32,
                            setTopoFlags: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pGridTopologies, gridCount, setTopoFlags) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_ValidateDisplayGrids(
    setTopoFlags: NvU32,
    pGridTopologies: *mut NV_MOSAIC_GRID_TOPO,
    pTopoStatus: *mut NV_MOSAIC_DISPLAY_TOPO_STATUS,
    gridCount: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3477311549u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            setTopoFlags: NvU32,
                            pGridTopologies: *mut NV_MOSAIC_GRID_TOPO,
                            pTopoStatus: *mut NV_MOSAIC_DISPLAY_TOPO_STATUS,
                            gridCount: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(setTopoFlags, pGridTopologies, pTopoStatus, gridCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_EnumDisplayModes(
    pGridTopology: *mut NV_MOSAIC_GRID_TOPO,
    pDisplaySettings: *mut NV_MOSAIC_DISPLAY_SETTING,
    pDisplayCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2027657175u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pGridTopology: *mut NV_MOSAIC_GRID_TOPO,
                            pDisplaySettings: *mut NV_MOSAIC_DISPLAY_SETTING,
                            pDisplayCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pGridTopology, pDisplaySettings, pDisplayCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Mosaic_EnumDisplayGrids(
    pGridTopologies: *mut NV_MOSAIC_GRID_TOPO,
    pGridCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3743975343u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pGridTopologies: *mut NV_MOSAIC_GRID_TOPO,
                            pGridCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pGridTopologies, pGridCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetSupportedMosaicTopologies(
    pMosaicTopos: *mut NV_MOSAIC_SUPPORTED_TOPOLOGIES,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1091263525u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pMosaicTopos: *mut NV_MOSAIC_SUPPORTED_TOPOLOGIES,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pMosaicTopos) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GetCurrentMosaicTopology(
    pMosaicTopo: *mut NV_MOSAIC_TOPOLOGY,
    pEnabled: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4127740605u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pMosaicTopo: *mut NV_MOSAIC_TOPOLOGY,
                            pEnabled: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pMosaicTopo, pEnabled) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SetCurrentMosaicTopology(
    pMosaicTopo: *mut NV_MOSAIC_TOPOLOGY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3578497417u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pMosaicTopo: *mut NV_MOSAIC_TOPOLOGY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pMosaicTopo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_EnableCurrentMosaicTopology(
    enable: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1946631369u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(enable: NvU32) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(enable) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_EnumSyncDevices(
    nvGSyncHandles: *mut NvGSyncDeviceHandle,
    gsyncCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3647182337u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            nvGSyncHandles: *mut NvGSyncDeviceHandle,
                            gsyncCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(nvGSyncHandles, gsyncCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_QueryCapabilities(
    hNvGSyncDevice: NvGSyncDeviceHandle,
    pNvGSyncCapabilities: *mut NV_GSYNC_CAPABILITIES,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1151594961u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvGSyncDevice: NvGSyncDeviceHandle,
                            pNvGSyncCapabilities: *mut NV_GSYNC_CAPABILITIES,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvGSyncDevice, pNvGSyncCapabilities) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_GetTopology(
    hNvGSyncDevice: NvGSyncDeviceHandle,
    gsyncGpuCount: *mut NvU32,
    gsyncGPUs: *mut NV_GSYNC_GPU,
    gsyncDisplayCount: *mut NvU32,
    gsyncDisplays: *mut NV_GSYNC_DISPLAY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1164098616u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvGSyncDevice: NvGSyncDeviceHandle,
                            gsyncGpuCount: *mut NvU32,
                            gsyncGPUs: *mut NV_GSYNC_GPU,
                            gsyncDisplayCount: *mut NvU32,
                            gsyncDisplays: *mut NV_GSYNC_DISPLAY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(
                        hNvGSyncDevice,
                        gsyncGpuCount,
                        gsyncGPUs,
                        gsyncDisplayCount,
                        gsyncDisplays,
                    )
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_SetSyncStateSettings(
    gsyncDisplayCount: NvU32,
    pGsyncDisplays: *mut NV_GSYNC_DISPLAY,
    flags: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1621942237u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            gsyncDisplayCount: NvU32,
                            pGsyncDisplays: *mut NV_GSYNC_DISPLAY,
                            flags: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(gsyncDisplayCount, pGsyncDisplays, flags) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_GetControlParameters(
    hNvGSyncDevice: NvGSyncDeviceHandle,
    pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(383655018u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvGSyncDevice: NvGSyncDeviceHandle,
                            pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvGSyncDevice, pGsyncControls) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_SetControlParameters(
    hNvGSyncDevice: NvGSyncDeviceHandle,
    pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2344614027u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvGSyncDevice: NvGSyncDeviceHandle,
                            pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvGSyncDevice, pGsyncControls) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_AdjustSyncDelay(
    hNvGSyncDevice: NvGSyncDeviceHandle,
    delayType: NVAPI_GSYNC_DELAY_TYPE,
    pGsyncDelay: *mut NV_GSYNC_DELAY,
    syncSteps: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(756154193u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvGSyncDevice: NvGSyncDeviceHandle,
                            delayType: NVAPI_GSYNC_DELAY_TYPE,
                            pGsyncDelay: *mut NV_GSYNC_DELAY,
                            syncSteps: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvGSyncDevice, delayType, pGsyncDelay, syncSteps) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_GetSyncStatus(
    hNvGSyncDevice: NvGSyncDeviceHandle,
    hPhysicalGpu: NvPhysicalGpuHandle,
    status: *mut NV_GSYNC_STATUS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4059411508u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvGSyncDevice: NvGSyncDeviceHandle,
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            status: *mut NV_GSYNC_STATUS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvGSyncDevice, hPhysicalGpu, status) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GSync_GetStatusParameters(
    hNvGSyncDevice: NvGSyncDeviceHandle,
    pStatusParams: *mut NV_GSYNC_STATUS_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1892943084u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hNvGSyncDevice: NvGSyncDeviceHandle,
                            pStatusParams: *mut NV_GSYNC_STATUS_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hNvGSyncDevice, pStatusParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_NGX_GetNGXOverrideState(
    pGetOverrideStateParams: *mut NV_NGX_DLSS_OVERRIDE_GET_STATE_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1071214522u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pGetOverrideStateParams: *mut NV_NGX_DLSS_OVERRIDE_GET_STATE_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pGetOverrideStateParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_NGX_SetNGXOverrideState(
    pSetOverrideStateParams: *mut NV_NGX_DLSS_OVERRIDE_SET_STATE_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3054488398u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pSetOverrideStateParams: *mut NV_NGX_DLSS_OVERRIDE_SET_STATE_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pSetOverrideStateParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_NGX_OpenSession(
    pParams: *mut NV_NGX_SESSION_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(32006526u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pParams: *mut NV_NGX_SESSION_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_NGX_CloseSession(
    pParams: *mut NV_NGX_SESSION_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3929295128u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pParams: *mut NV_NGX_SESSION_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_NGX_SetRuntimeParams(
    pSetParams: *mut NV_NGX_SET_RUNTIME_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1197050202u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pSetParams: *mut NV_NGX_SET_RUNTIME_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pSetParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_NGX_GetRuntimeParams(
    pGetParams: *mut NV_NGX_GET_RUNTIME_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(192855420u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pGetParams: *mut NV_NGX_GET_RUNTIME_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pGetParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_GetCapabilities(
    hVioHandle: NvVioHandle,
    pAdapterCaps: *mut NVVIOCAPS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(499716867u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pAdapterCaps: *mut NVVIOCAPS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pAdapterCaps) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_Open(
    hVioHandle: NvVioHandle,
    vioClass: NvU32,
    ownerType: NVVIOOWNERTYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1156466753u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            vioClass: NvU32,
                            ownerType: NVVIOOWNERTYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, vioClass, ownerType) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_Close(
    hVioHandle: NvVioHandle,
    bRelease: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3491484215u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            bRelease: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, bRelease) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_Status(
    hVioHandle: NvVioHandle,
    pStatus: *mut NVVIOSTATUS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(242017521u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pStatus: *mut NVVIOSTATUS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pStatus) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_SyncFormatDetect(
    hVioHandle: NvVioHandle,
    pWait: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(294471843u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pWait: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pWait) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_GetConfig(
    hVioHandle: NvVioHandle,
    pConfig: *mut NVVIOCONFIG,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3544873115u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pConfig: *mut NVVIOCONFIG,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pConfig) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_SetConfig(
    hVioHandle: NvVioHandle,
    pConfig: *const NVVIOCONFIG,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(240053255u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pConfig: *const NVVIOCONFIG,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pConfig) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_SetCSC(
    hVioHandle: NvVioHandle,
    pCSC: *mut NVVIOCOLORCONVERSION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2716634484u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pCSC: *mut NVVIOCOLORCONVERSION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pCSC) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_GetCSC(
    hVioHandle: NvVioHandle,
    pCSC: *mut NVVIOCOLORCONVERSION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2064478883u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pCSC: *mut NVVIOCOLORCONVERSION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pCSC) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_SetGamma(
    hVioHandle: NvVioHandle,
    pGamma: *mut NVVIOGAMMACORRECTION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2521560146u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pGamma: *mut NVVIOGAMMACORRECTION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pGamma) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_GetGamma(
    hVioHandle: NvVioHandle,
    pGamma: *mut NVVIOGAMMACORRECTION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1372929286u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pGamma: *mut NVVIOGAMMACORRECTION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pGamma) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_SetSyncDelay(
    hVioHandle: NvVioHandle,
    pSyncDelay: *const NVVIOSYNCDELAY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(647473361u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pSyncDelay: *const NVVIOSYNCDELAY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pSyncDelay) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_GetSyncDelay(
    hVioHandle: NvVioHandle,
    pSyncDelay: *mut NVVIOSYNCDELAY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1176638633u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pSyncDelay: *mut NVVIOSYNCDELAY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pSyncDelay) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_GetPCIInfo(
    hVioHandle: NvVioHandle,
    pVioPCIInfo: *mut NVVIOPCIINFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3112294709u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            pVioPCIInfo: *mut NVVIOPCIINFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, pVioPCIInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_IsRunning(hVioHandle: NvVioHandle) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2528969742u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(hVioHandle: NvVioHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_Start(hVioHandle: NvVioHandle) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3454591395u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(hVioHandle: NvVioHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_Stop(hVioHandle: NvVioHandle) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1805821398u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(hVioHandle: NvVioHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_IsFrameLockModeCompatible(
    hVioHandle: NvVioHandle,
    srcEnumIndex: NvU32,
    destEnumIndex: NvU32,
    pbCompatible: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2079369549u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            srcEnumIndex: NvU32,
                            destEnumIndex: NvU32,
                            pbCompatible: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, srcEnumIndex, destEnumIndex, pbCompatible) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_EnumDevices(
    hVioHandle: *mut NvVioHandle,
    vioDeviceCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4252783959u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: *mut NvVioHandle,
                            vioDeviceCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, vioDeviceCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_QueryTopology(
    pNvVIOTopology: *mut NV_VIO_TOPOLOGY,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2257925346u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pNvVIOTopology: *mut NV_VIO_TOPOLOGY,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pNvVIOTopology) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_EnumSignalFormats(
    hVioHandle: NvVioHandle,
    enumIndex: NvU32,
    pSignalFormatDetail: *mut NVVIOSIGNALFORMATDETAIL,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3939971044u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            enumIndex: NvU32,
                            pSignalFormatDetail: *mut NVVIOSIGNALFORMATDETAIL,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, enumIndex, pSignalFormatDetail) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_VIO_EnumDataFormats(
    hVioHandle: NvVioHandle,
    enumIndex: NvU32,
    pDataFormatDetail: *mut NVVIODATAFORMATDETAIL,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(572500200u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hVioHandle: NvVioHandle,
                            enumIndex: NvU32,
                            pDataFormatDetail: *mut NVVIODATAFORMATDETAIL,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hVioHandle, enumIndex, pDataFormatDetail) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_CreateConfigurationProfileRegistryKey(
    registryProfileType: NV_STEREO_REGISTRY_PROFILE_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3195441900u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            registryProfileType: NV_STEREO_REGISTRY_PROFILE_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(registryProfileType) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_DeleteConfigurationProfileRegistryKey(
    registryProfileType: NV_STEREO_REGISTRY_PROFILE_TYPE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4044863540u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            registryProfileType: NV_STEREO_REGISTRY_PROFILE_TYPE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(registryProfileType) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetConfigurationProfileValue(
    registryProfileType: NV_STEREO_REGISTRY_PROFILE_TYPE,
    valueRegistryID: NV_STEREO_REGISTRY_ID,
    pValue: *mut ::std::os::raw::c_void,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(608214856u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            registryProfileType: NV_STEREO_REGISTRY_PROFILE_TYPE,
                            valueRegistryID: NV_STEREO_REGISTRY_ID,
                            pValue: *mut ::std::os::raw::c_void,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(registryProfileType, valueRegistryID, pValue) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_DeleteConfigurationProfileValue(
    registryProfileType: NV_STEREO_REGISTRY_PROFILE_TYPE,
    valueRegistryID: NV_STEREO_REGISTRY_ID,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1237118671u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            registryProfileType: NV_STEREO_REGISTRY_PROFILE_TYPE,
                            valueRegistryID: NV_STEREO_REGISTRY_ID,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(registryProfileType, valueRegistryID) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_GetStereoSupport(
    hMonitor: NvMonitorHandle,
    pCaps: *mut NVAPI_STEREO_CAPS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(694960973u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hMonitor: NvMonitorHandle,
                            pCaps: *mut NVAPI_STEREO_CAPS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hMonitor, pCaps) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_DecreaseSeparation(
    stereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3657712728u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(stereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_IncreaseSeparation(
    stereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3383291116u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(stereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_DecreaseConvergence(
    stereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1283973911u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(stereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_IncreaseConvergence(
    stereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2709367486u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(stereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_GetFrustumAdjustMode(
    stereoHandle: StereoHandle,
    pFrustumAdjustMode: *mut NV_FRUSTUM_ADJUST_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3867384643u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            stereoHandle: StereoHandle,
                            pFrustumAdjustMode: *mut NV_FRUSTUM_ADJUST_MODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle, pFrustumAdjustMode) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetFrustumAdjustMode(
    stereoHandle: StereoHandle,
    newFrustumAdjustModeValue: NV_FRUSTUM_ADJUST_MODE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2078441378u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            stereoHandle: StereoHandle,
                            newFrustumAdjustModeValue: NV_FRUSTUM_ADJUST_MODE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle, newFrustumAdjustModeValue) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_CaptureJpegImage(
    stereoHandle: StereoHandle,
    quality: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2469179712u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            stereoHandle: StereoHandle,
                            quality: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle, quality) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_InitActivation(
    hStereoHandle: StereoHandle,
    flags: NVAPI_STEREO_INIT_ACTIVATION_FLAGS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3340203778u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hStereoHandle: StereoHandle,
                            flags: NVAPI_STEREO_INIT_ACTIVATION_FLAGS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle, flags) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_Trigger_Activation(
    hStereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(225209554u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(hStereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_CapturePngImage(
    stereoHandle: StereoHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2340329909u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(stereoHandle: StereoHandle) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(stereoHandle) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_ReverseStereoBlitControl(
    hStereoHandle: StereoHandle,
    TurnOn: NvU8,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1020628873u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hStereoHandle: StereoHandle,
                            TurnOn: NvU8,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle, TurnOn) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_Stereo_SetNotificationMessage(
    hStereoHandle: StereoHandle,
    hWnd: NvU64,
    messageID: NvU64,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1805336734u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hStereoHandle: StereoHandle,
                            hWnd: NvU64,
                            messageID: NvU64,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hStereoHandle, hWnd, messageID) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_CreateSession(
    phSession: *mut NvDRSSessionHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(110417198u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            phSession: *mut NvDRSSessionHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(phSession) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_DestroySession(
    hSession: NvDRSSessionHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3671707640u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_LoadSettings(
    hSession: NvDRSSessionHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(928890219u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_SaveSettings(
    hSession: NvDRSSessionHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4240211476u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_LoadSettingsFromFile(
    hSession: NvDRSSessionHandle,
    fileName: *mut NvU16,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3555584137u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            fileName: *mut NvU16,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, fileName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_SaveSettingsToFile(
    hSession: NvDRSSessionHandle,
    fileName: *mut NvU16,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(736255480u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            fileName: *mut NvU16,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, fileName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_CreateProfile(
    hSession: NvDRSSessionHandle,
    pProfileInfo: *mut NVDRS_PROFILE,
    phProfile: *mut NvDRSProfileHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3424084072u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            pProfileInfo: *mut NVDRS_PROFILE,
                            phProfile: *mut NvDRSProfileHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, pProfileInfo, phProfile) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_DeleteProfile(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(386478598u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_SetCurrentGlobalProfile(
    hSession: NvDRSSessionHandle,
    wszGlobalProfileName: *mut NvU16,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(478791135u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            wszGlobalProfileName: *mut NvU16,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, wszGlobalProfileName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_GetCurrentGlobalProfile(
    hSession: NvDRSSessionHandle,
    phProfile: *mut NvDRSProfileHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1635516319u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            phProfile: *mut NvDRSProfileHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, phProfile) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_GetProfileInfo(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    pProfileInfo: *mut NVDRS_PROFILE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1640853462u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            pProfileInfo: *mut NVDRS_PROFILE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, pProfileInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_SetProfileInfo(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    pProfileInfo: *mut NVDRS_PROFILE,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(380359593u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            pProfileInfo: *mut NVDRS_PROFILE,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, pProfileInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_FindProfileByName(
    hSession: NvDRSSessionHandle,
    profileName: *mut NvU16,
    phProfile: *mut NvDRSProfileHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2118818315u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            profileName: *mut NvU16,
                            phProfile: *mut NvDRSProfileHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, profileName, phProfile) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_EnumProfiles(
    hSession: NvDRSSessionHandle,
    index: NvU32,
    phProfile: *mut NvDRSProfileHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3157728992u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            index: NvU32,
                            phProfile: *mut NvDRSProfileHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, index, phProfile) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_GetNumProfiles(
    hSession: NvDRSSessionHandle,
    numProfiles: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(497962940u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            numProfiles: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, numProfiles) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_CreateApplication(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    pApplication: *mut NVDRS_APPLICATION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1128770014u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            pApplication: *mut NVDRS_APPLICATION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, pApplication) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_DeleteApplicationEx(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    pApp: *mut NVDRS_APPLICATION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3320481185u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            pApp: *mut NVDRS_APPLICATION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, pApp) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_DeleteApplication(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    appName: *mut NvU16,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(745098182u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            appName: *mut NvU16,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, appName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_GetApplicationInfo(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    appName: *mut NvU16,
    pApplication: *mut NVDRS_APPLICATION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3978267753u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            appName: *mut NvU16,
                            pApplication: *mut NVDRS_APPLICATION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, appName, pApplication) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_EnumApplications(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    startIndex: NvU32,
    appCount: *mut NvU32,
    pApplication: *mut NVDRS_APPLICATION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2141329210u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            startIndex: NvU32,
                            appCount: *mut NvU32,
                            pApplication: *mut NVDRS_APPLICATION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(hSession, hProfile, startIndex, appCount, pApplication)
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_FindApplicationByName(
    hSession: NvDRSSessionHandle,
    appName: *mut NvU16,
    phProfile: *mut NvDRSProfileHandle,
    pApplication: *mut NVDRS_APPLICATION,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4008011442u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            appName: *mut NvU16,
                            phProfile: *mut NvDRSProfileHandle,
                            pApplication: *mut NVDRS_APPLICATION,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, appName, phProfile, pApplication) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_SetSetting(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    pSetting: *mut NVDRS_SETTING,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1467863554u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            pSetting: *mut NVDRS_SETTING,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, pSetting) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_GetSetting(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    settingId: NvU32,
    pSetting: *mut NVDRS_SETTING,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1941930808u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            settingId: NvU32,
                            pSetting: *mut NVDRS_SETTING,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, settingId, pSetting) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_EnumSettings(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    startIndex: NvU32,
    settingsCount: *mut NvU32,
    pSetting: *mut NVDRS_SETTING,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2922396122u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            startIndex: NvU32,
                            settingsCount: *mut NvU32,
                            pSetting: *mut NVDRS_SETTING,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe {
                    __nv_fn(hSession, hProfile, startIndex, settingsCount, pSetting)
                }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_EnumAvailableSettingIds(
    pSettingIds: *mut NvU32,
    pMaxCount: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4028653898u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pSettingIds: *mut NvU32,
                            pMaxCount: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pSettingIds, pMaxCount) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_EnumAvailableSettingValues(
    settingId: NvU32,
    pMaxNumValues: *mut NvU32,
    pSettingValues: *mut NVDRS_SETTING_VALUES,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(784572304u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            settingId: NvU32,
                            pMaxNumValues: *mut NvU32,
                            pSettingValues: *mut NVDRS_SETTING_VALUES,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(settingId, pMaxNumValues, pSettingValues) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_GetSettingIdFromName(
    settingName: *mut NvU16,
    pSettingId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3413313997u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            settingName: *mut NvU16,
                            pSettingId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(settingName, pSettingId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_GetSettingNameFromId(
    settingId: NvU32,
    pSettingName: *mut NvAPI_UnicodeString,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3592207982u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            settingId: NvU32,
                            pSettingName: *mut NvAPI_UnicodeString,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(settingId, pSettingName) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_DeleteProfileSetting(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    settingId: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3835847522u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            settingId: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, settingId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_RestoreAllDefaults(
    hSession: NvDRSSessionHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1495773332u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_RestoreProfileDefault(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(4200554804u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_RestoreProfileDefaultSetting(
    hSession: NvDRSSessionHandle,
    hProfile: NvDRSProfileHandle,
    settingId: NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1408251934u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            hProfile: NvDRSProfileHandle,
                            settingId: NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, hProfile, settingId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_DRS_GetBaseProfile(
    hSession: NvDRSSessionHandle,
    phProfile: *mut NvDRSProfileHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3666110112u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hSession: NvDRSSessionHandle,
                            phProfile: *mut NvDRSProfileHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hSession, phProfile) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetChipSetInfo(
    pChipSetInfo: *mut NV_CHIPSET_INFO_v4,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1406843850u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pChipSetInfo: *mut NV_CHIPSET_INFO_v4,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pChipSetInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetLidAndDockInfo(
    pLidAndDock: *mut NV_LID_DOCK_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3449900426u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pLidAndDock: *mut NV_LID_DOCK_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pLidAndDock) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetDisplayIdFromGpuAndOutputId(
    hPhysicalGpu: NvPhysicalGpuHandle,
    outputId: NvU32,
    displayId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(150125236u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            outputId: NvU32,
                            displayId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, outputId, displayId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetGpuAndOutputIdFromDisplayId(
    displayId: NvU32,
    hPhysicalGpu: *mut NvPhysicalGpuHandle,
    outputId: *mut NvU32,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(288072101u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            hPhysicalGpu: *mut NvPhysicalGpuHandle,
                            outputId: *mut NvU32,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, hPhysicalGpu, outputId) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetPhysicalGpuFromDisplayId(
    displayId: NvU32,
    hPhysicalGpu: *mut NvPhysicalGpuHandle,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2661762649u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            displayId: NvU32,
                            hPhysicalGpu: *mut NvPhysicalGpuHandle,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(displayId, hPhysicalGpu) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetDisplayDriverInfo(
    pDriverInfo: *mut NV_DISPLAY_DRIVER_INFO,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1914678507u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pDriverInfo: *mut NV_DISPLAY_DRIVER_INFO,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pDriverInfo) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetPhysicalGPUs(
    pPhysicalGPUs: *mut NV_PHYSICAL_GPUS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3551677741u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pPhysicalGPUs: *mut NV_PHYSICAL_GPUS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pPhysicalGPUs) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_SYS_GetLogicalGPUs(
    pLogicalGPUs: *mut NV_LOGICAL_GPUS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(3439328272u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pLogicalGPUs: *mut NV_LOGICAL_GPUS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pLogicalGPUs) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_NGX_GetDriverFeatureSupport(
    pParams: *mut NV_NGX_GET_DRIVER_FEATURE_SUPPORT_PARAMS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1637134749u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pParams: *mut NV_NGX_GET_DRIVER_FEATURE_SUPPORT_PARAMS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pParams) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_GPU_ClientRegisterForUtilizationSampleUpdates(
    hPhysicalGpu: NvPhysicalGpuHandle,
    pCallbackSettings: *mut NV_GPU_CLIENT_UTILIZATION_PERIODIC_CALLBACK_SETTINGS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2918100839u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            hPhysicalGpu: NvPhysicalGpuHandle,
                            pCallbackSettings: *mut NV_GPU_CLIENT_UTILIZATION_PERIODIC_CALLBACK_SETTINGS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(hPhysicalGpu, pCallbackSettings) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_RegisterRiseCallback(
    pCallbackSettings: *mut NV_RISE_CALLBACK_SETTINGS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2633928596u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            pCallbackSettings: *mut NV_RISE_CALLBACK_SETTINGS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(pCallbackSettings) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_RequestRise(
    requestContent: *mut NV_REQUEST_RISE_SETTINGS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(1346887320u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            requestContent: *mut NV_REQUEST_RISE_SETTINGS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(requestContent) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn NvAPI_UninstallRise(
    requestContent: *mut NV_UNINSTALL_RISE_SETTINGS,
) -> NvAPI_Status {
    match nvapi_api() {
        Ok(__nv_qi) => {
            // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
            let __nv_ptr = unsafe { (__nv_qi.query)(2878147062u32) };
            if __nv_ptr.is_null() {
                _NvAPI_Status_NVAPI_NO_IMPLEMENTATION
            } else {
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                let __nv_fn = unsafe {
                    ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        unsafe extern "C" fn(
                            requestContent: *mut NV_UNINSTALL_RISE_SETTINGS,
                        ) -> NvAPI_Status,
                    >(__nv_ptr)
                };
                // SAFETY: the header declares this exact id's signature and the query's library remains loaded for the process; the caller upholds the native call contract.
                unsafe { __nv_fn(requestContent) }
            }
        }
        Err(__nv_error) => _NvAPI_Status_NVAPI_LIBRARY_NOT_FOUND,
    }
}
