//! Safe cuBLAS and cuBLASLt host API on top of `cublas-sys`, `cublaslt-sys`, and `cuda-core`.
//!
//! **What.**
//! - [`handle`] — [`Handle`], [`Operation`], [`PointerMode`], [`MathMode`], [`DataType`],
//!   [`ComputeType`], and [`GemmAlgo`].
//! - [`gemm`] — `sgemm`, `dgemm`, `sgemm_batched`, `dgemm_batched`, `sgemm_strided_batched`,
//!   `dgemm_strided_batched`, and `gemm_ex` / `gemm_strided_batched_ex`.
//! - [`lt`] — [`LtHandle`], [`MatmulDesc`], [`MatrixLayout`], [`Preference`], [`HeuristicResult`],
//!   [`Order`], and [`Epilogue`].
//! - [`Scratch`] — caller-owned reusable device workspace with byte-budget checks, re-exported
//!   from `cuda_core_ext` (the one nv-rs workspace implementation).
//! - [`error`] — [`Status`] (`Copy` integer newtype) and [`Error`].
//!
//! **Conventions.** Every handle owns its native cuBLAS/cuBLASLt object, holds `Arc<Context>`, and
//! destroys it on `Drop`; a failed destroy is kept, with its real status, in the
//! `cuda_core_ext::teardown` sink (`Library::Cublas` / `Library::CublasLt`). cuBLASLt descriptors
//! (`MatmulDesc`, `MatrixLayout`, `Preference`, `MatrixTransformDesc`, `EmulationDesc`) are host
//! objects and take no context. Enqueueing calls take `&Stream` and perform zero heap allocations.
//! **Used by** dense linear algebra workloads and the nv-rs T0/T3 test suite.

pub mod error;
pub mod gemm;
pub mod handle;
pub mod lt;

pub use error::{Error, Result, Status};
pub use handle::{ComputeType, DataType, GemmAlgo, Handle, MathMode, Operation, PointerMode};
pub use lt::{
    AlgoCapAttr, AlgoConfigAttr, BatchMode, EmulationDesc, EmulationDescAttr, Epilogue,
    HeuristicResult, LibraryProperty, LoggerCallback, LtHandle, MatmulAlgo, MatmulDesc,
    MatmulDescAttr, MatrixLayout, MatrixLayoutAttr, MatrixTransformDesc, MatrixTransformDescAttr,
    Order, Preference, PreferenceAttr, ReductionScheme, SearchMode,
};
pub use cuda_core_ext::Scratch;

