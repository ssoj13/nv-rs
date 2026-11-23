//! Safe cuRAND host and device API on top of `cuda-bindings` and `cuda-core`.
//!
//! **What.**
//! - [`generator`] — [`Generator`] (host and device), [`RngType`] (Philox, MTGP32, MT19937,
//!   Xorwow, Mrg32k3a, Sobol32/64), [`Ordering`], [`PoissonDistribution`], seeds, offsets,
//!   dimensions, stream binding, and uniform/normal/log-normal/Poisson/integer generation.
//! - [`error`] — [`CurandStatus`] (`Copy` integer newtype) and [`Error`].
//!
//! **Conventions.** Uses `cuda-bindings` directly (no `curand-sys` crate). Device generators hold
//! `Arc<Context>`; a `Drop` teardown failure is kept, with its real status, in the
//! `cuda_core_ext::teardown` sink (`Library::Curand`). Enqueueing
//! calls take `&Stream` and perform zero heap allocations, except for the completion lease that
//! every device `Generator::generate_*` takes on its output buffer.
//! **Used by** Monte Carlo, rendering, and simulation workloads, and the nv-rs T0/T3 test suite.

pub mod error;
pub mod generator;

pub use error::{CurandStatus, Error, Result};
pub use generator::{Generator, Ordering, PoissonDistribution, RngType};

pub use cuda_bindings as sys;

