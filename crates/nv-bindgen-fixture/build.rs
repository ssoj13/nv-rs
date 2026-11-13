//! Generates bindings for nv-bindgen's synthetic headers exactly as real sys crates would:
//! `fake.h` (exports shape) and `qi.h` + `qi_interface.h` (query-interface shape, like NVAPI).
use nv_bindgen::{LibSpec, Sdk, Shape};

const FIXTURE: &[Sdk] = &[Sdk::CrateDir("../nv-bindgen/tests/fixture")];

const FAKE: LibSpec = LibSpec {
    name: "fake",
    headers: &["fake.h"],
    sdks: FIXTURE,
    allow_fn: &["^fake.*"],
    allow_type: &["^fake.*"],
    allow_var: &["^FAKE_.*"],
    reuse_cuda_types: &[],
    positive_enums: &["fakeStatus_enum"],
    clang_args: &[],
    shape: Shape::Exports {
        api_type: "FakeApi",
        loader_fn: "fake_api",
        status_types: &["fakeStatus"],
        not_loaded: "fakeStatus_enum_FAKE_ERROR_NOT_LOADED",
        missing_symbol: "fakeStatus_enum_FAKE_ERROR_MISSING_SYMBOL",
    },
};

const QI: LibSpec = LibSpec {
    name: "qi",
    headers: &["qi.h"],
    sdks: FIXTURE,
    allow_fn: &["^qi.*"],
    allow_type: &["^qi.*", "^_qi.*"],
    allow_var: &[],
    reuse_cuda_types: &[],
    positive_enums: &[],
    clang_args: &[],
    shape: Shape::QueryInterface {
        interface_header: "qi_interface.h",
        loader_fn: "qi_api",
        status_types: &["qiStatus"],
        not_loaded: "_qiStatus_QI_LIBRARY_NOT_FOUND",
        missing_symbol: "_qiStatus_QI_NO_IMPLEMENTATION",
    },
};

fn main() {
    for spec in [&FAKE, &QI] {
        if let Err(e) = nv_bindgen::generate(spec) {
            println!("cargo::error={e}");
            std::process::exit(1);
        }
    }
}
