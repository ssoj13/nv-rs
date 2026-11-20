#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlInit_v2() -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlInit_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlInitWithFlags(
    flags: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlInitWithFlags {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(flags) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlShutdown() -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlShutdown {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlErrorString(
    result: nvmlReturn_t,
) -> *const ::std::os::raw::c_char {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlErrorString {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(result) }
                Err(__nv_error) => ::core::ptr::null(),
            }
        }
        Err(__nv_error) => ::core::ptr::null(),
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetDriverVersion(
    version: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetDriverVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(version, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetNVMLVersion(
    version: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetNVMLVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(version, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetCudaDriverVersion(
    cudaDriverVersion: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetCudaDriverVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(cudaDriverVersion) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetCudaDriverVersion_v2(
    cudaDriverVersion: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetCudaDriverVersion_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(cudaDriverVersion) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetProcessName(
    pid: ::std::os::raw::c_uint,
    name: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetProcessName {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pid, name, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetHicVersion(
    hwbcCount: *mut ::std::os::raw::c_uint,
    hwbcEntries: *mut nvmlHwbcEntry_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetHicVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(hwbcCount, hwbcEntries) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetTopologyGpuSet(
    cpuNumber: ::std::os::raw::c_uint,
    count: *mut ::std::os::raw::c_uint,
    deviceArray: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetTopologyGpuSet {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(cpuNumber, count, deviceArray) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetDriverBranch(
    branchInfo: *mut nvmlSystemDriverBranchInfo_t,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetDriverBranch {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(branchInfo, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetCPER_v1(
    cper: *mut nvmlGetCPER_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetCPER_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(cper) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitGetCount(
    unitCount: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitGetCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(unitCount) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitGetHandleByIndex(
    index: ::std::os::raw::c_uint,
    unit: *mut nvmlUnit_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitGetHandleByIndex {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(index, unit) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitGetUnitInfo(
    unit: nvmlUnit_t,
    info: *mut nvmlUnitInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitGetUnitInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(unit, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitGetLedState(
    unit: nvmlUnit_t,
    state: *mut nvmlLedState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitGetLedState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(unit, state) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitGetPsuInfo(
    unit: nvmlUnit_t,
    psu: *mut nvmlPSUInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitGetPsuInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(unit, psu) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitGetTemperature(
    unit: nvmlUnit_t,
    type_: ::std::os::raw::c_uint,
    temp: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitGetTemperature {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(unit, type_, temp) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitGetFanSpeedInfo(
    unit: nvmlUnit_t,
    fanSpeeds: *mut nvmlUnitFanSpeeds_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitGetFanSpeedInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(unit, fanSpeeds) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitGetDevices(
    unit: nvmlUnit_t,
    deviceCount: *mut ::std::os::raw::c_uint,
    devices: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitGetDevices {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(unit, deviceCount, devices) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCount_v2(
    deviceCount: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCount_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(deviceCount) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAttributes_v2(
    device: nvmlDevice_t,
    attributes: *mut nvmlDeviceAttributes_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAttributes_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, attributes) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetHandleByIndex_v2(
    index: ::std::os::raw::c_uint,
    device: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetHandleByIndex_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(index, device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetHandleBySerial(
    serial: *const ::std::os::raw::c_char,
    device: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetHandleBySerial {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(serial, device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetHandleByUUID(
    uuid: *const ::std::os::raw::c_char,
    device: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetHandleByUUID {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(uuid, device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetHandleByUUIDV(
    uuid: *const nvmlUUID_t,
    device: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetHandleByUUIDV {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(uuid, device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetHandleByPciBusId_v2(
    pciBusId: *const ::std::os::raw::c_char,
    device: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetHandleByPciBusId_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pciBusId, device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetName(
    device: nvmlDevice_t,
    name: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetName {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, name, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetBrand(
    device: nvmlDevice_t,
    type_: *mut nvmlBrandType_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetBrand {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, type_) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetIndex(
    device: nvmlDevice_t,
    index: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetIndex {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, index) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSerial(
    device: nvmlDevice_t,
    serial: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSerial {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, serial, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetModuleId(
    device: nvmlDevice_t,
    moduleId: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetModuleId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, moduleId) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetC2cModeInfoV(
    device: nvmlDevice_t,
    c2cModeInfo: *mut nvmlC2cModeInfo_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetC2cModeInfoV {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, c2cModeInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMemoryAffinity(
    device: nvmlDevice_t,
    nodeSetSize: ::std::os::raw::c_uint,
    nodeSet: *mut ::std::os::raw::c_ulong,
    scope: nvmlAffinityScope_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMemoryAffinity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, nodeSetSize, nodeSet, scope) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCpuAffinityWithinScope(
    device: nvmlDevice_t,
    cpuSetSize: ::std::os::raw::c_uint,
    cpuSet: *mut ::std::os::raw::c_ulong,
    scope: nvmlAffinityScope_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCpuAffinityWithinScope {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, cpuSetSize, cpuSet, scope) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCpuAffinity(
    device: nvmlDevice_t,
    cpuSetSize: ::std::os::raw::c_uint,
    cpuSet: *mut ::std::os::raw::c_ulong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCpuAffinity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, cpuSetSize, cpuSet) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetCpuAffinity(device: nvmlDevice_t) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetCpuAffinity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceClearCpuAffinity(
    device: nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceClearCpuAffinity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNumaNodeId(
    device: nvmlDevice_t,
    node: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNumaNodeId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, node) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAddressingMode(
    device: nvmlDevice_t,
    mode: *mut nvmlDeviceAddressingMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAddressingMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetRepairStatus(
    device: nvmlDevice_t,
    repairStatus: *mut nvmlRepairStatus_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetRepairStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, repairStatus) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetUnrepairableMemoryFlag_v1(
    device: nvmlDevice_t,
    unrepairableMemoryStatus: *mut nvmlUnrepairableMemoryStatus_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetUnrepairableMemoryFlag_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, unrepairableMemoryStatus) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetTopologyCommonAncestor(
    device1: nvmlDevice_t,
    device2: nvmlDevice_t,
    pathInfo: *mut nvmlGpuTopologyLevel_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetTopologyCommonAncestor {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device1, device2, pathInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetTopologyNearestGpus(
    device: nvmlDevice_t,
    level: nvmlGpuTopologyLevel_t,
    count: *mut ::std::os::raw::c_uint,
    deviceArray: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetTopologyNearestGpus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, level, count, deviceArray) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetP2PStatus(
    device1: nvmlDevice_t,
    device2: nvmlDevice_t,
    p2pIndex: nvmlGpuP2PCapsIndex_t,
    p2pStatus: *mut nvmlGpuP2PStatus_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetP2PStatus {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device1, device2, p2pIndex, p2pStatus) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetUUID(
    device: nvmlDevice_t,
    uuid: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetUUID {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, uuid, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMinorNumber(
    device: nvmlDevice_t,
    minorNumber: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMinorNumber {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, minorNumber) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetBoardPartNumber(
    device: nvmlDevice_t,
    partNumber: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetBoardPartNumber {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, partNumber, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetInforomVersion(
    device: nvmlDevice_t,
    object: nvmlInforomObject_t,
    version: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetInforomVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, object, version, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetInforomImageVersion(
    device: nvmlDevice_t,
    version: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetInforomImageVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, version, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetInforomConfigurationChecksum(
    device: nvmlDevice_t,
    checksum: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetInforomConfigurationChecksum {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, checksum) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceValidateInforom(
    device: nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceValidateInforom {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetLastBBXFlushTime(
    device: nvmlDevice_t,
    timestamp: *mut ::std::os::raw::c_ulonglong,
    durationUs: *mut ::std::os::raw::c_ulong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetLastBBXFlushTime {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, timestamp, durationUs) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetBBXTimeData_v1(
    device: nvmlDevice_t,
    timeData: *mut nvmlBBXTimeData_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetBBXTimeData_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, timeData) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDisplayMode(
    device: nvmlDevice_t,
    display: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDisplayMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, display) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDisplayActive(
    device: nvmlDevice_t,
    isActive: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDisplayActive {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, isActive) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPersistenceMode(
    device: nvmlDevice_t,
    mode: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPersistenceMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPciInfoExt(
    device: nvmlDevice_t,
    pci: *mut nvmlPciInfoExt_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPciInfoExt {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pci) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPciInfo_v3(
    device: nvmlDevice_t,
    pci: *mut nvmlPciInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPciInfo_v3 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pci) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMaxPcieLinkGeneration(
    device: nvmlDevice_t,
    maxLinkGen: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMaxPcieLinkGeneration {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, maxLinkGen) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuMaxPcieLinkGeneration(
    device: nvmlDevice_t,
    maxLinkGenDevice: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuMaxPcieLinkGeneration {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, maxLinkGenDevice) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMaxPcieLinkWidth(
    device: nvmlDevice_t,
    maxLinkWidth: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMaxPcieLinkWidth {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, maxLinkWidth) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCurrPcieLinkGeneration(
    device: nvmlDevice_t,
    currLinkGen: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCurrPcieLinkGeneration {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, currLinkGen) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCurrPcieLinkWidth(
    device: nvmlDevice_t,
    currLinkWidth: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCurrPcieLinkWidth {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, currLinkWidth) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPcieThroughput(
    device: nvmlDevice_t,
    counter: nvmlPcieUtilCounter_t,
    value: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPcieThroughput {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, counter, value) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPcieReplayCounter(
    device: nvmlDevice_t,
    value: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPcieReplayCounter {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, value) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetClockInfo(
    device: nvmlDevice_t,
    type_: nvmlClockType_t,
    clock: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetClockInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, type_, clock) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMaxClockInfo(
    device: nvmlDevice_t,
    type_: nvmlClockType_t,
    clock: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMaxClockInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, type_, clock) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpcClkVfOffset(
    device: nvmlDevice_t,
    offset: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpcClkVfOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, offset) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetApplicationsClock(
    device: nvmlDevice_t,
    clockType: nvmlClockType_t,
    clockMHz: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetApplicationsClock {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, clockType, clockMHz) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDefaultApplicationsClock(
    device: nvmlDevice_t,
    clockType: nvmlClockType_t,
    clockMHz: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDefaultApplicationsClock {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, clockType, clockMHz) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetClock(
    device: nvmlDevice_t,
    clockType: nvmlClockType_t,
    clockId: nvmlClockId_t,
    clockMHz: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetClock {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, clockType, clockId, clockMHz) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMaxCustomerBoostClock(
    device: nvmlDevice_t,
    clockType: nvmlClockType_t,
    clockMHz: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMaxCustomerBoostClock {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, clockType, clockMHz) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSupportedMemoryClocks(
    device: nvmlDevice_t,
    count: *mut ::std::os::raw::c_uint,
    clocksMHz: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSupportedMemoryClocks {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, count, clocksMHz) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSupportedGraphicsClocks(
    device: nvmlDevice_t,
    memoryClockMHz: ::std::os::raw::c_uint,
    count: *mut ::std::os::raw::c_uint,
    clocksMHz: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSupportedGraphicsClocks {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, memoryClockMHz, count, clocksMHz) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAutoBoostedClocksEnabled(
    device: nvmlDevice_t,
    isEnabled: *mut nvmlEnableState_t,
    defaultIsEnabled: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAutoBoostedClocksEnabled {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, isEnabled, defaultIsEnabled) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetFanSpeed(
    device: nvmlDevice_t,
    speed: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetFanSpeed {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, speed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetFanSpeed_v2(
    device: nvmlDevice_t,
    fan: ::std::os::raw::c_uint,
    speed: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetFanSpeed_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, fan, speed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetFanSpeedRPM(
    device: nvmlDevice_t,
    fanSpeed: *mut nvmlFanSpeedInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetFanSpeedRPM {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, fanSpeed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetTargetFanSpeed(
    device: nvmlDevice_t,
    fan: ::std::os::raw::c_uint,
    targetSpeed: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetTargetFanSpeed {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, fan, targetSpeed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMinMaxFanSpeed(
    device: nvmlDevice_t,
    minSpeed: *mut ::std::os::raw::c_uint,
    maxSpeed: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMinMaxFanSpeed {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, minSpeed, maxSpeed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetFanControlPolicy_v2(
    device: nvmlDevice_t,
    fan: ::std::os::raw::c_uint,
    policy: *mut nvmlFanControlPolicy_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetFanControlPolicy_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, fan, policy) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNumFans(
    device: nvmlDevice_t,
    numFans: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNumFans {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, numFans) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetTemperature(
    device: nvmlDevice_t,
    sensorType: nvmlTemperatureSensors_t,
    temp: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetTemperature {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, sensorType, temp) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCoolerInfo(
    device: nvmlDevice_t,
    coolerInfo: *mut nvmlCoolerInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCoolerInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, coolerInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetTemperatureV(
    device: nvmlDevice_t,
    temperature: *mut nvmlTemperature_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetTemperatureV {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, temperature) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetTemperatureThreshold(
    device: nvmlDevice_t,
    thresholdType: nvmlTemperatureThresholds_t,
    temp: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetTemperatureThreshold {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, thresholdType, temp) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMarginTemperature(
    device: nvmlDevice_t,
    marginTempInfo: *mut nvmlMarginTemperature_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMarginTemperature {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, marginTempInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetThermalSettings(
    device: nvmlDevice_t,
    sensorIndex: ::std::os::raw::c_uint,
    pThermalSettings: *mut nvmlGpuThermalSettings_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetThermalSettings {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, sensorIndex, pThermalSettings) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPerformanceState(
    device: nvmlDevice_t,
    pState: *mut nvmlPstates_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPerformanceState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCurrentClocksEventReasons(
    device: nvmlDevice_t,
    clocksEventReasons: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCurrentClocksEventReasons {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, clocksEventReasons) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCurrentClocksThrottleReasons(
    device: nvmlDevice_t,
    clocksThrottleReasons: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCurrentClocksThrottleReasons {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, clocksThrottleReasons) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSupportedClocksEventReasons(
    device: nvmlDevice_t,
    supportedClocksEventReasons: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSupportedClocksEventReasons {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, supportedClocksEventReasons) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSupportedClocksThrottleReasons(
    device: nvmlDevice_t,
    supportedClocksThrottleReasons: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSupportedClocksThrottleReasons {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, supportedClocksThrottleReasons) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPowerState(
    device: nvmlDevice_t,
    pState: *mut nvmlPstates_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPowerState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDynamicPstatesInfo(
    device: nvmlDevice_t,
    pDynamicPstatesInfo: *mut nvmlGpuDynamicPstatesInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDynamicPstatesInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pDynamicPstatesInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMemClkVfOffset(
    device: nvmlDevice_t,
    offset: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMemClkVfOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, offset) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMinMaxClockOfPState(
    device: nvmlDevice_t,
    type_: nvmlClockType_t,
    pstate: nvmlPstates_t,
    minClockMHz: *mut ::std::os::raw::c_uint,
    maxClockMHz: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMinMaxClockOfPState {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(device, type_, pstate, minClockMHz, maxClockMHz)
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSupportedPerformanceStates(
    device: nvmlDevice_t,
    pstates: *mut nvmlPstates_t,
    size: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSupportedPerformanceStates {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pstates, size) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpcClkMinMaxVfOffset(
    device: nvmlDevice_t,
    minOffset: *mut ::std::os::raw::c_int,
    maxOffset: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpcClkMinMaxVfOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, minOffset, maxOffset) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMemClkMinMaxVfOffset(
    device: nvmlDevice_t,
    minOffset: *mut ::std::os::raw::c_int,
    maxOffset: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMemClkMinMaxVfOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, minOffset, maxOffset) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetClockOffsets(
    device: nvmlDevice_t,
    info: *mut nvmlClockOffset_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetClockOffsets {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetClockOffsets(
    device: nvmlDevice_t,
    info: *mut nvmlClockOffset_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetClockOffsets {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPerformanceModes(
    device: nvmlDevice_t,
    perfModes: *mut nvmlDevicePerfModes_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPerformanceModes {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, perfModes) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCurrentClockFreqs(
    device: nvmlDevice_t,
    currentClockFreqs: *mut nvmlDeviceCurrentClockFreqs_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCurrentClockFreqs {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, currentClockFreqs) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPowerManagementMode(
    device: nvmlDevice_t,
    mode: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPowerManagementMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPowerManagementLimit(
    device: nvmlDevice_t,
    limit: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPowerManagementLimit {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, limit) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPowerManagementLimitConstraints(
    device: nvmlDevice_t,
    minLimit: *mut ::std::os::raw::c_uint,
    maxLimit: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPowerManagementLimitConstraints {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, minLimit, maxLimit) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPowerManagementDefaultLimit(
    device: nvmlDevice_t,
    defaultLimit: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPowerManagementDefaultLimit {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, defaultLimit) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPowerUsage(
    device: nvmlDevice_t,
    power: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPowerUsage {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, power) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPowerMizerMode_v1(
    device: nvmlDevice_t,
    powerMizerMode: *mut nvmlDevicePowerMizerModes_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPowerMizerMode_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, powerMizerMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetPowerMizerMode_v1(
    device: nvmlDevice_t,
    powerMizerMode: *mut nvmlDevicePowerMizerModes_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetPowerMizerMode_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, powerMizerMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetTotalEnergyConsumption(
    device: nvmlDevice_t,
    energy: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetTotalEnergyConsumption {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, energy) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetEnforcedPowerLimit(
    device: nvmlDevice_t,
    limit: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetEnforcedPowerLimit {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, limit) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuOperationMode(
    device: nvmlDevice_t,
    current: *mut nvmlGpuOperationMode_t,
    pending: *mut nvmlGpuOperationMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuOperationMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, current, pending) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMemoryInfo(
    device: nvmlDevice_t,
    memory: *mut nvmlMemory_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMemoryInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, memory) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMemoryInfo_v2(
    device: nvmlDevice_t,
    memory: *mut nvmlMemory_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMemoryInfo_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, memory) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetComputeMode(
    device: nvmlDevice_t,
    mode: *mut nvmlComputeMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetComputeMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCudaComputeCapability(
    device: nvmlDevice_t,
    major: *mut ::std::os::raw::c_int,
    minor: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCudaComputeCapability {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, major, minor) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDramEncryptionMode(
    device: nvmlDevice_t,
    current: *mut nvmlDramEncryptionInfo_t,
    pending: *mut nvmlDramEncryptionInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDramEncryptionMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, current, pending) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetDramEncryptionMode(
    device: nvmlDevice_t,
    dramEncryption: *const nvmlDramEncryptionInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetDramEncryptionMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, dramEncryption) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetEccMode(
    device: nvmlDevice_t,
    current: *mut nvmlEnableState_t,
    pending: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetEccMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, current, pending) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDefaultEccMode(
    device: nvmlDevice_t,
    defaultMode: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDefaultEccMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, defaultMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetBoardId(
    device: nvmlDevice_t,
    boardId: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetBoardId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, boardId) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMultiGpuBoard(
    device: nvmlDevice_t,
    multiGpuBool: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMultiGpuBoard {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, multiGpuBool) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetTotalEccErrors(
    device: nvmlDevice_t,
    errorType: nvmlMemoryErrorType_t,
    counterType: nvmlEccCounterType_t,
    eccCounts: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetTotalEccErrors {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, errorType, counterType, eccCounts) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDetailedEccErrors(
    device: nvmlDevice_t,
    errorType: nvmlMemoryErrorType_t,
    counterType: nvmlEccCounterType_t,
    eccCounts: *mut nvmlEccErrorCounts_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDetailedEccErrors {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, errorType, counterType, eccCounts) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMemoryErrorCounter(
    device: nvmlDevice_t,
    errorType: nvmlMemoryErrorType_t,
    counterType: nvmlEccCounterType_t,
    locationType: nvmlMemoryLocation_t,
    count: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMemoryErrorCounter {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(device, errorType, counterType, locationType, count)
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetUtilizationRates(
    device: nvmlDevice_t,
    utilization: *mut nvmlUtilization_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetUtilizationRates {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, utilization) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetEncoderUtilization(
    device: nvmlDevice_t,
    utilization: *mut ::std::os::raw::c_uint,
    samplingPeriodUs: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetEncoderUtilization {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, utilization, samplingPeriodUs) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetEncoderCapacity(
    device: nvmlDevice_t,
    encoderQueryType: nvmlEncoderType_t,
    encoderCapacity: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetEncoderCapacity {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, encoderQueryType, encoderCapacity) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetEncoderStats(
    device: nvmlDevice_t,
    sessionCount: *mut ::std::os::raw::c_uint,
    averageFps: *mut ::std::os::raw::c_uint,
    averageLatency: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetEncoderStats {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(device, sessionCount, averageFps, averageLatency)
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetEncoderSessions(
    device: nvmlDevice_t,
    sessionCount: *mut ::std::os::raw::c_uint,
    sessionInfos: *mut nvmlEncoderSessionInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetEncoderSessions {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, sessionCount, sessionInfos) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDecoderUtilization(
    device: nvmlDevice_t,
    utilization: *mut ::std::os::raw::c_uint,
    samplingPeriodUs: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDecoderUtilization {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, utilization, samplingPeriodUs) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetJpgUtilization(
    device: nvmlDevice_t,
    utilization: *mut ::std::os::raw::c_uint,
    samplingPeriodUs: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetJpgUtilization {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, utilization, samplingPeriodUs) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetOfaUtilization(
    device: nvmlDevice_t,
    utilization: *mut ::std::os::raw::c_uint,
    samplingPeriodUs: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetOfaUtilization {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, utilization, samplingPeriodUs) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetFBCStats(
    device: nvmlDevice_t,
    fbcStats: *mut nvmlFBCStats_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetFBCStats {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, fbcStats) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetFBCSessions(
    device: nvmlDevice_t,
    sessionCount: *mut ::std::os::raw::c_uint,
    sessionInfo: *mut nvmlFBCSessionInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetFBCSessions {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, sessionCount, sessionInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDriverModel_v2(
    device: nvmlDevice_t,
    current: *mut nvmlDriverModel_t,
    pending: *mut nvmlDriverModel_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDriverModel_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, current, pending) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVbiosVersion(
    device: nvmlDevice_t,
    version: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVbiosVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, version, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetBridgeChipInfo(
    device: nvmlDevice_t,
    bridgeHierarchy: *mut nvmlBridgeChipHierarchy_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetBridgeChipInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, bridgeHierarchy) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetComputeRunningProcesses_v3(
    device: nvmlDevice_t,
    infoCount: *mut ::std::os::raw::c_uint,
    infos: *mut nvmlProcessInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetComputeRunningProcesses_v3 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, infoCount, infos) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGraphicsRunningProcesses_v3(
    device: nvmlDevice_t,
    infoCount: *mut ::std::os::raw::c_uint,
    infos: *mut nvmlProcessInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGraphicsRunningProcesses_v3 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, infoCount, infos) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMPSComputeRunningProcesses_v3(
    device: nvmlDevice_t,
    infoCount: *mut ::std::os::raw::c_uint,
    infos: *mut nvmlProcessInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMPSComputeRunningProcesses_v3 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, infoCount, infos) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetRunningProcessDetailList(
    device: nvmlDevice_t,
    plist: *mut nvmlProcessDetailList_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetRunningProcessDetailList {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, plist) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceOnSameBoard(
    device1: nvmlDevice_t,
    device2: nvmlDevice_t,
    onSameBoard: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceOnSameBoard {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device1, device2, onSameBoard) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAPIRestriction(
    device: nvmlDevice_t,
    apiType: nvmlRestrictedAPI_t,
    isRestricted: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAPIRestriction {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, apiType, isRestricted) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSamples(
    device: nvmlDevice_t,
    type_: nvmlSamplingType_t,
    lastSeenTimeStamp: ::std::os::raw::c_ulonglong,
    sampleValType: *mut nvmlValueType_t,
    sampleCount: *mut ::std::os::raw::c_uint,
    samples: *mut nvmlSample_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSamples {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device,
                            type_,
                            lastSeenTimeStamp,
                            sampleValType,
                            sampleCount,
                            samples,
                        )
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetBAR1MemoryInfo(
    device: nvmlDevice_t,
    bar1Memory: *mut nvmlBAR1Memory_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetBAR1MemoryInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, bar1Memory) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetViolationStatus(
    device: nvmlDevice_t,
    perfPolicyType: nvmlPerfPolicyType_t,
    violTime: *mut nvmlViolationTime_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetViolationStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, perfPolicyType, violTime) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetIrqNum(
    device: nvmlDevice_t,
    irqNum: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetIrqNum {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, irqNum) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNumGpuCores(
    device: nvmlDevice_t,
    numCores: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNumGpuCores {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, numCores) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPowerSource(
    device: nvmlDevice_t,
    powerSource: *mut nvmlPowerSource_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPowerSource {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, powerSource) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMemoryBusWidth(
    device: nvmlDevice_t,
    busWidth: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMemoryBusWidth {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, busWidth) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPcieLinkMaxSpeed(
    device: nvmlDevice_t,
    maxSpeed: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPcieLinkMaxSpeed {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, maxSpeed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPcieSpeed(
    device: nvmlDevice_t,
    pcieSpeed: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPcieSpeed {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pcieSpeed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAdaptiveClockInfoStatus(
    device: nvmlDevice_t,
    adaptiveClockStatus: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAdaptiveClockInfoStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, adaptiveClockStatus) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetBusType(
    device: nvmlDevice_t,
    type_: *mut nvmlBusType_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetBusType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, type_) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuFabricInfo(
    device: nvmlDevice_t,
    gpuFabricInfo: *mut nvmlGpuFabricInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuFabricInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, gpuFabricInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuFabricInfoV(
    device: nvmlDevice_t,
    gpuFabricInfo: *mut nvmlGpuFabricInfoV_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuFabricInfoV {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, gpuFabricInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetConfComputeCapabilities(
    capabilities: *mut nvmlConfComputeSystemCaps_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetConfComputeCapabilities {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(capabilities) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetConfComputeState(
    state: *mut nvmlConfComputeSystemState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetConfComputeState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(state) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetConfComputeMemSizeInfo(
    device: nvmlDevice_t,
    memInfo: *mut nvmlConfComputeMemSizeInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetConfComputeMemSizeInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, memInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetConfComputeGpusReadyState(
    isAcceptingWork: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetConfComputeGpusReadyState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(isAcceptingWork) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetConfComputeProtectedMemoryUsage(
    device: nvmlDevice_t,
    memory: *mut nvmlMemory_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetConfComputeProtectedMemoryUsage {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, memory) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetConfComputeGpuCertificate(
    device: nvmlDevice_t,
    gpuCert: *mut nvmlConfComputeGpuCertificate_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetConfComputeGpuCertificate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, gpuCert) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetConfComputeGpuAttestationReport(
    device: nvmlDevice_t,
    gpuAtstReport: *mut nvmlConfComputeGpuAttestationReport_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetConfComputeGpuAttestationReport {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, gpuAtstReport) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetConfComputeKeyRotationThresholdInfo(
    pKeyRotationThrInfo: *mut nvmlConfComputeGetKeyRotationThresholdInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetConfComputeKeyRotationThresholdInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pKeyRotationThrInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetConfComputeUnprotectedMemSize(
    device: nvmlDevice_t,
    sizeKiB: ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetConfComputeUnprotectedMemSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, sizeKiB) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemSetConfComputeGpusReadyState(
    isAcceptingWork: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemSetConfComputeGpusReadyState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(isAcceptingWork) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemSetConfComputeKeyRotationThresholdInfo(
    pKeyRotationThrInfo: *mut nvmlConfComputeSetKeyRotationThresholdInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemSetConfComputeKeyRotationThresholdInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pKeyRotationThrInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetConfComputeSettings(
    settings: *mut nvmlSystemConfComputeSettings_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetConfComputeSettings {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(settings) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGspFirmwareVersion(
    device: nvmlDevice_t,
    version: *mut ::std::os::raw::c_char,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGspFirmwareVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, version) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGspFirmwareMode(
    device: nvmlDevice_t,
    isEnabled: *mut ::std::os::raw::c_uint,
    defaultMode: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGspFirmwareMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, isEnabled, defaultMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSramEccErrorStatus(
    device: nvmlDevice_t,
    status: *mut nvmlEccSramErrorStatus_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSramEccErrorStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, status) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetPowerManagementLimit_v2(
    device: nvmlDevice_t,
    powerValue: *mut nvmlPowerValue_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetPowerManagementLimit_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, powerValue) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAccountingMode(
    device: nvmlDevice_t,
    mode: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAccountingMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAccountingStats(
    device: nvmlDevice_t,
    pid: ::std::os::raw::c_uint,
    stats: *mut nvmlAccountingStats_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAccountingStats {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pid, stats) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAccountingPids(
    device: nvmlDevice_t,
    count: *mut ::std::os::raw::c_uint,
    pids: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAccountingPids {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, count, pids) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAccountingBufferSize(
    device: nvmlDevice_t,
    bufferSize: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAccountingBufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, bufferSize) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetAccountingStats_v2(
    device: nvmlDevice_t,
    stats: *mut nvmlAccountingStats_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetAccountingStats_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, stats) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetRetiredPages(
    device: nvmlDevice_t,
    cause: nvmlPageRetirementCause_t,
    pageCount: *mut ::std::os::raw::c_uint,
    addresses: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetRetiredPages {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, cause, pageCount, addresses) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetRetiredPages_v2(
    device: nvmlDevice_t,
    cause: nvmlPageRetirementCause_t,
    pageCount: *mut ::std::os::raw::c_uint,
    addresses: *mut ::std::os::raw::c_ulonglong,
    timestamps: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetRetiredPages_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(device, cause, pageCount, addresses, timestamps)
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetRetiredPagesPendingStatus(
    device: nvmlDevice_t,
    isPending: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetRetiredPagesPendingStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, isPending) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetRemappedRows(
    device: nvmlDevice_t,
    corrRows: *mut ::std::os::raw::c_uint,
    uncRows: *mut ::std::os::raw::c_uint,
    isPending: *mut ::std::os::raw::c_uint,
    failureOccurred: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetRemappedRows {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(device, corrRows, uncRows, isPending, failureOccurred)
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetRowRemapperHistogram(
    device: nvmlDevice_t,
    values: *mut nvmlRowRemapperHistogramValues_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetRowRemapperHistogram {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, values) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetArchitecture(
    device: nvmlDevice_t,
    arch: *mut nvmlDeviceArchitecture_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetArchitecture {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, arch) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetClkMonStatus(
    device: nvmlDevice_t,
    status: *mut nvmlClkMonStatus_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetClkMonStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, status) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetProcessUtilization(
    device: nvmlDevice_t,
    utilization: *mut nvmlProcessUtilizationSample_t,
    processSamplesCount: *mut ::std::os::raw::c_uint,
    lastSeenTimeStamp: ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetProcessUtilization {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device,
                            utilization,
                            processSamplesCount,
                            lastSeenTimeStamp,
                        )
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetProcessesUtilizationInfo(
    device: nvmlDevice_t,
    procesesUtilInfo: *mut nvmlProcessesUtilizationInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetProcessesUtilizationInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, procesesUtilInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPlatformInfo(
    device: nvmlDevice_t,
    platformInfo: *mut nvmlPlatformInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPlatformInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, platformInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPdi(
    device: nvmlDevice_t,
    pdi: *mut nvmlPdi_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPdi {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pdi) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetHostname_v1(
    device: nvmlDevice_t,
    hostname: *mut nvmlHostname_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetHostname_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, hostname) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetHostname_v1(
    device: nvmlDevice_t,
    hostname: *mut nvmlHostname_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetHostname_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, hostname) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlUnitSetLedState(
    unit: nvmlUnit_t,
    color: nvmlLedColor_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlUnitSetLedState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(unit, color) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetPersistenceMode(
    device: nvmlDevice_t,
    mode: nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetPersistenceMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetComputeMode(
    device: nvmlDevice_t,
    mode: nvmlComputeMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetComputeMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetEccMode(
    device: nvmlDevice_t,
    ecc: nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetEccMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, ecc) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceClearEccErrorCounts(
    device: nvmlDevice_t,
    counterType: nvmlEccCounterType_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceClearEccErrorCounts {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, counterType) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetDriverModel(
    device: nvmlDevice_t,
    driverModel: nvmlDriverModel_t,
    flags: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetDriverModel {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, driverModel, flags) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetGpuLockedClocks(
    device: nvmlDevice_t,
    minGpuClockMHz: ::std::os::raw::c_uint,
    maxGpuClockMHz: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetGpuLockedClocks {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, minGpuClockMHz, maxGpuClockMHz) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceResetGpuLockedClocks(
    device: nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceResetGpuLockedClocks {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetMemoryLockedClocks(
    device: nvmlDevice_t,
    minMemClockMHz: ::std::os::raw::c_uint,
    maxMemClockMHz: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetMemoryLockedClocks {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, minMemClockMHz, maxMemClockMHz) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceResetMemoryLockedClocks(
    device: nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceResetMemoryLockedClocks {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetApplicationsClocks(
    device: nvmlDevice_t,
    memClockMHz: ::std::os::raw::c_uint,
    graphicsClockMHz: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetApplicationsClocks {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, memClockMHz, graphicsClockMHz) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceResetApplicationsClocks(
    device: nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceResetApplicationsClocks {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetAutoBoostedClocksEnabled(
    device: nvmlDevice_t,
    enabled: nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetAutoBoostedClocksEnabled {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, enabled) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetDefaultAutoBoostedClocksEnabled(
    device: nvmlDevice_t,
    enabled: nvmlEnableState_t,
    flags: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetDefaultAutoBoostedClocksEnabled {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, enabled, flags) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetDefaultFanSpeed_v2(
    device: nvmlDevice_t,
    fan: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetDefaultFanSpeed_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, fan) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetFanControlPolicy(
    device: nvmlDevice_t,
    fan: ::std::os::raw::c_uint,
    policy: nvmlFanControlPolicy_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetFanControlPolicy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, fan, policy) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetTemperatureThreshold(
    device: nvmlDevice_t,
    thresholdType: nvmlTemperatureThresholds_t,
    temp: *mut ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetTemperatureThreshold {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, thresholdType, temp) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetPowerManagementLimit(
    device: nvmlDevice_t,
    limit: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetPowerManagementLimit {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, limit) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetGpuOperationMode(
    device: nvmlDevice_t,
    mode: nvmlGpuOperationMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetGpuOperationMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetAPIRestriction(
    device: nvmlDevice_t,
    apiType: nvmlRestrictedAPI_t,
    isRestricted: nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetAPIRestriction {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, apiType, isRestricted) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetFanSpeed_v2(
    device: nvmlDevice_t,
    fan: ::std::os::raw::c_uint,
    speed: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetFanSpeed_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, fan, speed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetGpcClkVfOffset(
    device: nvmlDevice_t,
    offset: ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetGpcClkVfOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, offset) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetMemClkVfOffset(
    device: nvmlDevice_t,
    offset: ::std::os::raw::c_int,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetMemClkVfOffset {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, offset) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetAccountingMode(
    device: nvmlDevice_t,
    mode: nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetAccountingMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceClearAccountingPids(
    device: nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceClearAccountingPids {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkState(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    isActive: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, isActive) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkVersion(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    version: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, version) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkCapability(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    capability: nvmlNvLinkCapability_t,
    capResult: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkCapability {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, capability, capResult) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkRemotePciInfo_v2(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    pci: *mut nvmlPciInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkRemotePciInfo_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, pci) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkErrorCounter(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    counter: nvmlNvLinkErrorCounter_t,
    counterValue: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkErrorCounter {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, counter, counterValue) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceResetNvLinkErrorCounters(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceResetNvLinkErrorCounters {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetNvLinkUtilizationControl(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    counter: ::std::os::raw::c_uint,
    control: *mut nvmlNvLinkUtilizationControl_t,
    reset: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetNvLinkUtilizationControl {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, link, counter, control, reset) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkUtilizationControl(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    counter: ::std::os::raw::c_uint,
    control: *mut nvmlNvLinkUtilizationControl_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkUtilizationControl {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, counter, control) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkUtilizationCounter(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    counter: ::std::os::raw::c_uint,
    rxcounter: *mut ::std::os::raw::c_ulonglong,
    txcounter: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkUtilizationCounter {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, link, counter, rxcounter, txcounter) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceFreezeNvLinkUtilizationCounter(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    counter: ::std::os::raw::c_uint,
    freeze: nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceFreezeNvLinkUtilizationCounter {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, counter, freeze) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceResetNvLinkUtilizationCounter(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    counter: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceResetNvLinkUtilizationCounter {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, counter) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkRemoteDeviceType(
    device: nvmlDevice_t,
    link: ::std::os::raw::c_uint,
    pNvLinkDeviceType: *mut nvmlIntNvLinkDeviceType_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkRemoteDeviceType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, link, pNvLinkDeviceType) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetNvLinkDeviceLowPowerThreshold(
    device: nvmlDevice_t,
    info: *mut nvmlNvLinkPowerThres_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetNvLinkDeviceLowPowerThreshold {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemSetNvlinkBwMode(
    nvlinkBwMode: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemSetNvlinkBwMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nvlinkBwMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemGetNvlinkBwMode(
    nvlinkBwMode: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemGetNvlinkBwMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(nvlinkBwMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvlinkSupportedBwModes(
    device: nvmlDevice_t,
    supportedBwMode: *mut nvmlNvlinkSupportedBwModes_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvlinkSupportedBwModes {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, supportedBwMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvlinkBwMode(
    device: nvmlDevice_t,
    getBwMode: *mut nvmlNvlinkGetBwMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvlinkBwMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, getBwMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetNvlinkBwMode(
    device: nvmlDevice_t,
    setBwMode: *mut nvmlNvlinkSetBwMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetNvlinkBwMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, setBwMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetNvLinkInfo(
    device: nvmlDevice_t,
    info: *mut nvmlNvLinkInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetNvLinkInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlEventSetCreate(set: *mut nvmlEventSet_t) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlEventSetCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(set) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceRegisterEvents(
    device: nvmlDevice_t,
    eventTypes: ::std::os::raw::c_ulonglong,
    set: nvmlEventSet_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceRegisterEvents {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, eventTypes, set) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSupportedEventTypes(
    device: nvmlDevice_t,
    eventTypes: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSupportedEventTypes {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, eventTypes) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlEventSetWait_v2(
    set: nvmlEventSet_t,
    data: *mut nvmlEventData_t,
    timeoutms: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlEventSetWait_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(set, data, timeoutms) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlEventSetFree(set: nvmlEventSet_t) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlEventSetFree {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(set) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemEventSetCreate(
    request: *mut nvmlSystemEventSetCreateRequest_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemEventSetCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(request) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemEventSetFree(
    request: *mut nvmlSystemEventSetFreeRequest_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemEventSetFree {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(request) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemRegisterEvents(
    request: *mut nvmlSystemRegisterEventRequest_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemRegisterEvents {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(request) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSystemEventSetWait(
    request: *mut nvmlSystemEventSetWaitRequest_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSystemEventSetWait {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(request) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceModifyDrainState(
    pciInfo: *mut nvmlPciInfo_t,
    newState: nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceModifyDrainState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pciInfo, newState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceQueryDrainState(
    pciInfo: *mut nvmlPciInfo_t,
    currentState: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceQueryDrainState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pciInfo, currentState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceRemoveGpu_v2(
    pciInfo: *mut nvmlPciInfo_t,
    gpuState: nvmlDetachGpuState_t,
    linkState: nvmlPcieLinkState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceRemoveGpu_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pciInfo, gpuState, linkState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceDiscoverGpus(
    pciInfo: *mut nvmlPciInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceDiscoverGpus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pciInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetFieldValues(
    device: nvmlDevice_t,
    valuesCount: ::std::os::raw::c_int,
    values: *mut nvmlFieldValue_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetFieldValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, valuesCount, values) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceClearFieldValues(
    device: nvmlDevice_t,
    valuesCount: ::std::os::raw::c_int,
    values: *mut nvmlFieldValue_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceClearFieldValues {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, valuesCount, values) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVirtualizationMode(
    device: nvmlDevice_t,
    pVirtualMode: *mut nvmlGpuVirtualizationMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVirtualizationMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pVirtualMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetHostVgpuMode(
    device: nvmlDevice_t,
    pHostVgpuMode: *mut nvmlHostVgpuMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetHostVgpuMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pHostVgpuMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetVirtualizationMode(
    device: nvmlDevice_t,
    virtualMode: nvmlGpuVirtualizationMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetVirtualizationMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, virtualMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuHeterogeneousMode(
    device: nvmlDevice_t,
    pHeterogeneousMode: *mut nvmlVgpuHeterogeneousMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuHeterogeneousMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pHeterogeneousMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetVgpuHeterogeneousMode(
    device: nvmlDevice_t,
    pHeterogeneousMode: *const nvmlVgpuHeterogeneousMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetVgpuHeterogeneousMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pHeterogeneousMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetPlacementId(
    vgpuInstance: nvmlVgpuInstance_t,
    pPlacement: *mut nvmlVgpuPlacementId_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetPlacementId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, pPlacement) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuTypeSupportedPlacements(
    device: nvmlDevice_t,
    vgpuTypeId: nvmlVgpuTypeId_t,
    pPlacementList: *mut nvmlVgpuPlacementList_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuTypeSupportedPlacements {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, vgpuTypeId, pPlacementList) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuTypeCreatablePlacements(
    device: nvmlDevice_t,
    vgpuTypeId: nvmlVgpuTypeId_t,
    pPlacementList: *mut nvmlVgpuPlacementList_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuTypeCreatablePlacements {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, vgpuTypeId, pPlacementList) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetGspHeapSize(
    vgpuTypeId: nvmlVgpuTypeId_t,
    gspHeapSize: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetGspHeapSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, gspHeapSize) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetFbReservation(
    vgpuTypeId: nvmlVgpuTypeId_t,
    fbReservation: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetFbReservation {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, fbReservation) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetRuntimeStateSize(
    vgpuInstance: nvmlVgpuInstance_t,
    pState: *mut nvmlVgpuRuntimeState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetRuntimeStateSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, pState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetVgpuCapabilities(
    device: nvmlDevice_t,
    capability: nvmlDeviceVgpuCapability_t,
    state: nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetVgpuCapabilities {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, capability, state) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceVgpuForceGspUnload(
    device: nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceVgpuForceGspUnload {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGridLicensableFeatures_v4(
    device: nvmlDevice_t,
    pGridLicensableFeatures: *mut nvmlGridLicensableFeatures_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGridLicensableFeatures_v4 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pGridLicensableFeatures) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGetVgpuDriverCapabilities(
    capability: nvmlVgpuDriverCapability_t,
    capResult: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGetVgpuDriverCapabilities {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(capability, capResult) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuCapabilities(
    device: nvmlDevice_t,
    capability: nvmlDeviceVgpuCapability_t,
    capResult: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuCapabilities {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, capability, capResult) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSupportedVgpus(
    device: nvmlDevice_t,
    vgpuCount: *mut ::std::os::raw::c_uint,
    vgpuTypeIds: *mut nvmlVgpuTypeId_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSupportedVgpus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, vgpuCount, vgpuTypeIds) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCreatableVgpus(
    device: nvmlDevice_t,
    vgpuCount: *mut ::std::os::raw::c_uint,
    vgpuTypeIds: *mut nvmlVgpuTypeId_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCreatableVgpus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, vgpuCount, vgpuTypeIds) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetClass(
    vgpuTypeId: nvmlVgpuTypeId_t,
    vgpuTypeClass: *mut ::std::os::raw::c_char,
    size: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetClass {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, vgpuTypeClass, size) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetName(
    vgpuTypeId: nvmlVgpuTypeId_t,
    vgpuTypeName: *mut ::std::os::raw::c_char,
    size: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetName {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, vgpuTypeName, size) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetGpuInstanceProfileId(
    vgpuTypeId: nvmlVgpuTypeId_t,
    gpuInstanceProfileId: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetGpuInstanceProfileId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, gpuInstanceProfileId) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetDeviceID(
    vgpuTypeId: nvmlVgpuTypeId_t,
    deviceID: *mut ::std::os::raw::c_ulonglong,
    subsystemID: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetDeviceID {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, deviceID, subsystemID) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetFramebufferSize(
    vgpuTypeId: nvmlVgpuTypeId_t,
    fbSize: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetFramebufferSize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, fbSize) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetNumDisplayHeads(
    vgpuTypeId: nvmlVgpuTypeId_t,
    numDisplayHeads: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetNumDisplayHeads {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, numDisplayHeads) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetResolution(
    vgpuTypeId: nvmlVgpuTypeId_t,
    displayIndex: ::std::os::raw::c_uint,
    xdim: *mut ::std::os::raw::c_uint,
    ydim: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetResolution {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(vgpuTypeId, displayIndex, xdim, ydim) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetLicense(
    vgpuTypeId: nvmlVgpuTypeId_t,
    vgpuTypeLicenseString: *mut ::std::os::raw::c_char,
    size: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetLicense {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(vgpuTypeId, vgpuTypeLicenseString, size) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetFrameRateLimit(
    vgpuTypeId: nvmlVgpuTypeId_t,
    frameRateLimit: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetFrameRateLimit {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, frameRateLimit) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetMaxInstances(
    device: nvmlDevice_t,
    vgpuTypeId: nvmlVgpuTypeId_t,
    vgpuInstanceCount: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetMaxInstances {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, vgpuTypeId, vgpuInstanceCount) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetMaxInstancesPerVm(
    vgpuTypeId: nvmlVgpuTypeId_t,
    vgpuInstanceCountPerVm: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetMaxInstancesPerVm {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, vgpuInstanceCountPerVm) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetBAR1Info(
    vgpuTypeId: nvmlVgpuTypeId_t,
    bar1Info: *mut nvmlVgpuTypeBar1Info_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetBAR1Info {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, bar1Info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetActiveVgpus(
    device: nvmlDevice_t,
    vgpuCount: *mut ::std::os::raw::c_uint,
    vgpuInstances: *mut nvmlVgpuInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetActiveVgpus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, vgpuCount, vgpuInstances) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetVmID(
    vgpuInstance: nvmlVgpuInstance_t,
    vmId: *mut ::std::os::raw::c_char,
    size: ::std::os::raw::c_uint,
    vmIdType: *mut nvmlVgpuVmIdType_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetVmID {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, vmId, size, vmIdType) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetUUID(
    vgpuInstance: nvmlVgpuInstance_t,
    uuid: *mut ::std::os::raw::c_char,
    size: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetUUID {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, uuid, size) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetVmDriverVersion(
    vgpuInstance: nvmlVgpuInstance_t,
    version: *mut ::std::os::raw::c_char,
    length: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetVmDriverVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, version, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetFbUsage(
    vgpuInstance: nvmlVgpuInstance_t,
    fbUsage: *mut ::std::os::raw::c_ulonglong,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetFbUsage {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, fbUsage) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetLicenseStatus(
    vgpuInstance: nvmlVgpuInstance_t,
    licensed: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetLicenseStatus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, licensed) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetType(
    vgpuInstance: nvmlVgpuInstance_t,
    vgpuTypeId: *mut nvmlVgpuTypeId_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, vgpuTypeId) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetFrameRateLimit(
    vgpuInstance: nvmlVgpuInstance_t,
    frameRateLimit: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetFrameRateLimit {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, frameRateLimit) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetEccMode(
    vgpuInstance: nvmlVgpuInstance_t,
    eccMode: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetEccMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, eccMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetEncoderCapacity(
    vgpuInstance: nvmlVgpuInstance_t,
    encoderCapacity: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetEncoderCapacity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, encoderCapacity) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceSetEncoderCapacity(
    vgpuInstance: nvmlVgpuInstance_t,
    encoderCapacity: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceSetEncoderCapacity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, encoderCapacity) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetEncoderStats(
    vgpuInstance: nvmlVgpuInstance_t,
    sessionCount: *mut ::std::os::raw::c_uint,
    averageFps: *mut ::std::os::raw::c_uint,
    averageLatency: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetEncoderStats {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            vgpuInstance,
                            sessionCount,
                            averageFps,
                            averageLatency,
                        )
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetEncoderSessions(
    vgpuInstance: nvmlVgpuInstance_t,
    sessionCount: *mut ::std::os::raw::c_uint,
    sessionInfo: *mut nvmlEncoderSessionInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetEncoderSessions {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(vgpuInstance, sessionCount, sessionInfo) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetFBCStats(
    vgpuInstance: nvmlVgpuInstance_t,
    fbcStats: *mut nvmlFBCStats_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetFBCStats {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, fbcStats) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetFBCSessions(
    vgpuInstance: nvmlVgpuInstance_t,
    sessionCount: *mut ::std::os::raw::c_uint,
    sessionInfo: *mut nvmlFBCSessionInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetFBCSessions {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(vgpuInstance, sessionCount, sessionInfo) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetGpuInstanceId(
    vgpuInstance: nvmlVgpuInstance_t,
    gpuInstanceId: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetGpuInstanceId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, gpuInstanceId) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetGpuPciId(
    vgpuInstance: nvmlVgpuInstance_t,
    vgpuPciId: *mut ::std::os::raw::c_char,
    length: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetGpuPciId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, vgpuPciId, length) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetCapabilities(
    vgpuTypeId: nvmlVgpuTypeId_t,
    capability: nvmlVgpuCapability_t,
    capResult: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetCapabilities {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuTypeId, capability, capResult) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetMdevUUID(
    vgpuInstance: nvmlVgpuInstance_t,
    mdevUuid: *mut ::std::os::raw::c_char,
    size: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetMdevUUID {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, mdevUuid, size) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetCreatableVgpus(
    gpuInstance: nvmlGpuInstance_t,
    pVgpus: *mut nvmlVgpuTypeIdInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetCreatableVgpus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pVgpus) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuTypeGetMaxInstancesPerGpuInstance(
    pMaxInstance: *mut nvmlVgpuTypeMaxInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuTypeGetMaxInstancesPerGpuInstance {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(pMaxInstance) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetActiveVgpus(
    gpuInstance: nvmlGpuInstance_t,
    pVgpuInstanceInfo: *mut nvmlActiveVgpuInstanceInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetActiveVgpus {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pVgpuInstanceInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceSetVgpuSchedulerState(
    gpuInstance: nvmlGpuInstance_t,
    pScheduler: *mut nvmlVgpuSchedulerState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceSetVgpuSchedulerState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pScheduler) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuSchedulerState(
    gpuInstance: nvmlGpuInstance_t,
    pSchedulerStateInfo: *mut nvmlVgpuSchedulerStateInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetVgpuSchedulerState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pSchedulerStateInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuSchedulerLog(
    gpuInstance: nvmlGpuInstance_t,
    pSchedulerLogInfo: *mut nvmlVgpuSchedulerLogInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetVgpuSchedulerLog {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pSchedulerLogInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuTypeCreatablePlacements(
    gpuInstance: nvmlGpuInstance_t,
    pCreatablePlacementInfo: *mut nvmlVgpuCreatablePlacementInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetVgpuTypeCreatablePlacements {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(gpuInstance, pCreatablePlacementInfo) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuHeterogeneousMode(
    gpuInstance: nvmlGpuInstance_t,
    pHeterogeneousMode: *mut nvmlVgpuHeterogeneousMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetVgpuHeterogeneousMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pHeterogeneousMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceSetVgpuHeterogeneousMode(
    gpuInstance: nvmlGpuInstance_t,
    pHeterogeneousMode: *const nvmlVgpuHeterogeneousMode_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceSetVgpuHeterogeneousMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pHeterogeneousMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuSchedulerState_v2(
    device: nvmlDevice_t,
    pSchedulerStateInfo: *mut nvmlVgpuSchedulerStateInfo_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuSchedulerState_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pSchedulerStateInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuSchedulerState_v2(
    gpuInstance: nvmlGpuInstance_t,
    pSchedulerStateInfo: *mut nvmlVgpuSchedulerStateInfo_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetVgpuSchedulerState_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pSchedulerStateInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuSchedulerLog_v2(
    device: nvmlDevice_t,
    pSchedulerLogInfo: *mut nvmlVgpuSchedulerLogInfo_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuSchedulerLog_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pSchedulerLogInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetVgpuSchedulerLog_v2(
    gpuInstance: nvmlGpuInstance_t,
    pSchedulerLogInfo: *mut nvmlVgpuSchedulerLogInfo_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetVgpuSchedulerLog_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pSchedulerLogInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetVgpuSchedulerState_v2(
    device: nvmlDevice_t,
    pSchedulerState: *mut nvmlVgpuSchedulerState_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetVgpuSchedulerState_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pSchedulerState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceSetVgpuSchedulerState_v2(
    gpuInstance: nvmlGpuInstance_t,
    pSchedulerState: *mut nvmlVgpuSchedulerState_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceSetVgpuSchedulerState_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, pSchedulerState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetMetadata(
    vgpuInstance: nvmlVgpuInstance_t,
    vgpuMetadata: *mut nvmlVgpuMetadata_t,
    bufferSize: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetMetadata {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(vgpuInstance, vgpuMetadata, bufferSize) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuMetadata(
    device: nvmlDevice_t,
    pgpuMetadata: *mut nvmlVgpuPgpuMetadata_t,
    bufferSize: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuMetadata {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pgpuMetadata, bufferSize) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGetVgpuCompatibility(
    vgpuMetadata: *mut nvmlVgpuMetadata_t,
    pgpuMetadata: *mut nvmlVgpuPgpuMetadata_t,
    compatibilityInfo: *mut nvmlVgpuPgpuCompatibility_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGetVgpuCompatibility {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(vgpuMetadata, pgpuMetadata, compatibilityInfo) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetPgpuMetadataString(
    device: nvmlDevice_t,
    pgpuMetadata: *mut ::std::os::raw::c_char,
    bufferSize: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetPgpuMetadataString {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pgpuMetadata, bufferSize) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuSchedulerLog(
    device: nvmlDevice_t,
    pSchedulerLog: *mut nvmlVgpuSchedulerLog_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuSchedulerLog {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pSchedulerLog) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuSchedulerState(
    device: nvmlDevice_t,
    pSchedulerState: *mut nvmlVgpuSchedulerGetState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuSchedulerState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pSchedulerState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetVgpuSchedulerState(
    device: nvmlDevice_t,
    pSchedulerState: *mut nvmlVgpuSchedulerSetState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetVgpuSchedulerState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pSchedulerState) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuSchedulerCapabilities(
    device: nvmlDevice_t,
    pCapabilities: *mut nvmlVgpuSchedulerCapabilities_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuSchedulerCapabilities {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, pCapabilities) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGetVgpuVersion(
    supported: *mut nvmlVgpuVersion_t,
    current: *mut nvmlVgpuVersion_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGetVgpuVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(supported, current) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlSetVgpuVersion(
    vgpuVersion: *mut nvmlVgpuVersion_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlSetVgpuVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuVersion) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuUtilization(
    device: nvmlDevice_t,
    lastSeenTimeStamp: ::std::os::raw::c_ulonglong,
    sampleValType: *mut nvmlValueType_t,
    vgpuInstanceSamplesCount: *mut ::std::os::raw::c_uint,
    utilizationSamples: *mut nvmlVgpuInstanceUtilizationSample_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuUtilization {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device,
                            lastSeenTimeStamp,
                            sampleValType,
                            vgpuInstanceSamplesCount,
                            utilizationSamples,
                        )
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuInstancesUtilizationInfo(
    device: nvmlDevice_t,
    vgpuUtilInfo: *mut nvmlVgpuInstancesUtilizationInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuInstancesUtilizationInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, vgpuUtilInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuProcessUtilization(
    device: nvmlDevice_t,
    lastSeenTimeStamp: ::std::os::raw::c_ulonglong,
    vgpuProcessSamplesCount: *mut ::std::os::raw::c_uint,
    utilizationSamples: *mut nvmlVgpuProcessUtilizationSample_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuProcessUtilization {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device,
                            lastSeenTimeStamp,
                            vgpuProcessSamplesCount,
                            utilizationSamples,
                        )
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetVgpuProcessesUtilizationInfo(
    device: nvmlDevice_t,
    vgpuProcUtilInfo: *mut nvmlVgpuProcessesUtilizationInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetVgpuProcessesUtilizationInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, vgpuProcUtilInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetAccountingMode(
    vgpuInstance: nvmlVgpuInstance_t,
    mode: *mut nvmlEnableState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetAccountingMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, mode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetAccountingPids(
    vgpuInstance: nvmlVgpuInstance_t,
    count: *mut ::std::os::raw::c_uint,
    pids: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetAccountingPids {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, count, pids) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetAccountingStats(
    vgpuInstance: nvmlVgpuInstance_t,
    pid: ::std::os::raw::c_uint,
    stats: *mut nvmlAccountingStats_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetAccountingStats {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, pid, stats) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceClearAccountingPids(
    vgpuInstance: nvmlVgpuInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceClearAccountingPids {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlVgpuInstanceGetLicenseInfo_v2(
    vgpuInstance: nvmlVgpuInstance_t,
    licenseInfo: *mut nvmlVgpuLicenseInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlVgpuInstanceGetLicenseInfo_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(vgpuInstance, licenseInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGetExcludedDeviceCount(
    deviceCount: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGetExcludedDeviceCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(deviceCount) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGetExcludedDeviceInfoByIndex(
    index: ::std::os::raw::c_uint,
    info: *mut nvmlExcludedDeviceInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGetExcludedDeviceInfoByIndex {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(index, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceReadWritePRM_v1(
    device: nvmlDevice_t,
    buffer: *mut nvmlPRMTLV_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceReadWritePRM_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, buffer) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceReadPRMCounters_v1(
    device: nvmlDevice_t,
    counterList: *mut nvmlPRMCounterList_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceReadPRMCounters_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, counterList) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetMigMode(
    device: nvmlDevice_t,
    mode: ::std::os::raw::c_uint,
    activationStatus: *mut nvmlReturn_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetMigMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, mode, activationStatus) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMigMode(
    device: nvmlDevice_t,
    currentMode: *mut ::std::os::raw::c_uint,
    pendingMode: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMigMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, currentMode, pendingMode) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceProfileInfo(
    device: nvmlDevice_t,
    profile: ::std::os::raw::c_uint,
    info: *mut nvmlGpuInstanceProfileInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuInstanceProfileInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, profile, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceProfileInfoV(
    device: nvmlDevice_t,
    profile: ::std::os::raw::c_uint,
    info: *mut nvmlGpuInstanceProfileInfo_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuInstanceProfileInfoV {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, profile, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceProfileInfoByIdV(
    device: nvmlDevice_t,
    profileId: ::std::os::raw::c_uint,
    info: *mut nvmlGpuInstanceProfileInfo_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuInstanceProfileInfoByIdV {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, profileId, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstancePossiblePlacements_v2(
    device: nvmlDevice_t,
    profileId: ::std::os::raw::c_uint,
    placements: *mut nvmlGpuInstancePlacement_t,
    count: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuInstancePossiblePlacements_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, profileId, placements, count) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceRemainingCapacity(
    device: nvmlDevice_t,
    profileId: ::std::os::raw::c_uint,
    count: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuInstanceRemainingCapacity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, profileId, count) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceCreateGpuInstance(
    device: nvmlDevice_t,
    profileId: ::std::os::raw::c_uint,
    gpuInstance: *mut nvmlGpuInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceCreateGpuInstance {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, profileId, gpuInstance) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceCreateGpuInstanceWithPlacement(
    device: nvmlDevice_t,
    profileId: ::std::os::raw::c_uint,
    placement: *const nvmlGpuInstancePlacement_t,
    gpuInstance: *mut nvmlGpuInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceCreateGpuInstanceWithPlacement {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, profileId, placement, gpuInstance) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceDestroy(
    gpuInstance: nvmlGpuInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstances(
    device: nvmlDevice_t,
    profileId: ::std::os::raw::c_uint,
    gpuInstances: *mut nvmlGpuInstance_t,
    count: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuInstances {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, profileId, gpuInstances, count) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceById(
    device: nvmlDevice_t,
    id: ::std::os::raw::c_uint,
    gpuInstance: *mut nvmlGpuInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuInstanceById {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, id, gpuInstance) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetInfo(
    gpuInstance: nvmlGpuInstance_t,
    info: *mut nvmlGpuInstanceInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstanceProfileInfo(
    gpuInstance: nvmlGpuInstance_t,
    profile: ::std::os::raw::c_uint,
    engProfile: ::std::os::raw::c_uint,
    info: *mut nvmlComputeInstanceProfileInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetComputeInstanceProfileInfo {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(gpuInstance, profile, engProfile, info) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstanceProfileInfoV(
    gpuInstance: nvmlGpuInstance_t,
    profile: ::std::os::raw::c_uint,
    engProfile: ::std::os::raw::c_uint,
    info: *mut nvmlComputeInstanceProfileInfo_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetComputeInstanceProfileInfoV {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(gpuInstance, profile, engProfile, info) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstanceRemainingCapacity(
    gpuInstance: nvmlGpuInstance_t,
    profileId: ::std::os::raw::c_uint,
    count: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetComputeInstanceRemainingCapacity {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, profileId, count) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstancePossiblePlacements(
    gpuInstance: nvmlGpuInstance_t,
    profileId: ::std::os::raw::c_uint,
    placements: *mut nvmlComputeInstancePlacement_t,
    count: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetComputeInstancePossiblePlacements {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(gpuInstance, profileId, placements, count) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceCreateComputeInstance(
    gpuInstance: nvmlGpuInstance_t,
    profileId: ::std::os::raw::c_uint,
    computeInstance: *mut nvmlComputeInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceCreateComputeInstance {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(gpuInstance, profileId, computeInstance) }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceCreateComputeInstanceWithPlacement(
    gpuInstance: nvmlGpuInstance_t,
    profileId: ::std::os::raw::c_uint,
    placement: *const nvmlComputeInstancePlacement_t,
    computeInstance: *mut nvmlComputeInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceCreateComputeInstanceWithPlacement {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(gpuInstance, profileId, placement, computeInstance)
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlComputeInstanceDestroy(
    computeInstance: nvmlComputeInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlComputeInstanceDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(computeInstance) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstances(
    gpuInstance: nvmlGpuInstance_t,
    profileId: ::std::os::raw::c_uint,
    computeInstances: *mut nvmlComputeInstance_t,
    count: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetComputeInstances {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(gpuInstance, profileId, computeInstances, count)
                    }
                }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpuInstanceGetComputeInstanceById(
    gpuInstance: nvmlGpuInstance_t,
    id: ::std::os::raw::c_uint,
    computeInstance: *mut nvmlComputeInstance_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpuInstanceGetComputeInstanceById {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpuInstance, id, computeInstance) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlComputeInstanceGetInfo_v2(
    computeInstance: nvmlComputeInstance_t,
    info: *mut nvmlComputeInstanceInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlComputeInstanceGetInfo_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(computeInstance, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceIsMigDeviceHandle(
    device: nvmlDevice_t,
    isMigDevice: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceIsMigDeviceHandle {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, isMigDevice) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetGpuInstanceId(
    device: nvmlDevice_t,
    id: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetGpuInstanceId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, id) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetComputeInstanceId(
    device: nvmlDevice_t,
    id: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetComputeInstanceId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, id) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMaxMigDeviceCount(
    device: nvmlDevice_t,
    count: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMaxMigDeviceCount {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, count) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetMigDeviceHandleByIndex(
    device: nvmlDevice_t,
    index: ::std::os::raw::c_uint,
    migDevice: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetMigDeviceHandleByIndex {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, index, migDevice) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetDeviceHandleFromMigDeviceHandle(
    migDevice: nvmlDevice_t,
    device: *mut nvmlDevice_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetDeviceHandleFromMigDeviceHandle {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(migDevice, device) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpmMetricsGet(
    metricsGet: *mut nvmlGpmMetricsGet_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpmMetricsGet {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(metricsGet) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpmSampleFree(gpmSample: nvmlGpmSample_t) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpmSampleFree {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpmSample) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpmSampleAlloc(
    gpmSample: *mut nvmlGpmSample_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpmSampleAlloc {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(gpmSample) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpmSampleGet(
    device: nvmlDevice_t,
    gpmSample: nvmlGpmSample_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpmSampleGet {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, gpmSample) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpmMigSampleGet(
    device: nvmlDevice_t,
    gpuInstanceId: ::std::os::raw::c_uint,
    gpmSample: nvmlGpmSample_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpmMigSampleGet {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, gpuInstanceId, gpmSample) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpmQueryDeviceSupport(
    device: nvmlDevice_t,
    gpmSupport: *mut nvmlGpmSupport_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpmQueryDeviceSupport {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, gpmSupport) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpmQueryIfStreamingEnabled(
    device: nvmlDevice_t,
    state: *mut ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpmQueryIfStreamingEnabled {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, state) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlGpmSetStreamingEnabled(
    device: nvmlDevice_t,
    state: ::std::os::raw::c_uint,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlGpmSetStreamingEnabled {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, state) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetCapabilities(
    device: nvmlDevice_t,
    caps: *mut nvmlDeviceCapabilities_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetCapabilities {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, caps) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileGetProfilesInfo(
    device: nvmlDevice_t,
    profilesInfo: *mut nvmlWorkloadPowerProfileProfilesInfo_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceWorkloadPowerProfileGetProfilesInfo {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, profilesInfo) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileGetCurrentProfiles(
    device: nvmlDevice_t,
    currentProfiles: *mut nvmlWorkloadPowerProfileCurrentProfiles_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceWorkloadPowerProfileGetCurrentProfiles {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, currentProfiles) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileSetRequestedProfiles(
    device: nvmlDevice_t,
    requestedProfiles: *mut nvmlWorkloadPowerProfileRequestedProfiles_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceWorkloadPowerProfileSetRequestedProfiles {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, requestedProfiles) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileClearRequestedProfiles(
    device: nvmlDevice_t,
    requestedProfiles: *mut nvmlWorkloadPowerProfileRequestedProfiles_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceWorkloadPowerProfileClearRequestedProfiles {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, requestedProfiles) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceWorkloadPowerProfileUpdateProfiles_v1(
    device: nvmlDevice_t,
    updateProfiles: *mut nvmlWorkloadPowerProfileUpdateProfiles_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceWorkloadPowerProfileUpdateProfiles_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, updateProfiles) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDevicePowerSmoothingActivatePresetProfile(
    device: nvmlDevice_t,
    profile: *mut nvmlPowerSmoothingProfile_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDevicePowerSmoothingActivatePresetProfile {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, profile) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDevicePowerSmoothingUpdatePresetProfileParam(
    device: nvmlDevice_t,
    profile: *mut nvmlPowerSmoothingProfile_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDevicePowerSmoothingUpdatePresetProfileParam {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, profile) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDevicePowerSmoothingSetState(
    device: nvmlDevice_t,
    state: *mut nvmlPowerSmoothingState_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDevicePowerSmoothingSetState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, state) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetSramUniqueUncorrectedEccErrorCounts(
    device: nvmlDevice_t,
    errorCounts: *mut nvmlEccSramUniqueUncorrectedErrorCounts_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetSramUniqueUncorrectedEccErrorCounts {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, errorCounts) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceGetRemappedRows_v2(
    device: nvmlDevice_t,
    info: *mut nvmlRemappedRowsInfo_v2_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceGetRemappedRows_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, info) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nvmlDeviceSetRusdSettings_v1(
    device: nvmlDevice_t,
    settings: *mut nvmlRusdSettings_v1_t,
) -> nvmlReturn_t {
    match nvml_api() {
        Ok(__nv_api) => {
            match &__nv_api.nvmlDeviceSetRusdSettings_v1 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, settings) }
                Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            }
        }
        Err(__nv_error) => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
    }
}