/// Returns the cuRAND library version reported by `curandGetVersion`.
pub fn version() -> Result<i32> {
    error::ensure_loaded()?;
    let mut v = 0i32;
    // SAFETY: `v` is a valid out-pointer.
    error::check("curandGetVersion", unsafe {
        cuda_bindings::curandGetVersion(&mut v)
    })?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{validate_normal_params, validate_poisson_lambda};

    #[test]
    fn curand_status_newtype_is_copy_and_preserves_unknown_codes() {
        let s = CurandStatus(7777);
        let copy = s;
        assert_eq!(s, copy);
        assert!(!s.is_success());
        assert!(CurandStatus::SUCCESS.is_success());
        assert_eq!(
            CurandStatus::LENGTH_NOT_MULTIPLE.name(),
            "CURAND_STATUS_LENGTH_NOT_MULTIPLE"
        );
        assert!(s.to_string().contains("7777"));
    }

    #[test]
    fn error_formatting_covers_all_variants() {
        let e1 = Error::Unavailable("missing curand64_10.dll".into());
        assert!(e1.to_string().contains("missing curand64_10.dll"));

        let e2 = Error::Curand {
            call: "curandGenerateUniform",
            code: CurandStatus::TYPE_ERROR,
        };
        let msg2 = e2.to_string();
        assert!(
            msg2.contains("curandGenerateUniform") && msg2.contains("CURAND_STATUS_TYPE_ERROR")
        );

        let e3 = Error::Invalid("bad dimensions");
        assert!(e3.to_string().contains("bad dimensions"));
    }

    #[test]
    fn rng_type_classification_and_parameter_validation() {
        assert!(RngType::Philox4_32_10.is_pseudo());
        assert!(RngType::Mtgp32.is_pseudo());
        assert!(RngType::Sobol32.is_quasi());
        assert!(RngType::ScrambledSobol64.is_quasi());

        // Pseudo-random normal requires even length and positive finite stddev
        assert!(validate_normal_params(RngType::Philox4_32_10, 1024, true, true).is_ok());
        assert!(matches!(
            validate_normal_params(RngType::Philox4_32_10, 1023, true, true),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            validate_normal_params(RngType::Philox4_32_10, 1024, false, true),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            validate_normal_params(RngType::Philox4_32_10, 1024, true, false),
            Err(Error::Invalid(_))
        ));
        // Quasi-random normal does not enforce even length
        assert!(validate_normal_params(RngType::Sobol32, 1023, true, true).is_ok());

        // Poisson lambda validation
        assert!(validate_poisson_lambda(3.5).is_ok());
        assert!(matches!(
            validate_poisson_lambda(0.0),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            validate_poisson_lambda(-1.0),
            Err(Error::Invalid(_))
        ));
        assert!(matches!(
            validate_poisson_lambda(f64::NAN),
            Err(Error::Invalid(_))
        ));
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + curand64_10.dll; run via xtask gpu-test"]
    fn gpu_philox_sobol_mtgp32_statistical_sanity_and_host_generator() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        assert!(version().expect("curand version") >= 10000);

        // Device Philox: uniform & normal statistical sanity
        let n = 16384usize;
        let gen_philox = Generator::new_device(&ctx, RngType::Philox4_32_10).unwrap();
        gen_philox.set_seed(12345).unwrap();
        gen_philox.set_offset(0).unwrap();
        // Dimensions rejected on pseudo-random
        assert!(matches!(
            gen_philox.set_dimensions(2),
            Err(Error::Invalid(_))
        ));

        let mut u_dev = cuda_core::DeviceBuffer::<f32>::zeroed(&stream, n).unwrap();
        gen_philox
            .generate_uniform_f32(&stream, &mut u_dev)
            .unwrap();
        let u_host = u_dev.to_host_vec(&stream).unwrap();
        let u_mean: f64 = u_host
            .iter()
            .map(|&x| {
                assert!(x > 0.0 && x <= 1.0, "uniform out of range: {x}");
                f64::from(x)
            })
            .sum::<f64>()
            / (n as f64);
        assert!(
            (u_mean - 0.5).abs() < 0.02,
            "uniform mean {u_mean} too far from 0.5"
        );

        let mut norm_dev = cuda_core::DeviceBuffer::<f32>::zeroed(&stream, n).unwrap();
        gen_philox
            .generate_normal_f32(&stream, &mut norm_dev, 0.0, 1.0)
            .unwrap();
        let norm_host = norm_dev.to_host_vec(&stream).unwrap();
        let n_mean: f64 = norm_host.iter().map(|&x| f64::from(x)).sum::<f64>() / (n as f64);
        let n_var: f64 = norm_host
            .iter()
            .map(|&x| (f64::from(x) - n_mean).powi(2))
            .sum::<f64>()
            / (n as f64);
        assert!(n_mean.abs() < 0.05, "normal mean {n_mean}");
        assert!((n_var - 1.0).abs() < 0.08, "normal variance {n_var}");

        // Device Sobol32: seed rejected, dimensions accepted, deterministic output
        let gen_sobol = Generator::new_device(&ctx, RngType::Sobol32).unwrap();
        assert!(matches!(gen_sobol.set_seed(1), Err(Error::Invalid(_))));
        gen_sobol.set_dimensions(4).unwrap();
        let mut s_dev = cuda_core::DeviceBuffer::<f32>::zeroed(&stream, 256).unwrap();
        gen_sobol.generate_uniform_f32(&stream, &mut s_dev).unwrap();
        let s1 = s_dev.to_host_vec(&stream).unwrap();
        gen_sobol.set_offset(0).unwrap();
        gen_sobol.generate_uniform_f32(&stream, &mut s_dev).unwrap();
        let s2 = s_dev.to_host_vec(&stream).unwrap();
        assert_eq!(s1, s2, "Sobol32 with reset offset must be deterministic");

        // Device MTGP32
        let gen_mtgp = Generator::new_device(&ctx, RngType::Mtgp32).unwrap();
        gen_mtgp.set_seed(42).unwrap();
        gen_mtgp.generate_uniform_f32(&stream, &mut s_dev).unwrap();

        // Host generator
        let host_gen = Generator::new_host(RngType::Philox4_32_10).unwrap();
        assert!(host_gen.is_host());
        host_gen.set_seed(99).unwrap();
        let mut host_buf = vec![0.0f64; 1024];
        host_gen.generate_host_uniform_f64(&mut host_buf).unwrap();
        let h_mean: f64 = host_buf.iter().sum::<f64>() / 1024.0;
        assert!((h_mean - 0.5).abs() < 0.05, "host uniform mean {h_mean}");
    }
}
