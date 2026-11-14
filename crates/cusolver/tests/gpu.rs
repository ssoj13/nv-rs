//! Numerical GPU tests (T3) for `cusolver`: run with
//! `cargo test -p cusolver -- --include-ignored --test-threads=1` on a CUDA GPU.

use cuda_core::{Context, DeviceBuffer};
use cusolver::{DnHandle, EigMode, FillMode, MatDescr, Op, Scratch, SpHandle, SvdJob};

/// Helper: column-major `n x n` matrix-vector multiply `y = A * x` on host.
fn matvec_col_major(a: &[f32], m: usize, n: usize, x: &[f32]) -> Vec<f32> {
    let mut y = vec![0.0f32; m];
    for c in 0..n {
        for r in 0..m {
            y[r] += a[c * m + r] * x[c];
        }
    }
    y
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusolver DLL and an NVIDIA GPU"]
fn dn_lu_getrf_getrs_solves_and_checks_residual() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let dn = DnHandle::new(&cuda).expect("DnHandle");
    let mut scratch = Scratch::new(&stream);

    // Column-major 3x3:
    // A = [[2, 1, 1], [4, 3, 3], [8, 7, 9]]
    let a_host = [2.0f32, 4.0, 8.0, 1.0, 3.0, 7.0, 1.0, 3.0, 9.0];
    // b = A * [1, 2, 3] = [7, 19, 49]
    let b_host = [7.0f32, 19.0, 49.0];

    let mut a = DeviceBuffer::from_host(&stream, &a_host).expect("a");
    let mut b = DeviceBuffer::from_host(&stream, &b_host).expect("b");
    let mut ipiv = DeviceBuffer::<i32>::zeroed(&stream, 3).expect("ipiv");
    let mut info = DeviceBuffer::<i32>::zeroed(&stream, 1).expect("info");

    dn.getrf(&stream, 3, 3, &mut a, 3, &mut ipiv, &mut info, &mut scratch, 64 * 1024 * 1024)
        .expect("getrf");
    dn.getrs(&stream, Op::NonTranspose, 3, 1, &a, 3, &ipiv, &mut b, 3, &mut info)
        .expect("getrs");
    stream.synchronize().expect("sync");

    assert_eq!(info.to_host_vec(&stream).expect("info")[0], 0);
    let x = b.to_host_vec(&stream).expect("x");
    let ax = matvec_col_major(&a_host, 3, 3, &x);
    let res = ax.iter().zip(b_host.iter()).map(|(u, v)| (u - v).abs()).fold(0.0f32, f32::max);
    assert!(res < 1e-4, "LU residual={res}, x={x:?}");
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusolver DLL and an NVIDIA GPU"]
fn dn_cholesky_potrf_potrs_solves_spd_system() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let dn = DnHandle::new(&cuda).expect("DnHandle");
    let mut scratch = Scratch::new(&stream);

    // SPD 3x3: A = [[4, 1, 0], [1, 4, 1], [0, 1, 4]]
    let a_host = [4.0f32, 1.0, 0.0, 1.0, 4.0, 1.0, 0.0, 1.0, 4.0];
    let b_host = [6.0f32, 12.0, 14.0]; // A * [1, 2, 3] = [6, 12, 14]

    let mut a = DeviceBuffer::from_host(&stream, &a_host).expect("a");
    let mut b = DeviceBuffer::from_host(&stream, &b_host).expect("b");
    let mut info = DeviceBuffer::<i32>::zeroed(&stream, 1).expect("info");

    dn.potrf(&stream, FillMode::Lower, 3, &mut a, 3, &mut info, &mut scratch, 64 * 1024 * 1024)
        .expect("potrf");
    dn.potrs(&stream, FillMode::Lower, 3, 1, &a, 3, &mut b, 3, &mut info)
        .expect("potrs");
    stream.synchronize().expect("sync");

    assert_eq!(info.to_host_vec(&stream).expect("info")[0], 0);
    let x = b.to_host_vec(&stream).expect("x");
    let ax = matvec_col_major(&a_host, 3, 3, &x);
    let res = ax.iter().zip(b_host.iter()).map(|(u, v)| (u - v).abs()).fold(0.0f32, f32::max);
    assert!(res < 1e-5, "Cholesky residual={res}, x={x:?}");
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusolver DLL and an NVIDIA GPU"]
fn dn_qr_geqrf_orgqr_checks_orthogonality() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let dn = DnHandle::new(&cuda).expect("DnHandle");
    let mut scratch = Scratch::new(&stream);

    let a_host = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0]; // 3x2 col-major
    let mut a = DeviceBuffer::from_host(&stream, &a_host).expect("a");
    let mut tau = DeviceBuffer::<f32>::zeroed(&stream, 2).expect("tau");
    let mut info = DeviceBuffer::<i32>::zeroed(&stream, 1).expect("info");

    dn.geqrf(&stream, 3, 2, &mut a, 3, &mut tau, &mut info, &mut scratch, 64 * 1024 * 1024)
        .expect("geqrf");
    dn.orgqr(&stream, 3, 2, 2, &mut a, 3, &tau, &mut info, &mut scratch, 64 * 1024 * 1024)
        .expect("orgqr");
    stream.synchronize().expect("sync");

    assert_eq!(info.to_host_vec(&stream).expect("info")[0], 0);
    let q = a.to_host_vec(&stream).expect("q");
    // Check Q^T * Q == I_2
    let mut max_err = 0.0f32;
    for i in 0..2 {
        for j in 0..2 {
            let dot: f32 = (0..3).map(|r| q[i * 3 + r] * q[j * 3 + r]).sum();
            let target = if i == j { 1.0 } else { 0.0 };
            max_err = max_err.max((dot - target).abs());
        }
    }
    assert!(max_err < 1e-5, "Q orthogonality error={max_err}");
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusolver DLL and an NVIDIA GPU"]
fn dn_svd_gesvd_reconstructs_and_checks_orthogonality() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let dn = DnHandle::new(&cuda).expect("DnHandle");
    let mut scratch = Scratch::new(&stream);

    let m = 3usize;
    let n = 2usize;
    let a_host = [1.0f32, 0.0, 2.0, 0.0, 3.0, 1.0];
    let mut a = DeviceBuffer::from_host(&stream, &a_host).expect("a");
    let mut s = DeviceBuffer::<f32>::zeroed(&stream, n).expect("s");
    let mut u = DeviceBuffer::<f32>::zeroed(&stream, m * m).expect("u");
    let mut vt = DeviceBuffer::<f32>::zeroed(&stream, n * n).expect("vt");
    let mut info = DeviceBuffer::<i32>::zeroed(&stream, 1).expect("info");

    dn.gesvd(
        &stream,
        SvdJob::All,
        SvdJob::All,
        m as i32,
        n as i32,
        &mut a,
        m as i32,
        &mut s,
        Some((&mut u, m as i32)),
        Some((&mut vt, n as i32)),
        &mut info,
        &mut scratch,
        64 * 1024 * 1024,
    )
    .expect("gesvd");
    stream.synchronize().expect("sync");

    assert_eq!(info.to_host_vec(&stream).expect("info")[0], 0);
    let s_h = s.to_host_vec(&stream).expect("s");
    let u_h = u.to_host_vec(&stream).expect("u");
    let vt_h = vt.to_host_vec(&stream).expect("vt");

    let mut max_err = 0.0f32;
    for c in 0..n {
        for r in 0..m {
            let mut sum = 0.0f32;
            for k in 0..n {
                sum += u_h[k * m + r] * s_h[k] * vt_h[c * n + k];
            }
            max_err = max_err.max((sum - a_host[c * m + r]).abs());
        }
    }
    assert!(max_err < 1e-5, "SVD reconstruction error={max_err}");
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusolver DLL and an NVIDIA GPU"]
fn dn_syevd_symmetric_eigen_residual() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let dn = DnHandle::new(&cuda).expect("DnHandle");
    let mut scratch = Scratch::new(&stream);

    // Symmetric 3x3
    let a_host = [4.0f32, 1.0, 0.0, 1.0, 3.0, 1.0, 0.0, 1.0, 2.0];
    let mut a = DeviceBuffer::from_host(&stream, &a_host).expect("a");
    let mut w = DeviceBuffer::<f32>::zeroed(&stream, 3).expect("w");
    let mut info = DeviceBuffer::<i32>::zeroed(&stream, 1).expect("info");

    dn.syevd(
        &stream,
        EigMode::Vector,
        FillMode::Lower,
        3,
        &mut a,
        3,
        &mut w,
        &mut info,
        &mut scratch,
        64 * 1024 * 1024,
    )
    .expect("syevd");
    stream.synchronize().expect("sync");

    assert_eq!(info.to_host_vec(&stream).expect("info")[0], 0);
    let v_h = a.to_host_vec(&stream).expect("v");
    let w_h = w.to_host_vec(&stream).expect("w");
    let mut max_err = 0.0f32;
    for k in 0..3 {
        let vk = &v_h[k * 3..(k + 1) * 3];
        let av = matvec_col_major(&a_host, 3, 3, vk);
        for r in 0..3 {
            max_err = max_err.max((av[r] - w_h[k] * vk[r]).abs());
        }
    }
    assert!(max_err < 1e-5, "SYEVD residual={max_err}");
}

