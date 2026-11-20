#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetResultString(
    result: CUptiResult,
    str_: *mut *const ::std::os::raw::c_char,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetResultString {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(result, str_) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetErrorMessage(
    result: CUptiResult,
    str_: *mut *const ::std::os::raw::c_char,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetErrorMessage {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(result, str_) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetVersion(version: *mut u32) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(version) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiSupportedDomains(
    domainCount: *mut usize,
    domainTable: *mut CUpti_DomainTable,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiSupportedDomains {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(domainCount, domainTable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiSubscribe(
    subscriber: *mut CUpti_SubscriberHandle,
    callback: CUpti_CallbackFunc,
    userdata: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiSubscribe {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, callback, userdata) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiSubscribe_v2(
    subscriber: *mut CUpti_SubscriberHandle,
    callback: CUpti_CallbackFunc,
    userdata: *mut ::std::os::raw::c_void,
    pParams: *mut CUpti_SubscriberParams,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiSubscribe_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(subscriber, callback, userdata, pParams) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiUnsubscribe(
    subscriber: CUpti_SubscriberHandle,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiUnsubscribe {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetCallbackState(
    enable: *mut u32,
    subscriber: CUpti_SubscriberHandle,
    domain: CUpti_CallbackDomain,
    cbid: CUpti_CallbackId,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetCallbackState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable, subscriber, domain, cbid) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetEnabledCallbacks(
    subscriber: CUpti_SubscriberHandle,
    domain: CUpti_CallbackDomain,
    buffer: *mut CUpti_CallbackId,
    bufferSize: *mut u32,
    enabledCallbacksCount: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetEnabledCallbacks {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            subscriber,
                            domain,
                            buffer,
                            bufferSize,
                            enabledCallbacksCount,
                        )
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEnableCallback(
    enable: u32,
    subscriber: CUpti_SubscriberHandle,
    domain: CUpti_CallbackDomain,
    cbid: CUpti_CallbackId,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEnableCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable, subscriber, domain, cbid) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEnableDomain(
    enable: u32,
    subscriber: CUpti_SubscriberHandle,
    domain: CUpti_CallbackDomain,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEnableDomain {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable, subscriber, domain) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEnableAllDomains(
    enable: u32,
    subscriber: CUpti_SubscriberHandle,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEnableAllDomains {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable, subscriber) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetCallbackName(
    domain: CUpti_CallbackDomain,
    cbid: u32,
    name: *mut *const ::std::os::raw::c_char,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetCallbackName {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(domain, cbid, name) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiSetEventCollectionMode(
    context: CUcontext,
    mode: CUpti_EventCollectionMode,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiSetEventCollectionMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, mode) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDeviceGetAttribute(
    device: CUdevice,
    attrib: CUpti_DeviceAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDeviceGetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, attrib, valueSize, value) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDeviceGetNumEventDomains(
    device: CUdevice,
    numDomains: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDeviceGetNumEventDomains {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, numDomains) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDeviceEnumEventDomains(
    device: CUdevice,
    arraySizeBytes: *mut usize,
    domainArray: *mut CUpti_EventDomainID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDeviceEnumEventDomains {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, arraySizeBytes, domainArray) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDeviceGetEventDomainAttribute(
    device: CUdevice,
    eventDomain: CUpti_EventDomainID,
    attrib: CUpti_EventDomainAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDeviceGetEventDomainAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(device, eventDomain, attrib, valueSize, value) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetNumEventDomains(numDomains: *mut u32) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetNumEventDomains {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(numDomains) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEnumEventDomains(
    arraySizeBytes: *mut usize,
    domainArray: *mut CUpti_EventDomainID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEnumEventDomains {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(arraySizeBytes, domainArray) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventDomainGetAttribute(
    eventDomain: CUpti_EventDomainID,
    attrib: CUpti_EventDomainAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventDomainGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(eventDomain, attrib, valueSize, value) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventDomainGetNumEvents(
    eventDomain: CUpti_EventDomainID,
    numEvents: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventDomainGetNumEvents {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventDomain, numEvents) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventDomainEnumEvents(
    eventDomain: CUpti_EventDomainID,
    arraySizeBytes: *mut usize,
    eventArray: *mut CUpti_EventID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventDomainEnumEvents {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(eventDomain, arraySizeBytes, eventArray) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGetAttribute(
    event: CUpti_EventID,
    attrib: CUpti_EventAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(event, attrib, valueSize, value) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGetIdFromName(
    device: CUdevice,
    eventName: *const ::std::os::raw::c_char,
    event: *mut CUpti_EventID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGetIdFromName {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, eventName, event) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupCreate(
    context: CUcontext,
    eventGroup: *mut CUpti_EventGroup,
    flags: u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupCreate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, eventGroup, flags) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupDestroy(
    eventGroup: CUpti_EventGroup,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroup) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupGetAttribute(
    eventGroup: CUpti_EventGroup,
    attrib: CUpti_EventGroupAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupGetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(eventGroup, attrib, valueSize, value) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupSetAttribute(
    eventGroup: CUpti_EventGroup,
    attrib: CUpti_EventGroupAttribute,
    valueSize: usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupSetAttribute {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(eventGroup, attrib, valueSize, value) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupAddEvent(
    eventGroup: CUpti_EventGroup,
    event: CUpti_EventID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupAddEvent {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroup, event) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupRemoveEvent(
    eventGroup: CUpti_EventGroup,
    event: CUpti_EventID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupRemoveEvent {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroup, event) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupRemoveAllEvents(
    eventGroup: CUpti_EventGroup,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupRemoveAllEvents {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroup) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupResetAllEvents(
    eventGroup: CUpti_EventGroup,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupResetAllEvents {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroup) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupEnable(
    eventGroup: CUpti_EventGroup,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupEnable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroup) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupDisable(
    eventGroup: CUpti_EventGroup,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupDisable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroup) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupReadEvent(
    eventGroup: CUpti_EventGroup,
    flags: CUpti_ReadEventFlags,
    event: CUpti_EventID,
    eventValueBufferSizeBytes: *mut usize,
    eventValueBuffer: *mut u64,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupReadEvent {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            eventGroup,
                            flags,
                            event,
                            eventValueBufferSizeBytes,
                            eventValueBuffer,
                        )
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupReadAllEvents(
    eventGroup: CUpti_EventGroup,
    flags: CUpti_ReadEventFlags,
    eventValueBufferSizeBytes: *mut usize,
    eventValueBuffer: *mut u64,
    eventIdArraySizeBytes: *mut usize,
    eventIdArray: *mut CUpti_EventID,
    numEventIdsRead: *mut usize,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupReadAllEvents {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            eventGroup,
                            flags,
                            eventValueBufferSizeBytes,
                            eventValueBuffer,
                            eventIdArraySizeBytes,
                            eventIdArray,
                            numEventIdsRead,
                        )
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupSetsCreate(
    context: CUcontext,
    eventIdArraySizeBytes: usize,
    eventIdArray: *mut CUpti_EventID,
    eventGroupPasses: *mut *mut CUpti_EventGroupSets,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupSetsCreate {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            context,
                            eventIdArraySizeBytes,
                            eventIdArray,
                            eventGroupPasses,
                        )
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupSetsDestroy(
    eventGroupSets: *mut CUpti_EventGroupSets,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupSetsDestroy {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroupSets) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupSetEnable(
    eventGroupSet: *mut CUpti_EventGroupSet,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupSetEnable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroupSet) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEventGroupSetDisable(
    eventGroupSet: *mut CUpti_EventGroupSet,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEventGroupSetDisable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(eventGroupSet) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEnableKernelReplayMode(context: CUcontext) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEnableKernelReplayMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDisableKernelReplayMode(
    context: CUcontext,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDisableKernelReplayMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiKernelReplaySubscribeUpdate(
    updateFunc: CUpti_KernelReplayUpdateFunc,
    customData: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiKernelReplaySubscribeUpdate {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(updateFunc, customData) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetNumMetrics(numMetrics: *mut u32) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetNumMetrics {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(numMetrics) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiEnumMetrics(
    arraySizeBytes: *mut usize,
    metricArray: *mut CUpti_MetricID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiEnumMetrics {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(arraySizeBytes, metricArray) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDeviceGetNumMetrics(
    device: CUdevice,
    numMetrics: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDeviceGetNumMetrics {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, numMetrics) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDeviceEnumMetrics(
    device: CUdevice,
    arraySizeBytes: *mut usize,
    metricArray: *mut CUpti_MetricID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDeviceEnumMetrics {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, arraySizeBytes, metricArray) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricGetAttribute(
    metric: CUpti_MetricID,
    attrib: CUpti_MetricAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricGetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(metric, attrib, valueSize, value) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricGetIdFromName(
    device: CUdevice,
    metricName: *const ::std::os::raw::c_char,
    metric: *mut CUpti_MetricID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricGetIdFromName {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(device, metricName, metric) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricGetNumEvents(
    metric: CUpti_MetricID,
    numEvents: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricGetNumEvents {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(metric, numEvents) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricEnumEvents(
    metric: CUpti_MetricID,
    eventIdArraySizeBytes: *mut usize,
    eventIdArray: *mut CUpti_EventID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricEnumEvents {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(metric, eventIdArraySizeBytes, eventIdArray) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricGetNumProperties(
    metric: CUpti_MetricID,
    numProp: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricGetNumProperties {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(metric, numProp) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricEnumProperties(
    metric: CUpti_MetricID,
    propIdArraySizeBytes: *mut usize,
    propIdArray: *mut CUpti_MetricPropertyID,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricEnumProperties {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(metric, propIdArraySizeBytes, propIdArray) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricGetRequiredEventGroupSets(
    context: CUcontext,
    metric: CUpti_MetricID,
    eventGroupSets: *mut *mut CUpti_EventGroupSets,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricGetRequiredEventGroupSets {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, metric, eventGroupSets) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricCreateEventGroupSets(
    context: CUcontext,
    metricIdArraySizeBytes: usize,
    metricIdArray: *mut CUpti_MetricID,
    eventGroupPasses: *mut *mut CUpti_EventGroupSets,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricCreateEventGroupSets {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            context,
                            metricIdArraySizeBytes,
                            metricIdArray,
                            eventGroupPasses,
                        )
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricGetValue(
    device: CUdevice,
    metric: CUpti_MetricID,
    eventIdArraySizeBytes: usize,
    eventIdArray: *mut CUpti_EventID,
    eventValueArraySizeBytes: usize,
    eventValueArray: *mut u64,
    timeDuration: u64,
    metricValue: *mut CUpti_MetricValue,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricGetValue {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            device,
                            metric,
                            eventIdArraySizeBytes,
                            eventIdArray,
                            eventValueArraySizeBytes,
                            eventValueArray,
                            timeDuration,
                            metricValue,
                        )
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiMetricGetValue2(
    metric: CUpti_MetricID,
    eventIdArraySizeBytes: usize,
    eventIdArray: *mut CUpti_EventID,
    eventValueArraySizeBytes: usize,
    eventValueArray: *mut u64,
    propIdArraySizeBytes: usize,
    propIdArray: *mut CUpti_MetricPropertyID,
    propValueArraySizeBytes: usize,
    propValueArray: *mut u64,
    metricValue: *mut CUpti_MetricValue,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiMetricGetValue2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(
                            metric,
                            eventIdArraySizeBytes,
                            eventIdArray,
                            eventValueArraySizeBytes,
                            eventValueArray,
                            propIdArraySizeBytes,
                            propIdArray,
                            propValueArraySizeBytes,
                            propValueArray,
                            metricValue,
                        )
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetTimestamp(timestamp: *mut u64) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetTimestamp {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(timestamp) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetTimestamp_v2(
    subscriber: CUpti_SubscriberHandle,
    timestamp: *mut u64,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetTimestamp_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, timestamp) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetContextId(
    context: CUcontext,
    contextId: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetContextId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, contextId) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetStreamId(
    context: CUcontext,
    stream: CUstream,
    streamId: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetStreamId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, stream, streamId) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetStreamIdEx(
    context: CUcontext,
    stream: CUstream,
    perThreadStream: u8,
    streamId: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetStreamIdEx {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(context, stream, perThreadStream, streamId) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetDeviceId(
    context: CUcontext,
    deviceId: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetDeviceId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, deviceId) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetGraphNodeId(
    node: CUgraphNode,
    nodeId: *mut u64,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetGraphNodeId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(node, nodeId) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetGraphId(graph: CUgraph, pId: *mut u32) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetGraphId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(graph, pId) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetGraphExecId(
    graphExec: CUgraphExec,
    pId: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetGraphExecId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(graphExec, pId) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnable(kind: CUpti_ActivityKind) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(kind) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableAndDump(
    kind: CUpti_ActivityKind,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableAndDump {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(kind) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityDisable(kind: CUpti_ActivityKind) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityDisable {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(kind) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnable_v2(
    subscriber: CUpti_SubscriberHandle,
    kind: CUpti_ActivityKind,
    pActivityConfig: *mut CUpti_ActivityConfig,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnable_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, kind, pActivityConfig) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityDisable_v2(
    subscriber: CUpti_SubscriberHandle,
    kind: CUpti_ActivityKind,
    pActivityConfig: *mut CUpti_ActivityConfig,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityDisable_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, kind, pActivityConfig) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableAndDump_v2(
    subscriber: CUpti_SubscriberHandle,
    kind: CUpti_ActivityKind,
    activityConfig: *mut CUpti_ActivityConfig,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableAndDump_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, kind, activityConfig) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityGetEnabledKinds(
    subscriber: CUpti_SubscriberHandle,
    buffer: *mut CUpti_ActivityKind,
    bufferSize: *mut u32,
    enabledKindsCount: *mut u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityGetEnabledKinds {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(subscriber, buffer, bufferSize, enabledKindsCount)
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityGetStructSize(
    activityKind: CUpti_ActivityKind,
    version: u32,
    activityStructSize: *mut usize,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityGetStructSize {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(activityKind, version, activityStructSize) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiIsTracingSessionRunning(
    isRunning: *mut u8,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiIsTracingSessionRunning {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(isRunning) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableContext(
    context: CUcontext,
    kind: CUpti_ActivityKind,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableContext {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, kind) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityDisableContext(
    context: CUcontext,
    kind: CUpti_ActivityKind,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityDisableContext {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, kind) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityGetNumDroppedRecords(
    context: CUcontext,
    streamId: u32,
    dropped: *mut usize,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityGetNumDroppedRecords {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, streamId, dropped) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityGetNumDroppedRecords_v2(
    subscriber: CUpti_SubscriberHandle,
    context: CUcontext,
    streamId: u32,
    dropped: *mut usize,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityGetNumDroppedRecords_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(subscriber, context, streamId, dropped) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityGetNextRecord(
    buffer: *mut u8,
    validBufferSizeBytes: usize,
    record: *mut *mut CUpti_Activity,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityGetNextRecord {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(buffer, validBufferSizeBytes, record) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityGetNextRecord_v2(
    subscriber: CUpti_SubscriberHandle,
    buffer: *mut u8,
    validBufferSizeBytes: usize,
    record: *mut *mut CUpti_Activity,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityGetNextRecord_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(subscriber, buffer, validBufferSizeBytes, record)
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityRegisterCallbacks(
    funcBufferRequested: CUpti_BuffersCallbackRequestFunc,
    funcBufferCompleted: CUpti_BuffersCallbackCompleteFunc,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityRegisterCallbacks {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe { (*__nv_fn)(funcBufferRequested, funcBufferCompleted) }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityRegisterCallbacks_v2(
    subscriber: CUpti_SubscriberHandle,
    funcBufferRequested: CUpti_BuffersCallbackRequestFunc_v2,
    funcBufferCompleted: CUpti_BuffersCallbackCompleteFunc_v2,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityRegisterCallbacks_v2 {
                Ok(__nv_fn) => {
                    // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                    unsafe {
                        (*__nv_fn)(subscriber, funcBufferRequested, funcBufferCompleted)
                    }
                }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityFlush(
    context: CUcontext,
    streamId: u32,
    flag: u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityFlush {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, streamId, flag) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityFlushAll(flag: u32) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityFlushAll {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(flag) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityGetAttribute(
    attr: CUpti_ActivityAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityGetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(attr, valueSize, value) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityGetAttribute_v2(
    subscriber: CUpti_SubscriberHandle,
    attr: CUpti_ActivityAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityGetAttribute_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, attr, valueSize, value) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivitySetAttribute(
    attr: CUpti_ActivityAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivitySetAttribute {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(attr, valueSize, value) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivitySetAttribute_v2(
    subscriber: CUpti_SubscriberHandle,
    attr: CUpti_ActivityAttribute,
    valueSize: *mut usize,
    value: *mut ::std::os::raw::c_void,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivitySetAttribute_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, attr, valueSize, value) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityConfigureUnifiedMemoryCounter(
    config: *mut CUpti_ActivityUnifiedMemoryCounterConfig,
    count: u32,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityConfigureUnifiedMemoryCounter {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(config, count) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetAutoBoostState(
    context: CUcontext,
    state: *mut CUpti_ActivityAutoBoostState,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetAutoBoostState {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(context, state) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityConfigurePCSampling(
    ctx: CUcontext,
    config: *mut CUpti_ActivityPCSamplingConfig,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityConfigurePCSampling {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(ctx, config) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetLastError() -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetLastError {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiSetThreadIdType(
    type_: CUpti_ActivityThreadIdType,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiSetThreadIdType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiGetThreadIdType(
    type_: *mut CUpti_ActivityThreadIdType,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiGetThreadIdType {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(type_) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiComputeCapabilitySupported(
    major: ::std::os::raw::c_int,
    minor: ::std::os::raw::c_int,
    support: *mut ::std::os::raw::c_int,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiComputeCapabilitySupported {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(major, minor, support) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDeviceSupported(
    dev: CUdevice,
    support: *mut ::std::os::raw::c_int,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDeviceSupported {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dev, support) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiDeviceVirtualizationMode(
    dev: CUdevice,
    mode: *mut CUpti_DeviceVirtualizationMode,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiDeviceVirtualizationMode {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(dev, mode) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiFinalize() -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiFinalize {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityPushExternalCorrelationId(
    kind: CUpti_ExternalCorrelationKind,
    id: u64,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityPushExternalCorrelationId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(kind, id) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityPopExternalCorrelationId(
    kind: CUpti_ExternalCorrelationKind,
    lastId: *mut u64,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityPopExternalCorrelationId {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(kind, lastId) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityPushExternalCorrelationId_v2(
    subscriber: CUpti_SubscriberHandle,
    kind: CUpti_ExternalCorrelationKind,
    id: u64,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityPushExternalCorrelationId_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, kind, id) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityPopExternalCorrelationId_v2(
    subscriber: CUpti_SubscriberHandle,
    kind: CUpti_ExternalCorrelationKind,
    lastId: *mut u64,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityPopExternalCorrelationId_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, kind, lastId) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableLatencyTimestamps(
    enable: u8,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableLatencyTimestamps {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityFlushPeriod(time: u32) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityFlushPeriod {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(time) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableLaunchAttributes(enable: u8) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableLaunchAttributes {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityRegisterTimestampCallback(
    funcTimestamp: CUpti_TimestampCallbackFunc,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityRegisterTimestampCallback {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(funcTimestamp) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableDeviceGraph(enable: u8) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableDeviceGraph {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableDriverApi(
    cbid: CUpti_CallbackId,
    enable: u8,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableDriverApi {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(cbid, enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableDriverApi_v2(
    subscriber: CUpti_SubscriberHandle,
    cbid: CUpti_CallbackId,
    enable: u8,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableDriverApi_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, cbid, enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableRuntimeApi(
    cbid: CUpti_CallbackId,
    enable: u8,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableRuntimeApi {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(cbid, enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableRuntimeApi_v2(
    subscriber: CUpti_SubscriberHandle,
    cbid: CUpti_CallbackId,
    enable: u8,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableRuntimeApi_v2 {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(subscriber, cbid, enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableHWTrace(enable: u8) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableHWTrace {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableAllSyncRecords(enable: u8) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableAllSyncRecords {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn cuptiActivityEnableCudaEventDeviceTimestamps(
    enable: u8,
) -> CUptiResult {
    match cupti_api() {
        Ok(__nv_api) => {
            match &__nv_api.cuptiActivityEnableCudaEventDeviceTimestamps {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)(enable) }
                Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_SUPPORTED,
            }
        }
        Err(__nv_error) => CUptiResult_CUPTI_ERROR_NOT_INITIALIZED,
    }
}
