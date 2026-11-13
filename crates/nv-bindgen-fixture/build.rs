//! Generates bindings for nv-bindgen's synthetic `fake.h` exactly as a real sys crate would.
use nv_bindgen::{LibSpec, Sdk, Shape};

const SPEC: LibSpec = LibSpec {
    name: "fake",
    headers: &["fake.h"],
    sdks: &[Sdk::CrateDir("../nv-bindgen/tests/fixture")],
    allow_fn: &["^fake.*"],
    allow_type: &["^fake.*"],
    allow_var: &["^FAKE_.*"],
    reuse_cuda_types: &[],
    positive_enums: &["fakeStatus_enum"],
    shape: Shape::Exports {
        api_type: "FakeApi",
        loader_fn: "fake_api",
        status_types: &["fakeStatus"],
        not_loaded: "fakeStatus_enum_FAKE_ERROR_NOT_LOADED",
        missing_symbol: "fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL",
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
