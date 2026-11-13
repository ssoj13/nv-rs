//! cuFFT: toolkit exports; cudaStream_t re-uses cuda-bindings' CUstream_st.
use nv_bindgen::{LibSpec, Shape, generate};
fn main() {
    generate(&LibSpec {
        name: "cufft",
        headers: &["cufft.h"],
        include_roots: &[("CUDA_PATH", "include")],
        allow_fn: &["cufft.*"],
        allow_type: &["cufft.*"],
        allow_var: &["CUFFT_.*"],
        reuse_cuda_types: &["CUstream_st"],
        raw_lines: &["pub use cuda_bindings::CUstream_st;"],
        shape: Shape::Exports { loader: "CufftLib" },
    });
}
