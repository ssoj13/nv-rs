//! Safe cuBLAS and cuBLASLt host API on top of `cublas-sys`, `cublaslt-sys`, and `cuda-core`.
//!
//! **What.**
//! - [`handle`] — [`Handle`], [`Operation`], [`PointerMode`], [`MathMode`], [`DataType`],
//!   [`ComputeType`], and [`GemmAlgo`].
//! - [`gemm`] — `sgemm`, `dgemm`, `sgemm_batched`, `dgemm_batched`, `sgemm_strided_batched`,
//!   `dgemm_strided_batched`, and `gemm_ex` / `gemm_strided_batched_ex`.
//! - [`lt`] — [`LtHandle`], [`MatmulDesc`], [`MatrixLayout`], [`Preference`], [`HeuristicResult`],
//!   [`Order`], and [`Epilogue`].
//! - [`scratch`] — [`Scratch`] caller-owned reusable device workspace with byte-budget checks.
//! - [`error`] — [`Status`] (`Copy` integer newtype) and [`Error`].
//!
//! **Conventions.** Every handle owns its native cuBLAS/cuBLASLt object, holds `Arc<Context>`, and
//! destroys it on `Drop` while recording any teardown error on the CUDA context. Enqueueing calls
//! take `&Stream` and perform zero heap allocations.
//! **Used by** dense linear algebra workloads and the nv-rs T0/T3 test suite.

pub mod error;
pub mod gemm;
pub mod handle;
pub mod lt;
pub mod scratch;

pub use error::{Error, Result, Status};
pub use handle::{ComputeType, DataType, GemmAlgo, Handle, MathMode, Operation, PointerMode};
pub use lt::{
    AlgoCapAttr, AlgoConfigAttr, BatchMode, EmulationDesc, EmulationDescAttr, Epilogue,
    HeuristicResult, LibraryProperty, LoggerCallback, LtHandle, MatmulAlgo, MatmulDesc,
    MatmulDescAttr, MatrixLayout, MatrixLayoutAttr, MatrixTransformDesc, MatrixTransformDescAttr,
    Order, Preference, PreferenceAttr, ReductionScheme, SearchMode,
};
pub use scratch::Scratch;

