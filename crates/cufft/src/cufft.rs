//! Safe cuFFT host API on top of `cufft-sys` and `cuda-core`.
//!
//! **What.**
//! - [`plan`] — [`Plan`] (`plan_1d`, `plan_2d`, `plan_3d`, `plan_many`, `estimate_*`),
//!   [`ManyDesc`], [`FftType`], [`Direction`], and typed execution (`exec_c2c`, `exec_r2c`,
//!   `exec_c2r`, `exec_z2z`, `exec_d2z`, `exec_z2d`).
//! - [`scratch`] — [`Scratch`] caller-owned reusable device workspace with byte-budget checks.
//! - [`error`] — [`FftResult`] (`Copy` integer newtype) and [`Error`].
//!
//! **Conventions.** Every [`Plan`] disables cuFFT auto-allocation (`cufftSetAutoAllocation(0)`),
//! sizes its workspace against a caller byte budget, and destroys its handle on `Drop` (recording
//! any teardown failure on the owning `cuda_core::Context`). Enqueueing calls take `&Stream` and
//! perform zero heap allocations.
//! **Used by** signal/image processing workloads and the nv-rs T0/T3 test suite.

pub mod error;
pub mod plan;
pub mod scratch;

pub use error::{Error, FftResult, Result};
pub use plan::{Complex32, Complex64, Direction, FftType, ManyDesc, Plan};
pub use scratch::Scratch;

pub use cufft_sys as sys;

