#[allow(non_snake_case, clippy::missing_safety_doc, clippy::too_many_arguments)]
#[inline]
pub unsafe extern "C" fn nppGetLibVersion() -> *const NppLibraryVersion {
    match nppc_api() {
        Ok(__nv_api) => {
            match &__nv_api.nppGetLibVersion {
                // SAFETY: the caller upholds the native contract and the pointer's library remains loaded for the process.
                Ok(__nv_fn) => unsafe { (*__nv_fn)() }
                Err(__nv_error) => ::core::ptr::null(),
            }
        }
        Err(__nv_error) => ::core::ptr::null(),
    }
}
