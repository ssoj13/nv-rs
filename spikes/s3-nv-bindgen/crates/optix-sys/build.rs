//! OptiX: types only; entry points come from optixQueryFunctionTable at runtime.
use nv_bindgen::{LibSpec, Shape, generate};
fn main() {
    generate(&LibSpec {
        name: "optix",
        headers: &["optix_function_table.h"],
        include_roots: &[("OPTIX_ROOT", "include"), ("CUDA_PATH", "include")],
        allow_fn: &[],
        allow_type: &["Optix.*"],
        allow_var: &["OPTIX_.*"],
        reuse_cuda_types: &[],
        raw_lines: &[],
        shape: Shape::FunctionTable,
    });
}