pub use cublas_sys as sys;
pub use cublaslt_sys as lt_sys;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gemm::{
        require_f32_scale, validate_gemm_dims, validate_output_stride, validate_strided_batch,
    };

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
    fn output_stride_rejects_overlapping_batches() {
        // m = 3, n = 4, ldc = 3: each C matrix spans 12 elements.
        assert!(validate_output_stride(3, 4, 3, 12, 8).is_ok());
        assert!(matches!(
            validate_output_stride(3, 4, 3, 0, 8),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            validate_output_stride(3, 4, 3, 11, 2),
            Err(Error::Invalid(_))
        ));
        // A single batch or an empty matrix cannot overlap.
        assert!(validate_output_stride(3, 4, 3, 0, 1).is_ok());
        assert!(validate_output_stride(0, 4, 3, 0, 8).is_ok());
        // Row-interleaved batches inside one column pitch: m = 2, ldc = 4, stride 2.
        assert!(validate_output_stride(2, 3, 4, 2, 2).is_ok());
        assert!(validate_output_stride(2, 3, 4, 2, 3).is_err());
        assert!(validate_output_stride(2, 3, 4, 1, 2).is_err());
        assert!(validate_output_stride(3, 4, 3, -1, 2).is_err());
    }

    #[test]
    fn grouped_gemm_requires_f32_scale_compute_type() {
        for ct in [
            ComputeType::F32,
            ComputeType::F32Pedantic,
            ComputeType::F32Fast16F,
            ComputeType::F32Fast16BF,
            ComputeType::F32FastTf32,
        ] {
            assert!(require_f32_scale(ct).is_ok(), "{ct:?}");
        }
        for ct in [
            ComputeType::F16,
            ComputeType::F16Pedantic,
            ComputeType::F64,
            ComputeType::F64Pedantic,
            ComputeType::I32,
            ComputeType::I32Pedantic,
        ] {
            assert!(
                matches!(require_f32_scale(ct), Err(Error::Invalid(_))),
                "{ct:?}"
            );
        }
    }

    /// All-ones 2x2 SGEMM on `stream`, synchronized, used by the workspace/stream GPU tests.
    fn sgemm_2x2(handle: &Handle, stream: &cuda_core::Stream) -> Result<()> {
        let a = cuda_core::DeviceBuffer::from_host(stream, &[1.0f32; 4]).expect("a");
        let b = cuda_core::DeviceBuffer::from_host(stream, &[1.0f32; 4]).expect("b");
        let mut c = cuda_core::DeviceBuffer::from_host(stream, &[0.0f32; 4]).expect("c");
        let res = handle.sgemm(
            stream,
            Operation::N,
            Operation::N,
            2,
            2,
            2,
            1.0,
            &a,
            2,
            &b,
            2,
            0.0,
            &mut c,
            2,
        );
        stream.synchronize().expect("sync");
        res
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cublas64_13.dll; run via xtask gpu-test"]
    fn gpu_set_stream_binds_context_on_cache_hit() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        let handle = Handle::new(&ctx).expect("cublas handle");
        handle.set_stream(&stream).expect("first bind");
        // SAFETY: binding null only detaches the calling thread's current context.
        let rc = unsafe { cuda_core::sys::cuCtxSetCurrent(std::ptr::null_mut()) };
        assert_eq!(rc, cuda_core::sys::cudaError_enum_CUDA_SUCCESS);
        // Same stream: the cuBLAS stream cache hits, but the context must still be made current.
        handle.set_stream(&stream).expect("cache-hit bind");
        let mut cur: cuda_core::sys::CUcontext = std::ptr::null_mut();
        // SAFETY: `cur` is a valid out-pointer for one `CUcontext`.
        let rc = unsafe { cuda_core::sys::cuCtxGetCurrent(&mut cur) };
        assert_eq!(rc, cuda_core::sys::cudaError_enum_CUDA_SUCCESS);
        assert_eq!(cur, ctx.cu_ctx());
        sgemm_2x2(&handle, &stream).expect("sgemm after cache-hit bind");
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cublas64_13.dll; run via xtask gpu-test"]
    fn gpu_handle_retains_workspace_after_scratch_drop() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        let handle = Handle::new(&ctx).expect("cublas handle");
        let mut scratch = Scratch::new(&stream);
        handle
            .set_workspace(&mut scratch, 1 << 20, 1 << 20)
            .expect("set workspace");
        let ws_ptr = scratch.shared_buffer().expect("workspace").cu_deviceptr();
        drop(scratch);
        stream.synchronize().expect("sync");
        // Without retention the stream-ordered pool hands the freed workspace straight back.
        // SAFETY: the new buffer is only compared by address, never read.
        let next = unsafe { cuda_core::DeviceBuffer::<u8>::uninitialized_async(&stream, 1 << 20) }
            .expect("next allocation");
        assert_ne!(
            next.cu_deviceptr(),
            ws_ptr,
            "workspace freed while the cuBLAS handle still points at it"
        );
        sgemm_2x2(&handle, &stream).expect("sgemm on retained workspace");
        handle.clear_workspace().expect("clear workspace");
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cublas64_13.dll; run via xtask gpu-test"]
    fn gpu_workspace_pins_handle_to_scratch_stream() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream_a = ctx.new_stream().expect("stream a");
        let stream_b = ctx.new_stream().expect("stream b");
        let handle = Handle::new(&ctx).expect("cublas handle");
        let mut scratch = Scratch::new(&stream_a);
        handle
            .set_workspace(&mut scratch, 1 << 20, 1 << 20)
            .expect("set workspace");
        // cublasSetStream would silently drop the budgeted workspace: switching must fail.
        assert!(matches!(
            sgemm_2x2(&handle, &stream_b),
            Err(Error::Invalid(_))
        ));
        sgemm_2x2(&handle, &stream_a).expect("sgemm on workspace stream");
        // clear_workspace restores the default pool and releases the pin.
        handle.clear_workspace().expect("clear workspace");
        sgemm_2x2(&handle, &stream_b).expect("sgemm on other stream after clear");
        handle.clear_workspace().expect("clear without workspace is a no-op");
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

        let desc = MatmulDesc::new(ComputeType::F32, DataType::F32).unwrap();
        let a_layout = MatrixLayout::new(DataType::F32, m, k, m as i64).unwrap();
        let b_layout = MatrixLayout::new(DataType::F32, k, n, k as i64).unwrap();
        let c_layout = MatrixLayout::new(DataType::F32, m, n, m as i64).unwrap();
        let d_layout = MatrixLayout::new(DataType::F32, m, n, m as i64).unwrap();
        let mut pref = Preference::new().unwrap();
        pref.set_max_workspace_bytes(1024 * 1024).unwrap();

        let heuristic = lt
            .matmul_heuristic(&desc, &a_layout, &b_layout, &c_layout, &d_layout, &pref)
            .unwrap();
        let mut scratch = Scratch::new(&stream);

        // Budget rejection must not depend on whether this tiny problem needs workspace: request
        // 4 KiB for the same algo with a zero budget, which `Scratch::reserve` rejects up front.
        let needs_ws = HeuristicResult::from_algo(heuristic.algo(), 4096);
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
            &needs_ws,
            &mut scratch,
            0,
        );
        assert!(matches!(err, Err(Error::Budget { .. })));

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
