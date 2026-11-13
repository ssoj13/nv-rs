//! NVML: plain exports from the driver's nvml.dll.
use nv_bindgen::{LibSpec, Shape, generate};
fn main() {
    generate(&LibSpec {
        name: "nvml",
        headers: &["nvml.h"],
        include_roots: &[("CUDA_PATH", "include")],
        allow_fn: &["nvml.*"],
        allow_type: &["nvml.*"],
        allow_var: &["NVML_.*"],
        reuse_cuda_types: &[],
        raw_lines: &[],
        shape: Shape::Exports { loader: "NvmlLib" },
    });
}
