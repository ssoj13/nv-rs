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
    dll: DllSpec { lib: "fake", group: Group::Sdk { root_var: "NV_RS_FAKE_ROOT_NEVER_SET" }, windows: &["fake.dll"], linux: &["libfake-nv-rs-fixture.so"] },
}

/// The query-interface fixture (`qi.h`, NVAPI-shaped), compiled through `query_interface_sys!`.
pub mod qi {
    nv_loader::query_interface_sys! {
        lib: "qi",
        loader: qi_api,
        query_symbol: b"qi_QueryInterface\0",
        dll: nv_loader::DllSpec {
            lib: "qi",
            group: nv_loader::Group::Sdk { root_var: "NV_RS_QI_ROOT_NEVER_SET" },
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
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_library_is_a_typed_load_error() {
        assert!(matches!(load(), Err(nv_loader::LoadError::NotFound { .. } | nv_loader::LoadError::Rejected { .. })));
    }

    #[test]
    fn status_shims_return_the_not_loaded_code() {
        // SAFETY: the shims never reach C code here; the library is absent.
        unsafe {
            assert_eq!(fakeInit(0), fakeStatus_enum_FAKE_ERROR_NOT_LOADED);
            // C parameters named `f` and `api` must not shadow the shim's own bindings.
            assert_eq!(fakeShadow(1, 2), fakeStatus_enum_FAKE_ERROR_NOT_LOADED);
        }
    }

    #[test]
    fn pointer_and_void_shims_fall_back_without_panicking() {
        // SAFETY: as above.
        unsafe {
            assert!(fakeErrorString(0).is_null());
            fakeReset();
        }
    }

    #[test]
    fn functions_without_a_failure_value_are_reported_not_shimmed() {
        assert!(INVENTORY.contains("\"unshimmed\": [\n    \"fakeVersionNumber\"\n  ]"), "{INVENTORY}");
    }
}