pub use cublas_sys as sys;
pub use cublaslt_sys as lt_sys;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gemm::{validate_gemm_dims, validate_strided_batch};

    #[test]
    fn status_newtype_is_copy_and_preserves_unknown_codes() {
        let s = Status(9999);
        let copy = s;
        assert_eq!(s, copy);
        assert!(!s.is_success());
        assert!(Status::SUCCESS.is_success());
        assert_eq!(
            Status::INVALID_VALUE.static_name(),
            "CUBLAS_STATUS_INVALID_VALUE"
        );
        assert!(s.to_string().contains("9999"));
    }

    #[test]
    fn error_formatting_covers_all_variants() {
        let e1 = Error::Unavailable("missing cublas64_13.dll".into());
        assert!(e1.to_string().contains("missing cublas64_13.dll"));

        let e2 = Error::Cublas {
            call: "cublasSgemm_v2",
            code: Status::INVALID_VALUE,
        };
        let msg2 = e2.to_string();
        assert!(msg2.contains("cublasSgemm_v2"));
        assert!(msg2.contains("CUBLAS_STATUS_INVALID_VALUE"));

        let e3 = Error::Budget {
            requested: 4096,
            available: 1024,
        };
        assert!(e3.to_string().contains("4096") && e3.to_string().contains("1024"));

        let e4 = Error::Invalid("bad ld");
        assert!(e4.to_string().contains("bad ld"));
    }

    #[test]
    fn gemm_dim_validation_enforces_leading_dims_and_non_negative_extents() {
        // Valid 3x4 = (3x2) * (2x4) non-transposed
        let (a, b, c) = validate_gemm_dims(3, 4, 2, Operation::N, Operation::N, 3, 2, 3).unwrap();
        assert_eq!((a, b, c), (6, 8, 12));

        // Transposed A: A is k x m = 2 x 3 -> lda >= 2
        let (a, b, c) = validate_gemm_dims(3, 4, 2, Operation::T, Operation::N, 2, 2, 3).unwrap();
        assert_eq!((a, b, c), (6, 8, 12));

        // Rejects lda < rows(op(A))
        assert!(matches!(
            validate_gemm_dims(4, 3, 2, Operation::N, Operation::N, 3, 2, 4),
            Err(Error::Invalid(_))
        ));
        // Rejects negative m
        assert!(matches!(
            validate_gemm_dims(-1, 3, 2, Operation::N, Operation::N, 1, 2, 1),
            Err(Error::Invalid(_))
        ));
        // Rejects ldc < m
        assert!(matches!(
            validate_gemm_dims(4, 3, 2, Operation::N, Operation::N, 4, 2, 3),
            Err(Error::Invalid(_))
        ));
    }

    #[test]
    fn strided_batch_validation_detects_negative_and_overflow() {
        let (ta, tb, tc) = validate_strided_batch(6, 6, 8, 8, 12, 12, 4).unwrap();
        assert_eq!((ta, tb, tc), (24, 32, 48));
        assert_eq!(
            validate_strided_batch(6, 6, 8, 8, 12, 12, 0).unwrap(),
            (0, 0, 0)
        );
        assert!(matches!(
            validate_strided_batch(6, -1, 8, 8, 12, 12, 2),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            validate_strided_batch(6, 6, 8, 8, 12, 12, -1),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            validate_strided_batch(usize::MAX, 1, 1, 1, 1, 1, 2),
            Err(Error::Invalid(_))
        ));
    }

    #[test]
    fn datatype_element_bytes_and_layout_invariants() {
        assert_eq!(DataType::F16.element_bytes(), 2);
        assert_eq!(DataType::Bf16.element_bytes(), 2);
        assert_eq!(DataType::F32.element_bytes(), 4);
        assert_eq!(DataType::F64.element_bytes(), 8);
        assert_eq!(DataType::C32F.element_bytes(), 8);
        assert_eq!(DataType::C64F.element_bytes(), 16);
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cublas64_13.dll; run via xtask gpu-test"]
    fn gpu_sgemm_and_dgemm_match_cpu_reference() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        let handle = Handle::new(&ctx).expect("cublas handle");
        assert!(handle.version().expect("version") > 0);

        // Pointer mode roundtrip
        handle
            .set_pointer_mode(PointerMode::Device)
            .expect("set device mode");
        assert_eq!(
            handle.pointer_mode().expect("get mode"),
            PointerMode::Device
        );
        handle
            .set_pointer_mode(PointerMode::Host)
            .expect("set host mode");
        assert_eq!(handle.pointer_mode().expect("get mode"), PointerMode::Host);

        // Column-major C(3x4) = 1.5 * A(3x2) * B(2x4) + 0.5 * C(3x4)
        let (m, n, k) = (3i32, 4i32, 2i32);
        let a_host: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let b_host: Vec<f32> = vec![0.5, 1.0, 1.5, 2.0, -1.0, 0.5, 2.0, -0.5];
        let c_init: Vec<f32> = vec![1.0; (m * n) as usize];
        let (alpha, beta) = (1.5f32, 0.5f32);

        let a_dev = cuda_core::DeviceBuffer::from_host(&stream, &a_host).unwrap();
        let b_dev = cuda_core::DeviceBuffer::from_host(&stream, &b_host).unwrap();
        let mut c_dev = cuda_core::DeviceBuffer::from_host(&stream, &c_init).unwrap();

        handle
            .sgemm(
                &stream,
                Operation::N,
                Operation::N,
                m,
                n,
                k,
                alpha,
                &a_dev,
                m,
                &b_dev,
                k,
                beta,
                &mut c_dev,
                m,
            )
            .unwrap();
        let c_gpu = c_dev.to_host_vec(&stream).unwrap();

        for col in 0..(n as usize) {
            for row in 0..(m as usize) {
                let mut acc = 0.0f32;
                for p in 0..(k as usize) {
                    acc += a_host[p * (m as usize) + row] * b_host[col * (k as usize) + p];
                }
                let expected = alpha * acc + beta * c_init[col * (m as usize) + row];
                assert!(
                    (c_gpu[col * (m as usize) + row] - expected).abs() < 1e-5,
                    "sgemm mismatch at ({row},{col})"
                );
            }
        }

        // DGEMM check
        let a_f64: Vec<f64> = a_host.iter().map(|&x| f64::from(x)).collect();
        let b_f64: Vec<f64> = b_host.iter().map(|&x| f64::from(x)).collect();
        let c_f64_init: Vec<f64> = c_init.iter().map(|&x| f64::from(x)).collect();
        let a_d = cuda_core::DeviceBuffer::from_host(&stream, &a_f64).unwrap();
        let b_d = cuda_core::DeviceBuffer::from_host(&stream, &b_f64).unwrap();
        let mut c_d = cuda_core::DeviceBuffer::from_host(&stream, &c_f64_init).unwrap();
        handle
            .dgemm(
                &stream,
                Operation::N,
                Operation::N,
                m,
                n,
                k,
                1.5,
                &a_d,
                m,
                &b_d,
                k,
                0.5,
                &mut c_d,
                m,
            )
            .unwrap();
        let c_d_gpu = c_d.to_host_vec(&stream).unwrap();
        for (g, s) in c_d_gpu.iter().zip(c_gpu.iter()) {
            assert!((g - f64::from(*s)).abs() < 1e-6);
        }
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cublasLt64_13.dll; run via xtask gpu-test"]
    fn gpu_cublaslt_matmul_and_budget_rejection() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        let lt = LtHandle::new(&ctx).expect("cublasLt handle");

        let (m, n, k) = (4u64, 4u64, 4u64);
        let a_host = vec![1.0f32; 16];
        let b_host = vec![2.0f32; 16];
        let c_host = vec![0.0f32; 16];

        let a_dev = cuda_core::DeviceBuffer::from_host(&stream, &a_host).unwrap();
        let b_dev = cuda_core::DeviceBuffer::from_host(&stream, &b_host).unwrap();
        let c_dev = cuda_core::DeviceBuffer::from_host(&stream, &c_host).unwrap();
        let mut d_dev = cuda_core::DeviceBuffer::zeroed(&stream, 16).unwrap();

        let desc = MatmulDesc::new(&ctx, ComputeType::F32, DataType::F32).unwrap();
        let a_layout = MatrixLayout::new(&ctx, DataType::F32, m, k, m as i64).unwrap();
        let b_layout = MatrixLayout::new(&ctx, DataType::F32, k, n, k as i64).unwrap();
        let c_layout = MatrixLayout::new(&ctx, DataType::F32, m, n, m as i64).unwrap();
        let d_layout = MatrixLayout::new(&ctx, DataType::F32, m, n, m as i64).unwrap();
        let mut pref = Preference::new(&ctx).unwrap();
        pref.set_max_workspace_bytes(1024 * 1024).unwrap();

        let heuristic = lt
            .matmul_heuristic(&desc, &a_layout, &b_layout, &c_layout, &d_layout, &pref)
            .unwrap();
        let mut scratch = Scratch::new(&stream);

        // Budget rejection when budget < workspace_size
        if heuristic.workspace_size() > 0 {
            let err = lt.matmul_f32(
                &stream,
                &desc,
                1.0,
                &a_dev,
                &a_layout,
                &b_dev,
                &b_layout,
                0.0,
                &c_dev,
                &c_layout,
                &mut d_dev,
                &d_layout,
                &heuristic,
                &mut scratch,
                0,
            );
            assert!(matches!(err, Err(Error::Budget { .. })));
        }

        lt.matmul_f32(
            &stream,
            &desc,
            1.0,
            &a_dev,
            &a_layout,
            &b_dev,
            &b_layout,
            0.0,
            &c_dev,
            &c_layout,
            &mut d_dev,
            &d_layout,
            &heuristic,
            &mut scratch,
            1024 * 1024,
        )
        .unwrap();
        let d_host = d_dev.to_host_vec(&stream).unwrap();
        for &val in &d_host {
            assert!((val - 8.0).abs() < 1e-4, "expected 8.0, got {val}");
        }
    }
}