/// Returns the cuFFT library version reported by `cufftGetVersion`.
pub fn version() -> Result<i32> {
    sys::load().map_err(|e| Error::Unavailable(e.to_string()))?;
    let mut v = 0i32;
    // SAFETY: `v` is a valid out-pointer.
    error::check("cufftGetVersion", unsafe { sys::cufftGetVersion(&mut v) })?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::dense_io_elems;

    #[test]
    fn fft_result_newtype_is_copy_and_formats_unknown_codes() {
        let r = FftResult(4242);
        let copy = r;
        assert_eq!(r, copy);
        assert!(!r.is_success());
        assert!(FftResult::SUCCESS.is_success());
        assert_eq!(FftResult::INVALID_PLAN.name(), "CUFFT_INVALID_PLAN");
        assert!(r.to_string().contains("4242"));
    }

    #[test]
    fn error_formatting_covers_all_variants() {
        let e1 = Error::Unavailable("missing cufft64_12.dll".into());
        assert!(e1.to_string().contains("missing cufft64_12.dll"));

        let e2 = Error::Cufft {
            call: "cufftExecC2C",
            code: FftResult::EXEC_FAILED,
        };
        let msg2 = e2.to_string();
        assert!(msg2.contains("cufftExecC2C") && msg2.contains("CUFFT_EXEC_FAILED"));

        let e3 = Error::Budget {
            requested: 8192,
            available: 512,
        };
        assert!(e3.to_string().contains("8192") && e3.to_string().contains("512"));

        let e4 = Error::Invalid("bad rank");
        assert!(e4.to_string().contains("bad rank"));
    }

    #[test]
    fn many_desc_and_dense_io_validation() {
        let valid = ManyDesc {
            rank: 2,
            n: [16, 32, 0],
            inembed: None,
            istride: 1,
            idist: 512,
            onembed: None,
            ostride: 1,
            odist: 512,
            fft_type: FftType::C2C,
            batch: 4,
        };
        assert!(valid.validate().is_ok());

        let bad_rank = ManyDesc { rank: 4, ..valid };
        assert!(matches!(bad_rank.validate(), Err(Error::Invalid(_))));

        let bad_dim = ManyDesc {
            n: [16, 0, 0],
            ..valid
        };
        assert!(matches!(bad_dim.validate(), Err(Error::Invalid(_))));

        let bad_stride = ManyDesc {
            istride: 0,
            ..valid
        };
        assert!(matches!(bad_stride.validate(), Err(Error::Invalid(_))));

        let bad_dist = ManyDesc { idist: 0, ..valid };
        assert!(matches!(bad_dist.validate(), Err(Error::Invalid(_))));

        assert_eq!(dense_io_elems(&[16], 2, FftType::R2C).unwrap(), (32, 18));
        assert_eq!(
            dense_io_elems(&[8, 16], 1, FftType::D2Z).unwrap(),
            (128, 72)
        );
        assert_eq!(
            dense_io_elems(&[8, 16], 1, FftType::C2R).unwrap(),
            (72, 128)
        );
        assert!(matches!(
            dense_io_elems(&[0], 1, FftType::C2C),
            Err(Error::Invalid(_))
        ));

        // embed[0] is ignored by cuFFT, so rank=1 n=[1024] with embed=[1] still requires 1024 elems
        let rank1_small_embed0 = ManyDesc {
            rank: 1,
            n: [1024, 1, 1],
            inembed: Some([1, 1, 1]),
            istride: 1,
            idist: 1024,
            onembed: Some([1, 1, 1]),
            ostride: 1,
            odist: 1024,
            fft_type: FftType::C2C,
            batch: 1,
        };
        assert_eq!(rank1_small_embed0.min_io_elems().unwrap(), (1024, 1024));

        // inner embed[1] < n[1] must be rejected (onembed is valid, so only inembed[1] is at fault)
        let bad_inner_embed = ManyDesc {
            inembed: Some([16, 16, 1]),
            onembed: Some([16, 32, 1]),
            ..valid
        };
        assert!(matches!(bad_inner_embed.validate(), Err(Error::Invalid(_))));
    }

    #[test]
    #[ignore = "GPU: needs CUDA driver + cufft64_12.dll; run via xtask gpu-test"]
    fn gpu_1d_and_2d_fft_roundtrip_and_parseval() {
        let ctx = cuda_core::Context::new(0).expect("cuda context");
        let stream = ctx.new_stream().expect("stream");
        assert!(version().expect("cufft version") >= 11000);

        let n = 64usize;
        let mut scratch = Scratch::new(&stream);

        // Budget rejection: a large prime length takes cuFFT's Bluestein path, which needs a work
        // area, so a zero budget must be refused (a power-of-two length may need none and would
        // skip the check).
        let prime_len = 1_000_003;
        let est = Plan::estimate_1d(prime_len, FftType::C2C, 1).unwrap();
        assert!(
            est > 0,
            "cuFFT estimates no work area for length {prime_len}; budget path not exercised"
        );
        let err = Plan::plan_1d(&ctx, prime_len, FftType::C2C, 1, 0, &mut scratch);
        assert!(matches!(err, Err(Error::Budget { .. })), "{err:?}");

        let plan = Plan::plan_1d(
            &ctx,
            n as i32,
            FftType::C2C,
            1,
            16 * 1024 * 1024,
            &mut scratch,
        )
        .unwrap();
        let input_host: Vec<Complex32> = (0..n)
            .map(|i| Complex32::new((i as f32 * 0.1).sin(), (i as f32 * 0.2).cos()))
            .collect();
        let time_energy: f64 = input_host
            .iter()
            .map(|c| f64::from(c.re()).powi(2) + f64::from(c.im()).powi(2))
            .sum();

        let in_dev = cuda_core::DeviceBuffer::from_host(&stream, &input_host).unwrap();
        let mut freq_dev = cuda_core::DeviceBuffer::<Complex32>::zeroed(&stream, n).unwrap();
        let mut back_dev = cuda_core::DeviceBuffer::<Complex32>::zeroed(&stream, n).unwrap();

        plan.exec_c2c(&stream, &in_dev, &mut freq_dev, Direction::Forward)
            .unwrap();
        let freq_host = freq_dev.to_host_vec(&stream).unwrap();
        let freq_energy: f64 = freq_host
            .iter()
            .map(|c| f64::from(c.re()).powi(2) + f64::from(c.im()).powi(2))
            .sum::<f64>()
            / (n as f64);
        assert!(
            (time_energy - freq_energy).abs() < 1e-3,
            "Parseval mismatch: {time_energy} vs {freq_energy}"
        );

        plan.exec_c2c(&stream, &freq_dev, &mut back_dev, Direction::Inverse)
            .unwrap();
        let back_host = back_dev.to_host_vec(&stream).unwrap();
        for (orig, rec) in input_host.iter().zip(back_host.iter()) {
            assert!((orig.re() - rec.re() / (n as f32)).abs() < 1e-4);
            assert!((orig.im() - rec.im() / (n as f32)).abs() < 1e-4);
        }

        // 2D R2C / C2R roundtrip
        let (nx, ny) = (16usize, 16usize);
        let plan_fwd = Plan::plan_2d(
            &ctx,
            nx as i32,
            ny as i32,
            FftType::R2C,
            16 * 1024 * 1024,
            &mut scratch,
        )
        .unwrap();
        let plan_inv = Plan::plan_2d(
            &ctx,
            nx as i32,
            ny as i32,
            FftType::C2R,
            16 * 1024 * 1024,
            &mut scratch,
        )
        .unwrap();
        let real_host: Vec<f32> = (0..(nx * ny)).map(|i| ((i % 7) as f32) * 0.25).collect();
        let real_dev = cuda_core::DeviceBuffer::from_host(&stream, &real_host).unwrap();
        let mut spec_dev =
            cuda_core::DeviceBuffer::<Complex32>::zeroed(&stream, nx * (ny / 2 + 1)).unwrap();
        let mut rec_dev = cuda_core::DeviceBuffer::<f32>::zeroed(&stream, nx * ny).unwrap();

        plan_fwd
            .exec_r2c(&stream, &real_dev, &mut spec_dev)
            .unwrap();
        plan_inv
            .exec_c2r(&stream, &mut spec_dev, &mut rec_dev)
            .unwrap();
        let rec_host = rec_dev.to_host_vec(&stream).unwrap();
        let scale = (nx * ny) as f32;
        for (orig, rec) in real_host.iter().zip(rec_host.iter()) {
            assert!((orig - rec / scale).abs() < 1e-4);
        }
    }
}
