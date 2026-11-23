//! Compile-and-run harness for `nv-bindgen` output (T0: no NVIDIA SDK, no GPU).
//!
//! **What.** `build.rs` runs the real generator on the synthetic `fake.h` and this crate compiles
//! the result through the same `nv_loader::exports_sys!` macro every sys crate uses. The library
//! itself never exists (its root variable is never set), so the tests exercise the failure paths
//! of the generated shims at runtime.
//! **Why.** Parsing the generated text is not enough: a C parameter named `f` once shadowed the
//! shim's own binding and only compilation caught it. Every generator bug class found in a real
//! header gets a line in `fake.h` and is locked here (error-to-harness ratchet).

use nv_loader::{DllSpec, Group};

nv_loader::exports_sys! {
    lib: "fake",
    api: FakeApi,
    loader: fake_api,
    dll: DllSpec { lib: "fake", group: Group::Sdk { root_var: "NV_RS_FAKE_ROOT_NEVER_SET", hint: None }, windows: &["fake.dll"], linux: &["libfake-nv-rs-fixture.so"] },
}

/// The query-interface fixture (`qi.h`, NVAPI-shaped), compiled through `query_interface_sys!`.
pub mod qi {
    nv_loader::query_interface_sys! {
        lib: "qi",
        loader: qi_api,
        query_symbol: b"qi_QueryInterface\0",
        dll: nv_loader::DllSpec {
            lib: "qi",
            group: nv_loader::Group::Sdk { root_var: "NV_RS_QI_ROOT_NEVER_SET", hint: None },
            windows: &["qi.dll"],
            linux: &["libqi-nv-rs-fixture.so"],
        },
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn query_shims_return_library_not_found_without_the_library() {
            // SAFETY: the library is absent, so no C code runs.
            unsafe {
                assert_eq!(qiInitialize(), _qiStatus_QI_LIBRARY_NOT_FOUND);
                let mut v = 0;
                // C parameters named `f` and `api` must not shadow the shim's bindings.
                assert_eq!(qiGetValue(&mut v, 1), _qiStatus_QI_LIBRARY_NOT_FOUND);
                let error = qiVersionNumber().expect_err("the query library is absent");
                assert!(matches!(error, nv_loader::CallError::Load(_)));
                assert!(std::error::Error::source(&error).is_some());
                let error = qiStore(&mut v).expect_err("the query library is absent");
                assert!(matches!(error, nv_loader::CallError::Load(_)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nv_loader::CallError;

    #[test]
    fn a_missing_library_is_a_typed_load_error() {
        assert!(matches!(
            load(),
            Err(nv_loader::LoadError::NotFound { .. } | nv_loader::LoadError::Rejected { .. })
        ));
    }

    #[test]
    fn status_shims_return_the_not_loaded_code() {
        // SAFETY: no native library exists, so the generated functions take their load-error arms.
        unsafe {
            assert_eq!(fakeInit(0), fakeStatus_enum_FAKE_ERROR_NOT_LOADED);
            assert_eq!(fakeShadow(1, 2), fakeStatus_enum_FAKE_ERROR_NOT_LOADED);
            let status = fakeIo();
            assert_eq!((status.err, status.cuda), (FAKE_IO_NOT_LOADED as i32, 0));
            assert_eq!(fakeTransfer(), -(FAKE_IO_NOT_LOADED as fakeCount));
        }
    }

    #[test]
    fn pointer_and_void_shims_fall_back_without_panicking() {
        // SAFETY: no native library exists; no C code runs.
        unsafe {
            assert!(fakeErrorString(0).is_null());
            let error = fakeReset().expect_err("void calls report an unavailable library");
            assert!(matches!(error, CallError::Load(_)));
        }
    }

    #[test]
    fn checked_functions_preserve_load_errors_and_have_a_rust_abi() {
        let _: unsafe fn() -> Result<std::ffi::c_int, CallError> = fakeVersionNumber;
        let _: unsafe fn() -> Result<(), CallError> = fakeReset;
        let _: unsafe fn(*mut std::ffi::c_int) -> Result<(), CallError> = fakeStore;
        let _: unsafe extern "C" fn() -> fakeIoStatus = fakeIo;
        let _: unsafe extern "C" fn() -> fakeCount = fakeTransfer;
        // SAFETY: no native library exists; no C code runs.
        let error = unsafe { fakeVersionNumber() }.expect_err("the library is absent");
        assert!(matches!(error, CallError::Load(_)));
        assert!(std::error::Error::source(&error).is_some());
        assert!(INVENTORY.contains("\"unshimmed\": [\n    \"fakeMissingNumber\",\n    \"fakeReset\",\n    \"fakeStore\",\n    \"fakeVersionNumber\"\n  ]"), "{INVENTORY}");
    }

    #[cfg(windows)]
    fn current_process() -> libloading::Library {
        libloading::os::windows::Library::this()
            .expect("the running executable has a module handle")
            .into()
    }

    #[cfg(unix)]
    fn current_process() -> libloading::Library {
        libloading::os::unix::Library::this().into()
    }

    unsafe extern "C" fn shadow(f: std::ffi::c_int, api: std::ffi::c_int) -> fakeStatus {
        if f == 7 && api == 11 {
            fakeStatus_enum_FAKE_SUCCESS
        } else {
            fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL
        }
    }

    unsafe extern "C" fn version() -> std::ffi::c_int {
        FAKE_VERSION as std::ffi::c_int
    }

    unsafe extern "C" fn store(value: *mut std::ffi::c_int) {
        // SAFETY: the fixture callers provide a writable, aligned integer for this call.
        unsafe { value.write(FAKE_VERSION as std::ffi::c_int) };
    }

    fn present_api() -> Result<&'static api::FakeApi, &'static nv_loader::LoadError> {
        static API: std::sync::OnceLock<api::FakeApi> = std::sync::OnceLock::new();
        Ok(API.get_or_init(|| {
            // SAFETY: this is the running executable, not a newly loaded foreign library.
            // Only explicit test function pointers below may run; the failure fields are replaced.
            let mut api = unsafe { api::FakeApi::from_library(current_process()) }
                .expect("optional symbols do not reject a library");
            api.fakeInit = Err(libloading::Error::DlSymUnknown);
            api.fakeCtxCreate = Err(libloading::Error::DlSymUnknown);
            api.fakeGetCount = Err(libloading::Error::DlSymUnknown);
            api.fakeErrorString = Err(libloading::Error::DlSymUnknown);
            api.fakeReset = Err(libloading::Error::DlSymUnknown);
            api.fakeIo = Err(libloading::Error::DlSymUnknown);
            api.fakeTransfer = Err(libloading::Error::DlSymUnknown);
            api.fakeMissingNumber = Err(libloading::Error::DlSymUnknown);
            api.fakeShadow = Ok(shadow);
            api.fakeVersionNumber = Ok(version);
            api.fakeStore = Ok(store);
            api
        }))
    }

    // Generated shim unsafe blocks carry `// SAFETY:` comments emitted by `nv-bindgen`.
    mod present {
        use super::super::types::*;
        use super::present_api as fake_api;
        include!(concat!(env!("OUT_DIR"), "/fake_shims.rs"));
    }

    #[test]
    fn mixed_return_policies_preserve_missing_symbol_errors() {
        // SAFETY: every missing field below contains an explicit error; no C code runs.
        unsafe {
            assert_eq!(
                present::fakeInit(0),
                fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL
            );
            assert_eq!(
                present::fakeCtxCreate(std::ptr::null_mut(), 0),
                fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL
            );
            assert_eq!(
                present::fakeGetCount(std::ptr::null_mut()),
                fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL
            );
            assert!(present::fakeErrorString(0).is_null());
            let error = present::fakeReset().expect_err("void calls report missing symbols");
            assert!(
                std::error::Error::source(&error)
                    .expect("loader cause")
                    .is::<libloading::Error>()
            );
            let CallError::Symbol(diagnostic) = error else {
                panic!("expected a missing void symbol")
            };
            assert_eq!(diagnostic.symbol, "fakeReset");
            let status = present::fakeIo();
            assert_eq!((status.err, status.cuda), (FAKE_IO_MISSING as i32, 0));
            assert_eq!(present::fakeTransfer(), -(FAKE_IO_MISSING as fakeCount));
            let error = present::fakeMissingNumber().expect_err("the field contains an error");
            let source =
                std::error::Error::source(&error).expect("the loader's cause is preserved");
            assert!(
                source.is::<libloading::Error>(),
                "the original cause must pass through the boxed diagnostic"
            );
            let CallError::Symbol(diagnostic) = error else {
                panic!("expected a missing-symbol diagnostic")
            };
            assert_eq!(
                (diagnostic.lib, diagnostic.symbol),
                ("fake", "fakeMissingNumber")
            );
            assert!(matches!(
                diagnostic.source,
                Some(libloading::Error::DlSymUnknown)
            ));
        }
    }

    #[test]
    fn generated_native_and_checked_functions_forward_success() {
        // SAFETY: the test table contains the exact signatures of these local functions.
        unsafe {
            assert_eq!(present::fakeShadow(7, 11), fakeStatus_enum_FAKE_SUCCESS);
            assert_eq!(
                present::fakeVersionNumber().expect("local pointer exists"),
                FAKE_VERSION as std::ffi::c_int
            );
            let mut value = 0;
            present::fakeStore(&mut value).expect("local void pointer exists");
            assert_eq!(value, FAKE_VERSION as std::ffi::c_int);
        }
    }

    struct Query {
        query: unsafe extern "C" fn(u32) -> *mut std::ffi::c_void,
    }

    unsafe extern "C" fn query(id: u32) -> *mut std::ffi::c_void {
        match id {
            0x1234_5678 => version as *const () as *mut std::ffi::c_void,
            0x1234_5680 => store as *const () as *mut std::ffi::c_void,
            _ => std::ptr::null_mut(),
        }
    }

    fn present_query() -> Result<&'static Query, &'static nv_loader::LoadError> {
        static QUERY: Query = Query { query };
        Ok(&QUERY)
    }

    // Same generated-code boundary as exports_sys!/query_interface_sys!, without hiding test code.
    mod present_qi {
        use super::present_query as qi_api;
        use crate::qi::*;
        include!(concat!(env!("OUT_DIR"), "/qi_shims.rs"));
    }

    #[test]
    fn checked_query_functions_distinguish_null_from_success() {
        let _: unsafe fn() -> Result<std::ffi::c_int, CallError> = present_qi::qiVersionNumber;
        let _: unsafe fn(*mut std::ffi::c_int) -> Result<(), CallError> = present_qi::qiStore;
        // SAFETY: fixture ids map only to the exact local version/store functions or null.
        unsafe {
            assert_eq!(
                present_qi::qiVersionNumber().expect("known id"),
                FAKE_VERSION as std::ffi::c_int
            );
            let error = present_qi::qiAbsentNumber().expect_err("the query returns null");
            assert!(std::error::Error::source(&error).is_none());
            let CallError::Symbol(diagnostic) = error else {
                panic!("expected a null-query diagnostic")
            };
            assert_eq!(
                (diagnostic.lib, diagnostic.symbol),
                ("qi", "qiAbsentNumber")
            );
            assert!(diagnostic.source.is_none());
            assert_eq!(
                present_qi::qiInitialize(),
                qi::_qiStatus_QI_NO_IMPLEMENTATION
            );
            assert_eq!(
                present_qi::qiGetValue(std::ptr::null_mut(), 0),
                qi::_qiStatus_QI_NO_IMPLEMENTATION
            );
            let mut value = 0;
            present_qi::qiStore(&mut value).expect("known void id");
            assert_eq!(value, FAKE_VERSION as std::ffi::c_int);
            let error =
                present_qi::qiMissingStore(&mut value).expect_err("the void query returns null");
            assert!(std::error::Error::source(&error).is_none());
            let CallError::Symbol(diagnostic) = error else {
                panic!("expected a missing void query")
            };
            assert_eq!(diagnostic.symbol, "qiMissingStore");
        }
    }
}
