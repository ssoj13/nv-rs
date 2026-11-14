//! Generates the NPP bindings from the CUDA Toolkit's `npp.h`, split over NPP's eleven DLLs.
//!
//! The header-to-DLL map was read from the DLLs' export tables (CUDA 13.3): every header's
//! functions live in exactly one DLL; `nppi_linear_transforms.h` ships in `nppist`.
use nv_bindgen::{ExportPart, LibSpec, ReturnPolicy, Sdk, Shape};

const PARTS: &[ExportPart] = &[
    ExportPart { name: "nppc", api_type: "NppcApi", loader_fn: "nppc_api", header_files: &["nppcore.h"] },
    ExportPart {
        name: "npps",
        api_type: "NppsApi",
        loader_fn: "npps_api",
        header_files: &[
            "npps_support_functions.h",
            "npps_initialization.h",
            "npps_conversion_functions.h",
            "npps_arithmetic_and_logical_operations.h",
            "npps_filtering_functions.h",
            "npps_statistics_functions.h",
        ],
    },
    ExportPart { name: "nppisu", api_type: "NppisuApi", loader_fn: "nppisu_api", header_files: &["nppi_support_functions.h"] },
    ExportPart {
        name: "nppidei",
        api_type: "NppideiApi",
        loader_fn: "nppidei_api",
        header_files: &["nppi_data_exchange_and_initialization.h"],
    },
    ExportPart {
        name: "nppial",
        api_type: "NppialApi",
        loader_fn: "nppial_api",
        header_files: &["nppi_arithmetic_and_logical_operations.h"],
    },
    ExportPart { name: "nppicc", api_type: "NppiccApi", loader_fn: "nppicc_api", header_files: &["nppi_color_conversion.h"] },
    ExportPart {
        name: "nppitc",
        api_type: "NppitcApi",
        loader_fn: "nppitc_api",
        header_files: &["nppi_threshold_and_compare_operations.h"],
    },
    ExportPart { name: "nppim", api_type: "NppimApi", loader_fn: "nppim_api", header_files: &["nppi_morphological_operations.h"] },
    ExportPart { name: "nppif", api_type: "NppifApi", loader_fn: "nppif_api", header_files: &["nppi_filtering_functions.h"] },
    ExportPart {
        name: "nppist",
        api_type: "NppistApi",
        loader_fn: "nppist_api",
        header_files: &["nppi_statistics_functions.h", "nppi_linear_transforms.h"],
    },
    ExportPart { name: "nppig", api_type: "NppigApi", loader_fn: "nppig_api", header_files: &["nppi_geometry_transforms.h"] },
];

const SPEC: LibSpec = LibSpec {
    name: "npp",
    headers: &["npp.h"],
    sdks: &[Sdk::CudaToolkit],
    allow_fn: &["^npp.*"],
    allow_type: &["^Npp.*"],
    allow_var: &["^NPP_.*"],
    reuse_cuda_types: &["CUstream_st"],
    positive_enums: &[],
    clang_args: &[],
    opaque_types: &[],
    shape: Shape::SplitExports {
        parts: PARTS,
        returns: &[ReturnPolicy {
            types: &["NppStatus"],
            not_loaded: "NppStatus_NPP_ERROR",
            missing_symbol: "NppStatus_NPP_NOT_IMPLEMENTED_ERROR",
        }],
    },
};

fn main() {
    if let Err(e) = nv_bindgen::generate(&SPEC) {
        println!("cargo::error={e}");
        std::process::exit(1);
    }
}