#[test]
#[ignore = "GPU: requires CUDA toolkit cusolver DLL and an NVIDIA GPU"]
fn sp_csrlsvqr_and_csreigvsi_numerical_checks() {
    let cuda = Context::new(0).expect("CUDA device 0");
    let stream = cuda.default_stream();
    let sp = SpHandle::new(&cuda).expect("SpHandle");
    let descr = MatDescr::new(&cuda).expect("MatDescr");

    // 3x3 SPD CSR: [[4, 1, 0], [1, 4, 1], [0, 1, 4]]
    let csr_val = DeviceBuffer::from_host(&stream, &[4.0f32, 1.0, 1.0, 4.0, 1.0, 1.0, 4.0]).expect("csr_val");
    let csr_row = DeviceBuffer::from_host(&stream, &[0i32, 2, 5, 7]).expect("csr_row");
    let csr_col = DeviceBuffer::from_host(&stream, &[0i32, 1, 0, 1, 2, 1, 2]).expect("csr_col");
    let b = DeviceBuffer::from_host(&stream, &[6.0f32, 12.0, 14.0]).expect("b");
    let mut x = DeviceBuffer::<f32>::zeroed(&stream, 3).expect("x");

    let singularity = sp
        .csrlsvqr(&stream, 3, 7, &descr, &csr_val, &csr_row, &csr_col, &b, 1e-6, 0, &mut x)
        .expect("csrlsvqr");
    stream.synchronize().expect("sync");
    assert_eq!(singularity, -1);

    let x_h = x.to_host_vec(&stream).expect("read x");
    let expected = [1.0f32, 2.0, 3.0];
    let max_err = x_h.iter().zip(expected.iter()).map(|(u, v)| (u - v).abs()).fold(0.0f32, f32::max);
    assert!(max_err < 1e-4, "csrlsvqr x={x_h:?}, max_err={max_err}");

    // Shift `mu0` away from exact eigenvalue 4.0 (so `A - mu0*I` is nonsingular) and use
    // `x0` non-orthogonal to the target eigenvector `[1, 0, -1]`.
    let x0 = DeviceBuffer::from_host(&stream, &[1.0f32, 0.5, -0.5]).expect("x0");
    let mut eig_vec = DeviceBuffer::<f32>::zeroed(&stream, 3).expect("eig_vec");
    let mu = sp
        .csreigvsi(&stream, 3, 7, &descr, &csr_val, &csr_row, &csr_col, 3.9, &x0, 50, 1e-5, &mut eig_vec)
        .expect("csreigvsi");
    stream.synchronize().expect("sync");
    assert!((mu - 4.0).abs() < 1e-3, "expected eigenvalue near 4.0, got {mu}");
}
